pub use crate::SbError;
pub use crate::*;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::{token_interface::{MintTo, Token2022}, token::SyncNative};
use mpl_token_metadata::{instructions::{Create, CreateV1CpiAccounts, CreateCpiAccounts, CreateCpi, CreateBuilder}, types::CreateArgs};
use solana_program::program_pack::Pack;
use solend_sdk::{math::{Decimal, Rate, TryMul, TryDiv}, state::{Obligation, LendingMarket, Reserve}};
use std::str::FromStr;
use mpl_token_metadata::instructions::CreateInstructionArgs;

#[derive(Clone)]
pub struct StakeProgram;

impl anchor_lang::Id for StakeProgram {
    fn id() -> Pubkey {
        Pubkey::from_str("Stake11111111111111111111111111111111111111").unwrap()
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

const SEED_PREFIX: &[u8] = b"jarezi";

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
        seeds = [SEED_PREFIX, winner_winner_chickum_dinner.key().as_ref()],
        bump
    )]
    pub program: Box<Account<'info, MarginFiPda>>,
    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
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
        let winner = ctx.accounts.program.thewinnerog;
        let seeds: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], winner.as_ref(), &[mrgnfi_pda.bump]]];
        invoke_signed(
            &instruction,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
                ctx.accounts.base.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            seeds,
        )?;

        Ok(())
    }
}
#[account]
pub struct MarginFiPda {
    pub bump: u8,
    pub authority: Pubkey,
    pub kickback_percent_bpm: u64,
    pub winner_winner_chickum_dinner: Pubkey,
    pub seeded_seed: String,
    pub thewinnerog: Pubkey,
}

#[account]
pub struct MarginFiPdaSwitchboard {
    pub marginfi_pda: Pubkey,
    pub switchboard_function: Pubkey,
}
impl InitMrgnFiPda<'_> {
    pub fn set_jarezi_mint_metadata(
        ctx: Context<SetMetadata>, name: String, symbol: String, uri: String) -> anchor_lang::Result<()>
    {
        let marginfi_pda = &mut ctx.accounts.marginfi_pda;
        let jarezi_mint = &mut ctx.accounts.jarezi_mint;
        let metadata = &mut ctx.accounts.metadata;
        let metadata_account = metadata.to_account_info();
        marginfi_pda.thewinnerog = marginfi_pda.winner_winner_chickum_dinner;
        let winner = marginfi_pda.thewinnerog;
        let seeds: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], winner.as_ref(), &[marginfi_pda.bump]]];
