//use std::fmt;
use std::{process::Child, fs::remove_file};
use async_trait::async_trait;
use cucumber::{given, when, then, World, WorldInit};
use std::convert::Infallible;
use std::time::Duration;
use std::fmt;
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
            generate_vec_to_sent,
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

#[derive(Debug)]
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
		})
    }
}

impl fmt::Display for PackTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "number_transactions: {:?}\nduration_time: {:?}\nvec_amount: {:?})", self.number_transactions, self.duration_time, self.vec_amount)
    }
}

//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "The {string} binary is at {string}")]
fn set_binary(world: &mut TransWorld, epic_sys: String, path: String) {
    match epic_sys.as_str() {
        "epic-server" => {world.server_binary = path},
        "epic-wallet" => {world.wallet_binary = path},
        "epic-miner" => {world.miner_binary = path},
        _ => panic!("Invalid system of epic"),
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
    let wallet_init = create_wallet(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());

    // run server and save on world
    world.server = spawn_network(&world.chain_type, world.server_binary.as_str());
    
    // save passphrase on world
    world.passphrase = get_passphrase(&wallet_init);

    // save the wallet_listen process on world
    world.wallet = spawn_wallet_listen(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());

    // Run the miner
    world.miner = spawn_miner(&world.miner_binary);
}

#[given("I mine some blocks into my wallet")]
fn mine_some_coins(world: &mut TransWorld) {
    // TODO - Wait for 5~10 blocks
    let mut info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let mut current_spendable = info.last().expect("Can't get the current spendable!");
    while current_spendable == &0.0 {
        wait_for(30);
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

#[when(expr = "I send {word} coins with {word} method")]
fn send_coins(world: &mut TransWorld, amount: String, method: String) {
    // TODO destination (File and HTTP methods)

    // Update transactions information in WalletInformation
    let transaction_info = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_transactions_information = WalletInformation {
                                                            sent_tx: transaction_info[0],
                                                            received_tx: transaction_info[1],
                                                            confirmed_coinbase: transaction_info[2],
                                                            sent_path: String::new(),
                                                            receive_path: String::new(),};
    world.transactions = new_transactions_information; 
    
    // If method is HTTP or file, send command needs a destination
    let send_output = match method.as_str() {
        "http" => {
            let dest = get_http_wallet();
            send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &dest)
        },
        "self" => send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &String::new()),
        "emoji" => {
            let out_emoji = send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &String::new());
            let sent_str = String::from_utf8_lossy(&out_emoji.stdout).into_owned();
            let sent_vec:Vec<&str> = sent_str.split('\n').collect();

            // Save the emoji sent message
            world.transactions.sent_path = String::from(sent_vec[0]);

            out_emoji
        },
        "file" => {
            let file_name = generate_file_name();
            let response_file_name = generate_response_file_name(&file_name);
            let out_file = send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &file_name);
            
            // Save the send file name
            world.transactions.sent_path = file_name;
            // Save the response file name
            world.transactions.receive_path = response_file_name;

            out_file
        },

        _ => panic!("Method not found!")
    };
    assert!(send_output.status.success())

}

#[when(expr = "I await confirm the transaction")]
fn await_finalization(world: &mut TransWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password)
}
 
//I have 2 new transactions in txs
#[then(expr = "I have {int} new transactions in txs")]
fn check_new_transactions(world: &mut TransWorld, number_transactions: u32) {
    // Update transactions information in WalletInformation
    let transaction_info = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_info = WalletInformation {
                                        sent_tx: transaction_info[0],
                                        received_tx: transaction_info[1],
                                        confirmed_coinbase: transaction_info[2],
                                        sent_path: String::new(),
                                        receive_path: String::new(),};
    let int_number = number_transactions/2;
    
    // Sent tx
    assert_eq!(world.transactions.sent_tx + int_number, new_info.sent_tx);

    // Received tx
    assert_eq!(world.transactions.received_tx + int_number, new_info.received_tx);
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
            let out_emoji = receive_finalize_coins(&world.chain_type, &world.wallet_binary, method, &world.password, &receive_finalize, path_emoji_file);
            let out_str = String::from_utf8_lossy(&out_emoji.stdout).into_owned();
            let out_vec:Vec<&str> = out_str.split('\n').collect();

            // Save the emoji sent|receive message
            if receive_finalize.as_str() == "receive" {
                world.transactions.receive_path = String::from(out_vec[posic_output]);
            }

            out_emoji
        },
        "file" => {
            let out_file = receive_finalize_coins(&world.chain_type, &world.wallet_binary, method, &world.password, &receive_finalize, path_emoji_file);
            
            if receive_finalize.as_str() == "finalize" {
                remove_file(&world.transactions.sent_path).expect("Failed on delete sent file!");
                remove_file(&world.transactions.receive_path).expect("Failed on delete receive file!")
            }
            
            out_file
        },
        _ => panic!("Receive or Finalize method not found!")
    };

    assert!(output_receive_finalize.status.success())
}
 
#[when(expr = "I make a {int} transactions with {word} method")]
fn send_n_coins(world: &mut TransWorld, num_transactions: i32, method: String) {
    let pack_transaction = PackTransaction {
        number_transactions: num_transactions,
        duration_time: Vec::new(),
        vec_amount: generate_vec_to_sent(0, 1000, num_transactions)

    };
    // Update transactions information in WalletInformation
    let transaction_info = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_transactions_information = WalletInformation {
                                                            sent_tx: transaction_info[0],
                                                            received_tx: transaction_info[1],
                                                            confirmed_coinbase: transaction_info[2],
                                                            sent_path: String::new(),
                                                            receive_path: String::new(),};
    world.transactions = new_transactions_information; 
    for k in 0..num_transactions {
        // If method is HTTP or file, send command needs a destination
        let send_output = match method.as_str() {
            "http" => {
                let dest = get_http_wallet();
                send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &dest)
            },
            "self" => send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &String::new()),
            "emoji" => {
                let out_emoji = send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &String::new());
                let sent_str = String::from_utf8_lossy(&out_emoji.stdout).into_owned();
                let sent_vec:Vec<&str> = sent_str.split('\n').collect();

                // Save the emoji sent message
                world.transactions.sent_path = String::from(sent_vec[0]);

                out_emoji
            },
            "file" => {
                let file_name = generate_file_name();
                let response_file_name = generate_response_file_name(&file_name);
                let out_file = send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, &file_name);
                
                // Save the send file name
                world.transactions.sent_path = file_name;
                // Save the response file name
                world.transactions.receive_path = response_file_name;

                out_file
            },

            _ => panic!("Method not found!")
        };
    
    }
    assert!(send_output.status.success())

}

//I finalize the emoji transaction


//#[tokio::main]
fn main() {
    println!("Remember to close all running epic systems before running the test");
    futures::executor::block_on(TransWorld::run("./features/transactions.feature"));
}