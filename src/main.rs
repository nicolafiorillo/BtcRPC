use std::io::{self, Write};

use args::Args;
use clap::Parser;
use command_line::Command;
use config::Config;
use std_result::StdResult;

mod args;
mod command_line;
mod config;
mod node;
mod session;
mod std_result;

#[tokio::main]
async fn main() -> StdResult<()> {
    let args = Args::parse();

    let config = Config::load(args.network)?;
    println!("Bitcoin RPC command line, ver. {}", env!("CARGO_PKG_VERSION"));
    println!("Network: {:?}", config.network);
    println!("Node address: {}", config.node_address);

    let mut session = session::Session::new(&config);
    while !session.closed() {
        print!("Command: ");
        let _ = io::stdout().flush();

        let user_command = command_line::read_command();
        match command_line::translate_command(&user_command) {
            Ok(Command::Exit) => {
                session.close();
                println!("Bye.");
                break;
            },
            Ok(Command::GetBlockchainInfo) => {
                let blockchain_info = node::get_blockchain_info(&config).await;
            },
            Err(err) => println!("Error: {}", err),
        }
    }

    Ok(())
}
