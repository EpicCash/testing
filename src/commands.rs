use expectrl;
use std::fs::remove_dir_all;
use std::path::PathBuf;
use std::process::{Child, Command, Output};
use std::time::{Duration, Instant};

// Epic Server
use epic_core::global::ChainTypes;

use crate::types::InfoWallet;
use crate::utils::{get_home_chain, wait_for};

/// Spawn server process by chain type
pub fn spawn_network(chain_type: &ChainTypes, binary_path: &str, policy: Option<&str>) -> Child {
    let output = match policy {
        Some(pol) => match chain_type {
            ChainTypes::Floonet => Command::new(&binary_path)
                .arg("--floonet")
                .arg(pol)
                .spawn()
                .expect("failed to execute process"),
            ChainTypes::UserTesting => Command::new(&binary_path)
                .arg("--usernet")
                .arg(pol)
                .spawn()
                .expect("failed to execute process"),
            ChainTypes::Mainnet => Command::new(&binary_path)
                .spawn()
                .expect("failed to execute process"),
            _ => panic!("Specified network does not exist!"),
        },
        None => match chain_type {
            ChainTypes::Floonet => Command::new(&binary_path)
                .arg("--floonet")
                .spawn()
                .expect("failed to execute process"),
            ChainTypes::UserTesting => Command::new(&binary_path)
                .arg("--usernet")
                .spawn()
                .expect("failed to execute process"),
            ChainTypes::Mainnet => Command::new(&binary_path)
                .spawn()
                .expect("failed to execute process"),
            _ => panic!("Specified network does not exist!"),
        },
    };
    output
}

/// Return chain_data or wallet_data PathBuf
pub fn get_wallet_chain_data(chain_type: &ChainTypes, wallet_chain: &str) -> PathBuf {
    let mut general_dest = get_home_chain(chain_type);

    match wallet_chain {
        "wallet" => general_dest.push("wallet_data"),
        _ => general_dest.push("chain_data"),
    };

    general_dest
}

/// This function will remove the `.epic/chayn_type/wallet_data`
pub fn remove_wallet_chain_path(chain_type: &ChainTypes, wallet_chain: &str) {
    let dest = get_wallet_chain_data(chain_type, wallet_chain);

    // if wallet_data/chain_data exist -> remove
    if dest.exists() {
        remove_dir_all(dest).expect("Failed on remove old {wallet_chain}");
    }
}
/// Run the init command, if the wallet_data exist -> delete and create a new one
pub fn create_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) -> Output {
    // .epic/user ; .epic/floo or .epic/main
    // if wallet_data exist -> remove
    remove_wallet_chain_path(chain_type, "wallet");
    let wallet = match chain_type {
        ChainTypes::UserTesting => Command::new(binary_path)
            .args(["-p", password, "--usernet", "init"])
            .output()
            .expect("Failed on init a wallet"),
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["-p", password, "--floonet", "init"])
            .output()
            .expect("Failed on init a wallet"),
        _ => Command::new(binary_path)
            .args(["-p", password, "init"])
            .output()
            .expect("Failed on init a wallet"),
    };
    wallet
}

/// Run the init command, if the wallet_data exist -> delete and create a new one
pub fn get_restore_command(chain_type: &ChainTypes, binary_path: &str, password: &str) -> Output {
    let wallet = match chain_type {
        ChainTypes::UserTesting => Command::new(binary_path)
            .args(["-p", password, "--usernet", "recover"])
            .output()
            .expect("Failed on init a wallet"),
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["-p", password, "--floonet", "recover"])
            .output()
            .expect("Failed on init a wallet"),
        _ => Command::new(binary_path)
            .args(["-p", password, "recover"])
            .output()
            .expect("Failed on init a wallet"),
    };
    wallet
}

/// Spawn a wallet in listen mode
pub fn spawn_wallet_listen(chain_type: &ChainTypes, binary_path: &str, password: &str) -> Child {
    let output = match chain_type {
        ChainTypes::Floonet => Command::new(&binary_path)
            .args(["-p", password, "--floonet", "listen"])
            .spawn()
            .expect("failed to execute process"),
        ChainTypes::UserTesting => Command::new(&binary_path)
            .args(["-p", password, "--usernet", "listen"])
            .spawn()
            .expect("failed to execute process"),
        ChainTypes::Mainnet => Command::new(&binary_path)
            .args(["-p", password, "listen"])
            .spawn()
            .expect("failed to execute process"),
        _ => panic!("Specified network does not exist!"),
    };
    output
}

/// Spawn a miner
pub fn spawn_miner(binary_path: &str) -> Child {
    Command::new(&binary_path)
        .spawn()
        .expect("Failed on start the miner")
}

