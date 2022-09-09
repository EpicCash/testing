// std
//use std::{process::{Command, Child, Output}, array, str::Split, path::PathBuf};
//use std::f32;
//use std::env;
//use std::fs::remove_dir_all;

// Testing
use testing::{create_wallet, get_passphrase, wait_for};


// // Epir Server
 use epic_core::global::ChainTypes;
// //use epic_config::GlobalConfig;

// //#[derive(Serialize)]
// //struct Server_Toml {
// //    pub run_tui: Option<bool>,
// //    pub stdout_log_level: Level,
// //    pub file_log_level: Level,
// //    pub stratum_server_addr: Option<String>,
// //    pub seeding_type: Seeding,
// //    pub seeds: Option<Vec<PeerAddr, Global>>,
// //}

// #[allow(unused_variables)]
// fn main() {
//     let chain_type = ChainTypes::UserTesting;
//     let password = "1";

//     //get_test_configuration(&chain_type);
    
//     //wait_for(5);

//     //let server_binary = "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic\\target\\release\\epic.exe";//"/home/ba/Desktop/EpicV3/epic/target/release/epic";
//     let wallet_binary = "/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet";
//     //let miner_binary = "C:\\Users\\T-Gamer\\Desktop\\Brick\\EpicCash\\epic-miner\\epic-miner.exe";//"/home/ba/Desktop/epic-miner/target/debug/epic-miner";
//     //let mut serv = spawn_network(&chain_type, server_binary);

//     let created_wallet = create_wallet(&chain_type, wallet_binary, password);
//     //let mut te = Command::new(server_binary).args(["--usernet", "--onlyrandomx"]).spawn().expect("msg");
//     let pass = get_passphrase(&created_wallet);

//     wait_for(2);
//     let input_args = format!("{} -p {} --usernet init -r", &wallet_binary, &password);
//     let mut a = Command::new("sh")
//                         .arg("-c")
//                         .arg(input_args)
//                         //.arg(pass)
//                         .spawn()
//                         .expect("Failed get txs info a wallet");
    
//     println!("\nBEFORE\n");
//     wait_for(10);

//     println!("====== create {:?} ====== \n", &created_wallet);

//     println!("\n ====== pass {:?} ====== \n", &pass);

//     println!("\n ====== FINALIZED COINS {:?} ====== \n", &a);

//     // let info_str = String::from_utf8_lossy(&info.stdout).into_owned();
//     // let info_split = info_str.split(' ').collect::<Vec<&str>>();
    

//     // let values: Vec<f32> = info_split
//     //                             .into_iter()
//     //                             .flat_map(|x| x.parse::<f32>())
//     //                             .collect();
//     //                             //     if x.contains(".") {
//     //                             //         x
//     //                             //     } else {
//     //                             //         ""
//     //                             //     }
//     //                             // })
//     //                             //.collect();

//     //println!("INFO {:#?} \n", info_split);
//     // println!("INFO0 {:#?} \n", info_str.split(&['\n','|',' ']).collect::<Vec<&str>>());
//     // println!("COLLECT {:#?} \n", values);

    
//     a.kill().expect("Failed on kill wallet");

//     println!("---------- END OF TEST ----------")
// } 

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn start_process(sender: Sender<String>, receiver: Receiver<String>, binary_path: &str ,input_args: [&str; 6]) {
    let child = Command::new(binary_path)
        //.args(input_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    println!("OUT {:#?}", child);
    println!("Started process: {}", child.id());

    thread::spawn(move || {
        let mut f = BufReader::new(child.stdout.unwrap());
        let mut stdin = child.stdin.unwrap();
        println!("f {:?} - stdin {:?}", f, stdin)
        for line in receiver {
            println!("LINE {:?}", line.as_bytes());
            stdin.write_all(line.as_bytes()).unwrap();
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    sender.send(buf).unwrap();
                    continue;
                }
                Err(e) => {
                    println!("an error!: {:?}", e);
                    break;
                }
            }
        }
    });
}

fn start_command_thread(mutex: Mutex<Sender<String>>) {
    thread::spawn(move || {
        let sender = mutex.lock().unwrap();
        sleep(Duration::from_secs(3));
        sender
            .send(String::from("Command from the thread\n"))
            .unwrap();
    });
}

fn main() {

    let chain_type = ChainTypes::UserTesting;
    let password = "1";
   
    let wallet_binary = "/home/ba/Desktop/EpicV3/epic-wallet/target/release/epic-wallet";
    let input = [wallet_binary,"-p", password, "--usernet", "init", "-r"];
    let bb = format!("{} -p {} --usernet init -r", wallet_binary, password);
    println!("{bb}");
    let created_wallet = create_wallet(&chain_type, wallet_binary, password);
    let pass = get_passphrase(&created_wallet);

    println!("START");
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    start_process(tx1, rx2, &wallet_binary,input);

    wait_for(3);
    tx2.send(String::from("AAAA")).unwrap();
    //start_command_thread(Mutex::new(tx2));

    for line in rx1 {
        println!("Got this back: {}", line);
    }
    println!("DONE!")
}