use chrono::{
    DateTime,
    //Utc,
    Local,
};
use dirs::home_dir;
use expectrl;
use log::Level;
use rand::{self, distributions::Uniform, Rng};
use std::fs::remove_dir_all;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{Child, Command, Output};
use std::thread::sleep;
use std::time::{Duration, Instant};

// Epic Server
use epic_config::config::initial_setup_server;
use epic_core::global::ChainTypes;
use epic_p2p::{types::Seeding, PeerAddr};

/// The default file name to use when trying to derive
/// the node config file location
pub const TEST_SERVER_CONFIG_FILE_NAME: &'static str = "epic-server.toml";
pub const TEST_SERVER_LOG_FILE_NAME: &'static str = "epic-server.log";
pub const TEST_EPIC_HOME: &'static str = ".epic";
pub const TEST_EPIC_CHAIN_DIR: &'static str = "chain_data";
/// Node API secret
pub const TEST_API_SECRET_FILE_NAME: &'static str = ".api_secret";

// Force the code to await for secs seconds,
pub fn wait_for(secs: u64) {
    let duration = Duration::from_secs(secs);
    sleep(duration);
}

// ChainType to str shortname
pub fn chain_type_to_str(chain_type: ChainTypes) -> String {
    chain_type.shortname()
}

// str shortname to ChainTypes
pub fn str_to_chain_type(shortname: &str) -> ChainTypes {
    match shortname {
        "auto" => ChainTypes::AutomatedTesting,
        "user" | "usernet" => ChainTypes::UserTesting,
        "floo" | "floonet" => ChainTypes::Floonet,
        "main" | "mainnet" => ChainTypes::Mainnet,
        _ => panic!("Specified network does not exist!"),
    }
}

// Prepare the ip "a.b.c.d:port" to write to the server's toml
pub fn get_ip_new(ip_v4: &str) -> PeerAddr {
    let ip_floonet_vm: SocketAddr = ip_v4
        .parse()
        .expect("Can't change the IPV4 into SocketAddr");
    let ip_1 = PeerAddr(ip_floonet_vm);
    ip_1
}

// Spawn server process by chain type
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

// Configure the epic-servert.toml to custom configuration
pub fn change_server_toml_by_chain(toml_path: PathBuf, chain_type: &ChainTypes) {
    // If the .epic/network folder doesn't exist => create and generate the default toml with the name "epic-server.toml"
    // If the folder and file already exist it creates and overwrites "epic-server.toml" and ".api_secret"
    let mut server_toml = initial_setup_server(chain_type).unwrap();

    // Change the run tui to off, set true if you want to run the server in terminal (don't recommended because you run test)
    server_toml.members.as_mut().unwrap().server.run_tui = Some(false);

    server_toml
        .members
        .as_mut()
        .unwrap()
        .logging
        .as_mut()
        .unwrap()
        .stdout_log_level = Level::Error;

    server_toml
        .members
        .as_mut()
        .unwrap()
        .logging
        .as_mut()
        .unwrap()
        .file_log_level = Level::Debug;

    match chain_type {
        ChainTypes::UserTesting => {
            // Change the stratum_server_addr port to miner
            server_toml
                .members
                .as_mut()
                .unwrap()
                .server
                .stratum_mining_config
                .as_mut()
                .unwrap()
                .stratum_server_addr = Some("127.0.0.1:13416".to_owned());
        }
        ChainTypes::Floonet => {
            // Change the seeding_type to List
            server_toml
                .members
                .as_mut()
                .unwrap()
                .server
                .p2p_config
                .seeding_type = Seeding::List;

            let ip_1 = get_ip_new("15.229.31.27:23414");
            let mut vec_ip: Vec<PeerAddr> = Vec::new();
            vec_ip.push(ip_1);

            // Change the seeding_type to List
            server_toml
                .members
                .as_mut()
                .unwrap()
                .server
                .p2p_config
                .seeds = Some(vec_ip);
        }
        //For now mainnet can run with default configuration
        ChainTypes::Mainnet => {}
        _ => panic!("Specified network does not exist!"),
    };
    let mut server = server_toml.to_owned();
    server
        .write_to_file(toml_path.to_str().unwrap())
        .expect("Can't save custom toml file");
}

