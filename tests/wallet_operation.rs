use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use std::convert::Infallible;
use std::path::PathBuf;
use std::process::Child;
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::env::current_dir;
use std::fs::{remove_dir_all, remove_file};

//Testing
use testing::{
    confirm_transaction, create_wallet, generate_file_name, generate_response_file_name,
    get_home_chain, get_http_wallet, get_number_transactions_txs, get_passphrase,
    get_test_configuration, info_wallet, new_child, receive_finalize_coins, recover_wallet_shell,
    send_coins_smallest, spawn_miner, spawn_network, spawn_wallet_listen, str_to_chain_type,
    wait_for,
};

// Epic Server
use epic_core::global::ChainTypes;

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
            info_command: InfoWallet::default(),
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

#[derive(Debug, PartialEq)]
pub struct InfoWallet {
    pub chain_height: f32,
    pub confirmed_total: f32,
    pub immature_coinbase: f32,
    pub awaiting_confirmation: f32,
    pub awaiting_finalization: f32,
    pub locked_by_previus_transaction: f32,
    pub currently_spendable: f32,
}

impl std::default::Default for InfoWallet {
    fn default() -> InfoWallet {
        InfoWallet {
            chain_height: 0.0,
            confirmed_total: 0.0,
            immature_coinbase: 0.0,
            awaiting_confirmation: 0.0,
            awaiting_finalization: 0.0,
            locked_by_previus_transaction: 0.0,
            currently_spendable: 0.0,
        }
    }
}

impl std::convert::From<Vec<f32>> for InfoWallet {
    fn from(item: Vec<f32>) -> Self {
        // code to convert the vector into an instance of your struct goes here
        if item.len() > 0 {
            InfoWallet {
                chain_height: item[0],
                confirmed_total: item[1],
                immature_coinbase: item[2],
                awaiting_confirmation: item[3],
                awaiting_finalization: item[4],
                locked_by_previus_transaction: item[5],
                currently_spendable: item[6],
            }
        } else {
            InfoWallet::default()
        }
    }
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
    pub info_command: InfoWallet,
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
            info_command: InfoWallet::default(),
        })
    }
}
//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "Define {string} binary")]
fn set_binary(world: &mut WalletWorld, epic_sys: String) {
    match epic_sys.as_str() {
        "epic-server" => world.server_binary = env::var("EPIC_SERVER").unwrap(),
        "epic-wallet" => world.wallet_binary = env::var("EPIC_WALLET").unwrap(),
        "epic-miner" => world.miner_binary = env::var("EPIC_MINER").unwrap(),
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
    let _wallet_init = create_wallet(
        &world.chain_type,
        world.wallet_binary.as_str(),
        world.password.as_str(),
    );
}

#[then(expr = "I run and save info command")]
fn info_save(world: &mut WalletWorld) {
    let info_vec = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    world.info_command = InfoWallet::from(info_vec);
}

#[when(expr = "I delete the wallet folder")]
fn delete_wallet_data(world: &mut WalletWorld) {
    let dir = &world.wallet_binary;
    match remove_dir_all(dir) {
        Ok(()) => println!("Successfully deleted directory and all its contents"),
        Err(e) => println!("Error deleting directory: {}", e),
    }
}

#[when(expr = "I make the recovery")]
fn wallet_recover(world: &mut WalletWorld) {
    let shell_path = env::var("RECOVER_SHELL").unwrap();
    let result_recover = recover_wallet_shell(
        &world.chain_type,
        &world.wallet_binary,
        &shell_path,
        &world.password,
        &world.passphrase,
    );
    assert!(result_recover, "Can't recover the wallet!")
}

#[then(expr = "I have the same information")]
fn compare_info(world: &mut WalletWorld) {
    let info_now = InfoWallet::from(info_wallet(
        &world.chain_type,
        &world.wallet_binary,
        &world.password,
    ));
    assert_eq!(world.info_command, info_now)
}

#[when(expr = "I start the node with policy {string}")]
fn start_child_system(world: &mut WalletWorld, enter_policy: String) {
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
fn start_child_general(world: &mut WalletWorld, start_stop: String, epic_system: String) {
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
fn mine_some_coins(world: &mut WalletWorld) {
    // TODO - Wait for 5~10 blocks
    let mut info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let mut current_spendable = info.last().expect("Can't get the current spendable!");
    while current_spendable == &0.0 {
        wait_for(15);
        info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
        current_spendable = info.last().expect("Can't get the current spendable!");
    }
}

#[then(expr = "I await confirm the transaction")]
fn await_finalization(world: &mut WalletWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password)
}

// I mine 11 blocks and stop miner
#[given(expr = "I mine {int} blocks and stop miner")]
fn mine_x_blocks(world: &mut WalletWorld, blocks: u32) {
    let mut txs =
        get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let mut confirmed_coinbase = txs
        .last()
        .expect("Can't get the number of Confirmed Coinbase!");

    while confirmed_coinbase < &blocks {
        txs = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
        confirmed_coinbase = txs
            .last()
            .expect("Can't get the number of Confirmed Coinbase!");
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

    // If method is HTTP or file, send command needs a destination
    let send_output = match method.as_str() {
        "http" => {
            let dest = get_http_wallet(&world.chain_type);
            send_coins_smallest(
                &world.chain_type,
                &world.wallet_binary,
                method,
                &world.password,
                amount,
                &dest,
            )
        }
        "self" => send_coins_smallest(
            &world.chain_type,
            &world.wallet_binary,
            method,
            &world.password,
            amount,
            &String::new(),
        ),
        "emoji" => {
            let out_emoji = send_coins_smallest(
                &world.chain_type,
                &world.wallet_binary,
                method,
                &world.password,
                amount,
                &String::new(),
            );
            let sent_str = String::from_utf8_lossy(&out_emoji.stdout).into_owned();
            let sent_vec: Vec<&str> = sent_str.split('\n').collect();

            // Save the emoji sent message
            world.transactions.sent_path = String::from(sent_vec[0]);

            out_emoji
        }
        "file" => {
            let file_name = generate_file_name();
            let response_file_name = generate_response_file_name(&file_name);
            let out_file = send_coins_smallest(
                &world.chain_type,
                &world.wallet_binary,
                method,
                &world.password,
                amount,
                &file_name,
            );

            // Save the send file name
            world.transactions.sent_path = file_name;
            // Save the response file name
            world.transactions.receive_path = response_file_name;

            out_file
        }

        _ => panic!("Method not found!"),
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

//I check if wallet change to new DB
#[then(expr = "I check if wallet change to new DB")]
fn check_exist_new_db_file(world: &mut WalletWorld) {
    let mut home_dir = get_home_chain(&world.chain_type);
    home_dir.push("wallet_data");
    home_dir.push("db");
    //home_dir.push("lmdb");
    assert!(home_dir.is_dir())
}

//#[tokio::main]
fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    futures::executor::block_on(WalletWorld::run("./features/transactions.feature"));
}
