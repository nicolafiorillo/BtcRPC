use crate::std_result::StdResult;
use ini::Ini;

pub type Port = u16;

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub port: Port,
}

// implement load
impl Config {
    pub fn load(network: String) -> StdResult<Config> {
        let network_file = format!("{}.ini", network.to_lowercase());
        let conf = Ini::load_from_file(&network_file)
            .map_err(|_| format!("config file {} not found", network_file))?;

        let node_section = conf
            .section(Some("node".to_string()))
            .ok_or("node section missing")?;

        let address = node_section
            .get("address")
            .ok_or("address missing")?
            .to_string();

        let port = node_section
            .get("port")
            .ok_or("port missing")?
            .parse::<Port>()
            .or(Err("port must be a number"))?;

        Ok(Config { address, port })
    }
}
