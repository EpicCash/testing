use std::{fmt, process::Output};
use std::process::Child;
use async_trait::async_trait;
use cucumber::{given, when, then, World, WorldInit};
use std::convert::Infallible;
use std::process::Command;

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
            new_output,
            get_number_transactions_txs,
            };

// Epic Server
use epic_core::global::ChainTypes;

//Epic Wallet
//use epic_wallet_config::config::initial_setup_wallet;

impl fmt::Debug for TransWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "chain_type :{:?}", self.chain_type)
    }
}

impl fmt::Debug for WalletInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "chain_type :{:?}", self.sent_tx)
    }
}

impl std::default::Default for TransWorld {
	fn default() -> TransWorld {
		TransWorld {
            chain_type: ChainTypes::UserTesting,
            password: String::from("1"),
            passphrase: String::new(),
            server_binary: String::new(),
            wallet_binary: String::new(),
            miner_binary: String::new(),
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
            transactions: WalletInformation::default(),
		}
	}
}

impl std::default::Default for WalletInformation {
	fn default() -> WalletInformation {
		WalletInformation { 
            sent_tx: 0 as u32, 
            received_tx: 0 as u32,
            confirmed_coinbase: 0 as u32,
        }
	}
}

struct WalletInformation {
    pub sent_tx: u32,
    pub received_tx: u32,
    confirmed_coinbase: u32,
}

// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(WorldInit)]
struct TransWorld {
    pub chain_type: ChainTypes,
    pub server: Child,
    pub wallet: Child,
    pub miner: Child,
    pub password: String,
    pub passphrase: String, // only for recovery test
    pub server_binary: String,
    pub wallet_binary: String,
    pub miner_binary: String,
    pub transactions: WalletInformation,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for TransWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::default())
    }
}
//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "The {string} binary is at {string}")]
fn set_binary(world: &mut TransWorld, epic_sys: String, path: String) {
    match epic_sys.as_str() {
        "epic-server" => {world.server_binary = path},
        "epic-wallet" => {world.wallet_binary = path},
        "epic-miner" => {world.miner_binary = path},
        _ => panic!("Invalid system of epic"),
    };
}

#[given(expr = "I am using the {string} network")]
fn using_network(world: &mut TransWorld, str_chain: String) {

    let chain_t = str_to_chain_type(&str_chain);
    
    world.chain_type = chain_t;
    // config epic-server.toml with custom configuration
    get_test_configuration(&world.chain_type);
    // Wait the epic-servet.toml work
    wait_for(5);

    // run server and save on world
    world.server = spawn_network(&world.chain_type, world.server_binary.as_str());

    // run wallet and save on world
    let wallet_init = create_wallet(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());

    // save passphrase on world
    world.passphrase = get_passphrase(&wallet_init);

    // save the wallet_listen process on world
    world.wallet = spawn_wallet_listen(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());

    // Run the miner
    world.miner = spawn_miner(&world.miner_binary);
}

#[given("I mine some blocks into my wallet")]
fn mine_some_coins(_world: &mut TransWorld) {
    // Wait for 5~10 blocks
    wait_for(180);
}

#[when(expr = "I send {word} coins with {word} method")]
fn send_coins(world: &mut TransWorld, amount: String, method: String) {
    // TODO if wallet have > 0 coins
    
    // TODO destination (File and HTTP methods)

    // Update transactions information in WalletInformation
    let transaction_info = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_transactions_information = WalletInformation {
                                                            sent_tx: transaction_info[0],
                                                            received_tx: transaction_info[1],
                                                            confirmed_coinbase: transaction_info[2]};
    world.transactions = new_transactions_information; 
    
    // If method is HTTP or file, send command needs a destination
    let send_output = match method.as_str() {
        "HTTP" | "file" => {
            panic!("Destionation not implemented yet");
            new_output()
        },
        _ => send_coins_smallest(&world.chain_type, &world.wallet_binary, method, &world.password, amount, String::new())
    };
    
    assert!(send_output.status.success())

}

#[when(expr = "I await the confirm transaction")]
fn await_finalization(world: &mut TransWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password)
}

#[given(expr = "I have a wallet with coins")]
fn check_coins_in_wallet(world: &mut TransWorld) {
    let info = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);

    assert!(info.last().unwrap() > &0.0)
}
 
//I have 2 new transactions in txs
#[then(expr = "I have {int} new transactions in txs")]
fn check_new_transactions(world: &mut TransWorld, number_transactions: u32) {
    // Update transactions information in WalletInformation
    let transaction_info = get_number_transactions_txs(&world.chain_type, &world.wallet_binary, &world.password);
    let new_info = WalletInformation {
                                        sent_tx: transaction_info[0],
                                        received_tx: transaction_info[1],
                                        confirmed_coinbase: transaction_info[2]};
    let int_number = number_transactions/2;
    
    // Sent tx
    assert_eq!(world.transactions.sent_tx + int_number, new_info.sent_tx);

    // Received tx
    assert_eq!(world.transactions.received_tx + int_number, new_info.received_tx);
}

#[given(expr = "I kill all running epic systems")]
fn kill_all_childs(world: &mut TransWorld) {
    world.miner.kill().expect("Miner wasn't running");
    world.wallet.kill().expect("Wallet wasn't running");
    world.server.kill().expect("Server wasn't running");
}



#[tokio::main]
async fn main() {
    TransWorld::run("./features/transactions.feature").await;
}