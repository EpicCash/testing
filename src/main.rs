// std
use std::{process::{Command, Child, Output}, array, str::Split, path::PathBuf};
//use std::f32;
//use std::env;
//use std::fs::remove_dir_all;

// Testing
use testing::{spawn_network, wait_for, spawn_wallet_listen, create_wallet, send_coins_smallest,
    spawn_miner, get_number_transactions_txs, info_wallet, receive_finalize_coins,
    generate_response_file_name, generate_file_name};
use testing::get_test_configuration;


// Epir Server
use epic_core::global::ChainTypes;
//use epic_config::GlobalConfig;

//#[derive(Serialize)]
//struct Server_Toml {
//    pub run_tui: Option<bool>,
//    pub stdout_log_level: Level,
//    pub file_log_level: Level,
//    pub stratum_server_addr: Option<String>,
//    pub seeding_type: Seeding,
//    pub seeds: Option<Vec<PeerAddr, Global>>,
//}

#[allow(unused_variables)]
fn main() {
    let chain_type = ChainTypes::UserTesting;
    let password = "1";

    get_test_configuration(&chain_type);
    
    wait_for(5);

    let server_binary = "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic\\target\\release\\epic.exe";//"/home/ba/Desktop/EpicV3/epic/target/release/epic";
    let wallet_binary = "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic-wallet\\target\\release\\epic-wallet.exe";//"/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet";
    let miner_binary = "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic-miner\\epic-miner.exe";//"/home/ba/Desktop/epic-miner/target/debug/epic-miner";
    let mut serv = spawn_network(&chain_type, server_binary);

    let created_wallet = create_wallet(&chain_type, wallet_binary, password);
    //let mut te = Command::new(server_binary).args(["--usernet", "--onlyrandomx"]).spawn().expect("msg");

    wait_for(4);

    let mut listen = spawn_wallet_listen(&chain_type, wallet_binary, password);

    wait_for(5);

    let mut miner = spawn_miner(miner_binary);
    wait_for(180);

    let file_name = generate_file_name();
    let response_file_name = generate_response_file_name(&file_name);

    let a = send_coins_smallest(&chain_type, &wallet_binary.to_string(), "file".to_string(), &"1".to_string(), String::from("1"), &file_name);

    let b = receive_finalize_coins(&chain_type, &wallet_binary.to_string(), "file".to_string(), &"1".to_string(), &"receive".to_string(),&file_name);

    let c = receive_finalize_coins(&chain_type, &wallet_binary.to_string(), "file".to_string(), &"1".to_string(), &"finalize".to_string(),&response_file_name);

    println!("====== SEND COINS {:?} ====== \n", a);

    println!("\n ====== RECEIVED COINS {:?} ====== \n", b);

    println!("\n ====== FINALIZED COINS {:?} ====== \n", c);

    // let info_str = String::from_utf8_lossy(&info.stdout).into_owned();
    // let info_split = info_str.split(' ').collect::<Vec<&str>>();
    

    // let values: Vec<f32> = info_split
    //                             .into_iter()
    //                             .flat_map(|x| x.parse::<f32>())
    //                             .collect();
    //                             //     if x.contains(".") {
    //                             //         x
    //                             //     } else {
    //                             //         ""
    //                             //     }
    //                             // })
    //                             //.collect();

    //println!("INFO {:#?} \n", info_split);
    // println!("INFO0 {:#?} \n", info_str.split(&['\n','|',' ']).collect::<Vec<&str>>());
    // println!("COLLECT {:#?} \n", values);

    
    listen.kill().expect("Failed on kill wallet");
    serv.kill().expect("Failed on kill server");
    miner.kill().expect("Failed on kill miner");

    println!("---------- END OF TEST ----------")
} 