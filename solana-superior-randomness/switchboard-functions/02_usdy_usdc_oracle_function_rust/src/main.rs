use anchor_client::solana_sdk::program_pack::Pack;
use anchor_client::Client;
use rust_decimal::Decimal;
use solend_sdk::state::Reserve;
use switchboard_solana::solana_sdk::commitment_config::CommitmentConfig;
use std::sync::Arc;
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

declare_id!("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d");

pub const PROGRAM_SEED: &[u8] = b"USDY_USDC_ORACLE_V2";

pub const ORACLE_SEED: &[u8] = b"ORACLE_USDY_SEED_V2";
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
    let keypair = Keypair::new();
    let solend_wsol_reserve = Pubkey::from_str("8PbodeaosQP19SjYFx855UMqWxH2HynZLdBXmsrbac36").unwrap();
    let client = Client::new_with_options(
        Cluster::Custom("https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718".to_string(), "https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718".to_string()),
        Arc::new(keypair),
        CommitmentConfig::processed(),
    );
    let program_id = Pubkey::from_str("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d").unwrap();
    let program: anchor_client::Program<Arc<Keypair>> =
        client.program(program_id).unwrap();

    let data = program.async_rpc()
        .get_account_data(&solend_wsol_reserve)
        .await
        .unwrap();
    let reserve: Reserve = Reserve::unpack_from_slice(&data).unwrap();
    let reserve_borrow_rate = reserve.current_borrow_rate().unwrap().0.checked_div(u128::from(1_000_000_000 as u64).into()).unwrap().as_u128() as f64 / 1_000_000_000.0;
    println!("reserve_borrow_rate: {:?}", reserve_borrow_rate); // this is 0.049855926  should be 5.66%, 5.66% / 0.049855926 = 113.5
    let reserve_borrow_rate = reserve_borrow_rate * 1.135;
    println!("reserve_borrow_rate: {:?}", reserve_borrow_rate);
    let reserve_borrow_rate : u128 = (reserve_borrow_rate * 1_000_000_000.0) as u128;


    // Finally, emit the signed quote and partially signed transaction t    o the functionRunner oracle
    // The functionRunner oracle will use the last outputted word to stdout as the serialized result. This is what gets executed on-chain.
    let etherprices = EtherPrices::fetch(
        // implement error handling and map_err
        ethers::types::U256::from(ToPrimitive::to_u128(&bsol_mean).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&bsol_median).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&(0 as u64)).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&jitosol_mean).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&jitosol_median).unwrap()),
        ethers::types::U256::from(ToPrimitive::to_u128(&(0 as u64)).unwrap()),
        ethers::types::U256::from(reserve_borrow_rate),
        ethers::types::U256::from(reserve_borrow_rate),
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