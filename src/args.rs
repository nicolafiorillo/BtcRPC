use clap::Parser;

#[derive(Clone, Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// mainnet | testnet
    #[arg(short, long, default_value_t = String::from("mainnet"))]
    pub network: String,
}
