use crate::futures::future::join_all;
use rust_decimal::Decimal;
use std::boxed::Box;
use std::future::Future;
use std::pin::Pin;
pub use switchboard_solana::prelude::*;
pub mod etherprices;

pub use etherprices::*;
use std::str::FromStr;
use switchboard_solana::switchboard_function;
use switchboard_utils;
use switchboard_utils::FromPrimitive;
use switchboard_utils::SbError;
use switchboard_utils::ToPrimitive;
use tokio;

use ethers::types::I256;

use ethers_contract_derive::abigen;

declare_id!("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d");

pub const PROGRAM_SEED: &[u8] = b"USDY_USDC_ORACLE";

pub const ORACLE_SEED: &[u8] = b"ORACLE_USDY_SEED";
//
#[account(zero_copy(unsafe))]
pub struct MyProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub switchboard_function: Pubkey,
    pub btc_price: f64,
}
#[switchboard_function]
pub async fn etherprices_oracle_function(
    runner: FunctionRunner,
    _params: Vec<u8>,
) -> Result<Vec<Instruction>, SbFunctionError> {
    msg!("etherprices_oracle_function");
    
    let bsol_price: f64 = reqwest::get("https://quote-api.jup.ag/v6/quote?inputMint=bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1&outputMint=So11111111111111111111111111111111111111112&amount=1000000000")
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap()
        ["outAmount"]
        .as_str()
        .unwrap()
        .to_string().parse().unwrap();

    let bsol_median = bsol_price;
    let bsol_mean = bsol_price;

    println!("bsol_price: {:?}", bsol_mean);
    let population_std = 1 as f64;
    println!("population_std: {:?}", population_std);

    let bsol_mean =
        Decimal::from_f64(bsol_mean).unwrap() * Decimal::from(1 as u64);
    let bsol_median =
        Decimal::from_f64(bsol_median).unwrap() * Decimal::from(1 as u64);
        msg!("etherprices_oracle_function");

        let jitosol_price: f64 = reqwest::get("https://quote-api.jup.ag/v6/quote?inputMint=J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn&outputMint=So11111111111111111111111111111111111111112&amount=1000000000")
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap()
        ["outAmount"]
        .as_str()
        .unwrap()
        .to_string().parse().unwrap();

        println!("jitosol_price: {:?}", jitosol_price);

    //{"blazestake-staked-sol":{"usd":72.91}}
    let jitosol_median = jitosol_price;
    let jitosol_mean = jitosol_price;

    let population_std = 1 as f64;
    println!("population_std: {:?}", population_std);

    let jitosol_mean =
        Decimal::from_f64(jitosol_mean).unwrap() * Decimal::from(1 as u64);
    let jitosol_median =
        Decimal::from_f64(jitosol_median).unwrap() * Decimal::from(1 as u64);
    msg!("sending transaction");

    // Finally, emit the signed quote and partially signed transaction to the functionRunner oracle
    // The functionRunner oracle will use the last outputted word to stdout as the serialized result. This is what gets executed on-chain.
    let etherprices = EtherPrices::fetch(
        // implement error handling and map_err
        ethers::types::U256::from(ToPrimitive::to_u128(&bsol_mean).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&bsol_median).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&(0 as u64)).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&jitosol_mean).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&jitosol_median).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&(0 as u64)).unwrap())
    )
    .await
    .unwrap();
        
    let ixs: Vec<Instruction> = etherprices.to_ixns(&runner);
    Ok(ixs)
}

#[sb_error]
pub enum Error {
    InvalidResult,
}