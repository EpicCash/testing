use std::hash::Hash;
use std::path::PathBuf;
// use std::env::current_dir;
// use std::path::PathBuf;
// use std::fs::create_dir_all;
use std::process::{Command, Child};
use std::collections::HashMap;

use std::fs::{File, remove_file};
//use std::io::BufReader;
use std::io::prelude::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dirs::home_dir;
use epic_p2p::msg::PeerAddrs;
use epic_p2p::{PeerAddr, Peer};
use testing::{TEST_EPIC_HOME, TEST_SERVER_CONFIG_FILE_NAME, TEST_SERVER_LOG_FILE_NAME, TEST_EPIC_CHAIN_DIR, TEST_API_SECRET_FILE_NAME};

//Epir Server
use epic_config::config::{init_api_secret, initial_setup_server, SERVER_CONFIG_FILE_NAME};
use epic_core::global::ChainTypes;
use epic_config::types::ConfigError;
use epic_config::GlobalConfig;
use epic_p2p::types::Seeding;



// /// The default file name to use when trying to derive
// /// the node config file location
// pub const SERVER_CONFIG_FILE_NAME: &'static str = "epic-server.toml";
// pub const SERVER_LOG_FILE_NAME: &'static str = "epic-server.log";
// pub const TEST_FOLDER_NAME: &'static str = ".epic";
// pub const EPIC_CHAIN_DIR: &'static str = "chain_data";
// /// Node API secret
// pub const API_SECRET_FILE_NAME: &'static str = ".api_secret";


// fn get_epic_test_path(chain_type: ChainTypes) -> Result<PathBuf, ConfigError> {
//     // Check if epic dir exists
//     let mut epic_path = match current_dir() {
//         Ok(p) => p,
//         Err(_) => PathBuf::new(),
//     };
//     epic_path.push(TEST_FOLDER_NAME);
//     epic_path.push(chain_type.shortname());
//     // Create if the default path doesn't exist
//     if !epic_path.exists() {
//         create_dir_all(epic_path.clone())?;
//     }
//     Ok(epic_path)
// }

fn spawn_network(chain_type: ChainTypes, binary_path: &str) -> Child {
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

fn chain_type_to_str(chain_type: ChainTypes) -> String {
    chain_type.shortname()
}

// Don't check if exist, just build toml default path
fn generate_toml_path(chain_type: &ChainTypes) -> PathBuf {
    let mut toml_path = match home_dir() {
		Some(p) => p,
		None => PathBuf::new(),
	};
	toml_path.push(TEST_EPIC_HOME);
	toml_path.push(chain_type.shortname());
    toml_path.push(TEST_SERVER_CONFIG_FILE_NAME);
    toml_path
}

fn str_to_chain_type(shortname: &str) -> ChainTypes {
    match shortname {
        "auto" => ChainTypes::AutomatedTesting,
        "user" => ChainTypes::UserTesting,
        "floo" => ChainTypes::Floonet,
        "main" => ChainTypes::Mainnet,
        _ => panic!("Specified network does not exist!")
    }
}

// Prepare the ip "a.b.c.d:port" to write to the server's toml
fn get_ip_new(ip_v4: &str) -> PeerAddr {
    let ip_floonet_vm:SocketAddr = ip_v4.parse().expect("Can't change the IPV4 into SocketAddr");
    let ip_1 = PeerAddr(ip_floonet_vm);
    ip_1
}


fn change_server_toml_by_chain(toml_path: PathBuf ,chain_type: &ChainTypes) {
    // If the .epic/network folder doesn't exist => create and generate the default toml with the name "epic-server.toml"
    // If the folder and file already exist it creates and overwrites "epic-server.toml" and ".api_secret"
    let mut server_toml = initial_setup_server(chain_type).unwrap();

    // Change the run tui to off, set true if you want to run the server in terminal (don't recommended because you run test)
    server_toml.members.as_mut().unwrap()
                .server.run_tui = Some(false);

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
fn get_test_configuration(chain_type: &ChainTypes) {    
    // Just return path, don't change nothing
    let toml_path = generate_toml_path(chain_type);

    // make the steps
    change_server_toml_by_chain(toml_path, chain_type);
}

fn main() {
    let chain_type = ChainTypes::Floonet;
    //let toml_path = generate_toml_path(&chain_type);

    // let mut c = GlobalConfig::new(toml_path.to_str().unwrap()).unwrap();
    
    // let nee = c.ser_config();

    // println!(" {:#?}", nee);

    // c.write_to_file(toml_path.to_str().unwrap()).expect("Can't save custom toml file");

    get_test_configuration(&chain_type);
    // let ori = &mut c.unwrap();
    // let edi = &mut get_test_configuration(&chain_type);
    // let from_origin = &ori.members.as_mut().unwrap()
    //                                 .server.p2p_config
    //                                 .seeds;
    // let from_edit = &edi.members.as_mut().unwrap()
    //                                 .server.p2p_config
    //                                 .seeds;

    // println!("%% {:#?}", &from_origin);

    // assert_eq!(from_origin, from_edit)


    
    // let toml_path = generate_toml_path(&chain_type);//PathBuf::from("/home/ba/.epic/user/epic-server_.toml");
// 
    // let binary = "/home/ba/Desktop/EpicV3/epic/target/release/epic";
    
    //let mut a = GlobalConfig::for_chain(&chain_type);
    //a.members.as_mut().unwrap().server.stratum_mining_config.as_mut().unwrap().stratum_server_addr = Some("127.0.0.1:22222".to_owned());
    
    //println!("for chain {:#?}\n ---------", a);
    // let g = change_server_toml_by_chain(toml_path, chain_type);
    //let b = initial_setup_server(&chain_type);
    // let mut b = match GlobalConfig::new(toml_path) {
    //     Ok(glob) => glob,
    //     Err(e) => panic!("Can't open toml file, error {:?}", e),
    // };
    //b.members.as_mut().unwrap().server.stratum_mining_config.as_mut().unwrap().stratum_server_addr = Some("127.0.0.1:3333".to_owned());
    //b.write_to_file(toml_path).expect("AA");
    //println!("new {:#?}", b)
    println!("FINALIZOU")
} 