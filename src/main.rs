use args::Args;
use clap::Parser;
use config::Config;
use std_result::StdResult;

mod args;
mod config;
mod std_result;

fn main() -> StdResult<()> {
    let args = Args::parse();
    println!("args: {:?}", args);

    let config = Config::load(args.network)?;
    println!(
        "Network: {:?}, port: {}",
        config.network,
        config.network.port()
    );

    Ok(())
}
