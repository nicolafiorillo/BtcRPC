use reqwest::header::{HeaderMap, HeaderValue};

use crate::{config::Config, std_result::StdResult};

pub async fn get_blockchain_info(config: &Config) -> StdResult<()> {
    let result = get("getblockchaininfo".to_string(), config).await;

    match result {
        Ok(response) => {
            println!("Response: {:?}", response);
            Ok(())
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(err)
        }
    }
}

async fn get(command: String, config: &Config) -> StdResult<serde_json::Value> {
    let username = get_env_var("BITCOIN_RPC_USER");
    let password = get_env_var("BITCOIN_RPC_PASSWORD");

    let address = format!("{}:{}", config.address, config.port);
    let message = format!(
        "{{\"jsonrpc\": \"1.0\", \"id\": \"curltest\", \"method\": \"{}\", \"params\": []}}",
        command
    );

    let client = reqwest::Client::new();

    let headers: HeaderMap<HeaderValue> = {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("text/plain;"));
        headers
    };

    let response = client
        .post(address)
        .headers(headers)
        .basic_auth(username, Some(password))
        .body(message)
        .send()
        .await;

    match response {
        Ok(r) => {
            let json: serde_json::Value = r.json().await.unwrap();
            Ok(json)
        }
        Err(err) => {
            println!("Error sending notification: {}", err);
            Err(err.into())
        }
    }
}

fn get_env_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{} must be set", name))
}
