// std
use std::{process::{Command, Child, Output}, array};
use std::io;
use std::io::Write;

// Testing
use testing::{spawn_network, wait_for, get_test_configuration};

// Epir Server
use epic_core::global::ChainTypes;

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

fn create_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) -> Output {
    let mut wallet = match chain_type {
        ChainTypes::UserTesting => {
            Command::new(binary_path)
                    .args(["-p", password, "--usernet", "init"])
                    .output().expect("Failed on init a wallet")
        },
        ChainTypes::Floonet => {
            Command::new(binary_path)
                    .args(["-p", password, "--floonet", "init"])
                    .output().expect("Failed on init a wallet")
        },
        _ => {
            Command::new(binary_path)
                    .args(["-p", password, "init"])
                    .output().expect("Failed on init a wallet")
        },
    };
    wallet
}

fn check_stdout(output_stdout: &Vec<u8>) -> Vec<&str> {
    
    let str_msg = String::from_utf8_lossy(&output_stdout).to_string();
    
    let ss = str_msg.clone();

    let result:Vec<&str> = if str_msg.contains("successfully") {
        ss.split("\n\n").collect()
    } else {
        ss.split("\n").collect()
    };
    result
}

#[warn(unused_variables)]
fn main() {

    let chain_type = ChainTypes::UserTesting;
    let password = "1";
    get_test_configuration(&chain_type);
    wait_for(5);

    let server_binary = "/home/ba/Desktop/EpicV3/epic/target/release/epic";
    let wallet_binary = "/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet";
    //let mut serv = spawn_network(&chain_type, server_binary);

    let mut te = Command::new(server_binary).args(["--usernet", "--onlyrandomx"]).spawn().expect("msg");
    
    println!("----------------------------");
    
    let mut wallet = create_wallet(&chain_type, wallet_binary, password);
    println!("{:#?}", check_stdout(&wallet.stdout));
    if wallet.status.success() {
      println!("----Sucesso---");  
    } 


    wait_for(1);

    te.kill();
    //serv.kill().expect("Fail to kill server process!");
    //wallet.kill().expect("Fail to kill wallet process!");
    
    println!("---------- END OF TEST ----------")
} 