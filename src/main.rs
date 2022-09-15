//use std::fmt::Display;
use std::{thread, time};
use std::time::Duration;
//use std::sync::mpsc;
use std::{process::Child, fs::remove_file};
use std::convert::Infallible;
use std::time::Instant;
//use std::process::{Command, Output};

//Testing
use testing::{
            wait_for,
            get_test_configuration,
            spawn_network,
            create_wallet,
            str_to_chain_type,
            spawn_miner, spawn_wallet_listen,
            get_passphrase,
            send_coins_smallest,
            confirm_transaction,
            info_wallet,
            new_child,
            //new_output,
            get_number_transactions_txs,
            get_http_wallet,
            receive_finalize_coins,
            generate_file_name,
            generate_response_file_name,
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
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
		}
	}
}

#[derive(Debug)]
pub struct BigWalletWorld {
    pub chain_type: ChainTypes,
    pub send_method: String,
    pub http_path: String,
    pub server: Child,
    pub wallet: Child,
    pub miner: Child,
    pub password: String,
    pub server_binary: String,
    pub wallet_binary: String,
    pub miner_binary: String,
}

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

fn have_coins_in_wallet(world: &mut BigWalletWorld) -> bool {
    let info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let current_spendable = info.last().expect("Can't get the current spendable!");
    current_spendable > &0.0
}

//#[tokio::main]
fn main() {
    let method_to_sent = String::from("http");

    //// Init the world
    let mut big_wallet = BigWalletWorld::default();

    //// Init the variables    
    big_wallet.chain_type = ChainTypes::UserTesting;
    big_wallet.server_binary = String::from("/home/jualns/Desktop/epic/target/release/epic");
    big_wallet.wallet_binary = String::from("/home/jualns/Desktop/epic-wallet/target/release/epic-wallet");
    big_wallet.miner_binary = String::from("/home/jualns/Desktop/epic-miner/target/debug/epic-miner");
    
    //// Init the systems

    // config epic-server.toml with custom configuration
    get_test_configuration(&big_wallet.chain_type);
    // Wait the epic-servet.toml save
    wait_for(5);
    // run wallet and save on world
    _ = create_wallet(&big_wallet.chain_type, big_wallet.wallet_binary.as_str(), big_wallet.password.as_str());
    big_wallet.http_path =  get_http_wallet();
    // run server and save on world
    big_wallet.server = spawn_network(&big_wallet.chain_type, big_wallet.server_binary.as_str());
    // save the wallet_listen process on world
    big_wallet.wallet = spawn_wallet_listen(&big_wallet.chain_type, big_wallet.wallet_binary.as_str(), big_wallet.password.as_str());
    // Run the miner
    big_wallet.miner = spawn_miner(&big_wallet.miner_binary);

    // prepare the pack of transactions
    let mut now = time::Instant::now();
    let mut pack_transactions = PackTransaction {
        number_transactions: 1,
        duration_time: Vec::new(), //vec![now.elapsed(); 1],
        vec_amount: Vec::new(), //vec!["1.0".to_string()],
    };
    pack_transactions.duration_time.push(now.elapsed());
    pack_transactions.vec_amount.push("1.0".to_string());


    // check if have coins
    let mut coins_in_wallet = have_coins_in_wallet(&mut big_wallet);
    while !coins_in_wallet {
        wait_for(1);
        coins_in_wallet = have_coins_in_wallet(&mut big_wallet);
    }

    let mut amount: String = pack_transactions.vec_amount.first().expect("Can't have amount to send").to_string();
    for t_k in 0..pack_transactions.number_transactions as usize {
        amount = pack_transactions.vec_amount[t_k].to_string();
        now = time::Instant::now();
        send_coins_smallest(&big_wallet.chain_type, &big_wallet.wallet_binary, method_to_sent.to_owned(), &big_wallet.password, amount, &big_wallet.http_path);
        pack_transactions.duration_time.push(now.elapsed());
    }

    // Save the pack_transactions
    // TODO
        // TO SAVE THE STEP IN FILE https://doc.rust-lang.org/std/fmt/trait.Display.html
        // Then use fs::File to persist to the filesystem


    //// Run cucumber steps
    //futures::executor::block_on(TransWorld::run("./features/transactions.feature"));

    //// Confirm transactions
    confirm_transaction(&big_wallet.chain_type, &big_wallet.wallet_binary, &big_wallet.password);

    //// Finish all systems
    
    big_wallet.miner.kill().expect("Can't kill miner!");
    big_wallet.server.kill().expect("Can't kill server!");
    big_wallet.wallet.kill().expect("Can't kill wallet!");    
}