/// run the command ./epic-wallet --network -p <password> send -m <method> -s smallest <amount>
pub fn send_coins_smallest(
    chain_type: &ChainTypes,
    binary_path: &String,
    method: String,
    password: &String,
    amount: String,
    destination: &String,
) -> Output {
    let network = match chain_type {
        ChainTypes::Floonet => "--floonet",
        ChainTypes::UserTesting => "--usernet",
        ChainTypes::Mainnet => "",
        _ => panic!("Specified network does not exist!"),
    };

    let output = match destination.len() > 0 {
        true => match chain_type {
            ChainTypes::Mainnet => Command::new(&binary_path)
                .args([
                    "-p",
                    password.as_str(),
                    "send",
                    "-d",
                    destination.as_str(),
                    "-m",
                    method.as_str(),
                    "-s",
                    "smallest",
                    amount.as_str(),
                ])
                .output()
                .expect("failed to execute process"),

            _ => Command::new(&binary_path)
                .args([
                    "-p",
                    password.as_str(),
                    network,
                    "send",
                    "-m",
                    method.as_str(),
                    "-d",
                    destination.as_str(),
                    "-s",
                    "smallest",
                    amount.as_str(),
                ])
                .output()
                .expect("failed to execute process"),
        },
        false => match chain_type {
            ChainTypes::Mainnet => Command::new(&binary_path)
                .args([
                    "-p",
                    password.as_str(),
                    "send",
                    "-m",
                    method.as_str(),
                    "-s",
                    "smallest",
                    amount.as_str(),
                ])
                .output()
                .expect("failed to execute process"),
            _ => Command::new(&binary_path)
                .args([
                    "-p",
                    password.as_str(),
                    network,
                    "send",
                    "-m",
                    method.as_str(),
                    "-s",
                    "smallest",
                    amount.as_str(),
                ])
                .output()
                .expect("failed to execute process"),
        },
    };
    output
}

/// This function make the entire recovery process
pub fn recover_wallet_shell(
    chain_type: &ChainTypes,
    wallet_binary_path: &str,
    password: &str,
    passphrase: &str,
) {
    let network = match chain_type {
        ChainTypes::Floonet => "--floonet",
        ChainTypes::UserTesting => "--usernet",
        ChainTypes::Mainnet => "",
        _ => panic!("Specified network does not exist!"),
    };

    let command = format!("{} {} -p {} init -r", wallet_binary_path, network, password);

    let mut recover_process = expectrl::spawn(command).expect("Can't run init -r");
    // wait for 60 minutes
    let wait_time = Duration::from_secs(60 * 60);
    recover_process.set_expect_timeout(Some(wait_time));

    let recover_message = recover_process
        .expect("Please enter your recovery phrase:")
        .unwrap_or_else(|e| panic!("Can't get the recovery mesage, error: {:?}", e));

    recover_process
        .send_line(passphrase)
        .unwrap_or_else(|e| panic!("Can't communicate to the child process, error: {:?}", e));

    let finish_recover = recover_process
        .expect("Command 'init' completed successfully")
        .unwrap_or_else(|e| panic!("Can't finish recovery process, error: {:?}", e));

    let kill = recover_process.exit(true);

    match kill {
        Ok(bool) => assert!(bool, "Can't kill recovery process"),
        Err(e) => panic!("Error on stop the recovery process, error: {:?}", e),
    }
}

/// Run ./epic-wallet --network and take all values in info
/// return Vec<f32> with 7 values where the values are
/// \[chain_height, Confirmed Total, Immature Coinbase,
/// Awaiting Confirmation, Awaiting Finalization, Locked by previous transaction,
/// Currently Spendable\]
pub fn info_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) -> Vec<f32> {
    let info = match chain_type {
        ChainTypes::UserTesting => Command::new(binary_path)
            .args(["-p", password, "--usernet", "info"])
            .output()
            .expect("Failed get info a wallet"),
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["-p", password, "--floonet", "info"])
            .output()
            .expect("Failed get info a wallet"),
        _ => Command::new(binary_path)
            .args(["-p", password, "info"])
            .output()
            .expect("Failed get info a wallet"),
    };
    // binary to string
    let info_str = String::from_utf8_lossy(&info.stdout).into_owned();

    // split by " " space
    let info_split: Vec<&str> = info_str.split(' ').collect();
    // split by \n; | and ' '

    // f32, return only numbers between space ' '
    let values: Vec<f32> = info_split
        .into_iter()
        .flat_map(|x| x.parse::<f32>())
        .collect();
    values
}