// Generate the .epic folder, epic-server.toml, .api_secret and change the toml to special configuration
pub fn get_test_configuration(chain_type: &ChainTypes) {
    // Just return path, don't change nothing
    let toml_path = generate_toml_path(chain_type);

    // make the steps
    change_server_toml_by_chain(toml_path, chain_type);
}

// Don't check if exist, just build toml default path
pub fn get_home_chain(chain_type: &ChainTypes) -> PathBuf {
    let mut home_path = match home_dir() {
        Some(p) => p,
        None => PathBuf::new(),
    };
    home_path.push(TEST_EPIC_HOME);
    home_path.push(chain_type.shortname());
    home_path
}

// Don't check if exist, just build toml default path
pub fn generate_toml_path(chain_type: &ChainTypes) -> PathBuf {
    let mut toml_path = get_home_chain(chain_type);
    toml_path.push(TEST_SERVER_CONFIG_FILE_NAME);
    toml_path
}

// Entry is a wallet init output and return the passprhase
pub fn get_passphrase(output: &Output) -> String {
    // String of message
    let output_msg = String::from_utf8_lossy(&output.stdout).into_owned();

    // Split the message into a vector
    let output_msg_vec = output_msg.split("\n").collect::<Vec<&str>>();

    // If we got a error on init a new wallet, the vector will have only 4 element
    let result = match output_msg_vec.len() > 5 {
        true => output_msg_vec[3].to_owned(),
        false => panic!("Failed to get passphrase from wallet init!"),
    };
    result
}

