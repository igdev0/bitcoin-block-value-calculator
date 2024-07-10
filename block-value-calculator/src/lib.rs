use bitcoin::{Amount, Block, BlockHash};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::{path::PathBuf, str::FromStr};

pub mod config;
// Command
pub struct BlockValueCalculator {
    rpc: Client,
}

impl BlockValueCalculator {
    pub fn new(cfg: config::Config) -> Self {
        // Self::rp
        let rpc_auth: Auth;

        if cfg.auth_default {
            rpc_auth = Auth::CookieFile(PathBuf::from(cfg.path_to_auth_cookie));
        } else {
            rpc_auth = Auth::UserPass(cfg.rpc_username.unwrap(), cfg.rpc_password.unwrap());
        }

        let rpc = Client::new(cfg.rpc_url.as_str(), rpc_auth).expect("Failed to create RPC client");
        BlockValueCalculator { rpc }
    }

    pub fn calculate_total_value(&self, block: &Block) {
        let mut total_value: Amount = Amount::from_btc(0.0).expect("Some message");
        println!("Total transactions: {}", block.txdata.len());
        let mut count = 0;
        for tx in block.txdata.iter() {
            let txid = tx.compute_txid();
            count += 1;
            println!("Processing transaction: {}", &txid);
            let transaction = self
                .rpc
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

    pub fn get_block_from_height(&self, block_height: u64) -> Block {
        let block_hash = self
            .rpc
            .get_block_hash(block_height)
            .expect("Wasn't able to find the best hash");

        self.rpc
            .get_block(&block_hash)
            .expect("Wasn't able to receive block")
    }

    pub fn get_block_from_hash(&self, hash: String) -> Block {
        let block_hash = BlockHash::from_str(hash.as_str()).unwrap();
        self.rpc.get_block(&block_hash).unwrap()
    }

    pub fn get_best_block(&self) -> Block {
        let block_hash = self.rpc.get_best_block_hash().unwrap();
        self.rpc.get_block(&block_hash).unwrap()
    }
}