/*
        let metadata_pointer_ix = spl_token_2022::extension::metadata_pointer::instruction::update(
            &ctx.accounts.token_program_2022.key(),
            &jarezi_mint.key(),
            marginfi_pda.key()),
            &vec![],
            Some(metadata_account.key()),
        )?;
        invoke_signed(
            &metadata_pointer_ix,
            &[
                metadata_account.clone(),
                jarezi_mint.to_account_info(),
                marginfi_pda.to_account_info(),
                ctx.accounts.token_program_2022.to_account_info(),
            ],
            seeds,
        )?; */
        Ok(())
    }

    pub fn init_mrgn_fi_pda(ctx: Context<InitMrgnFiPda>, bump: u8, kickback: u64, seeded_seed: String, seed2: String) -> anchor_lang::Result<()> {
        let marginfi_pda = &mut ctx.accounts.marginfi_pda;
        marginfi_pda.authority = ctx.accounts.authority.key();
        assert!(kickback <= 1_000_000);
        marginfi_pda.kickback_percent_bpm = kickback;
        marginfi_pda.winner_winner_chickum_dinner = ctx.accounts.winner_winner_chickum_dinner.key();
        marginfi_pda.bump = bump;
        marginfi_pda.seeded_seed = seeded_seed;
        let mint = ctx.accounts.jarezi_mint.clone();
        marginfi_pda.thewinnerog = marginfi_pda.winner_winner_chickum_dinner;
        let token_program_2022 = ctx.accounts.token_program_2022.clone();
        let winner = marginfi_pda.thewinnerog;
        let seeds: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], winner.as_ref(), &[marginfi_pda.bump]]];
        {
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
}
{
    let create_seeded_ix = system_instruction::create_account_with_seed(
        &ctx.accounts.authority.key(),
        &ctx.accounts.to.key(),
        &ctx.accounts.marginfi_pda.key(),
        &seed2,
        Rent::default().minimum_balance(Obligation::LEN),
        Obligation::LEN.try_into().unwrap(),
        &ctx.accounts.solend_sdk.key(),
    );
    invoke_signed(
        &create_seeded_ix,
        &[
            ctx.accounts.to.to_account_info(),
            ctx.accounts.marginfi_pda.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.solend_sdk.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        seeds,
    )?;
}
{
    let instruction: Instruction = solend_sdk::instruction::init_obligation(
        ctx.accounts.solend_sdk.key(),
        ctx.accounts.to.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.marginfi_pda.key(),
    );
    invoke_signed(
        &instruction,
        &[
            ctx.accounts.to.to_account_info(),
            ctx.accounts.lending_market.to_account_info(),
            ctx.accounts.marginfi_pda.to_account_info(),
            ctx.accounts.solend_sdk.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ],
        seeds,
    )?;

}
{
    invoke_signed(
        &solend_sdk::instruction::deposit_reserve_liquidity_and_obligation_collateral(
            ctx.accounts.solend_sdk.key(),
            666 as u64,
            ctx.accounts.pool_token_receiver_account_wsol.key(),
            ctx.accounts.destination_deposit_collateral_pubkey.key(),
            ctx.accounts.marginfi_bank_wsol.key(),
            ctx.accounts.liquidity_vault_wsol.key(),
            ctx.accounts.reserve_collateral_mint_pubkey.key(),
            ctx.accounts.lending_market_pubkey.key(),
            ctx.accounts.user_collateral_pubkey.key(),
            ctx.accounts.to.key(),
            ctx.accounts.marginfi_pda.key(),
            ctx.accounts.pyth_oracle.key(),
            ctx.accounts.switchboard_oracle.key(),
            ctx.accounts.marginfi_pda.key(),
        ),
        &[
            ctx.accounts.pool_token_receiver_account_wsol.to_account_info(),
            ctx.accounts.user_collateral_pubkey.to_account_info(),
            ctx.accounts.marginfi_bank_wsol.to_account_info(),
            ctx.accounts.liquidity_vault_wsol.to_account_info(),
            ctx.accounts
                .destination_deposit_collateral_pubkey
                .to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts
                .reserve_collateral_mint_pubkey
                .to_account_info(),
            ctx.accounts.to.to_account_info(),
            ctx.accounts.marginfi_pda.to_account_info(),
            ctx.accounts.pyth_oracle.to_account_info(),
            ctx.accounts.marginfi_pda.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.solend_sdk.to_account_info(),
            ctx.accounts
                .lending_market_authority_pubkey
                .to_account_info(),
            ctx.accounts.switchboard_oracle.to_account_info(),
        ],
        &seeds,
    )
    .unwrap();

}
{
    invoke_signed(
        &solend_sdk::instruction::refresh_reserve(
            ctx.accounts.solend_sdk.key(),
            ctx.accounts.marginfi_bank_wsol.key(),
            ctx.accounts.pyth_oracle.key(),
            ctx.accounts.switchboard_oracle.key(),
        ),
        &[
            ctx.accounts.marginfi_bank_wsol.to_account_info(),
            ctx.accounts.pyth_oracle.to_account_info(),
            ctx.accounts.solend_sdk.to_account_info(),
            ctx.accounts.lending_market_authority_pubkey.to_account_info(),
            ctx.accounts.switchboard_oracle.to_account_info(),
        ],
        seeds,
    )
    .unwrap();
}
{
    invoke_signed(
        &solend_sdk::instruction::refresh_reserve(
            ctx.accounts.solend_sdk.key(),
            ctx.accounts.marginfi_bank_wsol2.key(),
            ctx.accounts.pyth_oracle2.key(),
            ctx.accounts.switchboard_oracle2.key(),
        ),
        &[
            ctx.accounts.marginfi_bank_wsol2.to_account_info(),
            ctx.accounts.pyth_oracle2.to_account_info(),
            ctx.accounts.solend_sdk.to_account_info(),
            ctx.accounts.lending_market_authority_pubkey.to_account_info(),
            ctx.accounts.switchboard_oracle2.to_account_info(),
        ],
        seeds,
    )
    .unwrap();
}
{
    invoke_signed(
        &solend_sdk::instruction::refresh_obligation(
            ctx.accounts.solend_sdk.key(),
            ctx.accounts.to.key(),
            vec![
                ctx.accounts.marginfi_bank_wsol.key(),
            ],
        ),
        &[
            ctx.accounts.to.to_account_info(),
            ctx.accounts.solend_sdk.to_account_info(),
            ctx.accounts.lending_market_authority_pubkey.to_account_info(),
            ctx.accounts.marginfi_bank_wsol.to_account_info(),
        ],
        seeds,
    )
    .unwrap();

    }
{
    invoke_signed(
        &solend_sdk::instruction::borrow_obligation_liquidity(
            ctx.accounts.solend_sdk.key(),
            100 as u64,
            ctx.accounts.liquidity_vault_wsol2.key(),
            ctx.accounts.pool_token_receiver_account_wsol2.key(),
            ctx.accounts.marginfi_bank_wsol2.key(),
            ctx.accounts.stake_pool_withdraw_authority_wsol2.key(),
            ctx.accounts.to.key(),
            ctx.accounts.lending_market_pubkey.key(),
            ctx.accounts.marginfi_pda.key(),
            Some(ctx.accounts.pool_token_receiver_account_wsol2.key()),
        ),
        &[
            ctx.accounts
                .lending_market_authority_pubkey
                .to_account_info(),
            ctx.accounts.liquidity_vault_wsol2.to_account_info(),
            ctx.accounts
                .pool_token_receiver_account_wsol2
                .to_account_info(),
            ctx.accounts.marginfi_bank_wsol2.to_account_info(),
            ctx.accounts
                .stake_pool_withdraw_authority_wsol2
                .to_account_info(),
            ctx.accounts.to.to_account_info(),
            ctx.accounts
                .lending_market_pubkey
                .to_owned()
                .to_account_info(),
            ctx.accounts.marginfi_pda.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.solend_sdk.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.liquidity_vault_wsol2.to_account_info(),
        ],
        &seeds,
    )
    .unwrap();
    }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SetMetadata<'info> {
                #[account(mut,
        constraint = marginfi_pda.authority == authority.key(),
        seeds = [SEED_PREFIX, winner_winner_chickum_dinner.key().as_ref()],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
    #[account(mut)]
    pub jarezi_mint: Box<InterfaceAccount<'info, anchor_spl::token_interface::Mint>>,
    #[account(mut)]
    /// CHECK:
    pub metadata: AccountInfo<'info>,
    pub token_program_2022: Program<'info, Token2022>,
    /// CHECK:
    pub mpl_token_metadata_program: AccountInfo<'info>,
    /// CHECK:
    pub sysvar_instructions: AccountInfo<'info>,
    #[account(init, 
        payer = authority,
        mint::authority = marginfi_pda,
        mint::decimals = 9,
    )]
    pub fake_mint: Box<Account<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    
}
#[derive(Accounts)]
pub struct InitMrgnFiPda<'info> {
    #[account(init,
        seeds = [SEED_PREFIX, winner_winner_chickum_dinner.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<MarginFiPda>(),
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    /// CHECK:
    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
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
    #[account(mut)]
    /// CHECK:
    pub to: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub base: AccountInfo<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    /// CHECK:
    pub lending_market: AccountInfo<'info>,

    pub solend_sdk: Program<'info, SolendProgram>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,


    #[account(mut)]
    /// CHECK:
    pub marginfi_bank_wsol: AccountInfo<'info>,
    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint_wsol,
    )]
    pub pool_token_receiver_account_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub liquidity_vault_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_wsol: Box<Account<'info, Mint>>,
    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub stake_pool_withdraw_authority_wsol: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub stake_pool_withdraw_authority_wsol2: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub lending_market_pubkey: AccountInfo<'info>,
    /// CHECK:
    pub lending_market_authority_pubkey: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK:
    pub marginfi_bank_wsol2: AccountInfo<'info>,
    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint_wsol2,
    )]
    pub pool_token_receiver_account_wsol2: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub liquidity_vault_wsol2: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_wsol2: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub destination_deposit_collateral_pubkey2: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub reserve_collateral_mint_pubkey2: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub destination_deposit_collateral_pubkey: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub reserve_collateral_mint_pubkey: Box<Account<'info, Mint>>,
    /// CHECK:
    pub pyth_oracle: AccountInfo<'info>,
    /// CHECK:
    pub switchboard_oracle: AccountInfo<'info>,
    /// CHECK:
    pub pyth_oracle2: AccountInfo<'info>,
    /// CHECK:
    pub switchboard_oracle2: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK:
    pub user_collateral_pubkey: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,
        constraint = winner_winner_chickum_dinner.key() == marginfi_pda.winner_winner_chickum_dinner,
        
        seeds = [SEED_PREFIX, marginfi_pda.thewinnerog.as_ref()],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint,
    )]
    pub pool_token_receiver_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub stake_pool: AccountInfo<'info>,
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
    pub marginfi_bank: AccountInfo<'info>,
    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub liquidity_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    /// CHECK:
    pub marginfi_bank_wsol: AccountInfo<'info>,
    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint_wsol,
    )]
    pub pool_token_receiver_account_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub liquidity_vault_wsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_wsol: Box<Account<'info, Mint>>,
    /// CHECK: Checked by CPI to Spl Stake Program
    #[account(mut)]
    pub stake_pool_withdraw_authority_wsol: AccountInfo<'info>,
    #[account(mut)]
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
    pub pyth_oracle: AccountInfo<'info>,
    /// CHECK:
    pub switchboard_oracle: AccountInfo<'info>,
    /// CHECK:
    pub pyth_oracle2: AccountInfo<'info>,
    /// CHECK:
    pub switchboard_oracle2: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub stake_history: Sysvar<'info, StakeHistory>,
    pub stake_program: Program<'info, StakeProgram>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle.load()?.bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,
    #[account(mut)]
    /// CHECK:
    pub hydra: AccountInfo<'info>,
    #[account(mut,
        token::authority = hydra,
        token::mint = pool_mint,
    )]
    pub hydra_referrer: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        token::authority = hydra,
        token::mint = pool_mint_wsol,
    )]
    pub hydra_host_fee_account: Box<Account<'info, TokenAccount>>,
}