pub fn remove_wallet_path(chain_type: &ChainTypes) {
    let mut wallet_data_path = get_home_chain(chain_type);
    wallet_data_path.push("wallet_data");

    // if wallet_data exist -> remove
    if wallet_data_path.exists() {
        remove_dir_all(wallet_data_path).expect("Failed on remove old wallet_data");
    }
}
// Run the init command, if the wallet_data exist -> delete and create a new one
pub fn create_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) -> Output {
    // .epic/user ; .epic/floo or .epic/main
    // if wallet_data exist -> remove
    remove_wallet_path(chain_type);
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

// Spawn a wallet in listen mode
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

// Spawn a miner
pub fn spawn_miner(binary_path: &str) -> Child {
    Command::new(&binary_path)
        .spawn()
        .expect("Failed on start the miner")
}

// run the command ./epic-wallet --network -p <password> send -m <method> -s smallest <amount>
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
    recover_process
        .expect("Please enter your recovery phrase:")
        .expect("Can't get the recovery mesage");
    recover_process
        .send_line(passphrase)
        .expect("Can't communicate to the child process");
    recover_process
        .expect("Command 'init' completed successfully")
        .expect("Can't finish recovery process");

    recover_process.exit(true);
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

// Check if locked coins == 0, await for 2 minutes to break
pub fn confirm_transaction(chain_type: &ChainTypes, binary_path: &str, password: &str) {
    // time dependence
    let t0 = Instant::now();
    let n_minute = Duration::from_secs(300);

    while t0.elapsed() < n_minute {
        let values_info = info_wallet(chain_type, binary_path, password);
        if values_info[5] > 0.0 {
            wait_for(15)
        } else {
            break;
        }
    }
}

// Check if locked coins == 0, await for 2 minutes to break
pub fn check_spendable(
    chain_type: &ChainTypes,
    binary_path: &str,
    password: &str,
    need_amount: &f32,
) {
    // time dependence
    let t0 = Instant::now();
    let two_minute = Duration::from_secs(120);

    let mut info = info_wallet(chain_type, binary_path, password);
    let mut current_spendable = info.last().expect("Can't get the current spendable!");
    while current_spendable < need_amount && t0.elapsed() < two_minute {
        wait_for(10);
        info = info_wallet(chain_type, binary_path, password);
        current_spendable = info.last().expect("Can't get the current spendable!");
    }
}

// Run wallet_info and get chain_height from title
pub fn get_chain_height(chain_type: &ChainTypes, binary_path: &str, password: &str) -> i32 {
    let values_info = info_wallet(chain_type, binary_path, password);
    values_info[0] as i32
}

// new_empty child command to build default structs
pub fn new_child() -> Child {
    //Command::new("").spawn().expect("Failed on run a empty Child process")
    Command::new("echo")
        .arg("")
        .spawn()
        .expect("Failed on run a empty Child process")
}

// new_empty output command to build default structs
pub fn new_output() -> Output {
    //Command::new("").output().expect("Failed on run a empty Output process")
    Command::new("echo")
        .arg("")
        .output()
        .expect("Failed on run a empty Output process")
}

// Run ./epic-wallet --network txs
// return Vec<usize> with 3 values where the values are
// [Sent Tx, Received Tx, Confirmed Coinbase]
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

pub fn get_http_wallet(chain_type: &ChainTypes) -> String {
    // TODO get from wallet toml (api_listen_interface = "127.0.0.1")
    let ip = "127.0.0.1";

    // TODO get from wallet toml (api_listen_port = 23415)
    let port = match chain_type {
        ChainTypes::Floonet => "13415",
        _ => "23415",
    };

    let http_ip = format!("http://{}:{}", ip, port);
    http_ip
}

// run the command ./epic-wallet --network -p <password> <receive|finalize> -m <method> -i <emoji|file>
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

pub fn local_now_str() -> String {
    let now: DateTime<Local> = Local::now();
    let now_string = format!("{}", now.format("%Y-%m-%d_%H-%M-%S"));
    now_string
}

pub fn generate_file_name() -> String {
    let name = local_now_str();
    let sent_file_name = format!("{}.txt", name);
    sent_file_name
}

pub fn generate_response_file_name(sent_file_name: &String) -> String {
    let response_file_name = format!("{}.response", sent_file_name);
    response_file_name
}

// Code to return a vector of heights of all connected peers
pub fn get_height_from_list_peers(output: &Output) -> Vec<i32> {
    // String of message
    let output_msg = String::from_utf8_lossy(&output.stdout).into_owned();

    // Split the message into a vector
    let output_msg_vec = output_msg.split("\nHeight: ").collect::<Vec<&str>>();
    let mut height_vec: Vec<i32> = Vec::new();
    for element in output_msg_vec {
        if element.contains("Total") {
            let all_splits = element.split("\n").collect::<Vec<&str>>();
            let b: i32 = all_splits[0].parse().unwrap();
            height_vec.push(b);
        }
    }
    height_vec
}

// Code to return the output referring to the peers
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

//
pub fn get_chain_height_peers(vec_height: Vec<i32>) -> i32 {
    let max_height = vec_height.iter().max();
    match max_height {
        Some(max) => max.to_owned(),
        None => 0,
    }
}

//
pub fn check_peers(vec_height: Vec<i32>) {
    assert!(vec_height.len() > 0)
}

//
pub fn get_height_from_status(output: &Output) -> i32 {
    // String of message
    let output_msg = String::from_utf8_lossy(&output.stdout).into_owned();

    // Split the message into a vector
    let output_msg_vec = output_msg.split("\nChain height: ").collect::<Vec<&str>>();
    let all_splits = output_msg_vec[1].split("\n").collect::<Vec<&str>>();
    let height: i32 = all_splits[0].parse().unwrap();

    height
}

//
pub fn get_status(chain_type: &ChainTypes, binary_path: &str) -> Output {
    let list_peers = match chain_type {
        ChainTypes::Floonet => Command::new(binary_path)
            .args(["--floonet", "client", "status"])
            .output()
            .expect("Failed on init a wallet"),
        _ => Command::new(binary_path)
            .args(["client", "status"])
            .output()
            .expect("Failed on init a wallet"),
    };
    list_peers
}

pub fn generate_vec_to_sent(
    min_include: i32,
    max_exclude: i32,
    number_elements: i32,
) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min_include, max_exclude); // [min, max)

    let vals: Vec<String> = (0..number_elements)
        .map(|_| format!("0.{}", rng.sample(&range).to_string()))
        .collect();
    vals
}
