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

fn new_child() -> Child {
    //Command::new("").spawn().expect("Failed on run a empty Child process")
    Command::new("echo")
                .arg("")
                .spawn()
                .expect("Failed on run a empty Child process")
}

fn new_output() -> Output {
    //Command::new("").output().expect("Failed on run a empty Output process")
    Command::new("echo")
                .arg("")
                .output()
                .expect("Failed on run a empty Output process")
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
		}
	}
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
    let mut wallet_init = create_wallet(&world.chain_type, world.wallet_binary.as_str(), world.password.as_str());

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

#[then(expr = "I await the confirm transaction")]
fn await_finalization(world: &mut TransWorld) {
    confirm_transaction(&world.chain_type, &world.wallet_binary, &world.password)
}

#[then(expr = "I kill all running epic systems")]
fn kill_all_childs(world: &mut TransWorld) {
    world.miner.kill().expect("Miner wasn't running");
    world.wallet.kill().expect("Wallet wasn't running");
    world.server.kill().expect("Server wasn't running");
}

#[tokio::main]
async fn main() {
    TransWorld::run("./features/transactions.feature").await;
}