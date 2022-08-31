// std
use std::process::{Command, Child};

// Testing
use testing::{spawn_network, wait_for, get_test_configuration};

// Epir Server
use epic_core::global::ChainTypes;

// For function
use std::time::{Duration, Instant};
use std::thread::sleep;

// Spawn wallet process by chain type
pub fn spawn_wallet(chain_type: &ChainTypes, binary_path: &str) -> Child {
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


fn main() {

    let chain_type = ChainTypes::UserTesting;
    get_test_configuration(&chain_type);
    wait_for(2);

    let binary = "/home/ba/Desktop/EpicV3/epic/target/release/epic";
    let serv = spawn_network(&chain_type, binary);

    let wallet = spawn_wallet(&chain_type, binary);



    wait_for(5);

    
    
    println!("---------- END OF TEST ----------")
} 