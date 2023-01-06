use cucumber::{given, then, when, WorldInit};
use testing::types::{InfoWallet, TestingWorld, WalletInformation};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::fs::remove_file;

use testing::commands::{
    confirm_transaction, create_wallet, get_number_transactions_txs, info_wallet,
    receive_finalize_coins, recover_wallet_shell, remove_wallet_path, send_coins_smallest,
    spawn_miner, spawn_network, spawn_wallet_listen,
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
fn using_network(world: &mut TestingWorld, str_chain: String) {
    let chain_t = str_to_chain_type(&str_chain);

    world.chain_type = chain_t;
    // config epic-server.toml with custom configuration
    get_test_configuration(&world.chain_type);
    // Wait the epic-servet.toml work
    wait_for(5);

    // NEED CREATE WALLET BEFORE SPAWN SERVER, Unable to delete folder if server is on
    // run wallet and save on world
    let wallet_init = create_wallet(
        &world.chain_type,
        world.wallet_binary.as_str(),
        world.password.as_str(),
    );

    world.passphrase = get_passphrase(&wallet_init);
}

#[then(expr = "I run and save info command")]
fn info_save(world: &mut TestingWorld) {
    let info_vec = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    world.info_command = InfoWallet::from(info_vec);
}

#[when(expr = "I delete the wallet folder")]
fn delete_wallet_data(world: &mut TestingWorld) {
    remove_wallet_path(&world.chain_type);
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
fn start_child_system(world: &mut TestingWorld, enter_policy: String) {
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
fn start_child_general(world: &mut TestingWorld, start_stop: String, epic_system: String) {
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

#[when(expr = "I mine {word} coins into my wallet")]
fn mine_some_coins(world: &mut TestingWorld, quantity: String) {
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
        wait_for(15);
        info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
        current_spendable = info.last().expect("Can't get the current spendable!");
    }
}

#[then(expr = "I await confirm the transaction")]
fn await_finalization(world: &mut TestingWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password)
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
    //home_dir.push("lmdb");
    assert!(home_dir.is_dir())
}

//#[tokio::main]
fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    futures::executor::block_on(TestingWorld::run("./features/wallet_operation.feature"));
}