#[derive(Accounts)]
pub struct SetWinner<'info> {
    #[account(mut,
        constraint = marginfi_pda_switchboard.switchboard_function == switchboard_function.key(),
        seeds = [SEED_PREFIX, marginfi_pda.thewinnerog.as_ref()],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    #[account(mut,
        constraint = marginfi_pda.key() == marginfi_pda_switchboard.marginfi_pda,
        seeds = [SEED_PREFIX, marginfi_pda.key().as_ref()],
        bump
    )]
    pub marginfi_pda_switchboard: Box<Account<'info, MarginFiPdaSwitchboard>>,
    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
    /// CHECK:
    
    pub new_winner_winner_chickum_dinner: AccountInfo<'info>,
    #[account(
        constraint =
                    switchboard_function.load()?.validate(
                    &enclave_signer.to_account_info()
                )? @ USDY_USDC_ORACLEError::FunctionValidationFailed,
        
        )]
        pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
        pub enclave_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetFunction<'info> {
    #[account(mut,
        constraint = winner_winner_chickum_dinner.key() == marginfi_pda.winner_winner_chickum_dinner,
        constraint = marginfi_pda.authority == authority.key(),
        seeds = [SEED_PREFIX, marginfi_pda.thewinnerog.as_ref()],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,

    #[account(init,
        space = 8 + std::mem::size_of::<MarginFiPdaSwitchboard>(),
        payer = authority,
        seeds = [SEED_PREFIX, marginfi_pda.key().as_ref()],
        bump
    )]
    pub marginfi_pda_switchboard: Box<Account<'info, MarginFiPdaSwitchboard>>,

    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
    /// CHECK: 
    
    /// CHECK:
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Winner<'info> {
    #[account(mut,

        seeds = [SEED_PREFIX, marginfi_pda.thewinnerog.as_ref()],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,

    #[account(mut,
        constraint = marginfi_pda_switchboard.switchboard_function == switchboard_function.key(),
        constraint = marginfi_pda.key() == marginfi_pda_switchboard.marginfi_pda,
        seeds = [SEED_PREFIX, marginfi_pda.key().as_ref()],
        bump
    )]
    pub marginfi_pda_switchboard: Box<Account<'info, MarginFiPdaSwitchboard>>,
    #[account(mut)]
    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
    #[account(mut,
        token::authority = winner_winner_chickum_dinner,
        token::mint = jarezi_mint,
        token::token_program = token_program_2022
    )]
    pub actual_destination: Box<InterfaceAccount<'info, anchor_spl::token_interface::TokenAccount>>,
   
    pub system_program: Program<'info, System>,
    pub token_program_2022: Program<'info, Token2022>,
    /// CHECK: no validation, for educational purpose only
    #[account(mut)]
    pub jarezi_mint: Box<InterfaceAccount<'info, anchor_spl::token_interface::Mint>>,

    #[account(mut,
        token::authority = marginfi_pda,
        token::mint = pool_mint,
    )]
    pub pool_token_receiver_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub pool_mint: Box<Account<'info, Mint>>,
     // We use this to verify the functions enclave state was verified successfully
   #[account(
    constraint =
                switchboard_function.load()?.validate(
                &enclave_signer.to_account_info()
            )? @ USDY_USDC_ORACLEError::FunctionValidationFailed  
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    pub enclave_signer: Signer<'info>,

    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle.load()?.bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,
    /// CHECK:
    #[account(mut)]
    pub obligation_pubkey: AccountInfo<'info>,
}
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
impl Deposit<'_> {
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> anchor_lang::Result<()> {
        let bsol_price = ctx.accounts.oracle.load()?.bsol_sol.mean;

        let winner = ctx.accounts.marginfi_pda.thewinnerog;
        let signer: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], winner.as_ref(),
         &[ctx.accounts.marginfi_pda.bump]]];
        // stake bsol
        {
            invoke(
                &spl_stake_pool::instruction::deposit_sol(
                    &spl_stake_pool::id(),
                    &ctx.accounts.stake_pool.key(),
                    &ctx.accounts.stake_pool_withdraw_authority.key(),
                    &ctx.accounts.reserve_stake_account.key(),
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.pool_token_receiver_account.key(),
                    &ctx.accounts.manager_fee_account.key(),
                    &ctx.accounts.hydra_referrer.key(),
                    &ctx.accounts.pool_mint.key(),
                    &anchor_spl::token::ID,
                    amount,
                ),
                &[
                    ctx.accounts.hydra_referrer.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.reserve_stake_account.to_account_info(),
                    ctx.accounts.pool_token_receiver_account.to_account_info(),
                    ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
                    ctx.accounts.manager_fee_account.to_account_info(),
                    ctx.accounts.pool_mint.to_account_info(),
                    ctx.accounts.stake_pool.to_account_info(),
                    ctx.accounts.stake_pool_program.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                ],
            )?;
        }
        {
            

            let rate: f64 = 1_000_000_000.0 / bsol_price as f64;
            msg!("rate: {}", rate);
            let stake_pool_tokens = amount as f64 * rate * (1.0-0.0005-0.00146);
            msg!("stake_pool_tokens: {}", stake_pool_tokens);
            invoke_signed(
                &solend_sdk::instruction::deposit_reserve_liquidity_and_obligation_collateral(
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
                    ctx.accounts.switchboard_oracle.key(),
                    ctx.accounts.marginfi_pda.key(),
                ),
                &[
                    ctx.accounts.pool_token_receiver_account.to_account_info(),
                    ctx.accounts.user_collateral_pubkey.to_account_info(),
                    ctx.accounts.marginfi_bank.to_account_info(),
                    ctx.accounts.liquidity_vault.to_account_info(),
                    ctx.accounts
                        .destination_deposit_collateral_pubkey
                        .to_account_info(),
                    ctx.accounts.lending_market_pubkey.to_account_info(),
                    ctx.accounts
                        .reserve_collateral_mint_pubkey
                        .to_account_info(),
                    ctx.accounts.obligation_pubkey.to_account_info(),
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.pyth_oracle.to_account_info(),
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.switchboard_oracle.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            invoke_signed(
                &solend_sdk::instruction::refresh_reserve(
                    ctx.accounts.solend_sdk.key(),
                    ctx.accounts.marginfi_bank.key(),
                    ctx.accounts.pyth_oracle.key(),
                    ctx.accounts.switchboard_oracle.key(),
                ),
                &[
                    ctx.accounts.marginfi_bank.to_account_info(),
                    ctx.accounts.pyth_oracle.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.switchboard_oracle.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            invoke_signed(
                &solend_sdk::instruction::refresh_reserve(
                    ctx.accounts.solend_sdk.key(),
                    ctx.accounts.marginfi_bank_wsol.key(),
                    ctx.accounts.pyth_oracle2.key(),
                    ctx.accounts.switchboard_oracle2.key(),
                ),
                &[
                    ctx.accounts.marginfi_bank_wsol.to_account_info(),
                    ctx.accounts.pyth_oracle2.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.switchboard_oracle2.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            invoke_signed(
                &solend_sdk::instruction::refresh_obligation(
                    ctx.accounts.solend_sdk.key(),
                    ctx.accounts.obligation_pubkey.key(),
                    vec![
                        ctx.accounts.marginfi_bank.key(),
                        ctx.accounts.marginfi_bank_wsol.key(),
                    ],
                ),
                &[
                    ctx.accounts.obligation_pubkey.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.marginfi_bank.to_account_info(),
                    ctx.accounts.marginfi_bank_wsol.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }

        {
            let rate: f64 = 1_000_000_000.0 / bsol_price as f64;

            let stake_pool_tokens = amount as f64 * rate * (1.0-0.0005-0.00146);
            let ltv: f64 = 0.535;
            msg!("ltv: {}", ltv);
            let amount = stake_pool_tokens * ltv;
            msg!("amount: {}", amount);

            invoke_signed(
                &solend_sdk::instruction::borrow_obligation_liquidity(
                    ctx.accounts.solend_sdk.key(),
                    amount as u64,
                    ctx.accounts.liquidity_vault_wsol.key(),
                    ctx.accounts.pool_token_receiver_account_wsol.key(),
                    ctx.accounts.marginfi_bank_wsol.key(),
                    ctx.accounts.stake_pool_withdraw_authority_wsol.key(),
                    ctx.accounts.obligation_pubkey.key(),
                    ctx.accounts.lending_market_pubkey.key(),
                    ctx.accounts.marginfi_pda.key(),
                    Some(ctx.accounts.hydra_host_fee_account.key()),
                ),
                &[
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.liquidity_vault_wsol.to_account_info(),
                    ctx.accounts
                        .pool_token_receiver_account_wsol
                        .to_account_info(),
                    ctx.accounts.marginfi_bank_wsol.to_account_info(),
                    ctx.accounts
                        .stake_pool_withdraw_authority_wsol
                        .to_account_info(),
                    ctx.accounts.obligation_pubkey.to_account_info(),
                    ctx.accounts
                        .lending_market_pubkey
                        .to_owned()
                        .to_account_info(),
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.hydra_host_fee_account.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            solana_program::program::invoke_signed(
                &spl_token::instruction::close_account(
                    &anchor_spl::token::ID,
                    &ctx.accounts.pool_token_receiver_account_wsol.key(),
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.marginfi_pda.key(),
                    &[], // TODO: support multisig
                )?,
                &[
                    ctx.accounts
                        .pool_token_receiver_account_wsol
                        .to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        
        {
            let rate: f64 = 1_000_000_000.0 / bsol_price as f64;


            let stake_pool_tokens = amount as f64 * rate * (1.0-0.0005-0.00146);
                        let ltv: f64 = 0.535;

            let amount = stake_pool_tokens * ltv;
            invoke(
                &spl_stake_pool::instruction::deposit_sol(
                    &spl_stake_pool::id(),
                    &ctx.accounts.stake_pool.key(),
                    &ctx.accounts.stake_pool_withdraw_authority.key(),
                    &ctx.accounts.reserve_stake_account.key(),
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.pool_token_receiver_account.key(),
                    &ctx.accounts.manager_fee_account.key(),
                    &ctx.accounts.hydra_referrer.key(),
                    &ctx.accounts.pool_mint.key(),
                    &anchor_spl::token::ID,
                    amount as u64,
                ),
                &[
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.reserve_stake_account.to_account_info(),
                    ctx.accounts.pool_token_receiver_account.to_account_info(),
                    ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
                    ctx.accounts.manager_fee_account.to_account_info(),
                    ctx.accounts.pool_mint.to_account_info(),
                    ctx.accounts.stake_pool.to_account_info(),
                    ctx.accounts.stake_pool_program.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.hydra_referrer.to_account_info(),
                ],
            )
            .unwrap();
        }
        {

            let rate: f64 = 1_000_000_000.0 / bsol_price as f64;
            let ltv: f64 = 0.535;
            let amount = amount as f64 * ltv;


            let amount = amount as f64 * rate * (1.0-0.0005-0.00146);
                        let rate: f64 = 1_000_000_000.0 / bsol_price as f64;
            msg!("rate: {}", rate);

            let amount = amount * rate as f64;
            msg!("amount: {}", amount);
            anchor_spl::token_interface::mint_to(CpiContext::new_with_signer(
                ctx.accounts.token_program_2022.to_account_info(),
                MintTo {
                    mint: ctx.accounts.jarezi_mint.to_account_info(),
                    to: ctx.accounts.jarezi_token_account.to_account_info(),
                    authority: ctx.accounts.marginfi_pda.to_account_info(),
                },
                &signer,
            ), amount as u64).unwrap();
        }

        Ok(())
    }
    pub fn set_winner_winner_chickum_dinner(
        ctx: Context<SetWinner>,
    ) -> anchor_lang::Result<()> {
        let marginfi_pda = &mut ctx.accounts.marginfi_pda;

        marginfi_pda.winner_winner_chickum_dinner = ctx.accounts.new_winner_winner_chickum_dinner.key();
        
        Ok(())   
    }

    pub fn set_function(
        ctx: Context<SetFunction>,
    ) -> anchor_lang::Result<()> {
        let marginfi_pda_switchboard = &mut ctx.accounts.marginfi_pda_switchboard;
        marginfi_pda_switchboard.switchboard_function = ctx.accounts.switchboard_function.key();


        Ok(())   
    }
    pub fn withdraw(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> anchor_lang::Result<()> {

        let bsol_price = ctx.accounts.oracle.load()?.bsol_sol.mean ;
        let wsol_borrow_rate = ctx.accounts.oracle.load()?.wsol_borrow.mean as f64 // this is a 10^18 we want it as a f64 so we divide by 10^18
        / 1_000_000_000.0;
        msg!("wsol_borrow_rate: {}", wsol_borrow_rate);
        let wsol_borrow_rate = 1.0 + wsol_borrow_rate;
        let marginfi_pda = ctx.accounts.marginfi_pda.clone();
        let winner = ctx.accounts.marginfi_pda.thewinnerog;
        let signer: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], winner.as_ref(),
        &[marginfi_pda.bump]]];
        let mint_supply = ctx.accounts.jarezi_mint.supply;
            let rate: CollateralExchangeRate = exchange_rate(
                ctx.accounts.pool_token_receiver_account.amount,
                mint_supply,
            ).unwrap();
        let amount = (amount as f64 * 0.999) as u64;
        // burn tokens
        {
            solana_program::program::invoke(
                &spl_token_2022::instruction::burn(
                    &ctx.accounts.token_program_2022.key(),
                    &ctx.accounts.jarezi_token_account.key(),
                    &ctx.accounts.jarezi_mint.key(),
                    &ctx.accounts.signer.key(),
                    &[],
                    amount as u64,
                )?,
                &[
                    ctx.accounts.token_program_2022.to_account_info(),
                    ctx.accounts.jarezi_token_account.to_account_info(),
                    ctx.accounts.jarezi_mint.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.clock.to_account_info(),
                ],
            )
            .unwrap();
        }
        {
            
            invoke_signed(
                &spl_stake_pool::instruction::withdraw_sol(
                    &spl_stake_pool::id(),
                    &ctx.accounts.stake_pool.key(),
                    &ctx.accounts.stake_pool_withdraw_authority.key(),
                    &ctx.accounts.marginfi_pda.key(),
                    &ctx.accounts.pool_token_receiver_account.key(),
                    &ctx.accounts.reserve_stake_account.key(),
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.manager_fee_account.key(),
                    &ctx.accounts.pool_mint.key(),
                    &anchor_spl::token::ID,
                    amount as u64,
                ),
                &[
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.stake_pool.to_account_info(),
                    ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
                    ctx.accounts.pool_token_receiver_account.to_account_info(),
                    ctx.accounts.reserve_stake_account.to_account_info(),
                    ctx.accounts.manager_fee_account.to_account_info(),
                    ctx.accounts.pool_mint.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.clock.to_account_info(),
                    ctx.accounts.stake_history.to_account_info(),
                    ctx.accounts.stake_program.to_account_info(),
                    ctx.accounts.rent.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            
            let  amount = rate.collateral_to_liquidity(amount).unwrap();
            let mut amount = amount as f64 * bsol_price as f64 / 1_000_000_000 as f64;
            // amount atm is the og deposit, without any of the accrued interest
            // we want to kickback kickback_percent_bpm of the interest to the user
            msg!("kickback_percent_bpm: {}", marginfi_pda.kickback_percent_bpm);
            msg!("amount: {}", amount);
            if marginfi_pda.kickback_percent_bpm > 0 {
                amount = amount * marginfi_pda.kickback_percent_bpm as f64 / 1_000_000.0;
                msg!("amount: {}", amount);
            }
            msg!("amount: {}", amount);
            // kickback

            /*tesitng 
            let seeded_tx_ix = &solana_program::system_instruction::transfer_with_seed(
                &ctx.accounts.to.key(),
                &ctx.accounts.marginfi_pda.key(),
                "robot001".to_string(),
                ctx.accounts.system_program.key(),
                &ctx.accounts.pool_token_receiver_account_wsol.key(),
                amount,
            );*/
            invoke(
                &solana_program::system_instruction::transfer(
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.pool_token_receiver_account_wsol.key(),
                    amount as u64, 
                ),
                &[
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts
                        .pool_token_receiver_account_wsol
                        .to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )
            .unwrap();
        }
        {

                anchor_spl::token::sync_native(
                    CpiContext::new_with_signer(
                        ctx.accounts.token_program.to_account_info(),
                        SyncNative {
                            account: ctx.accounts.pool_token_receiver_account_wsol.to_account_info(),
                        },
                        &signer,
                    ),
                )
                .unwrap();

               
        }
        {
            
            
            let  amount = rate.collateral_to_liquidity(amount).unwrap();
            let mut amount = amount as f64 * bsol_price as f64 / 1_000_000_000 as f64;
            
            msg!("kickback_percent_bpm: {}", marginfi_pda.kickback_percent_bpm);
            msg!("amount: {}", amount);
            if marginfi_pda.kickback_percent_bpm > 0 {
                amount = amount * marginfi_pda.kickback_percent_bpm as f64 / 1_000_000.0;
                msg!("amount: {}", amount);
            }
            
            msg!("amount: {}", amount);

            invoke_signed(
                &solend_sdk::instruction::repay_obligation_liquidity(
                    ctx.accounts.solend_sdk.key(),
                    amount as u64,
                    ctx.accounts.pool_token_receiver_account_wsol.key(),
                    ctx.accounts.liquidity_vault_wsol.key(),
                    ctx.accounts.marginfi_bank_wsol.key(),
                    ctx.accounts.obligation_pubkey.key(),
                    ctx.accounts.lending_market_pubkey.key(),
                    ctx.accounts.marginfi_pda.key(),
                ),
                &[
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.liquidity_vault_wsol.to_account_info(),
                    ctx.accounts
                        .pool_token_receiver_account_wsol
                        .to_account_info(),
                    ctx.accounts.marginfi_bank_wsol.to_account_info(),
                    ctx.accounts.obligation_pubkey.to_account_info(),
                    ctx.accounts
                        .lending_market_pubkey
                        .to_owned()
                        .to_account_info(),
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            // withdraw bsol

            invoke_signed(
                &solend_sdk::instruction::refresh_reserve(
                    ctx.accounts.solend_sdk.key(),
                    ctx.accounts.marginfi_bank.key(),
                    ctx.accounts.pyth_oracle.key(),
                    ctx.accounts.switchboard_oracle.key(),
                ),
                &[
                    ctx.accounts.marginfi_bank.to_account_info(),
                    ctx.accounts.pyth_oracle.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.switchboard_oracle.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            invoke_signed(
                &solend_sdk::instruction::refresh_reserve(
                    ctx.accounts.solend_sdk.key(),
                    ctx.accounts.marginfi_bank_wsol.key(),
                    ctx.accounts.pyth_oracle2.key(),
                    ctx.accounts.switchboard_oracle2.key(),
                ),
                &[
                    ctx.accounts.marginfi_bank_wsol.to_account_info(),
                    ctx.accounts.pyth_oracle2.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.switchboard_oracle2.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            invoke_signed(
                &solend_sdk::instruction::refresh_obligation(
                    ctx.accounts.solend_sdk.key(),
                    ctx.accounts.obligation_pubkey.key(),
                    vec![
                        ctx.accounts.marginfi_bank.key(),
                        ctx.accounts.marginfi_bank_wsol.key(),
                    ],
                ),
                &[
                    ctx.accounts.obligation_pubkey.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts
                        .lending_market_authority_pubkey
                        .to_account_info(),
                    ctx.accounts.marginfi_bank.to_account_info(),
                    ctx.accounts.marginfi_bank_wsol.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            
            
            let  amount = rate.collateral_to_liquidity(amount).unwrap();
            
            let mut amount = amount as f64 / (1_000_000_000.0 / bsol_price as f64) as f64;
            msg!("kickback_percent_bpm: {}", marginfi_pda.kickback_percent_bpm);
            msg!("amount: {}", amount);
            if marginfi_pda.kickback_percent_bpm > 0 {
                amount = amount * marginfi_pda.kickback_percent_bpm as f64 / 1_000_000.0;
                msg!("amount: {}", amount);
            }
            
            msg!("amount: {}", amount);
            let obligation: Obligation = Obligation::unpack(&ctx.accounts.obligation_pubkey.data.borrow()).unwrap();
            let borrowed = obligation.borrowed_value;
            let oracle = &mut ctx.accounts.oracle.load_mut()?;
            oracle.last_borrowed_amount = borrowed.0.as_u64();
            let old_timestamp = oracle.last_borrowed_amount_timestamp;
            oracle.last_borrowed_amount_timestamp = Clock::get()?.unix_timestamp;
            let time_diff = oracle.last_borrowed_amount_timestamp - old_timestamp;
            msg!("time_diff: {}", time_diff);
            let time_diff = time_diff as f64 / 60.0 / 60.0 / 24.0 / 365.0;
            msg!("time_diff: {}", time_diff);
            let rate = wsol_borrow_rate.powf(time_diff);
            msg!("rate: {}", rate);
            let amount = amount as f64 * rate as f64;
            msg!("amount: {}", amount);


            // / 0.535
            let amount = amount as f64 / 0.535;
            let amount = amount as u64;

            invoke_signed(
                &solend_sdk::instruction::withdraw_obligation_collateral_and_redeem_reserve_collateral(
                    ctx.accounts.solend_sdk.key(),
                    amount as u64,
                    ctx.accounts.destination_deposit_collateral_pubkey.key(),
                    ctx.accounts.user_collateral_pubkey.key(),
                    ctx.accounts.marginfi_bank.key(),
                    ctx.accounts.obligation_pubkey.key(),
                    ctx.accounts.lending_market_pubkey.key(),
                    ctx.accounts.pool_token_receiver_account.key(),
                    ctx.accounts.reserve_collateral_mint_pubkey.key(),
                    ctx.accounts.liquidity_vault.key(),
                    ctx.accounts.marginfi_pda.key(),
                    ctx.accounts.marginfi_pda.key()
                ),
                &[
                    ctx.accounts.pool_token_receiver_account.to_account_info(),
                    ctx.accounts.user_collateral_pubkey.to_account_info(),
                    ctx.accounts.marginfi_bank.to_account_info(),
                    ctx.accounts.liquidity_vault.to_account_info(),
                    ctx.accounts.destination_deposit_collateral_pubkey.to_account_info(),
                    ctx.accounts.lending_market_pubkey.to_account_info(),
                    ctx.accounts.reserve_collateral_mint_pubkey.to_account_info(),
                    ctx.accounts.obligation_pubkey.to_account_info(),
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.solend_sdk.to_account_info(),
                    ctx.accounts.lending_market_authority_pubkey.to_account_info(),
                ],
                 &signer,
            )
            .unwrap();
        }
        {
            
            
            let  amount = rate.collateral_to_liquidity(amount).unwrap();
            
            let mut amount = amount as f64 / (1_000_000_000.0 / bsol_price as f64) as f64;
            msg!("kickback_percent_bpm: {}", marginfi_pda.kickback_percent_bpm);
            msg!("amount: {}", amount);
            if marginfi_pda.kickback_percent_bpm > 0 {
                amount = amount * marginfi_pda.kickback_percent_bpm as f64 / 1_000_000.0;
                msg!("amount: {}", amount);
            }
            
            msg!("amount: {}", amount);
            let obligation: Obligation = Obligation::unpack(&ctx.accounts.obligation_pubkey.data.borrow()).unwrap();
            let borrowed = obligation.borrowed_value;
            let oracle = &mut ctx.accounts.oracle.load_mut()?;
            oracle.last_borrowed_amount = borrowed.0.as_u64();
            let old_timestamp = oracle.last_borrowed_amount_timestamp;
            oracle.last_borrowed_amount_timestamp = Clock::get()?.unix_timestamp;
            let time_diff = oracle.last_borrowed_amount_timestamp - old_timestamp;
            msg!("time_diff: {}", time_diff);
            let time_diff = time_diff as f64 / 60.0 / 60.0 / 24.0 / 365.0;
            msg!("time_diff: {}", time_diff);
            let rate = wsol_borrow_rate.powf(time_diff);
            msg!("rate: {}", rate);
            let amount = amount as f64 * rate as f64;
            msg!("amount: {}", amount);
            // / 0.535
            let amount = amount as f64 / 0.535;
            let amount = amount as u64;

            
            invoke_signed(
                &spl_stake_pool::instruction::withdraw_sol(
                    &spl_stake_pool::id(),
                    &ctx.accounts.stake_pool.key(),
                    &ctx.accounts.stake_pool_withdraw_authority.key(),
                    &ctx.accounts.marginfi_pda.key(),
                    &ctx.accounts.pool_token_receiver_account.key(),
                    &ctx.accounts.reserve_stake_account.key(),
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.manager_fee_account.key(),
                    &ctx.accounts.pool_mint.key(),
                    &anchor_spl::token::ID,
                    amount as u64,
                ),
                &[
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.stake_pool.to_account_info(),
                    ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
                    ctx.accounts.pool_token_receiver_account.to_account_info(),
                    ctx.accounts.reserve_stake_account.to_account_info(),
                    ctx.accounts.manager_fee_account.to_account_info(),
                    ctx.accounts.pool_mint.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.clock.to_account_info(),
                    ctx.accounts.stake_history.to_account_info(),
                    ctx.accounts.stake_program.to_account_info(),
                    ctx.accounts.rent.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        Ok(())
    }
    pub fn winner_winner_chickum_dinner_distribute(
        ctx: Context<Winner>,
        amount: u64,
    ) -> anchor_lang::Result<()> {
        let wsol_borrow_rate = ctx.accounts.oracle.load()?.wsol_borrow.mean as f64 // this is a 10^18 we want it as a f64 so we divide by 10^18
        / 1_000_000_000.0;

        let mint_supply = ctx.accounts.jarezi_mint.supply;
        let marginfi_pda = ctx.accounts.marginfi_pda.clone();
        msg!("amount {}", amount);
        // TODO: adjust calc for kickback
        let kickback = marginfi_pda.kickback_percent_bpm as f64 / 1_000_000.0;
        let rate = exchange_rate(
            ctx.accounts.pool_token_receiver_account.amount,
            mint_supply + amount ).unwrap().0.0.as_u64();
        msg!("rate {}", rate);
        let amount = amount as f64 * rate as f64 / 1_000_000_000.0;
        msg!("amount {}", amount);
        let amount = amount * (1.0 - kickback);
        msg!("amount {}", amount);
        let amount = amount as u64;

        msg!("amount: {}", amount);
        let obligation: Obligation = Obligation::unpack(&ctx.accounts.obligation_pubkey.data.borrow()).unwrap();
        let borrowed = obligation.borrowed_value;
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        oracle.last_borrowed_amount = borrowed.0.as_u64();
        let old_timestamp = oracle.last_borrowed_amount_timestamp;
        oracle.last_borrowed_amount_timestamp = Clock::get()?.unix_timestamp;
        let time_diff = oracle.last_borrowed_amount_timestamp - old_timestamp;
        msg!("time_diff: {}", time_diff);
        let time_diff = time_diff as f64 / 60.0 / 60.0 / 24.0 / 365.0;
        msg!("time_diff: {}", time_diff);
        let rate = wsol_borrow_rate.powf(time_diff);
        msg!("rate: {}", rate);
        let amount = amount as f64 * rate as f64;
        msg!("amount: {}", amount);
        
        if amount > 0.0 // TODO: ? 
        {
        let winner = ctx.accounts.marginfi_pda.thewinnerog;
        let signer: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], winner.as_ref(),
        &[marginfi_pda.bump]]];

        // mint amount to actual_destination

        anchor_spl::token_interface::mint_to(CpiContext::new_with_signer(
            ctx.accounts.token_program_2022.to_account_info(),
            MintTo {
                mint: ctx.accounts.jarezi_mint.to_account_info(),
                to: ctx.accounts.actual_destination.to_account_info(),
                authority: ctx.accounts.marginfi_pda.to_account_info(),
            },
            &signer,
        ), amount as u64).unwrap();
    }
        Ok(())
    }
}
