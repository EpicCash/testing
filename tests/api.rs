use std::convert::Infallible;
use std::process::Command;
use std::process::Child;

use async_trait::async_trait;
use cucumber::{given, when, then, World, WorldInit};
use std::time::Duration;
use std::thread::sleep;

#[derive(Debug, WorldInit)]
pub struct EPICWorld {
    binary_path: String,
    network: String,
    server_process: Child,
    method: String,
    range_init: u32,
    range_final: u32,
    response: String,
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
            method: "".to_string(),
            range_init: 0 as u32,
            range_final: 0 as u32,
            response: "".to_string(),
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
}

#[given("I started the node")]
fn start_the_node(world: &mut EPICWorld) {
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
    sleep(Duration::from_secs(10));
}

#[given(expr = "The JSON query is for {word} ranging from {string} to {string}")]
fn setting_json(world: &mut EPICWorld, method: String, range_init: String, range_final: String) {
    world.method = method;

    if range_init.contains("current") {
        let json = r#"{"jsonrpc": "2.0", "method": "get_tip", "params": [],"id": 1}"#;
        let request = Command::new("curl")
            .arg("-v")
            .arg("http://localhost:3413/v2/foreign")
            .arg("-d")
            .arg(json)
            .output()
            .expect("failed to execute process");
        
        let response = String::from_utf8(request.stdout).unwrap();
        let split_str: Vec<&str> = response.split(r#""height": "#).collect();
        let split_str: Vec<&str> = split_str[1].split(",").collect();
        let height = split_str[0].parse::<u32>().unwrap();

        if range_init.contains("-") {
            let split_str: Vec<&str> = range_init.split("-").collect();
            let final_value = height - split_str[1].trim().parse::<u32>().unwrap();
            world.range_init = final_value;
        } else if range_init.contains("+") {
            let split_str: Vec<&str> = range_init.split("-").collect();
            let final_value = height + split_str[1].trim().parse::<u32>().unwrap();
            world.range_init = final_value;
        } else {
            world.range_init = height;
        }
    } else {
        world.range_init = range_init.parse().unwrap();
    }

    if range_final.contains("current") {
        let json = r#"{"jsonrpc": "2.0", "method": "get_tip", "params": [],"id": 1}"#;
        let request = Command::new("curl")
            .arg("-v")
            .arg("http://localhost:3413/v2/foreign")
            .arg("-d")
            .arg(json)
            .output()
            .expect("failed to execute process");
        
        let response = String::from_utf8(request.stdout).unwrap();
        let split_str: Vec<&str> = response.split(r#""height": "#).collect();
        let split_str: Vec<&str> = split_str[1].split(",").collect();
        let height = split_str[0].parse::<u32>().unwrap();

        if range_final.contains("-") {
            let split_str: Vec<&str> = range_final.split("-").collect();
            let final_value = height - split_str[1].trim().parse::<u32>().unwrap();
            world.range_final = final_value;
        } else if range_final.contains("+") {
            let split_str: Vec<&str> = range_final.split("-").collect();
            let final_value = height + split_str[1].trim().parse::<u32>().unwrap();
            world.range_final = final_value;
        } else {
            world.range_final = height;
        }
    } else {
        world.range_final = range_final.parse().unwrap();
    }
}

#[given(expr = "The JSON query is for {word} with parameter defined as {int}")]
fn setting_json2(world: &mut EPICWorld, method: String, n: i32) {
    world.method = method;
    world.range_init = n as u32;
}

#[when("I make the HTTP POST request")]
fn http_request(world: &mut EPICWorld) {
    match world.method.as_str() {
        "get_blocks" => {
            let mut json: String = r#"{"jsonrpc": "2.0", "method": "get_blocks", "params": ["#.to_owned();
            if world.range_init < world.range_final {
                json.push_str(&world.range_init.to_string());
                json.push_str(", ");
                json.push_str(&world.range_final.to_string());
                json.push_str(r#", null, null], "id": 1}"#);
            } else {
                json.push_str(&world.range_final.to_string());
                json.push_str(", ");
                json.push_str(&world.range_init.to_string());
                json.push_str(r#", null, null], "id": 1}"#);
            }
            
            let request = Command::new("curl")
                .arg("-v")
                .arg("http://localhost:3413/v2/foreign")
                .arg("-d")
                .arg(json)
                .output()
                .expect("failed to execute process");
            
                world.response = String::from_utf8(request.stdout).unwrap();
        },
        "get_last_n_kernels" => {
            let mut json: String = r#"{"jsonrpc": "2.0", "method": "get_last_n_kernels", "params": ["#.to_owned();
            json.push_str(&world.range_init.to_string());
            json.push_str(r#"], "id": 1}"#);
            
            let request = Command::new("curl")
                .arg("-v")
                .arg("http://localhost:3413/v2/foreign")
                .arg("-d")
                .arg(json)
                .output()
                .expect("failed to execute process");
            
                world.response = String::from_utf8(request.stdout).unwrap();
        },
        _ => panic!("Specified HTTP request does not exist!")
    };
}

#[then("I got an empty set as response")]
fn empty_response(world: &mut EPICWorld) {
    if !(world.response.contains(r#""Ok": []"#)){
        world.server_process.kill().expect("falled to kill process");
        panic!("[FAILED] Should expect an empty set!");
    }
    world.server_process.kill().expect("falled to kill process");
}

#[then(expr = "I got a set with {int} blocks data")]
fn data_in_response(world: &mut EPICWorld, n_blocks: i32) {
    let words: Vec<&str> = world.response.split_whitespace().collect();
    let mut counter = 0;
    for word in words {
        if word.contains("header") {
            counter = counter + 1;
        }
    }

    if !(counter >= n_blocks){
        world.server_process.kill().expect("falled to kill process");
        panic!("[FAILED] Should expect an empty set!");
    }
    world.server_process.kill().expect("falled to kill process");
}

#[then(expr = "I got a set with {int} or more kernels data")]
fn data_in_response2(world: &mut EPICWorld, n_kernels: i32) {
    let words: Vec<&str> = world.response.split_whitespace().collect();
    let mut counter = 0;
    for word in words {
        if word.contains(r#""excess":"#) {
            counter = counter + 1;
        }
    }

    if !(counter >= n_kernels){
        world.server_process.kill().expect("falled to kill process");
        panic!("[FAILED] Should expect an empty set!");
    }
    world.server_process.kill().expect("falled to kill process");
}

// #[given(expr = "I am using the {word} network")]
// fn setting_network(world: &mut EPICWorld, network: String) {
//     world.network = network;
//     world.server_process = match world.network.as_str() {
//         "mainnet" => Command::new(&world.binary_path)
//                         .spawn()
//                         .expect("failed to execute process"),
//         "floonet" => Command::new(&world.binary_path)
//                         .arg("--floonet")
//                         .spawn()
//                         .expect("failed to execute process"),
//         "usernet" => Command::new(&world.binary_path)
//                         .arg("--usernet")
//                         .spawn()
//                         .expect("failed to execute process"),
//         _ => panic!("Specified network does not exist!")
//     };
// }

// #[given("The chain is synced")]
// fn check_sync(world: &mut EPICWorld) {
//     println!("AQUI 2");
//     world.server_process.kill().expect("falled to kill process");
// }

fn main() {
    futures::executor::block_on(EPICWorld::run("./features/api.feature"));
}