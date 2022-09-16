//use std::fmt::Display;
use std::{thread, time, fmt, fs::File};
use rand::{self, distributions::Uniform, Rng};
use std::sync::Arc;
use std::io::prelude::*;

use std::time::Duration;
//use std::sync::mpsc;
use std::process::Child;
//use std::time::Instant;
use std::process::{Command, Output};

//Testing
use testing::{
            wait_for,
            get_test_configuration,
            spawn_network,
            create_wallet,
            spawn_miner, spawn_wallet_listen,
            send_coins_smallest,
            confirm_transaction,
            info_wallet,
            new_child,
            get_http_wallet,
            };

// Epic Server
use epic_core::global::ChainTypes;

//Epic Wallet
//use epic_wallet_config::config::initial_setup_wallet;

//impl fmt::Debug for TransWorld {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.wallet_binary)
//    }
//}

//impl fmt::Debug for WalletInformation {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.sent_tx)
//    }
//}

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

// impl std::clone::Clone for BigWalletWorld {
//     fn clone(&self) -> Self {
//         self.chain_type.clone()
//     }
// }

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
        write!(f, "number_transactions: {:?}\nduration_time: {:?}\nvec_amount: {:?})", self.number_transactions, self.duration_time, self.vec_amount)
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

//I finalize the emoji transaction



////////////////////////////////////////////////
/// 
/// 
/// 
/// 
/// 

fn have_coins_in_wallet(_chain_type: &ChainTypes, _wallet_binary: &String, _password: &String, _multiple: &f32) -> bool {
    // let comparative_value: &f32 = &15.0;
    // let info = info_wallet(chain_type, wallet_binary, password);
    // let current_spendable = info.last().expect("Can't get the current spendable!");
    // current_spendable > &(comparative_value*multiple)
    wait_for(10);
    true
}

fn generate_vec_to_sent(min_include: i32, max_exclude: i32, number_elements: i32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min_include, max_exclude); // [min, max)

    let vals: Vec<String> = (0..number_elements).map(|_| format!("0.{}", rng.sample(&range).to_string())).collect();
    vals
}

fn save_transaction(Pack: PackTransaction, name_file: String) {
    let mut file = File::create(format!("{}.txt", name_file)).expect("Failed on create a transaction file");
    let text = format!("{}", Pack);
    file.write_all(text.as_bytes()).expect("Failed on write the transaction file");
}

fn save_data(pos_name: String) {
    let wallet_name = format!("/home/ubuntu/.epic/user/wallet_data_{}", pos_name);
    let chain_name = format!("/home/ubuntu/.epic/user/chain_data_{}", pos_name);

    let chain_cop = Command::new("cp")
                    .args(["-r","/home/ubuntu/.epic/user/chain_data", &chain_name])
                    .output()
                    .expect("Failed on copy chain_data");

    let wallet_cop = Command::new("cp")
                    .args(["-r","/home/ubuntu/.epic/user/wallet_data", &wallet_name])
                    .output()
                    .expect("Failed on copy wallet_data");
    println!("WALL {:?} -- name {:?}", wallet_cop, wallet_name);
}

//#[tokio::main]
fn main() {
    let method_send = String::from("http");
    let method_to_send = Arc::new(method_send);

    //// Init the world
    //let mut big_wallet = BigWalletWorld::default();

    //// Init the variables 
    let password = Arc::new(String::from("1"));
    let chain_type = Arc::new(ChainTypes::UserTesting);
    let server_binary = Arc::new(String::from("/home/ubuntu/testing/binaries/epic"));
    let wallet_binary = Arc::new(String::from("/home/ubuntu/testing/binaries/epiccash_E3_wallet_ubuntu/epic-wallet"));
    let miner_binary = Arc::new(String::from("/home/ubuntu/testing/binaries/epic-miner"));
    let http_path =  Arc::new(get_http_wallet());


    let mut childrens = ChildProcess::default();

       
    // big_wallet.chain_type = ChainTypes::UserTesting;
    // big_wallet.server_binary = String::from("/home/jualns/Desktop/epic/target/release/epic");
    // big_wallet.wallet_binary = String::from("/home/jualns/Desktop/epic-wallet/target/release/epic-wallet");
    // big_wallet.miner_binary = String::from("/home/jualns/Desktop/epic-miner/target/debug/epic-miner");
    
    //// Init the systems

    // config epic-server.toml with custom configuration
    get_test_configuration(&chain_type);
    // Wait the epic-servet.toml save
    wait_for(5);
    // run wallet and save on world
    _ = create_wallet(&chain_type, wallet_binary.as_str(), password.as_str());
    //big_wallet.http_path =  get_http_wallet();
    // run server and save on world
    childrens.server = spawn_network(&chain_type, server_binary.as_str());
    // save the wallet_listen process on world
    childrens.wallet = spawn_wallet_listen(&chain_type, wallet_binary.as_str(), password.as_str());
    // Run the miner
    childrens.miner = spawn_miner(&miner_binary);
    
    // wait for 30 secs to miner start
    println!("BEFORE SLEEP!");
    wait_for(60);
    println!("AFTER SLEEP!");

    let mut handles_vec = Vec::new();

    for _ in 0..1 {
        let method_to_sent = Arc::clone(&method_to_send);
        let pass = Arc::clone(&password);
        let chain_t = Arc::clone(&chain_type);
        //let server_bin = Arc::clone(&server_binary);
        let wallet_bin = Arc::clone(&wallet_binary);
        //let miner_bin = Arc::clone(&miner_binary);
        let http_pa =  Arc::clone(&http_path);

        let handle = thread::spawn(move|| {
            // prepare the pack of transactions
            let mut now = time::Instant::now();
            let mut pack_transactions = PackTransaction {
                number_transactions: 10,
                duration_time: Vec::new(), //vec![now.elapsed(); 1],
                vec_amount: generate_vec_to_sent(0, 1000, 10)//Vec::new(), //vec!["1.0".to_string()],
            };
            //pack_transactions.duration_time.push(now.elapsed());
            //pack_transactions.vec_amount.push("1.0".to_string());
    
            // check if have coins
            let mut coins_in_wallet = have_coins_in_wallet(&chain_t, &wallet_bin, &pass, &2.0);
            while !coins_in_wallet {
                wait_for(5);
                coins_in_wallet = have_coins_in_wallet(&chain_t, &wallet_bin, &pass, &2.0);
            }

            let mut amount: String = pack_transactions.vec_amount.first().expect("Can't have amount to send").to_string();
            let k_param = 5;
            for t_k in 0..pack_transactions.number_transactions as usize {
                amount = pack_transactions.vec_amount[t_k].to_string();
                println!("-- HERE  amount: {:?} --", &amount);
                // check if have coins
                coins_in_wallet = have_coins_in_wallet(&chain_t, &wallet_bin, &pass, &2.0);
                while !coins_in_wallet {
                    wait_for(10);
                    coins_in_wallet = have_coins_in_wallet(&chain_t, &wallet_bin, &pass, &2.0);
                }
                now = time::Instant::now();
                let out = send_coins_smallest(&chain_t, &wallet_bin, method_to_sent.to_string(), &pass, amount, &http_pa);
                println!("OUTPUT OF SENT {:?}", String::from_utf8_lossy(&out.stdout));
                
                pack_transactions.duration_time.push(now.elapsed());

                let tt_kk = t_k as f32;
                let div = tt_kk/k_param as f32;
                if div == div.floor() {
                    save_data(div.to_string());
                    println!("SAVE WALLET AND CHAIN DATA!")
                }
            }
            save_transaction(pack_transactions, "transactions_test".to_string());
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
}
