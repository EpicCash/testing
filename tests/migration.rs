//use std::fmt;
use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use std::convert::Infallible;
use std::process::Child;
extern crate dotenv;
use dotenv::dotenv;
use std::env;
//use std::process::{Command, Output};

//Testing
use testing::{
    create_wallet, get_test_configuration, info_wallet, new_child, spawn_network,
    str_to_chain_type, wait_for,
};

// Epic Server
use epic_core::global::ChainTypes;

//Epic Wallet
use epic_wallet_libwallet as libwallet;

//impl fmt::Debug for MigWorld {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.wallet_binary)
//    }
//}

//impl fmt::Debug for WalletInformation {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "chain_type :{:?}", self.sent_tx)
//    }
//}

impl std::default::Default for MigWorld {
    fn default() -> MigWorld {
        MigWorld {
            chain_type: ChainTypes::Floonet,
            password: String::from("1"),
            server_binary: String::new(),
            wallet_binary: String::new(),
            server: new_child(),
            wallet: new_child(),
        }
    }
}

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, WorldInit)]
pub struct MigWorld {
    pub chain_type: ChainTypes,
    pub server: Child,
    pub wallet: Child,
    pub password: String,
    pub server_binary: String,
    pub wallet_binary: String,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for MigWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        //Ok(Self::default())
        Ok(Self {
            chain_type: ChainTypes::UserTesting,
            password: String::from("1"),
            server_binary: String::new(),
            wallet_binary: String::new(),
            server: new_child(),
            wallet: new_child(),
        })
    }
}
//Given The epic-server binary is at /home/ba/Desktop/EpicV3/epic/target/release/epic
#[given(expr = "Define {string} binary")]
fn set_binary(world: &mut MigWorld, epic_sys: String) {
    match epic_sys.as_str() {
        "epic-server" => world.server_binary = env::var("EPIC_SERVER").unwrap(),
        "epic-wallet" => world.wallet_binary = env::var("EPIC_WALLET").unwrap(),
        _ => panic!("Invalid epic system"),
    };
}

#[given(expr = "I am using the {string} network")]
fn using_network(world: &mut MigWorld, str_chain: String) {
    let chain_t = str_to_chain_type(&str_chain);

    world.chain_type = chain_t;
    // config epic-server.toml with custom configuration
    get_test_configuration(&world.chain_type);
    // Wait the epic-servet.toml work
    wait_for(5);

    // NEED CREATE WALLET BEFORE SPAWN SERVER, Unable to delete folder if server is on
    // run wallet and save on world
    let _wallet_init = create_wallet(
        &world.chain_type,
        world.wallet_binary.as_str(),
        world.password.as_str(),
    );
}

#[when(expr = "I run info command")]
fn check_info_wallet(world: &mut MigWorld) {
    let _ = info_wallet(&world.chain_type, &world.wallet_binary, &world.password);
}

#[given(expr = "I start the node")]
fn start_child_system(world: &mut MigWorld) {
    // run server and save on world
    world.server = spawn_network(
        &world.chain_type,
        world.server_binary.as_str(),
        "--onlyrandomx",
    );
    wait_for(10)
}

#[then(expr = "I have an {word} based wallet")]
fn check_db(world: &mut MigWorld, db: String) {
    // run server and save on world
    let mig = libwallet::need_migration(&world.chain_type);

    match db.as_str() {
        "LMDB" => assert_eq!(mig, true),
        "SQLite" => assert_eq!(mig, false),
        _ => panic!("Specified database does not exist!"),
    }
}

#[then(expr = "I kill all running epic systems")]
fn kill_all_childs(world: &mut MigWorld) {
    world.wallet.kill().expect("Wallet wasn't running");
    world.server.kill().expect("Server wasn't running");
}

//#[tokio::main]
fn main() {
    dotenv().ok();
    println!("Remember to close all running epic systems before running the test");
    futures::executor::block_on(MigWorld::run("./features/migration.feature"));
}
