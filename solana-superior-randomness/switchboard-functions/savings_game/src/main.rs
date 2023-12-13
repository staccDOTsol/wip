use crate::futures::future::join_all;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use solana_account_decoder::UiDataSliceConfig;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use spl_associated_token_account::{get_associated_token_address_with_program_id, get_associated_token_address};
use superior_randomness::{MarginFiPda, Winner};
use switchboard_solana::anchor_client::Client;
use switchboard_solana::{anchor_client::Program, Keypair, Pubkey};
use solend_sdk::{math::{Decimal, Rate, TryMul, TryDiv}, state::Obligation};

use std::boxed::Box;
use std::future::Future;
use std::pin::Pin;
pub use switchboard_solana::prelude::*;
pub mod etherprices;
pub use solana_client::*;
use std::sync::Arc;
pub use switchboard_solana::get_ixn_discriminator;
pub use switchboard_solana::prelude::*;
use switchboard_solana::sb_error;
pub use etherprices::*;
use std::str::FromStr;
use switchboard_solana::switchboard_function;
use switchboard_utils;
use switchboard_utils::FromPrimitive;
use switchboard_utils::SbError;
use switchboard_utils::ToPrimitive;
use tokio;

use ethers::types::I256;


/// Collateral exchange rate
#[derive(Clone, Copy, Debug)]
pub struct CollateralExchangeRate(Rate);

impl CollateralExchangeRate {
    /// Convert reserve collateral to liquidity
    pub fn collateral_to_liquidity(&self, collateral_amount: u64) -> anchor_lang::Result<u64> {
        Ok(self.decimal_collateral_to_liquidity(collateral_amount.into())?
            .try_floor_u64().unwrap())
    }

    /// Convert reserve collateral to liquidity
    pub fn decimal_collateral_to_liquidity(
        &self,
        collateral_amount: Decimal,
    ) -> anchor_lang::Result<Decimal> {
        Ok(collateral_amount.try_div(self.0).unwrap())
    }

    /// Convert reserve liquidity to collateral
    pub fn liquidity_to_collateral(&self, liquidity_amount: u64) -> anchor_lang::Result<u64> {
        Ok(self.decimal_liquidity_to_collateral(liquidity_amount.into())?
            .try_floor_u64().unwrap())
    }

    /// Convert reserve liquidity to collateral
    pub fn decimal_liquidity_to_collateral(
        &self,
        liquidity_amount: Decimal,
    ) -> anchor_lang::Result<Decimal> {
        Ok(liquidity_amount.try_mul(self.0).unwrap())
    }
}

impl From<CollateralExchangeRate> for Rate {
    fn from(exchange_rate: CollateralExchangeRate) -> Self {
        exchange_rate.0
    }
}
/// Return the current collateral exchange rate.
pub fn exchange_rate(
    total_liquidity: u64,
    mint_total_supply: u64
) -> anchor_lang::Result<CollateralExchangeRate> {
    
    let mint_total_supply = Decimal::from(mint_total_supply);
    let rate = Rate::try_from(mint_total_supply.try_div(Decimal::from(total_liquidity))?)?;

    Ok(CollateralExchangeRate(rate))
}
declare_id!("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d");

#[derive(Clone)]
pub struct StakeProgram;

impl anchor_lang::Id for StakeProgram {
    fn id() -> Pubkey {
        Pubkey::from_str("Stake11111111111111111111111111111111111111").unwrap()
    }
}

#[derive(Clone)]
pub struct SolendProgram;

impl anchor_lang::Id for SolendProgram {
    fn id() -> Pubkey {
        Pubkey::from_str("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo").unwrap()
    }
}

const SEED_PREFIX: &[u8] = b"jarezi";
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
pub struct Holder {
    pub pubkey: Pubkey,
    pub amount: u64,
}

fn generate_randomness(min: u32, max: u32) -> u32 {
    if min == max {
        return min;
    }
    if min > max {
        return generate_randomness(max, min);
    }

    // We add one so its inclusive [min, max]
    let window = (max + 1) - min;

    let mut bytes: [u8; 4] = [0u8; 4];
    Gramine::read_rand(&mut bytes).expect("gramine failed to generate randomness");
    let raw_result: &[u32] = bytemuck::cast_slice(&bytes[..]);

    (raw_result[0] % window) + min
}

