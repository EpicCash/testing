//use std::fmt;
use std::{process::Child, fs::remove_file};
use async_trait::async_trait;
use cucumber::{given, when, then, World, WorldInit};
use std::convert::Infallible;
//use std::process::{Command, Output};

//Testing
use testing::{
            wait_for,
            get_test_configuration,
            spawn_network,
            create_wallet,
            str_to_chain_type,
            spawn_miner, spawn_wallet_listen,
            get_passphrase,
            send_coins_smallest,
            confirm_transaction,
            info_wallet,
            new_child,
            //new_output,
            get_number_transactions_txs,
            get_http_wallet,
            receive_finalize_coins,
            generate_file_name,
            generate_response_file_name,
            get_height_from_list_peers,
            get_list_peers,
            get_chain_height_peers,
            check_peers,
            get_height_from_status,
            get_status,    
        };

// Epic Server
use epic_core::global::ChainTypes;

//Epic Wallet
//use epic_wallet_config::config::initial_setup_wallet;

//impl fmt::Debug for FlooWorld {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.wallet_binary)
//    }
//}

//impl fmt::Debug for WalletInformation {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.sent_tx)
//    }
//}

impl std::default::Default for FlooWorld {
	fn default() -> FlooWorld {
		FlooWorld {
            chain_type: ChainTypes::Floonet,
            password: String::from("1"),
            server_binary: String::new(),
            wallet_binary: String::new(),
            miner_binary: String::new(),
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
		}
	}
}

// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, WorldInit)]
pub struct FlooWorld {
    pub chain_type: ChainTypes,
    pub server: Child,
    pub wallet: Child,
    pub miner: Child,
    pub password: String,
    pub server_binary: String,
    pub wallet_binary: String,
    pub miner_binary: String,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for FlooWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        //Ok(Self::default())
        Ok(Self {
            chain_type: ChainTypes::UserTesting,
            password: String::from("1"),
            server_binary: String::new(),
            wallet_binary: String::new(),
            miner_binary: String::new(),
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
		})
    }
}
//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "The {string} binary is at {string}")]
fn set_binary(world: &mut FlooWorld, epic_sys: String, path: String) {
    match epic_sys.as_str() {
        "epic-server" => {world.server_binary = path},
        "epic-wallet" => {world.wallet_binary = path},
        "epic-miner" => {world.miner_binary = path},
        _ => panic!("Invalid system of epic"),
    };
}

#[given(expr = "I am using the {string} network")]
fn using_network(world: &mut FlooWorld, str_chain: String) {

    let chain_t = str_to_chain_type(&str_chain);
    
    world.chain_type = chain_t;
    // config epic-server.toml with custom configuration
    get_test_configuration(&world.chain_type);
    // Wait the epic-servet.toml work
    wait_for(5);

    // NEED CREATE WALLET BEFORE SPAWN SERVER, Unable to delete folder if server is on
    // run wallet and save on world
    let wallet_init = create_wallet(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());
}

#[when(expr = "I start the node with policy {string}")]
fn start_child_system(world: &mut FlooWorld, enter_policy: String) {

        let mut poly = String::from("--");
        poly.push_str(enter_policy.as_str());
        // run server and save on world
        world.server = spawn_network(&world.chain_type, world.server_binary.as_str(), poly.as_str());
        wait_for(10)

}

#[then(expr = "The chain is downloaded and synced")]
fn check_chain_synced(world: &mut FlooWorld) {
    let mut chain_height_peers: i32 = 0; //peer_height
    let mut chain_height_status: i32 = 1; // local_height
    let mut num_checks: i32 = 0; // k interations
    while chain_height_status != chain_height_peers {
        // height from others peers
        let msg_list_of_peers = get_list_peers(&world.chain_type, &world.server_binary);
        let out_height = get_height_from_list_peers(&msg_list_of_peers);

        // get max of height from othres peers
        chain_height_peers = get_chain_height_peers(out_height);

        // height from local node
        let msg_status = get_status(&world.chain_type, &world.server_binary);
        chain_height_status = get_height_from_status(&msg_status);
        if chain_height_status < chain_height_peers && num_checks < 10 {
            wait_for(15);
            num_checks += 1
        } else {
            break;
        };
    }
    
    assert_eq!(chain_height_peers,chain_height_status, "\nWe are testing height by peers {} and local height {}", chain_height_peers,chain_height_status);
}

#[then(expr = "I am able to see more than one peer connected")]
fn check_connected_peers(world: &mut FlooWorld) {
    // height from others peers
    let msg_list_of_peers = get_list_peers(&world.chain_type, &world.server_binary);
    let out_height = get_height_from_list_peers(&msg_list_of_peers);

    assert!(out_height.len() > 0)
}

#[when(expr = "I {string} the {string}")]
fn start_child_general(world: &mut FlooWorld, start_stop: String , epic_system: String) {
    match start_stop.as_str() {
        "start" => {
            match epic_system.as_str() {
                "miner" => {
                    // Run the miner
                    world.miner = spawn_miner(&world.miner_binary);},
                "wallet" => {
                    // save the wallet_listen process on world
                    world.wallet = spawn_wallet_listen(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());},
                _ => panic!("Specified system does not exist to start!")
                }; 
            wait_for(2)
        }
        "stop" => {
            match epic_system.as_str() {
                "node" => world.server.kill().expect("Server wasn't running"),
                "miner" => world.miner.kill().expect("Miner wasn't running"),
                "wallet" => world.wallet.kill().expect("Wallet wasn't running"),
                _ => panic!("Specified system does not exist to kill!")
            }
        }
        _ => panic!("Specified command does not exist, try start or stop!")
    }
}

#[given("I mine some blocks into my wallet")]
fn mine_some_coins(world: &mut FlooWorld) {
    // TODO - Wait for 5~10 blocks
    let mut info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let mut current_spendable = info.last().expect("Can't get the current spendable!");
    while current_spendable == &0.0 {
        wait_for(30);
        info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
        current_spendable = info.last().expect("Can't get the current spendable!");
    }
}

#[given(expr = "I have a wallet with coins")]
fn check_coins_in_wallet(world: &mut FlooWorld) {
    let info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
    let current_spendable = info.last().expect("Can't get the current spendable!");
    assert!(current_spendable > &0.0)
}

#[when(expr = "I await confirm the transaction")]
fn await_finalization(world: &mut FlooWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password)
}

#[then(expr = "I kill all running epic systems")]
fn kill_all_childs(world: &mut FlooWorld) {
    world.miner.kill().expect("Miner wasn't running");
    world.wallet.kill().expect("Wallet wasn't running");
    world.server.kill().expect("Server wasn't running");
}



//#[tokio::main]
fn main() {
    println!("Remember to close all running epic systems before running the test");
    futures::executor::block_on(FlooWorld::run("./features/floonet.feature"));
}