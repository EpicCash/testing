use std::process::{Command, Output};
use epic_core::global::ChainTypes;

// Entry is a wallet init output and return the passprhase
pub fn get_height_from_list_peers(output: &Output) -> Vec<i32>  {
    // String of message
    let output_msg = String::from_utf8_lossy(&output.stdout).into_owned();

    // Split the message into a vector
    let output_msg_vec = output_msg.split("\nHeight: ").collect::<Vec<&str>>();
    let mut height_vec: Vec<i32> = Vec::new();
    for element in output_msg_vec {
        if element.contains("Total"){
            let all_splits = element.split("\n").collect::<Vec<&str>>();
            let b: i32 = all_splits[0].parse().unwrap();
            height_vec.push(b);
        }
    }
    //let height_vec = output_msg_vec[1].split("\n").collect::<Vec<&str>>;
    println!("AA {:#?}", height_vec);
    height_vec
} 

// Run the init command, if the wallet_data exist -> delete and create a new one
pub fn get_list_peers(chain_type: &ChainTypes, binary_path: &str) -> Output {
    let list_peers = match chain_type {
        ChainTypes::Floonet => {
            Command::new(binary_path)
                    .args(["--floonet", "client", "listconnectedpeers"])
                    .output().expect("Failed on init a wallet")
        },
        _ => {
            Command::new(binary_path)
                    .args(["client", "listconnectedpeers"])
                    .output().expect("Failed on init a wallet")
        },
    };
    list_peers
}

fn get_chain_height(vec_height: Vec<i32>) -> i32 {
    let max_height = vec_height.iter().max();
    max_height.unwrap().to_owned()
}

fn check_peers(vec_height: Vec<i32>) {
    assert!(vec_height.len() > 0)
}

// Entry is a wallet init output and return the passprhase
pub fn get_height_from_status(output: &Output) -> i32  {
    // String of message
    let output_msg = String::from_utf8_lossy(&output.stdout).into_owned();

    // Split the message into a vector
    let output_msg_vec = output_msg.split("\nChain height: ").collect::<Vec<&str>>();
    let all_splits = output_msg_vec[1].split("\n").collect::<Vec<&str>>();
    let height: i32 = all_splits[0].parse().unwrap();
    
    //let height_vec = output_msg_vec[1].split("\n").collect::<Vec<&str>>;
    height
}

// Run the init command, if the wallet_data exist -> delete and create a new one
pub fn get_status(chain_type: &ChainTypes, binary_path: &str) -> Output {
    let list_peers = match chain_type {
        ChainTypes::Floonet => {
            Command::new(binary_path)
                    .args(["--floonet", "client", "status"])
                    .output().expect("Failed on init a wallet")
        },
        _ => {
            Command::new(binary_path)
                    .args(["client", "status"])
                    .output().expect("Failed on init a wallet")
        },
    };
    list_peers
}

fn main() {
    let chain_type = ChainTypes::Floonet;
    let server_binary = String::from("/home/jualns/Desktop/epic/target/release/epic");

    // height from others peers
    let msg_list_of_peers = get_list_peers(&chain_type, &server_binary);
    let out_height = get_height_from_list_peers(&msg_list_of_peers);

    // get max of height from othres peers
    let chain_height_peers = get_chain_height(out_height);

    // height from local node
    let msg_status = get_status(&chain_type, &server_binary);
    let chain_height_status = get_height_from_status(&msg_status);

    println!("List of Peers: {:#?} * {:#?}", chain_height_peers, chain_height_status);
}