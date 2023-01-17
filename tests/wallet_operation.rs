use cucumber::{given, then, when, WorldInit};
use epic_util::file::copy_dir_to;
use testing::types::{InfoWallet, OutputList, TestingWorld, WalletInformation};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::fs::{remove_dir_all, remove_file};
use std::path::Path;

use testing::commands::{
    confirm_transaction, create_wallet, get_number_outputs, get_number_transactions_txs,
    get_restore_command, get_wallet_chain_data, info_wallet, receive_finalize_coins,
    recover_wallet_shell, remove_wallet_chain_path, send_coins_smallest, spawn_miner,
    spawn_network, spawn_wallet_listen,
};
use testing::utils::{
    generate_file_name, generate_response_file_name, get_home_chain, get_http_wallet,
    get_passphrase, get_test_configuration, str_to_chain_type, wait_for,
};

//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "Define {string} binary")]
fn set_binary(world: &mut TestingWorld, epic_sys: String) {
    match epic_sys.as_str() {
        "epic-server" => world.server_binary = env::var("EPIC_SERVER").unwrap(),
        "epic-wallet" => world.wallet_binary = env::var("EPIC_WALLET").unwrap(),
        "epic-miner" => world.miner_binary = env::var("EPIC_MINER").unwrap(),
        _ => panic!("Invalid epic system"),
    };
}

#[given(expr = "I am using the {string} network")]
async fn using_network(world: &mut TestingWorld, str_chain: String) {
    let chain_t = str_to_chain_type(&str_chain);

    world.chain_type = chain_t;
    // config epic-server.toml with custom configuration
    get_test_configuration(&world.chain_type);
    // Wait the epic-servet.toml work
    wait_for(5);
}

// I use a stored/new wallet
#[given(expr = "I use a {string} wallet")]
fn using_wallet(world: &mut TestingWorld, type_wallet: String) {
    // NEED CREATE WALLET BEFORE SPAWN SERVER, Unable to delete folder if server is on
    // run wallet and save on world]
    match type_wallet.as_str() {
        "stored-huge" | "stored-tiny" => {
            // 1. Wallet copy and paste step
            let stored_wallets = env::var("STORED_WALLETS").expect("Can't get the stored_wallets folder, see Epic [drive](https://drive.google.com/drive/folders/14gq0Mh8sL_I9XYuTWSrJAwxiR0CkhcPT)");
            let specif_wallet = match type_wallet.as_str() {
                "stored-huge" => format!("{}/wallet_data_huge", stored_wallets.clone()),
                "stored-tiny" | _ => {
                    format!("{}/wallet_data_tiny", stored_wallets.clone())
                }
            };
            let src_wallet = Path::new(&specif_wallet);

            // get wallet_data path
            let wallet_data = get_wallet_chain_data(&world.chain_type, "wallet");

            // remove wallet_data
            remove_wallet_chain_path(&world.chain_type, "wallet");

            // copy specifc wallet_data to default dest
            copy_dir_to(src_wallet, wallet_data.as_path())
                .expect("Can't copy wallet_data into .epic/network");

            wait_for(5);

            // 2. Chain copy and paste step
            let specif_chain = format!("{}/chain_data", stored_wallets.clone());
            let src_chain = Path::new(&specif_chain);

            // get chain_data path
            let chain_data = get_wallet_chain_data(&world.chain_type, "chain");

            // remove chain_data
            remove_wallet_chain_path(&world.chain_type, "chain");

            // copy specifc wallet_data to default dest
            copy_dir_to(src_chain, chain_data.as_path())
                .expect("Can't copy chain_data into .epic/network");

            wait_for(5);

            // 3. get passphrase from stored wallet
            let wallet_restore = get_restore_command(
                &world.chain_type,
                world.wallet_binary.as_str(),
                world.password.as_str(),
            );
            let passphrase = get_passphrase(&wallet_restore);
            assert!(passphrase.len() > 0, "Error, passphrase is: {passphrase}");
            world.passphrase = passphrase;
            //todo!("Copy and paste the chain_data and wallet_data into .epic/chain_type folder")
        }
        "new" => {
            let wallet_init = create_wallet(
                &world.chain_type,
                world.wallet_binary.as_str(),
                world.password.as_str(),
            );

            world.passphrase = get_passphrase(&wallet_init);
        }
        "passphrase-huge" | "passphrase-tiny" => {
            todo!(
                "Copy and paste the chain_data into .epic/chain_type folder
				Open a file that saves a passphrase of huge/tiny wallet and read.
				Run recovery process"
            )
        }
        _ => panic!("This {type_wallet} type of wallet has not yet been implemented!"),
    };
}

#[given(expr = "I have a wallet in LMDB")]
fn check_lmdb_wallet(world: &mut TestingWorld) {
    // get wallet_data
    let mut wallet_data = get_wallet_chain_data(&world.chain_type, "wallet");
    wallet_data.push("db");

    // get sqlite path
    let mut wallet_data_sqlite = wallet_data.clone();
    wallet_data_sqlite.push("sqlite");

    // get lmdb path
    wallet_data.push("lmdb");

    if wallet_data_sqlite.exists() {
        remove_dir_all(wallet_data_sqlite).expect("Can't remove wallet_data/db/sqlite")
    }

    assert!(wallet_data.exists());
}