#[switchboard_function]
pub async fn etherprices_oracle_function(
    runner: FunctionRunner,
    _params: Vec<u8>,
) -> Result<Vec<Instruction>, SbFunctionError> {
    msg!("etherprices_oracle_function");
    
    // Define the program ID of your deployed Anchor program
    let program_id = superior_randomness::id();
    let keypair = Keypair::new();
    let client = Client::new_with_options(
        Cluster::Custom("https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718".to_string(), "https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718".to_string()),
        Arc::new(keypair),
        CommitmentConfig::processed(),
    );
    let program: Program<Arc<Keypair>> =
        client.program(program_id).unwrap();

    // Define the accounts that will be passed to the function
    let (marginfi_pda, _bump) =
        Pubkey::find_program_address(&[b"jarezi", Pubkey::from_str("JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm").unwrap().as_ref()], &program_id);
    let marginfi_pda_account: MarginFiPda = program.account(marginfi_pda).await.unwrap();
    let winner_winner_chickum_dinner = marginfi_pda_account.winner_winner_chickum_dinner;
    // Initialize other accounts as required by the Winner struct

    let token_program_2022 = anchor_spl::token_interface::Token2022::id();
    // Define the amount to distribute

    let jarezi_mint = marginfi_pda_account.jarezi_mint;
    let token_supply = program.async_rpc().get_token_supply(&jarezi_mint).await.unwrap();

    let pool_mint_jitosol = Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap();

    let pool_token_receiver_account_jitosol = get_associated_token_address(
        &marginfi_pda,
        &pool_mint_jitosol,
    );
    println!("token_supply.amount {:?}", token_supply.amount);
    let jito_amount = program.async_rpc().get_token_account_balance(&pool_token_receiver_account_jitosol).await.unwrap();
    let rate = exchange_rate(
        u64::from_str(&jito_amount.amount).unwrap(),
        u64::from_str(&token_supply.amount).unwrap() 
    ).unwrap();
    let rate: f64 = (rate.0.to_scaled_val() / 1000000000000000000) as f64;

    println!("rate: {:?}", rate);
        
    // calculate the amount to bring the exchange rate to exactly 1
    let amount =  (u64::from_str(&token_supply.amount).unwrap()) - u64::from_str(&jito_amount.amount).unwrap() * 900 / 1000;

    println!("amount: {:?}", amount);

    
        
    let random_result = generate_randomness(0, (token_supply.ui_amount.unwrap()) as u32);
   
    let holders = program.async_rpc().get_program_accounts_with_config(
        &anchor_spl::token_interface::Token2022::id(),
        solana_client::rpc_config::RpcProgramAccountsConfig {
            filters: Some(vec![
                solana_client::rpc_filter::RpcFilterType::Memcmp(solana_client::rpc_filter::Memcmp {
                offset: 0,
                bytes: solana_client::rpc_filter::MemcmpEncodedBytes::Binary(marginfi_pda_account.jarezi_mint.to_string()),
                encoding: None
            })]),account_config: RpcAccountInfoConfig {
                min_context_slot: None,
                encoding: Some(solana_account_decoder::UiAccountEncoding::Base64Zstd),
                commitment: Some(CommitmentConfig::processed()),
                data_slice: Some(UiDataSliceConfig {
                    offset: 0,
                    length: 165 as usize
                }),
            },
            ..RpcProgramAccountsConfig::default()
        },
    ).await.unwrap();
    println!("holders: {:?}", holders.len());
    println!("jarezimint: {:?}", jarezi_mint);
    let mut holders: Vec<Holder> = holders
        .into_iter()
        .map(|acc| {
            let buf:&mut &[u8] = &mut &acc.1.data[..];


            let parsed: anchor_spl::token_interface::TokenAccount = anchor_spl::token_interface::TokenAccount::try_deserialize_unchecked(buf).unwrap();
            Holder {
                pubkey: acc.0,
                amount: parsed.amount,
            }
        })
        
        .collect(); 
        println!("random_result: {:?}", random_result);

    let mut total: u32 = (token_supply.ui_amount.unwrap()) as u32;
    println!("total: {:?}", total);
    let mut actual_destination = Pubkey::default();
    for holder in &holders {
        println!("holder: {:?}", holder.amount / 1_000_000_000);
        total -= (holder.amount / 1_000_000_000) as u32;
        println!("total: {:?}", total);
        println!("holder pubkey: {:?}", holder.pubkey);
        if total < random_result {
            // get associated token account for token_2022
            actual_destination = holder.pubkey;
            break;
        }
    }
    println!("actual_destination: {:?}", actual_destination);
    let system_program = solana_program::system_program::id();
    
    let params = amount.to_le_bytes().to_vec();
    let ixn = Instruction {
        program_id: program_id,
        accounts: vec![
            AccountMeta {
                pubkey: marginfi_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: winner_winner_chickum_dinner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: actual_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: token_program_2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: jarezi_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_token_receiver_account_jitosol,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pool_mint_jitosol,
                is_signer: false,
                is_writable: true,
            },

            AccountMeta {
                pubkey: runner.function,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: runner.signer,
                is_signer: true,
                is_writable: false,
            },
        ],
        data: [
            get_ixn_discriminator("winner_winner_chickum_dinner_distribute").to_vec(),
            params,
        ]
        .concat(),
    };
    Ok([ixn].to_vec())
}

#[sb_error]
pub enum Error {
    InvalidResult,
}