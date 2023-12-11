pub use crate::SbError;
pub use crate::*;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token_interface::{MintTo, Token2022};
use pyth_sdk_solana::state::PriceAccount;
use solana_program::program_pack::Pack;
use solend_sdk::state::*;
use solend_sdk::math::Decimal;
use arrayref::array_ref;
use solend_sdk::state::*;
use crate::StakePool;
use std::io::Read;
/// Max number of collateral and liquidity reserve accounts combined for an obligation
pub const MAX_OBLIGATION_RESERVES: usize = 10;

/// The unit of time given to a leader for encoding a block.
///
/// It is some some number of _ticks_ long.
pub type Slot = u64;

use std::cmp::Ordering;

/// Number of slots to consider stale after
pub const STALE_AFTER_SLOTS_ELAPSED: u64 = 1;

/// Last update state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LastUpdate {
    /// Last slot when updated
    pub slot: Slot,
    /// True when marked stale, false when slot updated
    pub stale: bool,
}
impl AnchorSerialize for LastUpdate {
    fn serialize<W: anchor_lang_26::prelude::borsh::maybestd::io::Write>(&self, writer: &mut W) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<()> {
        self.slot.serialize(writer)?;
        writer.write(&[self.stale as u8])?;
        Ok(())
    }
}
impl AnchorDeserialize for LastUpdate {
    fn deserialize_reader<R: Read>(reader: &mut R) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<Self> {
        let slot = Slot::deserialize_reader(reader)?;
        let stale = u8::deserialize_reader(reader)? != 0;
        Ok(Self {
            slot,
            stale,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RateLimiter {
    /// configuration parameters
    pub config: RateLimiterConfig,

    // state
    /// prev qty is the sum of all outflows from [window_start - config.window_duration, window_start)
    prev_qty: Decimal,
    /// window_start is the start of the current window
    window_start: Slot,
    /// cur qty is the sum of all outflows from [window_start, window_start + config.window_duration)
    cur_qty: Decimal,
}
impl AnchorDeserialize for RateLimiter {
    fn deserialize_reader<R: Read>(reader: &mut R) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<Self> {
        let config = RateLimiterConfig::deserialize_reader(reader)?;
        let prev_qty = u64::deserialize_reader(reader)?.into();
        let window_start = Slot::deserialize_reader(reader)?.into();
        let cur_qty = u64::deserialize_reader(reader)?.into();
        Ok(Self {
            config,
            prev_qty,
            window_start,
            cur_qty,
        })
    }
}
impl AnchorSerialize for RateLimiter {
    fn serialize<W: anchor_lang_26::prelude::borsh::maybestd::io::Write>(&self, writer: &mut W) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<()> {
        self.config.serialize(writer)?;
        self.prev_qty.0.0.serialize(writer)?;
        self.window_start.serialize(writer)?;
        self.cur_qty.0.0.serialize(writer)?;
        Ok(())
    }
}

/// Lending market configuration parameters
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RateLimiterConfig {
    /// Rate limiter window size in slots
    pub window_duration: u64,
    /// Rate limiter param. Max outflow of tokens in a window
    pub max_outflow: u64,
}
impl AnchorDeserialize for RateLimiterConfig {
    fn deserialize_reader<R: Read>(reader: &mut R) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<Self> {
        let window_duration = u64::deserialize_reader(reader)?.into();
        let max_outflow = u64::deserialize_reader(reader)?.into();
        Ok(Self {
            window_duration,
            max_outflow,
        })
    }
}
impl AnchorSerialize for RateLimiterConfig {
    fn serialize<W: anchor_lang_26::prelude::borsh::maybestd::io::Write>(&self, writer: &mut W) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<()> {
        self.window_duration.serialize(writer)?;
        self.max_outflow.serialize(writer)?;
        Ok(())
    }
}

/// Lending market state
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LendingMarketZi {
    /// Version of lending market
    pub version: u8,
    /// Bump seed for derived authority address
    pub bump_seed: u8,
    /// Owner authority which can add new reserves
    pub owner: Pubkey,
    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],
    /// Token program id
    pub token_program_id: Pubkey,
    /// Oracle (Pyth) program id
    pub oracle_program_id: Pubkey,
    /// Oracle (Switchboard) program id
    pub switchboard_oracle_program_id: Pubkey,
    /// Outflow rate limiter denominated in dollars
    pub rate_limiter: RateLimiter,
    /// whitelisted liquidator
    pub whitelisted_liquidator: Option<Pubkey>,
    /// risk authority (additional pubkey used for setting params)
    pub risk_authority: Pubkey,
}
impl anchor_lang::AccountDeserialize for LendingMarketZi {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        
        let version = u8::deserialize_reader(buf)?;
        let bump_seed = u8::deserialize_reader(buf)?;
        let owner = Pubkey::deserialize_reader(buf)?;
        let quote_currency = <[u8; 32]>::deserialize_reader(buf)?;
        let token_program_id = Pubkey::deserialize_reader(buf)?;
        let oracle_program_id = Pubkey::deserialize_reader(buf)?;
        let switchboard_oracle_program_id = Pubkey::deserialize_reader(buf)?;
        let rate_limiter = RateLimiter::deserialize_reader(buf)?;
        let whitelisted_liquidator = Pubkey::deserialize_reader(buf)?;

        let risk_authority = Pubkey::deserialize_reader(buf)?;
        if whitelisted_liquidator == Pubkey::new_from_array([0u8; 32]) {
            Ok(Self {
                version,
                bump_seed,
                owner,
                quote_currency,
                token_program_id,
                oracle_program_id,
                switchboard_oracle_program_id,
                rate_limiter,
                whitelisted_liquidator: None,
                risk_authority,
            })
        }
        else {
            Ok(Self {
                version,
                bump_seed,
                owner,
                quote_currency,
                token_program_id,
                oracle_program_id,
                switchboard_oracle_program_id,
                rate_limiter,
                whitelisted_liquidator: Some(whitelisted_liquidator),
                risk_authority,
            })
        }
        
    }
}
impl anchor_lang::AccountSerialize for LendingMarketZi {
    fn try_serialize<T:  std::io::Write>(&self, buf: &mut T) -> Result<()> {
        self.version.serialize(buf)?;
        self.bump_seed.serialize(buf)?;
        self.owner.serialize(buf)?;
        self.quote_currency.serialize(buf)?;
        self.token_program_id.serialize(buf)?;
        self.oracle_program_id.serialize(buf)?;
        self.switchboard_oracle_program_id.serialize(buf)?;
        self.rate_limiter.serialize(buf)?;
        (self.whitelisted_liquidator.is_some() as u8).serialize(buf)?;
        if let Some(whitelisted_liquidator) = self.whitelisted_liquidator {
            whitelisted_liquidator.serialize(buf)?;
        }
        self.risk_authority.serialize(buf)?;
        Ok(())
    }
}
// impl AnchorSerialize and Deserialize for LendingMarket
impl AnchorSerialize for LendingMarketZi {
    fn serialize<W: anchor_lang_26::prelude::borsh::maybestd::io::Write>(&self, writer: &mut W) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<()> {
        writer.write(&[self.version])?;
        writer.write(&[self.bump_seed])?;
        writer.write_all(self.owner.as_ref())?;
        writer.write_all(self.quote_currency.as_ref())?;
        writer.write_all(self.token_program_id.as_ref())?;
        writer.write_all(self.oracle_program_id.as_ref())?;
        writer.write_all(self.switchboard_oracle_program_id.as_ref())?;
        self.rate_limiter.serialize(writer)?;
        writer.write(&[self.whitelisted_liquidator.is_some() as u8])?;
        if let Some(whitelisted_liquidator) = self.whitelisted_liquidator {
            writer.write_all(whitelisted_liquidator.as_ref())?;
        }
        writer.write_all(self.risk_authority.as_ref())?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct LiteAggregatorAccountData {
    /// Use sliding windoe or round based resolution
    /// NOTE: This changes result propogation in latest_round_result
    pub resolution_mode: AggregatorResolutionMode,
    /// Latest confirmed update request result that has been accepted as valid.
    pub latest_confirmed_round_result: SwitchboardDecimal,
    pub latest_confirmed_round_num_success: u32,
    pub latest_confirmed_round_std_deviation: SwitchboardDecimal,
    /// Minimum number of oracle responses required before a round is validated.
    pub min_oracle_results: u32,
}

impl From<&AggregatorAccountData> for LiteAggregatorAccountData {
    fn from(agg: &AggregatorAccountData) -> Self {
        Self {
            resolution_mode: agg.resolution_mode,
            latest_confirmed_round_result: agg.latest_confirmed_round.result,
            latest_confirmed_round_num_success: agg.latest_confirmed_round.num_success,
            latest_confirmed_round_std_deviation: agg.latest_confirmed_round.std_deviation,
            min_oracle_results: agg.min_oracle_results,
        }
    }
}

impl LiteAggregatorAccountData {
    /// If sufficient oracle responses, returns the latest on-chain result in SwitchboardDecimal format
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use switchboard_v2::AggregatorAccountData;
    /// use std::convert::TryInto;
    ///
    /// let feed_result = AggregatorAccountData::new(feed_account_info)?.get_result()?;
    /// let decimal: f64 = feed_result.try_into()?;
    /// ```
    pub fn get_result(&self) -> anchor_lang::Result<SwitchboardDecimal> {
        if self.resolution_mode == AggregatorResolutionMode::ModeSlidingResolution {
            return Ok(self.latest_confirmed_round_result);
        }
        let min_oracle_results = self.min_oracle_results;
        let latest_confirmed_round_num_success = self.latest_confirmed_round_num_success;
        
        Ok(self.latest_confirmed_round_result)
    }
}

impl AnchorDeserialize for LiteAggregatorAccountData {
    fn deserialize_reader<R: Read>(reader: &mut R) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<Self> {
        let resolution_mode = AggregatorResolutionMode::deserialize_reader(reader)?;
        let latest_confirmed_round_result:  SwitchboardDecimal = SwitchboardDecimal::deserialize_reader(reader)?;
        let latest_confirmed_round_num_success = u32::deserialize_reader(reader)?;
        let latest_confirmed_round_std_deviation: SwitchboardDecimal = SwitchboardDecimal::deserialize_reader(reader)?;
        let min_oracle_results = u32::deserialize_reader(reader)?;
        Ok(Self {
            resolution_mode,
            latest_confirmed_round_result,
            latest_confirmed_round_num_success,
            latest_confirmed_round_std_deviation,
            min_oracle_results,
        })
    }
}
impl AnchorSerialize for LiteAggregatorAccountData {
    fn serialize<W: anchor_lang_26::prelude::borsh::maybestd::io::Write>(&self, writer: &mut W) -> anchor_lang_26::prelude::borsh::maybestd::io::Result<()> {
        
        self.resolution_mode.serialize(writer)?;
//        self.latest_confirmed_round_result.0.serialize(writer)?;
        self.latest_confirmed_round_num_success.serialize(writer)?;
  //      self.latest_confirmed_round_std_deviation.0.serialize(writer)?;
        self.min_oracle_results.serialize(writer)?;
        
        Ok(())
    }
}

#[derive(Clone)]
pub struct StakePoolProgram;

impl anchor_lang::Id for StakePoolProgram {
    fn id() -> Pubkey {
        spl_stake_pool::ID
    }
}

#[derive(Clone)]
pub struct SolendProgram;

impl anchor_lang::Id for SolendProgram {
    fn id() -> Pubkey {
        Pubkey::from_str("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo").unwrap()
    }
}


use std::str::FromStr;
const SEED_PREFIX: &[u8] = b"marginfi";
const SEEDED_SEED: &str = "robot001";

use anchor_lang::AnchorSerialize;
#[derive(Accounts)]
#[instruction(params: CreateSeededAccountParams)] // rpc parameters hint

pub struct CreateSeededAccount<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    /// CHECK:
    pub to: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub base: AccountInfo<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds = [SEED_PREFIX],
        bump
    )]
    pub program: Box<Account<'info, MarginFiPda>>,
    #[account(mut)]
    /// CHECK:
    pub lending_market: AccountInfo<'info>,

    pub solend_sdk: Program<'info, SolendProgram>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateSeededAccountParams {
    pub seed: String,
    pub lamports: u64,
    pub space: u64,
    pub bump: u8,
}

impl CreateSeededAccount<'_> {
    pub fn create_seeded_account(
        ctx: Context<CreateSeededAccount>,
        params: CreateSeededAccountParams,
    ) -> anchor_lang::Result<()> {
        let from_pubkey = ctx.accounts.from.key();
        let to_pubkey = ctx.accounts.to.key();
        let base = ctx.accounts.base.key();
        let lamports = params.lamports;
        let mrgnfi_pda = ctx.accounts.program.clone();
        let seed = params.seed;
        msg!("seed: {}", seed);
        let space = params.space;
        let owner = ctx.accounts.owner.key();
        let instruction = system_instruction::create_account_with_seed(
            &from_pubkey,
            &to_pubkey,
            &base,
            &seed,
            lamports,
            space,
            &owner,
        );
        let seeds: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], &[mrgnfi_pda.bump]]];
        invoke_signed(
            &instruction,
            &[
                ctx.accounts.from.to_account_info().clone(),
                ctx.accounts.to.to_account_info().clone(),
                ctx.accounts.base.to_account_info().clone(),
                ctx.accounts.owner.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
            seeds,
        )?;

        Ok(())
    }
    pub fn init_obligation_account(
        ctx: Context<CreateSeededAccount>,
        params: CreateSeededAccountParams,
    ) -> anchor_lang::Result<()> {
        let to_pubkey = ctx.accounts.to.key();
        let mrgnfi_pda = ctx.accounts.program.clone();
        let seed = params.seed;
        msg!("seed: {}", seed);
        let instruction: Instruction = solend_sdk::instruction::init_obligation(
            ctx.accounts.solend_sdk.key(),
            to_pubkey,
            ctx.accounts.lending_market.key(),
            mrgnfi_pda.key(),
        );

        let seeds: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], &[mrgnfi_pda.bump]]];
        invoke_signed(
            &instruction,
            &[
                ctx.accounts.to.to_account_info().clone(),
                ctx.accounts.lending_market.to_account_info().clone(),
                mrgnfi_pda.to_account_info().clone(),
                ctx.accounts.solend_sdk.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
                ctx.accounts.rent.to_account_info().clone(),
                ctx.accounts.token_program.to_account_info().clone(),
            ],
            seeds,
        )?;

        Ok(())
    }
}
#[account]
#[derive(Default)]
pub struct MarginFiPda {
    pub bump: u8,
    pub authority: Pubkey,
}
impl InitMrgnFiPda<'_> {
    pub fn init_mrgn_fi_pda(ctx: Context<InitMrgnFiPda>, bump: u8) -> anchor_lang::Result<()> {
        let marginfi_pda = &mut ctx.accounts.marginfi_pda;
        marginfi_pda.authority = ctx.accounts.authority.key();
        marginfi_pda.bump = bump;
        let marginfi_program = ctx.accounts.marginfi_program.clone();
        let marginfi_group = ctx.accounts.marginfi_group.clone();
        let marginfi_account = ctx.accounts.marginfi_account.clone();
        let system_program = ctx.accounts.system_program.clone();

        let signer: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], &[marginfi_pda.bump]]];
        /*
        let cpi_ctx = anchor_lang_26::context::CpiContext::new_with_signer(
            marginfi_program.clone(),
            marginfi::cpi::accounts::MarginfiAccountInitialize {
                marginfi_group: marginfi_group.to_account_info().clone(),
                marginfi_account: marginfi_account.to_account_info().clone(),
                authority: marginfi_pda.to_account_info().clone(),
                system_program: system_program.to_account_info().clone(),
                fee_payer: ctx.accounts.authority.to_account_info().clone(),
            },
             &signer,
        );
        marginfi::cpi::marginfi_account_initialize(cpi_ctx).unwrap();
        */
        let mint = ctx.accounts.jarezi_mint.clone();
        let token_program_2022 = ctx.accounts.token_program_2022.clone();
        // set fees to 1.38%
        spl_token_2022::extension::transfer_fee::instruction::set_transfer_fee(
            &token_program_2022.key(),
            &mint.key(),
            &marginfi_pda.key(),
            &[],
            138,
            u64::MAX,
        )
        .unwrap();
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitMrgnFiPda<'info> {
    #[account(init,
        seeds = [SEED_PREFIX],

        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<MarginFiPda>(),
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: no validation, for educational purpose only
    #[account(mut, signer)]
    pub marginfi_account: AccountInfo<'info>,
    /// CHECK: no validation, for educational purpose only
    pub marginfi_group: AccountInfo<'info>,
    /// CHECK: no validation, for educational purpose only
    pub marginfi_program: AccountInfo<'info>,
    #[account(init,
        payer = authority,
        mint::authority = marginfi_pda,
        mint::decimals = 9,
        mint::token_program = token_program_2022,
    )]
    pub jarezi_mint: Box<InterfaceAccount<'info, anchor_spl::token_interface::Mint>>,
    pub token_program_2022: Program<'info, Token2022>,
}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,
        seeds = [SEED_PREFIX],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint,
    )]
    pub pool_token_receiver_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::authority = Pubkey::from_str("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6").unwrap(),
        token::mint = pool_mint,
    )]
    pub referrer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub stake_pool: Box<Account<'info, StakePool>>,
    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub stake_pool_withdraw_authority: AccountInfo<'info>,
    /// CHECK: Checked by CPI to Spl Stake Program
    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub reserve_stake_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub manager_fee_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    pub stake_pool_program: Program<'info, StakePoolProgram>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub marginfi_bank: Box<Account<'info, Reserve>>,
    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub marginfi_bank_jito: Box<Account<'info, Reserve>>,
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub liquidity_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK:
    pub marginfi_bank_wsol: Box<Account<'info, Reserve>>,
    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint_wsol,
    )]
    pub pool_token_receiver_account_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub liquidity_vault_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_wsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub stake_pool_jitosol: Box<Account<'info, StakePool>>,
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub stake_pool_withdraw_authority_jitosol: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub reserve_stake_account_jitosol: AccountInfo<'info>,
    #[account(mut)]
    pub manager_fee_account_jitosol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_jitosol: Box<Account<'info, Mint>>,
    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint_jitosol
        
    )]
    pub pool_token_receiver_account_jitosol: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::authority = Pubkey::from_str("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6").unwrap(),
        token::mint = pool_mint_jitosol,
        
        
    )]
    pub referrer_token_account_jitosol: Box<Account<'info, TokenAccount>>,
    /// CHECK: Checked by CPI to Spl Stake Program
    #[account(mut)]
    pub stake_pool_withdraw_authority_wsol: AccountInfo<'info>,
    /// CHECK: Checked by CPI to Spl Stake Program
    pub bank_liquidity_vault_authority_wsol: AccountInfo<'info>,

    pub jarezi_mint: Box<InterfaceAccount<'info, anchor_spl::token_interface::Mint>>,
    #[account(mut,
        token::authority = signer,
        token::mint = jarezi_mint,
        token::token_program = token_program_2022
    )]
    pub jarezi_token_account:
    Box<InterfaceAccount<'info, anchor_spl::token_interface::TokenAccount>>,
    pub token_program_2022: Program<'info, Token2022>,
    /// CHECK:
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub obligation_pubkey: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub lending_market_pubkey: AccountInfo<'info>,
    /// CHECK:
    pub solend_sdk: Program<'info, SolendProgram>,

    /// CHECK:
    pub lending_market_authority_pubkey: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub user_collateral_pubkey: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub reserve_collateral_mint_pubkey: Box<Account<'info, Mint>>,
    /// CHECK:
    #[account(mut)]
    pub destination_deposit_collateral_pubkey: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    pub pyth_oracle: Box<Account<'info, PriceAccount>>,
    pub switchboard_oracle: AccountInfo<'info>,
}
impl Deposit<'_> {
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> anchor_lang::Result<()> {
        

        let marginfi_pda = ctx.accounts.marginfi_pda.clone();
        let signer: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], &[marginfi_pda.bump]]];
        // stake bsol
        {
            let stake_pool = ctx.accounts.stake_pool.clone();
            let pool_withdraw_authority = ctx.accounts.stake_pool_withdraw_authority.clone();
            let reserve_stake = ctx.accounts.reserve_stake_account.clone();
            let signer = ctx.accounts.signer.clone();
            let manager_fee_account = ctx.accounts.manager_fee_account.clone();
            let referrer_token_account = ctx.accounts.referrer_token_account.clone();
            let pool_mint = ctx.accounts.pool_mint.clone();
            let ix = spl_stake_pool::instruction::deposit_sol(
                &spl_stake_pool::id(),
                &stake_pool.key(),
                &pool_withdraw_authority.key(),
                &reserve_stake.key(),
                &signer.key(),
                &ctx.accounts.pool_token_receiver_account.key(),
                &manager_fee_account.key(),
                &referrer_token_account.key(),
                &pool_mint.key(),
                &spl_token::id(),
                amount,
            );
            invoke(
                &ix,
                &[
                    signer.to_account_info().clone(),
                    reserve_stake.to_account_info().clone(),
                    ctx.accounts
                        .pool_token_receiver_account
                        .to_account_info()
                        .clone(),
                    pool_withdraw_authority.to_account_info().clone(),
                    manager_fee_account.to_account_info().clone(),
                    referrer_token_account.to_account_info().clone(),
                    pool_mint.to_account_info().clone(),
                    stake_pool.to_account_info().clone(),
                    ctx.accounts.stake_pool_program.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                    ctx.accounts.token_program.to_account_info().clone(),
                ],
            )?;
        }
        // deposit bsol
        /*
        let cpi_ctx = anchor_lang_26::context::CpiContext::new_with_signer(marginfi_program.clone(), marginfi::cpi::accounts::LendingAccountDeposit {
            marginfi_group: marginfi_group.to_account_info().clone(),
            marginfi_account: marginfi_account.to_account_info().clone(),
            signer: marginfi_pda.to_account_info().clone(),
            bank: marginfi_bank.to_account_info().clone(),

            signer_token_account: pool_token_receiver_account.to_account_info().clone(),
            bank_liquidity_vault: liquidity_vault.to_account_info().clone(),
            token_program: token_program.to_account_info().clone(),
        },  &signer);
        marginfi::cpi::lending_account_deposit(cpi_ctx, stake_pool_tokens as u64).unwrap();
        */
        // deposit bsol to solend
        let mut stake_pool_tokens = 0;
        {
            let bsol_reserve: Reserve = Reserve::unpack(&ctx.accounts.marginfi_bank.to_account_info().data.borrow())?;
            let wsol_reserve: Reserve =
                Reserve::unpack(&ctx.accounts.marginfi_bank_wsol.to_account_info().data.borrow())?;
            let price = bsol_reserve.liquidity.market_price;
            let wsol_price = wsol_reserve.liquidity.market_price;
            msg!("wsol_price {}", wsol_price);
            msg!("price {}", price);
            let price_div_wsol = price.0.as_u64().checked_div(wsol_price.0.as_u64()).unwrap();
            stake_pool_tokens = amount.checked_div(price_div_wsol).unwrap();
            msg!("stake_pool_tokens {}", stake_pool_tokens);
            let ix =
                solend_sdk::instruction::deposit_reserve_liquidity_and_obligation_collateral(
                    ctx.accounts.solend_sdk.key(),
                    stake_pool_tokens as u64,
                    ctx.accounts.pool_token_receiver_account.key(),
                    ctx.accounts.user_collateral_pubkey.key(),
                    ctx.accounts.marginfi_bank.key(),
                    ctx.accounts.liquidity_vault.key(),
                    ctx.accounts.reserve_collateral_mint_pubkey.key(),
                    ctx.accounts.lending_market_pubkey.key(),
                    ctx.accounts.destination_deposit_collateral_pubkey.key(),
                    ctx.accounts.obligation_pubkey.key(),
                    ctx.accounts.marginfi_pda.key(),
                    ctx.accounts.pyth_oracle.key(),
                    Pubkey::from_str("nu11111111111111111111111111111111111111111").unwrap(),
                    ctx.accounts.marginfi_pda.key(),
                );
            invoke_signed(
                &ix,
                &[
                    ctx.accounts
                        .pool_token_receiver_account
                        .to_account_info()
                        .clone(),
                    ctx.accounts
                        .user_collateral_pubkey
                        .to_account_info()
                        .clone(),
                    ctx.accounts.marginfi_bank.to_account_info().clone(),
                    ctx.accounts.liquidity_vault.to_account_info().clone(),
                    ctx.accounts
                        .destination_deposit_collateral_pubkey
                        .to_account_info()
                        .clone(),
                    ctx.accounts.lending_market_pubkey.to_account_info().clone(),
                    ctx.accounts
                        .reserve_collateral_mint_pubkey
                        .to_account_info()
                        .clone(),
                    ctx.accounts.obligation_pubkey.to_account_info().clone(),
                    ctx.accounts.marginfi_pda.to_account_info().clone(),
                    ctx.accounts.pyth_oracle.to_account_info().clone(),
                    ctx.accounts.marginfi_pda.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                    ctx.accounts.solend_sdk.to_account_info().clone(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info()
                        .clone(),
                    ctx.accounts.switchboard_oracle.to_account_info().clone(),
                ],
                 &signer,
            )
            .unwrap();
        }
        {
            let refresh_reserve_ix = solend_sdk::instruction::refresh_reserve(
                ctx.accounts.solend_sdk.key(),
                ctx.accounts.marginfi_bank.key(),
                ctx.accounts.pyth_oracle.key(),
                Pubkey::from_str("nu11111111111111111111111111111111111111111").unwrap()
            );
            invoke_signed(
                &refresh_reserve_ix,
                &[
                    ctx.accounts.marginfi_bank.to_account_info().clone(),
                    ctx.accounts.pyth_oracle.to_account_info().clone(),
                    ctx.accounts.solend_sdk.to_account_info().clone(),
                    ctx.accounts.lending_market_authority_pubkey.to_account_info().clone(),
                    ctx.accounts.switchboard_oracle.to_account_info().clone(),
                ],
                 &signer,
            )
            .unwrap();
        }
        {
            let refresh_obligation_ix = solend_sdk::instruction::refresh_obligation(
                ctx.accounts.solend_sdk.key(),
                ctx.accounts.obligation_pubkey.key(),
                vec![ctx.accounts.marginfi_bank.key()],
            );
            invoke_signed(
                &refresh_obligation_ix,
                &[
                    ctx.accounts.obligation_pubkey.to_account_info().clone(),
                    ctx.accounts.solend_sdk.to_account_info().clone(),
                    ctx.accounts.lending_market_authority_pubkey.to_account_info().clone(),
                    ctx.accounts.marginfi_bank.to_account_info().clone(),
                ],
                 &signer,
            )
            .unwrap();
        }
        let mut amount : Decimal = Decimal::from( 0 as u64 );
        {
            let obligation: Obligation =
                Obligation::unpack(&ctx.accounts.obligation_pubkey.to_account_info().data.borrow())?;

            let deposited_value = obligation.allowed_borrow_value;
            let reserve: Reserve = Reserve::unpack(&ctx.accounts.marginfi_bank_wsol.to_account_info().data.borrow())?;

            amount = reserve
                .usd_to_liquidity_amount_lower_bound(deposited_value)
                .unwrap();

            msg!("amount {}", amount);
        }
        {
            let marginfi_bank_wsol = ctx.accounts.marginfi_bank_wsol.clone();

            // borrow wsol
            let source_liquidity_pubkey = &ctx.accounts.liquidity_vault_wsol.clone();
            let borrow_reserve_pubkey = marginfi_bank_wsol.clone();
            let borrow_reserve_liquidity_fee_receiver_pubkey =
                &ctx.accounts.stake_pool_withdraw_authority_wsol.clone();
            let lending_market_pubkey = &ctx.accounts.lending_market_pubkey.clone();
            let host_fee_receiver_pubkey = &ctx.accounts.pool_token_receiver_account_wsol.clone();

            let pool_token_receiver_account_wsol =
                ctx.accounts.pool_token_receiver_account_wsol.clone();
            let ix = solend_sdk::instruction::borrow_obligation_liquidity(
                ctx.accounts.solend_sdk.key(),
                amount.0.as_u64(),
                source_liquidity_pubkey.key(),
                pool_token_receiver_account_wsol.key(),
                borrow_reserve_pubkey.key(),
                borrow_reserve_liquidity_fee_receiver_pubkey.key(),
                ctx.accounts.obligation_pubkey.key(),
                lending_market_pubkey.key(),
                marginfi_pda.key(),
                Some(host_fee_receiver_pubkey.key()),
            );
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.lending_market_authority_pubkey.to_account_info().clone(),
                    source_liquidity_pubkey.to_account_info().clone(),
                    pool_token_receiver_account_wsol.to_account_info().clone(),
                    borrow_reserve_pubkey.to_account_info().clone(),
                    borrow_reserve_liquidity_fee_receiver_pubkey
                        .to_account_info()
                        .clone(),
                    ctx.accounts.obligation_pubkey.to_account_info().clone(),
                    lending_market_pubkey.to_owned().to_account_info().clone(),
                    marginfi_pda.to_account_info().clone(),
                    host_fee_receiver_pubkey.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                    ctx.accounts.solend_sdk.to_account_info().clone(),
                    ctx.accounts.token_program.to_account_info().clone(),
                ],
                 &signer,
            )
            .unwrap();
        }
        {
            let ix = spl_token::instruction::close_account(
                &spl_token::ID,
                &ctx.accounts.pool_token_receiver_account_wsol.key(),
                &ctx.accounts.to.key(),
                &ctx.accounts.marginfi_pda.key(),
                &[&ctx.accounts.to.key()], // TODO: support multisig
            )?;
            solana_program::program::invoke_signed(
                &ix,
                &[
                    ctx.accounts
                        .pool_token_receiver_account_wsol
                        .to_account_info()
                        .clone(),
                    ctx.accounts.to.to_account_info().clone(),
                    ctx.accounts.marginfi_pda.to_account_info().clone(),
                    ctx.accounts.token_program.to_account_info().clone(),
                ],
                 &signer,
            )
            .unwrap();
        }
        {
            
            
            let ix = spl_stake_pool::instruction::deposit_sol_with_authority(
                &spl_stake_pool::id(),
                &ctx.accounts.stake_pool_jitosol.key(),
                &ctx.accounts.marginfi_pda.key(),
                &ctx.accounts.stake_pool_withdraw_authority_jitosol.key(),
                &ctx.accounts.reserve_stake_account_jitosol.key(),
                &ctx.accounts.to.key(),
                &ctx.accounts.pool_token_receiver_account_jitosol.key(),
                &ctx.accounts.manager_fee_account_jitosol.key(),
                &ctx.accounts.referrer_token_account_jitosol.key(),
                &ctx.accounts.pool_mint_jitosol.key(),
                &spl_token::id(),
                amount.0.as_u64(),
            );
            

            invoke_signed(
                &ix,
                &[
                    ctx.accounts.marginfi_pda.to_account_info().clone(),
                    ctx.accounts.to.to_account_info().clone(),
                    ctx.accounts.stake_pool_jitosol.to_account_info().clone(),
                    ctx.accounts.stake_pool_withdraw_authority_jitosol.to_account_info().clone(),
                    ctx.accounts.reserve_stake_account_jitosol.to_account_info().clone(),
                    ctx.accounts.pool_token_receiver_account_jitosol
                        .to_account_info()
                        .clone(),
                        ctx.accounts.manager_fee_account_jitosol.to_account_info().clone(),
                        ctx.accounts.referrer_token_account_jitosol.to_account_info().clone(),
                        ctx.accounts.pool_mint_jitosol.to_account_info().clone(),
                        ctx.accounts.stake_pool_jitosol.to_account_info().clone(),
                        ctx.accounts.stake_pool_program.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info().clone(),
                    ctx.accounts.token_program.to_account_info().clone(),
                ],
                 &[&[&SEED_PREFIX[..], &[ctx.accounts.marginfi_pda.bump]]],
            )
            .unwrap();
        }
        {
            let jito_reserve: Reserve =
                Reserve::unpack(&ctx.accounts.marginfi_bank_jito.to_account_info().data.borrow())?;
                let jito_price = jito_reserve.liquidity.market_price;
            let stake_pool_tokens = amount
                .0
                .as_u64()
                .checked_div(jito_price.0.as_u64())
                .unwrap();
            // mint_to
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(),
                MintTo {
                    mint: ctx.accounts.jarezi_mint.to_account_info().clone(),
                    to: ctx.accounts.jarezi_token_account.to_account_info().clone(),
                    authority: ctx.accounts.marginfi_pda.to_account_info().clone(),
                },
                 &signer,
            );

            anchor_spl::token_interface::mint_to(cpi_ctx, stake_pool_tokens).unwrap();
        }

        Ok(())
    }
}
