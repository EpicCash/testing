// std
use std::{process::{Command, Child, Output}, array, str::Split, path::PathBuf};
use std::f32;
//use std::fs::remove_dir_all;

// Testing
use testing::{spawn_network, wait_for, spawn_wallet_listen, spawn_miner, create_wallet, send_coins_smallest};
use testing::get_test_configuration;

// Epir Server
use epic_core::global::ChainTypes;



#[allow(unused_variables)]
fn main() {

    let chain_type = ChainTypes::UserTesting;
    let password = "1";
    
    get_test_configuration(&chain_type);
    
    wait_for(5);

    let server_binary = "/home/ba/Desktop/EpicV3/epic/target/release/epic";
    let wallet_binary = "/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet";
    let miner_binary = "/home/ba/Desktop/epic-miner/target/debug/epic-miner";
    let mut serv = spawn_network(&chain_type, server_binary);

    let created_wallet = create_wallet(&chain_type, wallet_binary, password);
    //let mut te = Command::new(server_binary).args(["--usernet", "--onlyrandomx"]).spawn().expect("msg");

    wait_for(4);

    let mut listen = spawn_wallet_listen(&chain_type, wallet_binary, password);

    wait_for(5);

    let mut miner = spawn_miner(miner_binary);

    wait_for(180);

    //let a = send_coins_smallest(&chain_type, wallet_binary.to_string(), "self".to_string(), "1".to_string(), String::from("1"), String::new());
    
    //println!("====== {:?} SEND COINS {:?} ======", a, String::from_utf8_lossy(&a.stdout));
    listen.kill().expect("Failed on kill wallet");
    serv.kill().expect("Failed on kill server");
    miner.kill().expect("Failed on kill miner");

    println!("---------- END OF TEST ----------")
} 