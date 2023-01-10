use chrono::{
    DateTime,
    //Utc,
    Local,
};
use dirs::home_dir;
use log::Level;
use rand::{self, distributions::Uniform, Rng};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Output;
use std::thread::sleep;
use std::time::Duration;

// Epic Server
use epic_config::config::initial_setup_server;
use epic_core::global::ChainTypes;
use epic_p2p::{types::Seeding, PeerAddr};

/// The default file name to use when trying to derive
/// the node config file location
pub const TEST_SERVER_CONFIG_FILE_NAME: &'static str = "epic-server.toml";
/// default server log file
pub const TEST_SERVER_LOG_FILE_NAME: &'static str = "epic-server.log";
/// default epic folder
pub const TEST_EPIC_HOME: &'static str = ".epic";
/// default chain folder
pub const TEST_EPIC_CHAIN_DIR: &'static str = "chain_data";
/// Node API secret
pub const TEST_API_SECRET_FILE_NAME: &'static str = ".api_secret";

/// Force the code to await for secs seconds,
pub fn wait_for(secs: u64) {
    let duration = Duration::from_secs(secs);
    sleep(duration);
}

/// ChainType to str shortname
pub fn chain_type_to_str(chain_type: ChainTypes) -> String {
    chain_type.shortname()
}

/// str shortname to ChainTypes
pub fn str_to_chain_type(shortname: &str) -> ChainTypes {
    match shortname {
        "auto" => ChainTypes::AutomatedTesting,
        "user" | "usernet" => ChainTypes::UserTesting,
        "floo" | "floonet" => ChainTypes::Floonet,
        "main" | "mainnet" => ChainTypes::Mainnet,
        _ => panic!("Specified network does not exist!"),
    }
}

/// Prepare the ip "a.b.c.d:port" to write to the server's toml
pub fn get_ip_new(ip_v4: &str) -> PeerAddr {
    let ip_floonet_vm: SocketAddr = ip_v4
        .parse()
        .expect("Can't change the IPV4 into SocketAddr");
    let ip_1 = PeerAddr(ip_floonet_vm);
    ip_1
}

/// Configure the epic-servert.toml to custom configuration
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

/// Generate the .epic folder, epic-server.toml, .api_secret and change the toml to special configuration
pub fn get_test_configuration(chain_type: &ChainTypes) {
    // Just return path, don't change nothing
    let toml_path = generate_toml_path(chain_type);

    // make the steps
    change_server_toml_by_chain(toml_path, chain_type);
}

/// Return default epic home dir
pub fn get_home_chain(chain_type: &ChainTypes) -> PathBuf {
    let mut home_path = match home_dir() {
        Some(p) => p,
        None => PathBuf::new(),
    };
    home_path.push(TEST_EPIC_HOME);
    home_path.push(chain_type.shortname());
    home_path
}

/// Don't check if exist, just build toml default path
pub fn generate_toml_path(chain_type: &ChainTypes) -> PathBuf {
    let mut toml_path = get_home_chain(chain_type);
    toml_path.push(TEST_SERVER_CONFIG_FILE_NAME);
    toml_path
}

/// Entry is a wallet init output and return the passprhase
pub fn get_passphrase(output: &Output) -> String {
    // String of message
    let output_msg = String::from_utf8_lossy(&output.stdout).into_owned();

    // Split the message into a vector
    let output_msg_vec = output_msg.split("\n").collect::<Vec<&str>>();

    // if string.len() > 55 so it's passphrase (possibly)
    // This value needs to be greater than 51 because the sentence "Please back-up these words in a non-digital format." has length = 51;
    let limit = 55;

    let mut result: Option<&str> = None;
    for text in output_msg_vec {
        if text.len() > limit {
            result = Some(text)
        };
    }
    match result {
        Some(passphrase) => String::from(passphrase),
        None => panic!("Can't get passphrase from output: {:?}", &output_msg),
    }
}

/// Delete current .epic/network/wallet_data and copy and paste the stored wallet
/// TODO
pub fn use_stored_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) {
    todo!()
}

/// Return http send destination
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

/// Need to create the send file method
pub fn local_now_str() -> String {
    let now: DateTime<Local> = Local::now();
    let now_string = format!("{}", now.format("%Y-%m-%d_%H-%M-%S"));
    now_string
}

/// Return .txt file to send method
pub fn generate_file_name() -> String {
    let name = local_now_str();
    let sent_file_name = format!("{}.txt", name);
    sent_file_name
}

/// Return response file based on send file
pub fn generate_response_file_name(sent_file_name: &String) -> String {
    let response_file_name = format!("{}.response", sent_file_name);
    response_file_name
}

/// Code to return a vector of chain heights of all connected peers
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

/// Return max height from vector of chain height from peers
pub fn get_chain_height_peers(vec_height: Vec<i32>) -> i32 {
    let max_height = vec_height.iter().max();
    match max_height {
        Some(max) => max.to_owned(),
        None => 0,
    }
}

/// Check if we have peers to hear
pub fn check_peers(vec_height: Vec<i32>) {
    assert!(vec_height.len() > 0)
}

/// Get chain height from epic status
pub fn get_height_from_status(output: &Output) -> i32 {
    // String of message
    let output_msg = String::from_utf8_lossy(&output.stdout).into_owned();

    // Split the message into a vector
    let output_msg_vec = output_msg.split("\nChain height: ").collect::<Vec<&str>>();
    let all_splits = output_msg_vec[1].split("\n").collect::<Vec<&str>>();
    let height: i32 = all_splits[0].parse().unwrap();

    height
}

/// Create a vector with number_elements of random numbers between [0.min_include, 0.max_exclude) contained in the (0,1)
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
