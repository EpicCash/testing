//use std::fmt;
use std::{process::Child, fs::remove_file};
use async_trait::async_trait;
use cucumber::{given, when, then, World, WorldInit};
use std::convert::Infallible;
extern crate dotenv;
use dotenv::dotenv;
use std::env;
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
            info_wallet,
            new_child,
            //new_output,
            get_number_transactions_txs,
            get_http_wallet,
            receive_finalize_coins,
            generate_file_name,
            generate_response_file_name, get_home_chain,
            };

// Epic Server
use epic_core::global::ChainTypes;

//Epic Wallet
//use epic_wallet_config::config::initial_setup_wallet;

//impl fmt::Debug for WalletWorld {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.wallet_binary)
//    }
//}

//impl fmt::Debug for WalletInformation {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.sent_tx)
//    }
//}

impl std::default::Default for WalletWorld {
	fn default() -> WalletWorld {
		WalletWorld {
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

// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, WorldInit)]
pub struct WalletWorld {
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
impl World for WalletWorld {
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
//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "Define {string} binary")]
fn set_binary(world: &mut WalletWorld, epic_sys: String) {
    match epic_sys.as_str() {
        "epic-server" => {world.server_binary = env::var("EPIC_SERVER").unwrap()},
        "epic-wallet" => {world.wallet_binary = env::var("EPIC_WALLET").unwrap()},
        "epic-miner" => {world.miner_binary = env::var("EPIC_MINER").unwrap()},
        _ => panic!("Invalid epic system"),
    };
}

#[given(expr = "I am using the {string} network")]
fn using_network(world: &mut WalletWorld, str_chain: String) {

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
    world.server = spawn_network(&world.chain_type, world.server_binary.as_str(), &str_chain);
    
    // save passphrase on world
    world.passphrase = get_passphrase(&wallet_init);

    // save the wallet_listen process on world
    world.wallet = spawn_wallet_listen(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());

    // Run the miner
    world.miner = spawn_miner(&world.miner_binary);
}

//I initiate a wallet|miner
#[given(expr = "I initiate a {word}")]
fn init_wallet(world: &mut WalletWorld, service: String) {
    match service.as_str() {
        "wallet" => {
            // save the wallet_listen process on world
            world.wallet = spawn_wallet_listen(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());
        }
        "miner" => {
            // Run the miner
            world.miner = spawn_miner(&world.miner_binary);
        }
        _ => panic!("Can't initiate {service}!")
    }
}

// I mine 11 blocks and stop miner
#[given(expr = "I mine {int} blocks and stop miner")]
fn mine_x_blocks(world: &mut WalletWorld, blocks: u32) {
    let mut txs = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let mut confirmed_coinbase = txs.last().expect("Can't get the number of Confirmed Coinbase!");
    
    while confirmed_coinbase < &blocks {
        txs = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
        confirmed_coinbase = txs.last().expect("Can't get the number of Confirmed Coinbase!"); 
    }

    world.miner.kill().expect("Miner wasn't running");
}


#[given(expr = "I have a wallet with coins")]
fn check_coins_in_wallet(world: &mut WalletWorld) {
    let info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let current_spendable = info.last().expect("Can't get the current spendable!");
    assert!(current_spendable > &0.0)
}

#[when(expr = "I send {word} coins with {word} method")]
fn send_coins(world: &mut WalletWorld, amount: String, method: String) {
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
            let dest = get_http_wallet(&world.chain_type);
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

// I make a recovery
#[when(expr = "I make a recovery")]
fn recovery_process(world: &mut WalletWorld) {
    //let passphrase = world.passphrase;
    ()
}
 
//I have 2 new transactions in txs
#[then(expr = "I have {int} new transactions in txs")]
fn check_new_transactions(world: &mut WalletWorld, number_transactions: u32) {
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
fn kill_all_childs(world: &mut WalletWorld) {
    world.wallet.kill().expect("Wallet wasn't running");
    world.server.kill().expect("Server wasn't running");
}

#[when(expr = "I {word} the {word} transaction")]
fn receive_step(world: &mut WalletWorld, receive_finalize: String, method: String) {

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
 
//I check if wallet change to new DB
#[then(expr="I check if wallet change to new DB")]
fn check_exist_new_db_file(world: &mut WalletWorld) {
    let mut home_dir = get_home_chain(&world.chain_type);
    home_dir.push("wallet_data");
    home_dir.push("db");
    //home_dir.push("lmdb");
    assert!(home_dir.is_dir())
}   

//I finalize the emoji transaction


//#[tokio::main]
fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    futures::executor::block_on(WalletWorld::run("./features/transactions.feature"));
}