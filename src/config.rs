use std::{env, fs};
#[derive(Debug)]
pub struct Config {
    pub rpc_username: Option<String>,
    pub rpc_password: Option<String>,
    pub rpc_url: String,
    pub path_to_auth_cookie: String,
    pub auth_default: bool,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        let input_rpc_username = env::var("RPC_USERNAME");
        let input_rpc_password = env::var("RPC_PASSWORD");
        let input_rpc_url =
            env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8332".to_string());
        let input_use_default_auth: bool = env::var("RPC_DEFAULT_AUTH")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap();
        let path_to_auth_cookie = env::var("RPC_COOKIE_PATH")
            .unwrap_or_else(|_| "/Volumes/externalSSD/Bitcoin/.cookie".to_string());

        if input_use_default_auth {
            // Check if the file exists, if it does not exist, panic.
            fs::read(&path_to_auth_cookie).expect("Wasn't able to read the auth cookie file, make sure that the .cookie exists. Often happens that you have your credential setup in your bicoin.conf, delete those and restart bitcoind.");

            return Ok(Config {
                rpc_password: None,
                rpc_username: None,
                rpc_url: input_rpc_url,
                path_to_auth_cookie,
                auth_default: true,
            });
        }

        let mut username: Option<String> = None;
        let mut password: Option<String> = None;

        if let Ok(rpc_username) = input_rpc_username {
            username = Some(rpc_username);
        } else {
            return Err(
                "The RPC username is required, please update your environment!".to_string(),
            );
        }

        if let Ok(rpc_password) = input_rpc_password {
            password = Some(rpc_password);
        } else {
            return Err(
                "The RPC password is required, please update your environment!".to_string(),
            );
        }

        Ok(Config {
            path_to_auth_cookie,
            rpc_url: input_rpc_url,
            rpc_password: password,
            rpc_username: username,
            auth_default: false,
        })
    }
}
