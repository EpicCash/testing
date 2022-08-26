// Default
use std::fmt;
use std::fs::remove_dir_all;
use async_trait::async_trait;
use cucumber::{given, World, WorldInit};
use std::convert::Infallible;
use std::process::Command;
use std::process::Child;
use std::env;

//Epic Server
use epic_chain::Chain;
use epic_core::global;
use epic_core::global::ChainTypes;
use epic_core::core::{Block, pow::mine_genesis_block};
use epic_util::util::init_test_logger;

//Epic Wallet
use epic_wallet_config::config::initial_setup_wallet;

impl fmt::Debug for TransWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output_dir :{:?}", self.output_dir)
    }
}

impl std::default::Default for TransWorld {
	fn default() -> TransWorld {
		TransWorld {
			output_dir: ".epic_test".to_string(),
            chain_type: ChainTypes::AutomatedTesting,
            genesis: None,
			chain: None,
		}
	}
}
println!("{:?}",current_dir());
// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(WorldInit)]
struct TransWorld {
    pub output_dir: String,
    pub chain_type: ChainTypes,
    pub genesis: Option<Block>,
    pub chain: Option<Chain>,
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

fn clean_output_dir(dir_name: &str) {
    let _ = remove_dir_all(dir_name);
}


fn setup(dir_name: &str, genesis: Block) -> Chain {
    init_test_logger();
    clean_output_dir(dir_name);

    chain::Chain::init(
        dir_name.to_string(),
        Arc::new(NoopAdapter {}),
        genesis,
        pow::verify_size,
        false,
    )
    .unwrap()
}


#[given(expr = "I configure {string} toml")]
fn new_toml(world: &mut TransWorld, sys: String) {
    let chain_t = world.chain_type;
    let dir = world.output_dir;
    match sys.as_str() {
        "server and wallet" => {
            // Init a Server Toml
            // Init a Wallet Toml
            None
        },
        "server" => {
            // Init a Server Toml
            None
        },
        "wallet" => {
            // Init a Server Toml
            let mut config = config::initial_setup_wallet(&chain_t, dir).unwrap_or_else(|e| {
                panic!("Error loading wallet configuration: {}", e);
            });
        },
        _ => (),
    };
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(expr = "I have a {word} chain")]
fn network_chain(world: &mut TransWorld, chain_type: String) {
    world.output_dir = ".output_dir".to_string();
    let chain_typ = match chain_type.as_str() {
        "testing" => ChainTypes::AutomatedTesting,
        "mainnet" => ChainTypes::Mainnet,
        "floonet" => ChainTypes::Floonet,
        "usernet" => ChainTypes::UserTesting,
        _ => panic!("Unknown chain type"),
    };
    global::set_mining_mode(chain_typ);
    world.chain_type = chain_typ;
    world.genesis = Some(mine_genesis_block().unwrap());
    world.chain = Some(setup(&world.output_dir, world.genesis.as_ref().unwrap().clone()));
}

#[given(expr = "I have a wallet with {float} coins")]
fn new_wallet(_world: &mut TransWorld, _coins: f32) {
    println!("TESTANDO");
}

#[tokio::main]
async fn main() {
    TransWorld::run("./features/transactions.feature").await;
}