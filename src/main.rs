use bitcoin::{Amount, BlockHash};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::{
    hash::{DefaultHasher, Hash},
    path::PathBuf,
};

fn main() {
    // Path to the Bitcoin Core .cookie file
    let cookie_file_path = PathBuf::from("/Volumes/externalSSD/Bitcoin/.cookie"); // Update this path accordingly

    // Create an RPC client using the cookie for authentication
    let rpc_url = "http://localhost:8332";
    let rpc_auth = Auth::CookieFile(cookie_file_path);
    let block_height = 277316;

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
        // txid.as_raw_hash();
        println!("Processing transaction: {}", &txid);
        let transaction = rpc
            .get_raw_transaction(&txid, None)
            .expect("Wasn't able to get transaction ");
        let mut total_out = 0;
        for txout in transaction.output.iter() {
            total_value += txout.value;
            total_out += 1;
            // println!("Value of out {}: {}", idx, txout.value);
        }

        let st = "=".repeat(txid.as_raw_hash().to_string().len());
        println!("Total out: {}", total_out);
        println!("{}/{count}", block.txdata.len());
        println!("{}{}", "=".repeat(23), st);
    }
    println!("Total value: {}", total_value);
}
