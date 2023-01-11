use cucumber::{given, then, when, WorldInit};
use std::fs::remove_file;
use std::time::Instant;
use testing::types::{InfoWallet, OutputList, PackTransaction, TestingWorld, WalletInformation};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

use testing::commands::{
    check_spendable, confirm_transaction, create_wallet, get_number_outputs,
    get_number_transactions_txs, get_restore_command, info_wallet, receive_finalize_coins,
    recover_wallet_shell, remove_wallet_path, scan_wallet, send_coins_smallest, spawn_miner,
    spawn_network, spawn_wallet_listen,
};
use testing::utils::{
    generate_vec_to_sent, get_http_wallet, get_passphrase, get_test_configuration,
    str_to_chain_type, wait_for,
};

#[warn(unused_assignments)]
#[given(expr = "Define {string} binary")]
fn set_binary(world: &mut TestingWorld, epic_sys: String) {
    match epic_sys.as_str() {
        "epic-server" => world.server_binary = env::var("EPIC_SERVER").unwrap(),
        "epic-wallet" => world.wallet_binary = env::var("EPIC_WALLET").unwrap(),
        "epic-wallet-300" => world.wallet_binary = env::var("EPIC_WALLET_300").unwrap(),
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
}

// I use a stored/new wallet
#[given(expr = "I use a {string} wallet")]
fn using_wallet(world: &mut TestingWorld, type_wallet: String) {
    // NEED CREATE WALLET BEFORE SPAWN SERVER, Unable to delete folder if server is on
    // run wallet and save on world]
    assert_eq!("stored-huge", type_wallet.as_str());
    match type_wallet.as_str() {
        "stored-huge" | "stored-tiny" => {
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
    // for security
    wait_for(5).await;
}

#[when("I mine some blocks into my wallet")]
async fn mine_some_coins(world: &mut TestingWorld) {
    // TODO - Wait for 5~10 blocks
    let mut info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let mut current_spendable = info.last().expect("Can't get the current spendable!");
    while current_spendable == &0.0 {
        wait_for(10).await;
        info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
        current_spendable = info.last().expect("Can't get the current spendable!");
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

// I have the same information
#[then(expr = "I have the same {word}")]
fn compare_info(world: &mut TestingWorld, wallet_command: String) {
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

// I run scan
#[then(expr = "I run scan")]
async fn run_scan(world: &mut TestingWorld) {
    let scan_work = scan_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    wait_for(5).await;
    assert!(scan_work, "Scan don't work")
}

#[then(expr = "I await confirm the transaction")]
async fn await_finalization(world: &mut TestingWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password).await;
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
async fn send_n_coins(world: &mut TestingWorld, num_transactions: i32, method: String) {
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
    let _ = pack_transaction
        .vec_amount
        .first()
        .expect("Can't have amount to send")
        .to_string();
    let mmethod = method.clone();
    for k in 0..num_transactions as usize {
        let amount = pack_transaction.vec_amount[k].to_string();
        check_spendable(
            &world.chain_type,
            &world.wallet_binary,
            &world.password,
            &amount.parse().expect("Can't convert amount to f32!"),
        )
        .await;

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
fn average_transactions(world: &mut TestingWorld, secs_compare: f32) {
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
fn transactions_work(world: &mut TestingWorld) {
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

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    TestingWorld::run("./features/scalability.feature").await;
}
