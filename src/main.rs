// src/main.rs
/*
 * Main executable for BlockchainNFTVaultLabsPro
 */

use clap::Parser;
use blockchainnftvaultlabspro::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTVaultLabsPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
