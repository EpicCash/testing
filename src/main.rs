use testing::commands::{
    confirm_transaction, new_child, send_coins_smallest, spawn_miner, spawn_network,
    spawn_wallet_listen,
};
//use std::fmt::Display;
use rand::{self, distributions::Uniform, Rng};
use std::io::prelude::*;
use std::sync::Arc;
use std::{fmt, fs::File, thread, time};
use testing::utils::{get_http_wallet, get_test_configuration, wait_for};

use std::process::Child;
use std::process::Command;
use std::time::Duration;

use dotenv::dotenv;
use std::env;

// Epic Server
use epic_core::global::ChainTypes;

impl std::default::Default for BigWalletWorld {
    fn default() -> BigWalletWorld {
        BigWalletWorld {
            chain_type: ChainTypes::UserTesting,
            send_method: String::from("http"),
            http_path: String::new(),
            password: String::from("1"),
            server_binary: String::new(),
            wallet_binary: String::new(),
            miner_binary: String::new(),
        }
    }
}

impl std::default::Default for ChildProcess {
    fn default() -> ChildProcess {
        ChildProcess {
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
        }
    }
}

impl fmt::Display for PackTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "number_transactions: {:?}\nduration_time: {:?}\nvec_amount: {:?})",
            self.number_transactions, self.duration_time, self.vec_amount
        )
    }
}

#[derive(Debug, Clone)]
pub struct BigWalletWorld {
    pub chain_type: ChainTypes,
    pub send_method: String,
    pub http_path: String,
    pub password: String,
    pub server_binary: String,
    pub wallet_binary: String,
    pub miner_binary: String,
}

#[derive(Debug)]
pub struct ChildProcess {
    pub server: Child,
    pub wallet: Child,
    pub miner: Child,
}

#[derive(Debug)]
pub struct PackTransaction {
    pub number_transactions: u32,
    pub duration_time: Vec<Duration>,
    pub vec_amount: Vec<String>,
}

fn generate_vec_to_sent(min_include: i32, max_exclude: i32, number_elements: i32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min_include, max_exclude); // [min, max)

    let vals: Vec<String> = (0..number_elements)
        .map(|_| format!("0.{}", rng.sample(&range).to_string()))
        .collect();
    vals
}

fn save_transaction(pack: PackTransaction, name_file: String) {
    let mut file =
        File::create(format!("{}.txt", name_file)).expect("Failed on create a transaction file");
    let text = format!("{}", pack);
    file.write_all(text.as_bytes())
        .expect("Failed on write the transaction file");
}

fn save_data(pos_name: String) {
    let wallet_name = format!("/home/jualns/.epic/user/wallet_data_{}", pos_name);
    let chain_name = format!("/home/jualns/.epic/user/chain_data_{}", pos_name);

    let _chain_cop = Command::new("cp")
        .args(["-r", "/home/jualns/.epic/user/chain_data", &chain_name])
        .output()
        .expect("Failed on copy chain_data");

    let wallet_cop = Command::new("cp")
        .args(["-r", "/home/jualns/.epic/user/wallet_data", &wallet_name])
        .output()
        .expect("Failed on copy wallet_data");
    println!("WALL {:?} -- name {:?}", wallet_cop, wallet_name);
}

//#[tokio::main]
fn main() {
    dotenv().ok();

    let method_send = String::from("http");
    let method_to_send = Arc::new(method_send);

    //// Init the variables
    let password = Arc::new(String::from("1"));
    let chain_type = Arc::new(ChainTypes::UserTesting);
    let server_binary = Arc::new(env::var("EPIC_SERVER").unwrap());
    let wallet_binary = Arc::new(env::var("EPIC_WALLET").unwrap());
    let miner_binary = Arc::new(env::var("EPIC_MINER").unwrap());
    let http_path = Arc::new(get_http_wallet(&ChainTypes::UserTesting));

    //// Init the systems
    let mut childrens = ChildProcess::default();

    // config epic-server.toml with custom configuration
    get_test_configuration(&chain_type);
    // Wait the epic-servet.toml save
    wait_for(5);
    // run server and wallet, and save on world
    childrens.server = spawn_network(&chain_type, server_binary.as_str(), Some("--onlyrandomx"));
    // save the wallet_listen process on world
    childrens.wallet = spawn_wallet_listen(&chain_type, wallet_binary.as_str(), password.as_str());
    // Run the miner
    childrens.miner = spawn_miner(&miner_binary);

    // wait for 30 secs to miner start
    wait_for(30);

    let mut handles_vec = Vec::new();

    // Multi-thread but with only 1 thread, because wallet can't build multiple transactions in the same time (yet)
    for _ in 0..1 {
        //number of threads
        let method_to_sent = Arc::clone(&method_to_send);
        let pass = Arc::clone(&password);
        let chain_t = Arc::clone(&chain_type);
        let wallet_bin = Arc::clone(&wallet_binary);
        let http_pa = Arc::clone(&http_path);

        let handle = thread::spawn(move || {
            // number of transactions
            let number_transactions_total: u32 = 20;
            let mut pack_transactions = PackTransaction {
                number_transactions: number_transactions_total + 1, // +1 because we lost the fisrt transaction (logical error on code)
                duration_time: Vec::new(),                          //vec![now.elapsed(); 1],
                vec_amount: generate_vec_to_sent(0, 1000, number_transactions_total as i32 + 1), //Vec::new(), //vec!["1.0".to_string()],
            };

            // lost the first transaction
            let _ = pack_transactions
                .vec_amount
                .first()
                .expect("Can't have amount to send")
                .to_string();
            // step to code save the chain_data and wallet_data
            let k_param = 100;
            for t_k in 0..pack_transactions.number_transactions as usize {
                let amount = pack_transactions.vec_amount[t_k].to_string();

                let now = time::Instant::now();
                let out = send_coins_smallest(
                    &chain_t,
                    &wallet_bin,
                    method_to_sent.to_string(),
                    &pass,
                    amount,
                    &http_pa,
                );

                let t_elapsed = now.elapsed();

                pack_transactions.duration_time.push(t_elapsed);
                println!(
                    "\n-- Transaction number {:?} -- \n		Amount is: {:?}\n		Time elapsed: {:?}",
                    t_k, pack_transactions.vec_amount[t_k], t_elapsed
                );
                println!(
                    "      Output of sent: {:?}",
                    String::from_utf8_lossy(&out.stdout)
                );

                // Save step
                // float of step (to divide)
                let tt_kk = t_k as f32;
                // if transaction/step is Int => step done => save
                let div = tt_kk / k_param as f32;
                if div == div.floor() {
                    save_data(div.to_string());
                    println!("SAVED WALLET AND CHAIN DATA!")
                }
            }
            save_transaction(
                pack_transactions,
                "transactions_test_single_thread".to_string(),
            );
        });

        handles_vec.push(handle);
    }

    // join the handles in the vector
    for i in handles_vec {
        i.join().expect("Can't wait for the thread finish");
    }

    //// Confirm transactions all last
    confirm_transaction(&chain_type, &wallet_binary, &password);

    //// Finish all systems
    childrens.miner.kill().expect("Can't kill miner!");
    childrens.server.kill().expect("Can't kill server!");
    childrens.wallet.kill().expect("Can't kill wallet!");
    println!("\n\n---FINISH!---\n\n")
}
