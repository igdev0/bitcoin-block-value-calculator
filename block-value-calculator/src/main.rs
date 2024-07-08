use bitcoin::{Amount, BlockHash};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use block_value_calculator::Config;
use std::{
    hash::{DefaultHasher, Hash},
    path::PathBuf,
};

fn main() {
    // Path to the Bitcoin Core .cookie file

    // let

    let cfg: Config = Config::new().unwrap();
    dbg!(&cfg);
    // Create an RPC client using the cookie for authentication
    let rpc_url = "http://localhost:8332";
    let rpc_auth: Auth;

    if cfg.auth_default {
        rpc_auth = Auth::CookieFile(PathBuf::from(cfg.path_to_auth_cookie));
    } else {
        rpc_auth = Auth::UserPass(cfg.rpc_username.unwrap(), cfg.rpc_password.unwrap());
    }
    let block_height = 1000;

    let rpc = Client::new(rpc_url, rpc_auth).expect("Failed to create RPC client");

    let best_hash = rpc
        .get_block_hash(block_height)
        .expect("Wasn't able to find the best hash");

    let block = rpc
        .get_block(&best_hash)
        .expect("Wasn't able to receive block");
    let mut total_value: Amount = Amount::from_btc(0.0).expect("Some message");
    println!("Total transactions: {}", block.txdata.len());
    let mut count = 0;
    for tx in block.txdata.iter() {
        let txid = tx.compute_txid();
        count += 1;
        println!("Processing transaction: {}", &txid);
        let transaction = rpc
            .get_raw_transaction(&txid, None)
            .expect("Wasn't able to get transaction ");
        let mut total_out = 0;
        for txout in transaction.output.iter() {
            total_value += txout.value;
            total_out += 1;
        }

        let st = "=".repeat(txid.as_raw_hash().to_string().len());
        println!("Total out: {}", total_out);
        println!("{}/{count}", block.txdata.len());
        println!("{}{}", "=".repeat(23), st);
    }
    println!("Total value: {}", total_value);
}