#[then(expr = "I run and save {word} command")]
fn command_save(world: &mut TestingWorld, wallet_command: String) {
    match wallet_command.as_str() {
        "info" => {
            let info_vec = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
            println!("\nSAVE INFO: {:#?}", info_vec);
            world.info_command = InfoWallet::from(info_vec);
        }
        "txs" => {
            let txs_vec = get_number_transactions_txs(
                &world.chain_type,
                &world.wallet_binary,
                &world.password,
            );
            println!("\nSAVE TXS: {:#?}", txs_vec);
            world.txs_command = WalletInformation::from(txs_vec);
        }
        "outputs" | "outputs_full_history" => {
            // if we want full_history
            let show_full_history = wallet_command.as_str() == "outputs_full_history";

            let outputs_vec = get_number_outputs(
                &world.chain_type,
                &world.wallet_binary,
                &world.password,
                show_full_history,
            );
            println!("\nSAVE OUTPUTS: {:#?}", outputs_vec);
            world.outputs_command = OutputList::from(outputs_vec);
        }

        _ => println!("{wallet_command} is not a wallet command or not implemented yet"),
    };
}

#[when(expr = "I delete the wallet folder")]
fn delete_wallet_data(world: &mut TestingWorld) {
    remove_wallet_chain_path(&world.chain_type, "wallet");
}

#[when(expr = "I make the recover in my wallet")]
fn wallet_recover(world: &mut TestingWorld) {
    recover_wallet_shell(
        &world.chain_type,
        &world.wallet_binary,
        &world.password,
        &world.passphrase,
    );
}

#[then(expr = "I have the same information")]
fn compare_info(world: &mut TestingWorld) {
    let info_now = InfoWallet::from(info_wallet(
        &world.chain_type,
        &world.wallet_binary,
        &world.password,
    ));
    assert_eq!(world.info_command, info_now)
}

#[when(expr = "I start the node with policy {string}")]
async fn start_child_system(world: &mut TestingWorld, enter_policy: String) {
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
    wait_for(10).await;
}

// I start/stop the wallet/miner
#[when(expr = "I {word} the {word}")]
async fn start_child_general(world: &mut TestingWorld, start_stop: String, epic_system: String) {
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
            wait_for(2).await;
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

#[when(expr = "I mine {word} coins into my wallet")]
async fn mine_some_coins(world: &mut TestingWorld, quantity: String) {
    // TODO - Wait for 5~10 blocks
    let mut info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let mut current_spendable = info.last().expect("Can't get the current spendable!");
    let low_limit = match quantity.as_str() {
        "some" => 0.0,
        _ => {
            let number_quant: f32 = quantity.parse().expect("Unable to parse String as Float");
            number_quant
        }
    };
    while current_spendable <= &low_limit {
        wait_for(15).await;
        info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
        current_spendable = info.last().expect("Can't get the current spendable!");
    }
}

#[then(expr = "I await confirm the transaction")]
async fn await_finalization(world: &mut TestingWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password).await;
}

// I mine 11 blocks and stop miner
#[given(expr = "I mine {int} blocks and stop miner")]
fn mine_x_blocks(world: &mut TestingWorld, blocks: u32) {
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
fn check_coins_in_wallet(world: &mut TestingWorld) {
    let info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let current_spendable = info.last().expect("Can't get the current spendable!");
    assert!(current_spendable > &0.0)
}

#[when(expr = "I send {word} coins with {word} method")]
fn send_coins(world: &mut TestingWorld, amount: String, method: String) {
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

//I have 2 new transactions in txs
#[then(expr = "I have {int} new transactions in txs")]
fn check_new_transactions(world: &mut TestingWorld, number_transactions: u32) {
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
fn kill_all_childs(world: &mut TestingWorld) {
    world.wallet.kill().expect("Wallet wasn't running");
    world.server.kill().expect("Server wasn't running");
}

#[when(expr = "I {word} the {word} transaction")]
fn receive_step(world: &mut TestingWorld, receive_finalize: String, method: String) {
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
fn check_exist_new_db_file(world: &mut TestingWorld) {
    let mut home_dir = get_home_chain(&world.chain_type);
    home_dir.push("wallet_data");
    home_dir.push("db");
    home_dir.push("sqlite");

    assert!(home_dir.is_dir());

    // I have the same information
    let wallet_command = String::from("info");
    match wallet_command.as_str() {
        "information" | "info" => {
            let info_now = InfoWallet::from(info_wallet(
                &world.chain_type,
                &world.wallet_binary,
                &world.password,
            ));
            println!(
                "\nINFO COMP: \nBefore: {:#?} \n After {:#?}",
                world.info_command, info_now
            );
            assert_eq!(
                world.info_command, info_now,
                "Testing the before recover {:#?} and after recover {:#?}",
                world.info_command, info_now
            )
        }
        "transactions" | "txs" => {
            let txs_now = WalletInformation::from(get_number_transactions_txs(
                &world.chain_type,
                &world.wallet_binary,
                &world.password,
            ));
            println!(
                "\nTXS COMP: \nBefore: {:#?} \n After {:#?}",
                world.txs_command, txs_now
            );

            assert!(
                txs_now.sent_tx == 0
                    && world.txs_command.confirmed_coinbase == txs_now.confirmed_coinbase,
                "Testing the before recover {:#?} and after recover {:#?}",
                world.txs_command,
                txs_now
            )
        }
        "outputs" | "outputs_full_history" => {
            let show_full_history = wallet_command.as_str() == "outputs_full_history";
            let outputs_now = OutputList::from(get_number_outputs(
                &world.chain_type,
                &world.wallet_binary,
                &world.password,
                show_full_history,
            ));
            println!(
                "\nOUTPUT COMP: \nBefore: {:#?} \n After {:#?}",
                world.outputs_command, outputs_now
            );

            let check_outputs = world.outputs_command.eq(&outputs_now);

            assert!(
                check_outputs,
                "Testing the before recover {:#?} and after recover {:#?}",
                world.outputs_command, outputs_now
            );
        }
        _ => println!("{wallet_command} is not a wallet command or not implemented yet"),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    TestingWorld::run("./features/wallet_operation.feature").await;
}
