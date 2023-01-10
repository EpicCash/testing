// Epic Server
use async_trait::async_trait;
use cucumber::{World, WorldInit};
use epic_core::global::ChainTypes;
use std::convert::Infallible;
use std::fmt;
use std::process::Child;
use std::time::Duration;

use crate::commands::new_child;

/// This structure stores the `info` command information in an organized way. Mainly to facilitate the comparison.
#[derive(Debug, PartialEq)]
pub struct InfoWallet {
    /// chain height
    pub chain_height: f32,
    /// confirmed total
    pub confirmed_total: f32,
    /// immature coinbase
    pub immature_coinbase: f32,
    /// awaiting confirmation
    pub awaiting_confirmation: f32,
    /// awaiting finalization
    pub awaiting_finalization: f32,
    /// locked coins
    pub locked_by_previus_transaction: f32,
    /// currentlly spendable
    pub currently_spendable: f32,
}

impl std::default::Default for InfoWallet {
    fn default() -> InfoWallet {
        InfoWallet {
            chain_height: 0.0,
            confirmed_total: 0.0,
            immature_coinbase: 0.0,
            awaiting_confirmation: 0.0,
            awaiting_finalization: 0.0,
            locked_by_previus_transaction: 0.0,
            currently_spendable: 0.0,
        }
    }
}

impl std::convert::From<Vec<f32>> for InfoWallet {
    fn from(item: Vec<f32>) -> Self {
        // code to convert the vector into an instance of your struct goes here
        if item.len() > 6 {
            InfoWallet {
                chain_height: item[0],
                confirmed_total: item[1],
                immature_coinbase: item[2],
                awaiting_confirmation: item[3],
                awaiting_finalization: item[4],
                locked_by_previus_transaction: item[5],
                currently_spendable: item[6],
            }
        } else if item.len() == 6 {
            // When a wallet does not have mined blocks, the `info` will not have the coins in `Immature Coinbase`
            InfoWallet {
                chain_height: item[0],
                confirmed_total: item[1],
                immature_coinbase: 0.0,
                awaiting_confirmation: item[2],
                awaiting_finalization: item[3],
                locked_by_previus_transaction: item[4],
                currently_spendable: item[5],
            }
        } else {
            InfoWallet::default()
        }
    }
}

impl std::default::Default for WalletInformation {
    fn default() -> WalletInformation {
        WalletInformation {
            sent_tx: 0 as u32,
            received_tx: 0 as u32,
            confirmed_coinbase: 0 as u32,
            sent_path: String::new(),
            receive_path: String::new(),
        }
    }
}

/// This struct save information of `txs` command
#[derive(Debug)]
pub struct WalletInformation {
    /// Number of SentTx in txs
    pub sent_tx: u32,
    /// Number of ReceivedTx in txs
    pub received_tx: u32,
    /// Number of Confirmed Coinbase
    pub confirmed_coinbase: u32,
    /// Sent name for methods `file` and `emoji`
    pub sent_path: String,
    /// Receive name for methods `file` and `emoji`
    pub receive_path: String,
}

impl PartialEq for WalletInformation {
    fn eq(&self, other: &Self) -> bool {
        self.sent_tx + self.received_tx == other.sent_tx + other.received_tx
            && self.confirmed_coinbase == other.confirmed_coinbase
    }
}

impl std::convert::From<Vec<u32>> for WalletInformation {
    fn from(item: Vec<u32>) -> Self {
        // code to convert the vector into an instance of your struct goes here
        if item.len() != 0 {
            WalletInformation {
                sent_tx: item[0],
                received_tx: item[1],
                confirmed_coinbase: item[2],
                sent_path: String::new(),
                receive_path: String::new(),
            }
        } else {
            WalletInformation::default()
        }
    }
}

/// This structs save all information of `outputs`
#[derive(Debug)]
pub struct OutputList {
    /// Number of unconfirmed
    pub unconfirmed: u32,
    /// Number of Unspent
    pub unspent: u32,
    /// Number of locked coins
    pub locked: u32,
    /// Number of spent
    pub spent: u32,
    /// Number of deleted
    pub deleted: u32,
    /// Unconfirmed && is_coinbase
    pub mining: u32,
    /// Number of is_conbase
    pub num_is_coinbase: u32,
}

impl std::default::Default for OutputList {
    fn default() -> OutputList {
        OutputList {
            unconfirmed: 0,
            unspent: 0,
            locked: 0,
            spent: 0,
            deleted: 0,
            mining: 0,
            num_is_coinbase: 0,
        }
    }
}

impl PartialEq for OutputList {
    fn eq(&self, other: &Self) -> bool {
        // let supose it's equal
        let equal = true;

        // 1. Check Spent and Deleted (independent and don't change)
        let equal = equal && { self.spent == other.spent && self.deleted == other.deleted };

        if !equal {
            println!("Outputs differ because the number of spent and deleted was not assigned to another check:\nFirst:{:#?},\nSecond:{:#?}", self, other);
            return false;
        }

        // 2. Check mining blocks (mining will merged and `number_is_coinbase` will decrease with the same value)
        let equal = equal && { self.num_is_coinbase - self.mining == other.num_is_coinbase };

        if !equal {
            println!("Outputs differ because the number of mined and unconfirmed blocks (mining) was not assigned to another check:\nFirst:{:#?},\nSecond:{:#?}", self, other);
            return false;
        }

        // 3. Check unconfirmed, unspent and locked
        // If it's smaller, it's a problem and we want the error
        let diff_lock = if self.locked >= other.locked {
            self.locked - other.locked
        } else {
            0
        };

        let equal = equal && {
            self.unspent - diff_lock == other.unspent && self.locked - diff_lock == other.locked
            // locked diff is necessary because diff_lock can be 0
        };

        if !equal {
            println!("Outputs differ because the number of unspent or locked was not assigned to another check:\nFirst:{:#?},\nSecond:{:#?}", self, other);
            return false;
        }

        // 4. check unconfirmed (before need be even and after need be 0)
        let equal = equal && { self.unconfirmed % 2 == 0 && other.unconfirmed == 0 };

        if !equal {
            println!("Outputs differ because the number of unconfirmed is odd before or is not confirmed after:\nFirst:{:#?},\nSecond:{:#?}", self, other);
            return false;
        }

        equal
    }
}

