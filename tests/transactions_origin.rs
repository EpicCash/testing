use cucumber_rust::{async_trait, Cucumber, World};
use std::{convert::Infallible, default};
use epic_chain::Chain;

pub enum MyWorld {
    Init,
    Input(i32, i32),
    Result(i32),
    Error,
}

pub struct TransWorld {
    pub output_dir: String,
    pub chain: Option<Chain>,
}

impl std::default::Default for TransWorld {
	fn default() -> TransWorld {
		TransWorld {
			output_dir: ".epic".to_string(),
			chain: None,
		}
	}
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Init)
    }
}

#[async_trait(?Send)]
impl World for TransWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::default())
    }
}

mod test_steps {
    use crate::{MyWorld, TransWorld};
    use cucumber_rust::Steps;
    use testing::mult;

    pub fn steps_trans() -> Steps<TransWorld> {
        let mut builder: Steps<TransWorld> = Steps::new();

        builder.given(
            "I have a chain", 
            |world, _context|{
                world
            }
        );

        builder.given_regex(
            "I have a </d> chain", 
            |world, context|{
                println!("---- {}", context.matches[1].to_lowercase().as_str());
                match context.matches[1].to_lowercase().as_str() {
					"testing" => Some(1), //global::set_mining_mode(ChainTypes::AutomatedTesting),
					"mainnet" => Some(2),//global::set_mining_mode(ChainTypes::Mainnet),
					"floonet" => Some(3),//global::set_mining_mode(ChainTypes::Floonet),
					_ => panic!("Unknown chain type"),
				};
            world
            }
        );

        builder
    }

    pub fn steps() -> Steps<MyWorld> {
        
        let mut builder: Steps<MyWorld> = Steps::new();

        builder.given_regex(
            // This will match the "given" of multiplication
            r#"^the numbers "(\d)" and "(\d)"$"#,
            // and store the values inside context, which is a Vec<String>
            |_world, context| {
                // We start from [1] because [0] is the entire regex match
                let world = MyWorld::Input(
                    context.matches[1].parse::<i32>().unwrap(),
                    context.matches[2].parse::<i32>().unwrap(),
                );
                world
            }
        );

        builder.when(
            "the User multiply them", 
            |world, _context|{
                match world {
                    MyWorld::Input(l, r) => MyWorld::Result(mult(l,r)),
                    _ => MyWorld::Error,
                }
            }
        );

        builder.then_regex(
            r#"^the User gets "(\d)" as result$"#, 
            |world, context|{
                match world {
                    MyWorld::Result(x) => assert_eq!(x.to_string(), context.matches[1]),
                    _ => panic!("Invalid world state"),
                };
                MyWorld::Init
            }
        );
        builder
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<TransWorld>::new()
        .features(&["./features/transactions.feature"])
        .steps(test_steps::steps_trans())
        .run_and_exit()
        .await
}