/// Check if locked coins == 0, await for 5 five minutes to break
pub async fn confirm_transaction(chain_type: &ChainTypes, binary_path: &str, password: &str) {
    // time dependence
    let mut t0 = Instant::now();
    let n_minute = Duration::from_secs(10 * 60);
    let values_info = info_wallet(chain_type, binary_path, password);
    let struct_values = InfoWallet::from(values_info);
    let mut locked_0 = struct_values.locked_by_previus_transaction;
    let mut awaiting_0 = struct_values.awaiting_finalization;
    while t0.elapsed() < n_minute {
        let values_info = info_wallet(chain_type, binary_path, password);
        let struct_values = InfoWallet::from(values_info);
        let locked = struct_values.locked_by_previus_transaction;
        let awaiting = struct_values.awaiting_finalization;
        if locked > 0.0 && awaiting > 0.0 {
            wait_for(15).await;
        } else {
            if locked_0 > locked && locked != 0.0 {
                println!("	Reset time, The new info is {:?}", struct_values);
                t0 = Instant::now();
                locked_0 = locked.clone();
            } else if awaiting_0 > awaiting && awaiting != 0.0 {
                println!("	Reset time, The new info is {:?}", struct_values);
                t0 = Instant::now();
                awaiting_0 = awaiting.clone();
            } else {
                println!(
					"Can't confirm all transactions, Missing confirmation {:?} coins\n\nWe got this info: {:#?}",
					locked, struct_values
				);
                break;
            }
        }
    }
}

/// Check if locked coins == 0, await for 2 minutes to break
pub async fn check_spendable(
    chain_type: &ChainTypes,
    binary_path: &str,
    password: &str,
    need_amount: &f32,
) {
    // time dependence
    let t0 = Instant::now();
    let two_minute = Duration::from_secs(120);

    let mut info = info_wallet(chain_type, binary_path, password);
    let info_struct = InfoWallet::from(info);
    let mut current_spendable = &info_struct.currently_spendable;
    while current_spendable < need_amount && t0.elapsed() < two_minute {
        wait_for(10).await;
        info = info_wallet(chain_type, binary_path, password);
        current_spendable = info.last().expect("Can't get the current spendable!");
    }
}

/// Run wallet_info and get chain_height from title
pub fn get_chain_height(chain_type: &ChainTypes, binary_path: &str, password: &str) -> i32 {
    let values_info = info_wallet(chain_type, binary_path, password);
    values_info[0] as i32
}

/// new_empty child command to build default structs
pub fn new_child() -> Child {
    //Command::new("").spawn().expect("Failed on run a empty Child process")
    Command::new("echo")
        .arg("")
        .spawn()
        .expect("Failed on run a empty Child process")
}

/// new_empty output command to build default structs
pub fn new_output() -> Output {
    //Command::new("").output().expect("Failed on run a empty Output process")
    Command::new("echo")
        .arg("")
        .output()
        .expect("Failed on run a empty Output process")
}

/// Run ./epic-wallet --network txs
/// return String with all message
pub fn txs_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) -> String {
    let txs = match chain_type {
        ChainTypes::UserTesting => Command::new(binary_path)
            .args(["-p", password, "--usernet", "txs"])
            .output()
            .expect("Failed get txs info a wallet"),
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["-p", password, "--floonet", "txs"])
            .output()
            .expect("Failed get txs info a wallet"),
        _ => Command::new(binary_path)
            .args(["-p", password, "txs"])
            .output()
            .expect("Failed get txs info a wallet"),
    };
    // binary to string
    let txs_str = String::from_utf8_lossy(&txs.stdout).into_owned();

    txs_str
}

/// Run ./epic-wallet --network outputs
/// return String with all message
pub fn outputs_wallet(
    chain_type: &ChainTypes,
    binary_path: &str,
    password: &str,
    show_full_history: bool,
) -> String {
    let outputs = if show_full_history {
        match chain_type {
            ChainTypes::UserTesting => Command::new(binary_path)
                .args([
                    "-p",
                    password,
                    "--usernet",
                    "outputs",
                    "--show_full_history",
                ])
                .output()
                .expect("Failed get txs info a wallet"),
            ChainTypes::Floonet => Command::new(binary_path)
                .args([
                    "-p",
                    password,
                    "--floonet",
                    "outputs",
                    "--show_full_history",
                ])
                .output()
                .expect("Failed get txs info a wallet"),
            _ => Command::new(binary_path)
                .args(["-p", password, "outputs", "--show_full_history"])
                .output()
                .expect("Failed get outputs info a wallet"),
        }
    } else {
        match chain_type {
            ChainTypes::UserTesting => Command::new(binary_path)
                .args(["-p", password, "--usernet", "outputs"])
                .output()
                .expect("Failed get txs info a wallet"),
            ChainTypes::Floonet => Command::new(binary_path)
                .args(["-p", password, "--floonet", "outputs"])
                .output()
                .expect("Failed get txs info a wallet"),
            _ => Command::new(binary_path)
                .args(["-p", password, "outputs"])
                .output()
                .expect("Failed get outputs info a wallet"),
        }
    };
    // binary to string
    let outputs_str = String::from_utf8_lossy(&outputs.stdout).into_owned();

    outputs_str
}

