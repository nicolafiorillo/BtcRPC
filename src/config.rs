use crate::std_result::StdResult;
use ini::Ini;

type Port = u16;

#[derive(Debug)]
pub enum Network {
    Mainnet(Port),
    Testnet(Port),
}

impl Network {
    pub fn port(&self) -> Port {
        match self {
            Network::Mainnet(port) => *port,
            Network::Testnet(port) => *port,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub network: Network,
}

static CONFIG_FILE: &str = ".config.ini";

// implement load
impl Config {
    pub fn load(network: String) -> StdResult<Config> {
        let network_lowercase = network.to_lowercase();
        let conf = Ini::load_from_file(CONFIG_FILE)?;

        let section = conf
            .section(Some(network_lowercase.clone()))
            .ok_or("invalid network")?;
        let port = section.get("port").unwrap().parse::<Port>()?;

        match network_lowercase.as_str() {
            "mainnet" => Ok(Config {
                network: Network::Mainnet(port),
            }),
            "testnet" => Ok(Config {
                network: Network::Testnet(port),
            }),
            _ => Err("invalid network".into()),
        }
    }
}
