use block_value_calculator;
use clap::{Arg, Command};

fn main() {
    // Path to the Bitcoin Core .cookie file
    let cfg = block_value_calculator::config::Config::new().unwrap();
    let calculator = block_value_calculator::BlockValueCalculator::new(cfg);

    let app = Command::new("app")
        .arg(Arg::new("height").long("height").short('e'))
        .arg(Arg::new("hash").short('a').long("hash"))
        .arg(Arg::new("best").short('b'))
        .get_matches();

    if let Some(height) = app.get_one::<String>("height") {
        let block_height: u64 = height.parse().unwrap();
        let block = calculator.get_block_from_height(block_height);
        let (tx_count, total_amount, block_hash) = calculator.calculate_total_value(&block);
        println!(
            "Block hash: {block_hash}\nTotal transactions:{tx_count}\nTotal BTC: {total_amount}",
        )
    }

    if let Some(hash) = app.get_one::<String>("hash") {
        let block = calculator.get_block_from_hash(hash.to_string());
        let (tx_count, total_amount, block_hash) = calculator.calculate_total_value(&block);
        println!(
            "Block hash: {block_hash}\nTotal transactions:{tx_count}\nTotal BTC: {total_amount}",
        )
    }

    if let Some(best) = app.get_one::<String>("best") {
        if best.parse::<bool>().unwrap() {
            let block = calculator.get_best_block();
            let (tx_count, total_amount, block_hash) = calculator.calculate_total_value(&block);
            println!(
            "Block hash: {block_hash}\nTotal transactions:{tx_count}\nTotal BTC: {total_amount}",
        )
        }
    }
}