/// return Vec<u32> with 3 values where the values are
/// [Sent Tx, Received Tx, Confirmed Coinbase]
pub fn get_number_transactions_txs(
    chain_type: &ChainTypes,
    binary_path: &str,
    password: &str,
) -> Vec<u32> {
    let txs_str = txs_wallet(chain_type, binary_path, password);

    // Count
    let sent_receive_coinbase = vec![
        txs_str.matches("Sent Tx").count() as u32,
        txs_str.matches("Received Tx").count() as u32,
        txs_str.matches("Confirmed").count() as u32 - 1, // attempt to subtract with overflow (It will probably give an overflow error if you don't find any Confirmed and remove 1 so it's -1 at u32;)
    ]; // -1 because header of txs command have "Confirmed?"
    sent_receive_coinbase
}

/// return Vec<u32> with X values where the values are
// [a,b,...]
pub fn get_number_outputs(
    chain_type: &ChainTypes,
    binary_path: &str,
    password: &str,
    show_full_history: bool,
) -> Vec<u32> {
    let outputs_str = outputs_wallet(chain_type, binary_path, password, show_full_history);

    // Count
    let vec_outputs = vec![
        outputs_str.matches("Unconfirmed").count() as u32,
        outputs_str.matches("Unspent").count() as u32,
        outputs_str.matches("Locked").count() as u32,
        outputs_str.matches("Spent").count() as u32,
        outputs_str.matches("Deleted").count() as u32,
        outputs_str.matches("Mining").count() as u32, // Unconfirmed && is_coinbase
        outputs_str.matches("true").count() as u32,   // is_coinbase
    ];
    vec_outputs
}

/// Run ./epic-wallet --network scan
pub fn scan_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) -> bool {
    let scan = match chain_type {
        ChainTypes::UserTesting => Command::new(binary_path)
            .args(["-p", password, "--usernet", "scan"])
            .output()
            .expect("Failed get scan a wallet"),
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["-p", password, "--floonet", "scan"])
            .output()
            .expect("Failed get scan a wallet"),
        _ => Command::new(binary_path)
            .args(["-p", password, "scan"])
            .output()
            .expect("Failed get scan a wallet"),
    };

    // binary to string
    let scan_str = String::from_utf8_lossy(&scan.stdout).into_owned();

    scan_str.contains("successfully")
}

/// run the command ./epic-wallet --network -p <password> <receive|finalize> -m <method> -i <emoji|file>
pub fn receive_finalize_coins(
    chain_type: &ChainTypes,
    binary_path: &String,
    method: String,
    password: &String,
    receive_finalize: &String,
    destination: &str,
) -> Output {
    let network = match chain_type {
        ChainTypes::Floonet => "--floonet",
        ChainTypes::UserTesting => "--usernet",
        ChainTypes::Mainnet => "",
        _ => panic!("Specified network does not exist!"),
    };

    let output = match chain_type {
        ChainTypes::Mainnet => Command::new(&binary_path)
            .args([
                "-p",
                password.as_str(),
                receive_finalize,
                "-m",
                method.as_str(),
                "-i",
                destination,
            ])
            .output()
            .expect("failed to execute process"),
        _ => Command::new(&binary_path)
            .args([
                "-p",
                password.as_str(),
                network,
                receive_finalize,
                "-m",
                method.as_str(),
                "-i",
                destination,
            ])
            .output()
            .expect("failed to execute process"),
    };
    output
}

/// Code to return the output referring to the peers
pub fn get_list_peers(chain_type: &ChainTypes, binary_path: &str) -> Output {
    let list_peers = match chain_type {
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["--floonet", "client", "listconnectedpeers"])
            .output()
            .expect("Failed on init a wallet"),
        _ => Command::new(binary_path)
            .args(["client", "listconnectedpeers"])
            .output()
            .expect("Failed on init a wallet"),
    };
    list_peers
}

/// Run the command `epic --chain_type client status` and get the list of peers in Output format
pub fn get_status(chain_type: &ChainTypes, binary_path: &str) -> Output {
    let list_peers = match chain_type {
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["--floonet", "client", "status"])
            .output()
            .expect("Failed on get status from node"),
        _ => Command::new(binary_path)
            .args(["client", "status"])
            .output()
            .expect("Failed on get status from node"),
    };
    list_peers
}