impl std::convert::From<Vec<u32>> for OutputList {
    fn from(item: Vec<u32>) -> Self {
        // code to convert the vector into an instance of your struct goes here
        if item.len() != 0 {
            OutputList {
                unconfirmed: item[0],
                unspent: item[1],
                locked: item[2],
                spent: item[3],
                deleted: item[4],
                mining: item[5],
                num_is_coinbase: item[6],
            }
        } else {
            OutputList::default()
        }
    }
}

/// This struct prepare all information of a large number of transactions
#[derive(Debug, Clone)]
pub struct PackTransaction {
    /// Number of transactions
    pub number_transactions: i32,
    /// Vector with all duration
    pub duration_time: Vec<Duration>,
    /// Vector with amount of every transaction
    pub vec_amount: Vec<String>,
}

impl fmt::Display for PackTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "number_transactions: {:?}\nduration_time: {:?}\nvec_amount: {:?})",
            self.number_transactions, self.duration_time, self.vec_amount
        )
    }
}

/// This structure is the Cucumber's World, all the information it will use between each step must be stored in this structure.
/// Here we have the following, chain_type is whether we are using `usernet`, `floonet`, or other.
/// server, wallet and miner keep the child processes responsible for continuing to run as long as we want.
/// server_binary, wallet_binary and miner_binary is to run the 3 services wherever we want.
/// password and passphrase are for using the wallet and its activities.
/// transactions is the struct that stores information about transactions.
/// dur_transactions saves the duration of all transactions on specific resources
/// n transactions save number of transactions for some features
/// info_command stores the fields of the `epic-wallet info` command, same for `txs` and `outputs`
/// initial height: Saves the initial height
#[derive(Debug, WorldInit)]
pub struct TestingWorld {
    /// chain_type is whether we are using `usernet`, `floonet`, or other.
    pub chain_type: ChainTypes,
    /// keep the child processes responsible for continuing to run as long as we want.
    pub server: Child,
    /// keep the child processes responsible for continuing to run as long as we want.
    pub wallet: Child,
    /// keep the child processes responsible for continuing to run as long as we want.
    pub miner: Child,
    /// using for all wallet commands.
    pub password: String,
    /// using in recovery process
    pub passphrase: String,
    /// To run server when we want
    pub server_binary: String,
    /// To run wallet when we want
    pub wallet_binary: String,
    /// To run miner when we want
    pub miner_binary: String,
    /// Save all txs information and send and recive names to `file` and `emoji`
    pub transactions: WalletInformation,
    /// Save time vector for each transaction
    pub dur_transactions: Vec<Duration>,
    /// Number of transactions
    pub n_transactions: i32,
    /// Save the values for info command
    pub info_command: InfoWallet,
    /// Save the values for txs command
    pub txs_command: WalletInformation,
    /// Save the values for outputs command
    pub outputs_command: OutputList,
    /// Saves the initial height
    pub initial_height: i32,
}

#[async_trait(?Send)]
impl World for TestingWorld {
    // We do require some error type.
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        //Ok(Self::default())
        Ok(TestingWorld::default())
    }
}

impl std::default::Default for TestingWorld {
    fn default() -> TestingWorld {
        TestingWorld {
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
            dur_transactions: Vec::new(),
            n_transactions: 1,
            info_command: InfoWallet::default(),
            txs_command: WalletInformation::default(),
            outputs_command: OutputList::default(),
            initial_height: 0,
        }
    }
}

/// To main function create the huge wallet
#[derive(Debug, Clone)]
pub struct BigWalletWorld {
    /// Chain type
    pub chain_type: ChainTypes,
    /// Send method
    pub send_method: String,
    /// http destination
    pub http_path: String,
    /// password
    pub password: String,
    /// binary
    pub server_binary: String,
    /// binary
    pub wallet_binary: String,
    /// binary
    pub miner_binary: String,
}

impl std::default::Default for BigWalletWorld {
    fn default() -> BigWalletWorld {
        BigWalletWorld {
            chain_type: ChainTypes::UserTesting,
            send_method: String::from("http"),
            http_path: String::new(),
            password: String::from("1"),
            server_binary: String::new(),
            wallet_binary: String::new(),
            miner_binary: String::new(),
        }
    }
}

/// Like a world from cucumber but to main function create the huge wallet
#[derive(Debug)]
pub struct ChildProcess {
    /// server process
    pub server: Child,
    /// wallet process
    pub wallet: Child,
    /// miner process
    pub miner: Child,
}

impl std::default::Default for ChildProcess {
    fn default() -> ChildProcess {
        ChildProcess {
            server: new_child(),
            wallet: new_child(),
            miner: new_child(),
        }
    }
}
