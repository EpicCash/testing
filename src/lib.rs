use std::process::{Command, Child};
use std::path::PathBuf;
use std::net::SocketAddr;
use dirs::home_dir;
use log::Level;
use std::time::Duration;
use std::thread::sleep;

// Epic Server
use epic_core::global::ChainTypes;
use epic_p2p::{PeerAddr, types::Seeding};
use epic_config::config::initial_setup_server;


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
    println!("BEFORE SLEEP");
    let duration = Duration::from_secs(secs);
    sleep(duration);
    println!("AFTER SLEEP");
}

// Spawn server process by chain type
pub fn spawn_network(chain_type: &ChainTypes, binary_path: &str) -> Child {
    let output = match chain_type {
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
        _ => panic!("Specified network does not exist!")
    };
    // let output = if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //             .args(["/C", "echo hello"])
    //             .output()
    //             .expect("failed to execute server process")
    // } else {
    //     Command::new(binary_path)
    //             .arg("--floonet")
    //             .arg("echo hello")
    //             .output()
    //             .expect("failed to execute server process")
    // };
    output
}

// ChainType to str shortname
pub fn chain_type_to_str(chain_type: ChainTypes) -> String {
    chain_type.shortname()
}

// str shortname to ChainTypes
pub fn str_to_chain_type(shortname: &str) -> ChainTypes {
    match shortname {
        "auto" => ChainTypes::AutomatedTesting,
        "user" => ChainTypes::UserTesting,
        "floo" => ChainTypes::Floonet,
        "main" => ChainTypes::Mainnet,
        _ => panic!("Specified network does not exist!")
    }
}

// Prepare the ip "a.b.c.d:port" to write to the server's toml
pub fn get_ip_new(ip_v4: &str) -> PeerAddr {
    let ip_floonet_vm:SocketAddr = ip_v4.parse().expect("Can't change the IPV4 into SocketAddr");
    let ip_1 = PeerAddr(ip_floonet_vm);
    ip_1
}

// Configure the epic-servert.toml to custom configuration
pub fn change_server_toml_by_chain(toml_path: PathBuf ,chain_type: &ChainTypes) {
    // If the .epic/network folder doesn't exist => create and generate the default toml with the name "epic-server.toml"
    // If the folder and file already exist it creates and overwrites "epic-server.toml" and ".api_secret"
    let mut server_toml = initial_setup_server(chain_type).unwrap();

    // Change the run tui to off, set true if you want to run the server in terminal (don't recommended because you run test)
    server_toml.members.as_mut().unwrap()
                .server.run_tui = Some(false);
    
    server_toml.members.as_mut().unwrap()
                .logging.as_mut().unwrap()
                .stdout_log_level = Level::Error;
    
    server_toml.members.as_mut().unwrap()
                .logging.as_mut().unwrap()
                .file_log_level = Level::Debug;

    match chain_type {
        ChainTypes::UserTesting => {
            // Change the stratum_server_addr port to miner
            server_toml.members.as_mut().unwrap()
                        .server.stratum_mining_config.as_mut().unwrap()
                        .stratum_server_addr = Some("127.0.0.1:13416".to_owned());
        },
        ChainTypes::Floonet => {
            // Change the seeding_type to List
            server_toml.members.as_mut().unwrap()
                        .server.p2p_config
                        .seeding_type = Seeding::List;//Some("List".to_owned());
            
            // ip Floonet VM 15.229.31.27:23414
            //let ip_1 = get_ip(15, 229, 31, 27, 23414);
            let ip_1 = get_ip_new("15.229.31.27:23414");
            let mut vec_ip:Vec<PeerAddr> = Vec::new();
            vec_ip.push(ip_1);
            //let vec_ip = vec![ip_1];

            // Change the seeding_type to List
            server_toml.members.as_mut().unwrap()
                         .server.p2p_config
                         .seeds = Some(vec_ip);//Some(vec_ip.to_owned());
        },
        //For now mainnet can run with default configuration
        ChainTypes::Mainnet => {},
        _ => panic!("Specified network does not exist!"),
    };

    server_toml.write_to_file(toml_path.to_str().unwrap()).expect("Can't save custom toml file");
}


// Generate the .epic folder, epic-server.toml, .api_secret and change the toml to special configuration
pub fn get_test_configuration(chain_type: &ChainTypes) {    
    // Just return path, don't change nothing
    let toml_path = generate_toml_path(chain_type);

    // make the steps
    change_server_toml_by_chain(toml_path, chain_type);
}

// Don't check if exist, just build toml default path
pub fn generate_toml_path(chain_type: &ChainTypes) -> PathBuf {
    let mut toml_path = match home_dir() {
		Some(p) => p,
		None => PathBuf::new(),
	};
	toml_path.push(TEST_EPIC_HOME);
	toml_path.push(chain_type.shortname());
    toml_path.push(TEST_SERVER_CONFIG_FILE_NAME);
    toml_path
}