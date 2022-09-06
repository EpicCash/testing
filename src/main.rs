// std
use std::{process::{Command, Child, Output}, array, str::Split, path::PathBuf};
use std::f32;
use std::env;
//use std::fs::remove_dir_all;

// Testing
use testing::{spawn_network, wait_for, spawn_wallet_listen, create_wallet, send_coins_smallest,
    spawn_miner, txs_wallet, get_number_transactions_txs};
use testing::get_test_configuration;


// Epir Server
use epic_core::global::ChainTypes;
use epic_config::GlobalConfig;

//#[derive(Serialize)]
//struct Server_Toml {
//    pub run_tui: Option<bool>,
//    pub stdout_log_level: Level,
//    pub file_log_level: Level,
//    pub stratum_server_addr: Option<String>,
//    pub seeding_type: Seeding,
//    pub seeds: Option<Vec<PeerAddr, Global>>,
//}


// Run ./epic-wallet --network and take all values in info
// return Vec<f32> with 7 values where the values are 
// [chain_height, Confirmed Total, Immature Coinbase, 
// Awaiting Confirmation, Awaiting Finalization, Locked by previous transaction, 
// Currently Spendable]
pub fn info_wallet(chain_type: &ChainTypes, binary_path: &str, password: &str) -> Vec<f32> {
    let info = match chain_type {
        ChainTypes::UserTesting => {
            Command::new(binary_path)
                    .args(["-p", password, "--usernet", "info"])
                    .output().expect("Failed get info a wallet")
        },
        ChainTypes::Floonet => {
            Command::new(binary_path)
                    .args(["-p", password, "--floonet", "info"])
                    .output().expect("Failed get info a wallet")
        },
        _ => {
            Command::new(binary_path)
                    .args(["-p", password, "info"])
                    .output().expect("Failed get info a wallet")
        },
    };
    // binary to string
    let info_str = String::from_utf8_lossy(&info.stdout).into_owned();

    // split by " " space
    let info_split:Vec<&str> = info_str.split(' ').collect();
    // split by \n; | and ' '
    //let info_mult_split: Vec<&str> = info_str.split(&['\n','|',' ']).collect();

    // f32, return only numbers between space ' '
    let values: Vec<f32> = info_split
                                .into_iter()
                                .flat_map(|x| x.parse::<f32>())
                                .collect();
    values
}

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

    let a = send_coins_smallest(&chain_type, &wallet_binary.to_string(), "self".to_string(), &"1".to_string(), String::from("1"), String::new());
    
    println!("====== {:?} SEND COINS {:?} ======", a, String::from_utf8_lossy(&a.stdout));

    let info = info_wallet(&chain_type, wallet_binary, password);

    let txs = get_number_transactions_txs(&chain_type, wallet_binary, password);

    let c = txs[0];
    let d = txs[1];
    let e = txs[2];

    println!("\n TXS {:?} \n", txs);
    println!("\n COUND {:?} =  {:?} = {:?} \n ", c, d, e);
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

    println!("MSG {:#?}", info);
    //println!("INFO {:#?} \n", info_split);
    // println!("INFO0 {:#?} \n", info_str.split(&['\n','|',' ']).collect::<Vec<&str>>());
    // println!("COLLECT {:#?} \n", values);


    
    listen.kill().expect("Failed on kill wallet");
    serv.kill().expect("Failed on kill server");
    miner.kill().expect("Failed on kill miner");

    println!("---------- END OF TEST ----------")
} 