//use std::fmt;
use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use std::convert::Infallible;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{fs::remove_file, process::Child};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
//use std::process::{Command, Output};

//Testing
use testing::{
    check_spendable, confirm_transaction, create_wallet, generate_vec_to_sent, get_http_wallet,
    get_number_transactions_txs, get_test_configuration, info_wallet, new_child,
    receive_finalize_coins, send_coins_smallest, spawn_miner, spawn_network, spawn_wallet_listen,
    str_to_chain_type, wait_for,
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

#[warn(unused_assignments)]

impl std::default::Default for TransWorld {
    fn default() -> TransWorld {
        TransWorld {
            chain_type: ChainTypes::UserTesting,
            password: String::from("1"),
            passphrase: String::new(),
            server_binary: String::new(),
            wallet_binary: String::new(),
            miner_binary: String::new(),
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
            transactions: WalletInformation::default(),
            dur_transactions: Vec::new(),
            n_transactions: 1,
        }
    }
}

impl std::default::Default for WalletInformation {
    fn default() -> WalletInformation {
        WalletInformation {
            sent_tx: 0 as u32,
            received_tx: 0 as u32,
            confirmed_coinbase: 0 as u32,
            sent_path: String::new(),
            receive_path: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct WalletInformation {
    pub sent_tx: u32,
    pub received_tx: u32,
    pub confirmed_coinbase: u32,
    pub sent_path: String,
    pub receive_path: String,
}

#[derive(Debug, Clone)]
pub struct PackTransaction {
    pub number_transactions: i32,
    pub duration_time: Vec<Duration>,
    pub vec_amount: Vec<String>,
}

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, WorldInit)]
pub struct TransWorld {
    pub chain_type: ChainTypes,
    pub server: Child,
    pub wallet: Child,
    pub miner: Child,
    pub password: String,
    pub passphrase: String, // only for recovery test
    pub server_binary: String,
    pub wallet_binary: String,
    pub miner_binary: String,
    pub transactions: WalletInformation,
    pub dur_transactions: Vec<Duration>,
    pub n_transactions: i32,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for TransWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        //Ok(Self::default())
        Ok(Self {
            chain_type: ChainTypes::UserTesting,
            password: String::from("1"),
            passphrase: String::new(),
            server_binary: String::new(),
            wallet_binary: String::new(),
            miner_binary: String::new(),
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
            transactions: WalletInformation::default(),
            dur_transactions: Vec::new(),
            n_transactions: 1,
        })
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

//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "Define {string} binary")]
fn set_binary(world: &mut TransWorld, epic_sys: String) {
    match epic_sys.as_str() {
        "epic-server" => world.server_binary = env::var("EPIC_SERVER").unwrap(),
        "epic-wallet" => world.wallet_binary = env::var("EPIC_WALLET").unwrap(),
        "epic-wallet-300" => world.wallet_binary = env::var("EPIC_WALLET_300").unwrap(),
        "epic-miner" => world.miner_binary = env::var("EPIC_MINER").unwrap(),
        _ => panic!("Invalid epic system"),
    };
}

#[given(expr = "I am using the {string} network")]
fn using_network(world: &mut TransWorld, str_chain: String) {
    let chain_t = str_to_chain_type(&str_chain);

    world.chain_type = chain_t;
    // config epic-server.toml with custom configuration
    get_test_configuration(&world.chain_type);
    // Wait the epic-servet.toml work
    wait_for(5);

    // NEED CREATE WALLET BEFORE SPAWN SERVER, Unable to delete folder if server is on
    // run wallet and save on world
    let _wallet_init = create_wallet(
        &world.chain_type,
        world.wallet_binary.as_str(),
        world.password.as_str(),
    );
}

#[when(expr = "I start the node with policy {string}")]
fn start_child_system(world: &mut TransWorld, enter_policy: String) {
    if enter_policy.len() == 0 {
        world.server = spawn_network(&world.chain_type, world.server_binary.as_str(), None);
    } else {
        let mut poly = String::from("--");
        poly.push_str(enter_policy.as_str());
        // run server and save on world
        world.server = spawn_network(
            &world.chain_type,
            world.server_binary.as_str(),
            Some(poly.as_str()),
        );
    };
    wait_for(10)
}

// I start/stop the wallet/miner
#[when(expr = "I {word} the {word}")]
fn start_child_general(world: &mut TransWorld, start_stop: String, epic_system: String) {
    match start_stop.as_str() {
        "start" => {
            match epic_system.as_str() {
                "miner" => {
                    // Run the miner
                    world.miner = spawn_miner(&world.miner_binary);
                }
                "wallet" => {
                    // save the wallet_listen process on world
                    world.wallet = spawn_wallet_listen(
                        &world.chain_type,
                        world.wallet_binary.as_str(),
                        world.password.as_str(),
                    );
                }
                _ => panic!("Specified system does not exist to start!"),
            };
            wait_for(2)
        }
        "stop" => match epic_system.as_str() {
            "node" => world.server.kill().expect("Server wasn't running"),
            "miner" => world.miner.kill().expect("Miner wasn't running"),
            "wallet" => world.wallet.kill().expect("Wallet wasn't running"),
            _ => panic!("Specified system does not exist to kill!"),
        },
        _ => panic!("Specified command does not exist, try start or stop!"),
    }
}

#[when("I mine some blocks into my wallet")]
fn mine_some_coins(world: &mut TransWorld) {
    // TODO - Wait for 5~10 blocks
    let mut info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let mut current_spendable = info.last().expect("Can't get the current spendable!");
    while current_spendable == &0.0 {
        wait_for(10);
        info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
        current_spendable = info.last().expect("Can't get the current spendable!");
    }
}

#[given(expr = "I have a wallet with coins")]
fn check_coins_in_wallet(world: &mut TransWorld) {
    let info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let current_spendable = info.last().expect("Can't get the current spendable!");
    assert!(current_spendable > &0.0)
}

#[then(expr = "I await confirm the transaction")]
fn await_finalization(world: &mut TransWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password)
}

//I have 2 new transactions in txs
#[then(expr = "I have {int} new transactions in txs")]
fn check_new_transactions(world: &mut TransWorld, number_transactions: u32) {
    // Update transactions information in WalletInformation
    let transaction_info =
        get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_info = WalletInformation {
        sent_tx: transaction_info[0],
        received_tx: transaction_info[1],
        confirmed_coinbase: transaction_info[2],
        sent_path: String::new(),
        receive_path: String::new(),
    };
    let int_number = number_transactions / 2;

    // Sent tx
    assert_eq!(world.transactions.sent_tx + int_number, new_info.sent_tx);

    // Received tx
    assert_eq!(
        world.transactions.received_tx + int_number,
        new_info.received_tx
    );
}

#[then(expr = "I kill all running epic systems")]
fn kill_all_childs(world: &mut TransWorld) {
    world.miner.kill().expect("Miner wasn't running");
    world.wallet.kill().expect("Wallet wasn't running");
    world.server.kill().expect("Server wasn't running");
}

#[when(expr = "I {word} the {word} transaction")]
fn receive_step(world: &mut TransWorld, receive_finalize: String, method: String) {
    let path_emoji_file = match receive_finalize.as_str() {
        "receive" => &world.transactions.sent_path,
        "finalize" => &world.transactions.receive_path,
        _ => panic!("This operation isn't valid!"),
    };

    let posic_output = 4;

    let output_receive_finalize = match method.as_str() {
        "emoji" => {
            let out_emoji = receive_finalize_coins(
                &world.chain_type,
                &world.wallet_binary,
                method,
                &world.password,
                &receive_finalize,
                path_emoji_file,
            );
            let out_str = String::from_utf8_lossy(&out_emoji.stdout).into_owned();
            let out_vec: Vec<&str> = out_str.split('\n').collect();

            // Save the emoji sent|receive message
            if receive_finalize.as_str() == "receive" {
                world.transactions.receive_path = String::from(out_vec[posic_output]);
            }

            out_emoji
        }
        "file" => {
            let out_file = receive_finalize_coins(
                &world.chain_type,
                &world.wallet_binary,
                method,
                &world.password,
                &receive_finalize,
                path_emoji_file,
            );

            if receive_finalize.as_str() == "finalize" {
                remove_file(&world.transactions.sent_path).expect("Failed on delete sent file!");
                remove_file(&world.transactions.receive_path)
                    .expect("Failed on delete receive file!")
            }

            out_file
        }
        _ => panic!("Receive or Finalize method not found!"),
    };

    assert!(output_receive_finalize.status.success())
}

#[when(expr = "I make a {int} transactions with {word} method")]
fn send_n_coins(world: &mut TransWorld, num_transactions: i32, method: String) {
    let mut pack_transaction = PackTransaction {
        number_transactions: num_transactions,
        duration_time: Vec::new(),
        vec_amount: generate_vec_to_sent(0, 1000, num_transactions),
    };
    // Update transactions information in WalletInformation
    let transaction_info =
        get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_transactions_information = WalletInformation {
        sent_tx: transaction_info[0],
        received_tx: transaction_info[1],
        confirmed_coinbase: transaction_info[2],
        sent_path: String::new(),
        receive_path: String::new(),
    };
    world.transactions = new_transactions_information;
    let mut amount: String = pack_transaction
        .vec_amount
        .first()
        .expect("Can't have amount to send")
        .to_string();
    let mmethod = method.clone();
    for k in 0..num_transactions as usize {
        amount = pack_transaction.vec_amount[k].to_string();
        check_spendable(
            &world.chain_type,
            &world.wallet_binary,
            &world.password,
            &amount.parse().expect("Can't convert amount to f32!"),
        );
        println!("\n SEND {:?} -- Amount {:?}\n", k, amount);

        // If method is HTTP or file, send command needs a destination
        let dest = get_http_wallet(&world.chain_type);

        let now = Instant::now();
        send_coins_smallest(
            &world.chain_type,
            &world.wallet_binary,
            mmethod.clone(),
            &world.password,
            amount,
            &dest,
        );
        let elapsed = now.elapsed();
        pack_transaction.duration_time.push(elapsed);
    }
    //save_transaction(pack_transaction, "transactions_test".to_string());
    world.dur_transactions = pack_transaction.duration_time;
    world.n_transactions = num_transactions;
}

//The average transaction time is less than "1" second
#[then(expr = "The average transaction time is less than {float} second")]
fn average_transactions(world: &mut TransWorld, secs_compare: f32) {
    let a = &world.dur_transactions;
    println!("\nAll transaction time: {:?}\n", &a);
    let avg: f32;

    {
        // calculate average
        let mut sum: f32 = 0.0;
        for x in a {
            sum = sum + x.as_secs_f32();
        }

        avg = sum as f32 / a.len() as f32;
    }
    println!("\n\nAverage transaction time: {:?} seconds\n\n", avg);
    assert!(avg < secs_compare)
}

//All transactions work
#[then(expr = "All transactions work")]
fn transactions_work(world: &mut TransWorld) {
    // Update transactions information in WalletInformation
    let transaction_info =
        get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);

    let new_info = WalletInformation {
        sent_tx: transaction_info[0],
        received_tx: transaction_info[1],
        confirmed_coinbase: transaction_info[2],
        sent_path: String::new(),
        receive_path: String::new(),
    };
    //let int_number = number_transactions/2;

    // Sent tx
    assert_eq!(
        world.transactions.sent_tx + world.n_transactions as u32,
        new_info.sent_tx
    );

    // Received tx
    assert_eq!(
        world.transactions.received_tx + world.n_transactions as u32,
        new_info.received_tx
    );
}

//#[tokio::main]
fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    futures::executor::block_on(TransWorld::run("./features/scalability.feature"));
}
