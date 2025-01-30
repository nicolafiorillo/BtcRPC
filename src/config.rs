use crate::std_result::StdResult;
use ini::Ini;

pub type Port = u16;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Config {
    pub network: Network,
    pub node_address: String,
}

static CONFIG_FILE: &str = ".config.ini";

// implement load
impl Config {
    pub fn load(network: String) -> StdResult<Config> {
        let network_lowercase = network.to_lowercase();
        let conf = Ini::load_from_file(CONFIG_FILE)?;

        let main_section = conf
            .section(Some("main".to_string()))
            .ok_or("main section missing")?;
        let node_address = main_section.get("node_address").ok_or("node_address missing")?.to_string();

        let network_section = conf
            .section(Some(network_lowercase.clone()))
            .ok_or("invalid network")?;
        let port = network_section.get("port").unwrap().parse::<Port>()?;

        match network_lowercase.as_str() {
            "mainnet" => Ok(Config {
                network: Network::Mainnet(port),
                node_address,
            }),
            "testnet" => Ok(Config {
                network: Network::Testnet(port),
                node_address,
            }),
            _ => Err("invalid network".into()),
        }
    }
}
