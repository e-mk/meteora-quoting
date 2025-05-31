#[macro_use]
extern crate lazy_static;

pub mod constant;
pub mod quote_service;
pub mod rpc_service;
pub mod state;

use anchor_lang::prelude::*;
use constant::RPC_URL;
use quote_service::QuoteService;
use rpc_service::RpcService;
use solana_program::pubkey::Pubkey;

use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // METAV
    let metav_mint = Pubkey::from_str("HCgvbV9Qcf9TVGPGKMGbVEj8WwwVD6HhTt5E2i3qkeN9").unwrap();

    // USDC
    let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap();

    // WSOL
    let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

    // USDC/WSOL
    let usdc_sol_pool_address =
        Pubkey::from_str("5yuefgbJJpmFNK2iiYbLSpv1aZXq7F9AUKkZKErTYCvs").unwrap();

    // METAV/WSOL
    let metav_wsol_pool_address =
        Pubkey::from_str("EH8xLzfq2YARgQC846NWP6EfRK9gfcjDJMxcHhxFLruv").unwrap();

    let rpc_service = RpcService::new(RPC_URL.to_string());

    let quote_service = QuoteService::new(rpc_service);
    // Note: we will need RPC calls to get the decimals of the tokens so it is not implemented
    let quote_result = quote_service.get_for_pair(wsol_mint, metav_wsol_pool_address, 1000000000);
    println!("Quote result: {:?}", quote_result);

    Ok(())
}
