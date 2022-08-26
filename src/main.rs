use std::env::current_dir;
use std::path::PathBuf;
use std::fs::create_dir_all;

//Epir Server
use epic_config::config::{init_api_secret, initial_setup_server};
use epic_core::global::ChainTypes;
use epic_config::types::ConfigError;


/// The default file name to use when trying to derive
/// the node config file location
pub const SERVER_CONFIG_FILE_NAME: &'static str = "epic-server.toml";
pub const SERVER_LOG_FILE_NAME: &'static str = "epic-server.log";
pub const TEST_FOLDER_NAME: &'static str = ".epic_test";
pub const EPIC_CHAIN_DIR: &'static str = "chain_data";
/// Node API secret
pub const API_SECRET_FILE_NAME: &'static str = ".api_secret";


fn get_epic_test_path(chain_type: ChainTypes) -> Result<PathBuf, ConfigError> {
    // Check if epic dir exists
    let mut epic_path = match current_dir() {
        Ok(p) => p,
        Err(_) => PathBuf::new(),
    };
    epic_path.push(TEST_FOLDER_NAME);
    epic_path.push(chain_type.shortname());
    // Create if the default path doesn't exist
    if !epic_path.exists() {
        create_dir_all(epic_path.clone())?;
    }
    Ok(epic_path)
}




fn main() {
    //let mut dir = current_dir().unwrap();
    let mut chain_typ = ChainTypes::UserTesting;//ChainTypes::AutomatedTesting;
    let mut a = Some(initial_setup_server(&chain_typ).unwrap_or_else(|e| {
            panic!("Error loading server configuration: {}", e)}),);
    println!("{:#?}", a);
    //dir.push("TE");
    //println!("{:?}", dir);
    //assert_eq!("",dir);
}