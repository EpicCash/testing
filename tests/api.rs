use std::convert::Infallible;
use std::process::Command;
use std::process::Child;

use async_trait::async_trait;
use cucumber::{given, World, WorldInit};

#[derive(Debug, WorldInit)]
pub struct EPICWorld {
    binary_path: String,
    network: String,
    server_process: Child,
}

#[async_trait(?Send)]
impl World for EPICWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            binary_path: "".to_string(),
            network: "".to_string(),
            server_process: Command::new("echo")
                .arg("")
                .spawn()
                .expect("failed to execute child"),
        })
    }
}

#[given(expr = "The epic-server binary is at {word}")]
fn setting_binary_path(world: &mut EPICWorld, path: String) {
    world.binary_path = path;
}

#[given(expr = "I am using the {word} network")]
fn setting_network(world: &mut EPICWorld, network: String) {
    world.network = network;
    world.server_process = match world.network.as_str() {
        "mainnet" => Command::new(&world.binary_path)
                        .spawn()
                        .expect("failed to execute process"),
        "floonet" => Command::new(&world.binary_path)
                        .arg("--floonet")
                        .spawn()
                        .expect("failed to execute process"),
        "usernet" => Command::new(&world.binary_path)
                        .arg("--usernet")
                        .spawn()
                        .expect("failed to execute process"),
        _ => panic!("Specified network does not exist!")
    };
}

#[given("The chain is synced")]
fn check_sync(world: &mut EPICWorld) {
    println!("AQUI 2");
    world.server_process.kill().expect("falled to kill process");
}

fn main() {
    futures::executor::block_on(EPICWorld::run("./features/api.feature"));
}