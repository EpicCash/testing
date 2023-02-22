use std::fs::remove_file;
extern crate dotenv;
use cucumber::{given, then, when, WorldInit};
use dotenv::dotenv;
use std::env;
use testing::types::{InfoWallet, OutputList, TestingWorld, WalletInformation};

use testing::commands::{
    confirm_transaction, create_wallet, get_number_outputs, get_number_transactions_txs,
    info_wallet, receive_finalize_coins, send_coins_smallest, spawn_miner, spawn_network,
    spawn_wallet_listen,
};
use testing::utils::{
    generate_file_name, generate_response_file_name, get_http_wallet, get_passphrase,
    get_test_configuration, str_to_chain_type, wait_for,
};

/// Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
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
    wait_for(5).await;

    // NEED CREATE WALLET BEFORE SPAWN SERVER, Unable to delete folder if server is on
    // run wallet and save on world
    let wallet_init = create_wallet(
        &world.chain_type,
        world.wallet_binary.as_str(),
        world.password.as_str(),
    );

    // save passphrase on world
    world.passphrase = get_passphrase(&wallet_init);
}

#[given("I mine some blocks into my wallet")]
async fn mine_some_coins(world: &mut TestingWorld) {
    // TODO - Wait for 5~10 blocks
    let mut info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let mut current_spendable = InfoWallet::from(info).currently_spendable;
    let low_limit = 30.0;
    while current_spendable <= low_limit {
        wait_for(15).await;
        info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
        current_spendable = InfoWallet::from(info).currently_spendable;
    }
}

#[when(expr = "I start the node with policy {string}")]
async fn start_child_system(world: &mut TestingWorld, enter_policy: String) {
    let mut poly = String::from("--");
    poly.push_str(enter_policy.as_str());
    // run server and save on world
    world.server = spawn_network(
        &world.chain_type,
        world.server_binary.as_str(),
        Some(poly.as_str()),
    );
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

#[given(expr = "I have a wallet with coins")]
fn check_coins_in_wallet(world: &mut TestingWorld) {
    let info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let current_spendable = info.last().expect("Can't get the current spendable!");
    assert!(current_spendable > &0.0)
}

#[then(expr = "I run and save {word} command")]
fn command_save(world: &mut TestingWorld, wallet_command: String) {
    match wallet_command.as_str() {
        "info" => {
            let info_vec = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);

            world.info_command = InfoWallet::from(info_vec);
        }
        "txs" => {
            let txs_vec = get_number_transactions_txs(
                &world.chain_type,
                &world.wallet_binary,
                &world.password,
            );

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

            world.outputs_command = OutputList::from(outputs_vec);
        }

        _ => println!("{wallet_command} is not a wallet command or not implemented yet"),
    };
}

#[when(expr = "I send {word} coins with {word} method")]
async fn send_coins(world: &mut TestingWorld, amount: String, method: String) {
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
            let file_name = generate_file_name("txt");
            let response_file_name = generate_response_file_name(&file_name, "file");
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
        "qr" => {
            let qr_name = generate_file_name("png");
            let response_qr_name = generate_response_file_name(&qr_name, "qr");
            let out_qr = send_coins_smallest(
                &world.chain_type,
                &world.wallet_binary,
                method,
                &world.password,
                amount,
                &qr_name,
            );

            // Save the send qr name
            world.transactions.sent_path = qr_name;
            // Save the response qr name
            world.transactions.receive_path = response_qr_name;

            out_qr
        }

        _ => panic!("Method not found!"),
    };

    wait_for(2).await;
    if !send_output.status.success() {
        panic!("Error on send proceess, output: {send_output:#?}")
    }
}

#[when(expr = "I await confirm the transaction")]
async fn await_finalization(world: &mut TestingWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password).await;
}

//I have 2 new transactions in txs
#[then(expr = "I have {int} new transactions in txs")]
fn check_new_transactions(world: &mut TestingWorld, number_transactions: u32) {
    // Update transactions information in WalletInformation
    let transaction_info =
        get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_info = WalletInformation::from(transaction_info);

    let int_number = number_transactions / 2;

    // Sent tx
    assert_eq!(world.txs_command.sent_tx + int_number, new_info.sent_tx);

    // Received tx
    assert_eq!(
        world.txs_command.received_tx + int_number,
        new_info.received_tx
    );
}

#[then(expr = "I kill all running epic systems")]
fn kill_all_childs(world: &mut TestingWorld) {
    world.miner.kill().expect("Miner wasn't running");
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
        "file" | "qr" => {
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

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    TestingWorld::run("./features/transactions.feature").await;
}
