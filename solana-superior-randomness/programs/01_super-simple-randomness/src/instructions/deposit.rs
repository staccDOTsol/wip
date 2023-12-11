pub use crate::SbError;
pub use crate::*;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token_interface::{MintTo, Token2022};
use std::str::FromStr;
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

const SEED_PREFIX: &[u8] = b"marginfi";

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
    pub fn init_obligation_account(
        ctx: Context<CreateSeededAccount>,
        _params: CreateSeededAccountParams,
    ) -> anchor_lang::Result<()> {
        let to_pubkey = ctx.accounts.to.key();
        let mrgnfi_pda = ctx.accounts.program.clone();
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
                ctx.accounts.to.to_account_info(),
                ctx.accounts.lending_market.to_account_info(),
                mrgnfi_pda.to_account_info(),
                ctx.accounts.solend_sdk.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
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
        /*
        let cpi_ctx = anchor_lang_26::context::CpiContext::new_with_signer(
            marginfi_program,
            marginfi::cpi::accounts::MarginfiAccountInitialize {
                marginfi_group: marginfi_group.to_account_info(),
                marginfi_account: marginfi_account.to_account_info(),
                authority: marginfi_pda.to_account_info(),
                system_program: system_program.to_account_info(),
                fee_payer: ctx.accounts.authority.to_account_info(),
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
    pub marginfi_bank_jito: AccountInfo<'info>,
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
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub stake_pool_jitosol: AccountInfo<'info>,
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
    /// CHECK: Checked by CPI to Spl Stake Program
    #[account(mut)]
    pub stake_pool_withdraw_authority_wsol: AccountInfo<'info>,
    /// CHECK: Checked by CPI to Spl Stake Program
    pub bank_liquidity_vault_authority_wsol: AccountInfo<'info>,
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
}
impl Deposit<'_> {
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        bsol_price: u64,
        jitosol_price: u64,
    ) -> anchor_lang::Result<()> {
        let marginfi_pda = ctx.accounts.marginfi_pda.clone();
        let signer: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], &[marginfi_pda.bump]]];
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
                    &ctx.accounts.pool_token_receiver_account.key(),
                    &ctx.accounts.pool_mint.key(),
                    &spl_token::id(),
                    amount,
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
                ],
            )?;
        }
        {
            let rate: f64 = 1_000_000_000 as f64 / bsol_price as f64;
            let stake_pool_tokens = amount as f64 * rate * (1.0 - 0.008);

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
            let rate: f64 = 1_000_000_000 as f64 / bsol_price as f64;
            let stake_pool_tokens = amount as f64 * rate * (1.0 - 0.008);
            let ltv: f64 = 0.625;
            let rate: f64 = 1_000_000_000 as f64 / jitosol_price as f64;

            let amount = stake_pool_tokens * ltv;
            let amount = amount * rate as f64;

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
                    Some(ctx.accounts.pool_token_receiver_account_wsol.key()),
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
                ],
                &signer,
            )
            .unwrap();
        }
        {
            solana_program::program::invoke_signed(
                &spl_token::instruction::close_account(
                    &spl_token::ID,
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
            let rate: f64 = 1_000_000_000 as f64 / bsol_price as f64;
            let stake_pool_tokens = amount as f64 * rate * (1.0 - 0.008);
            let ltv: f64 = 0.625;
            let rate: f64 = 1_000_000_000 as f64 / jitosol_price as f64;

            let amount = stake_pool_tokens * ltv;
            let amount = amount * rate as f64;
            invoke(
                &spl_stake_pool::instruction::deposit_sol(
                    &spl_stake_pool::id(),
                    &ctx.accounts.stake_pool_jitosol.key(),
                    &ctx.accounts.stake_pool_withdraw_authority_jitosol.key(),
                    &ctx.accounts.reserve_stake_account_jitosol.key(),
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.pool_token_receiver_account_jitosol.key(),
                    &ctx.accounts.manager_fee_account_jitosol.key(),
                    &ctx.accounts.pool_token_receiver_account_jitosol.key(),
                    &ctx.accounts.pool_mint_jitosol.key(),
                    &spl_token::id(),
                    amount as u64,
                ),
                &[
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.stake_pool_jitosol.to_account_info(),
                    ctx.accounts
                        .stake_pool_withdraw_authority_jitosol
                        .to_account_info(),
                    ctx.accounts.reserve_stake_account_jitosol.to_account_info(),
                    ctx.accounts
                        .pool_token_receiver_account_jitosol
                        .to_account_info(),
                    ctx.accounts.manager_fee_account_jitosol.to_account_info(),
                    ctx.accounts.pool_mint_jitosol.to_account_info(),
                    ctx.accounts.stake_pool_jitosol.to_account_info(),
                    ctx.accounts.stake_pool_program.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                ],
            )
            .unwrap();
        }
        {
            let rate: f64 = 1_000_000_000 as f64 / bsol_price as f64;
            let stake_pool_tokens = amount as f64 * rate * (1.0 - 0.008);
            // mint_to
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program_2022.to_account_info(),
                MintTo {
                    mint: ctx.accounts.jarezi_mint.to_account_info(),
                    to: ctx.accounts.jarezi_token_account.to_account_info(),
                    authority: ctx.accounts.marginfi_pda.to_account_info(),
                },
                &signer,
            );

            anchor_spl::token_interface::mint_to(cpi_ctx, stake_pool_tokens as u64).unwrap();
        }

        Ok(())
    }
    pub fn withdraw(
        ctx: Context<Deposit>,
        amount: u64,
        _bsol_price: u64,
        jitosol_price: u64,
    ) -> anchor_lang::Result<()> {
        let marginfi_pda = ctx.accounts.marginfi_pda.clone();
        let signer: &[&[&[u8]]] = &[&[&SEED_PREFIX[..], &[marginfi_pda.bump]]];

        // burn tokens
        {
            solana_program::program::invoke(
                &spl_token_2022::instruction::burn(
                    &ctx.accounts.token_program_2022.key(),
                    &ctx.accounts.jarezi_token_account.key(),
                    &ctx.accounts.jarezi_mint.key(),
                    &ctx.accounts.signer.key(),
                    &[],
                    amount,
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
                    &ctx.accounts.stake_pool_jitosol.key(),
                    &ctx.accounts.stake_pool_withdraw_authority_jitosol.key(),
                    &ctx.accounts.marginfi_pda.key(),
                    &ctx.accounts.pool_token_receiver_account_jitosol.key(),
                    &ctx.accounts.reserve_stake_account_jitosol.key(),
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.manager_fee_account_jitosol.key(),
                    &ctx.accounts.pool_mint_jitosol.key(),
                    &spl_token::id(),
                    amount as u64,
                ),
                &[
                    ctx.accounts.marginfi_pda.to_account_info(),
                    ctx.accounts.stake_pool_jitosol.to_account_info(),
                    ctx.accounts
                        .stake_pool_withdraw_authority_jitosol
                        .to_account_info(),
                    ctx.accounts
                        .pool_token_receiver_account_jitosol
                        .to_account_info(),
                    ctx.accounts.reserve_stake_account_jitosol.to_account_info(),
                    ctx.accounts.manager_fee_account_jitosol.to_account_info(),
                    ctx.accounts.pool_mint_jitosol.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.stake_history.to_account_info(),
                    ctx.accounts.stake_program.to_account_info(),
                    ctx.accounts.clock.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                ],
                &signer,
            )
            .unwrap();
        }
        {
            // transfer minimum rent + lamports to to

            let minimum_rent = Rent::get()?.minimum_balance(165);
            let rate = 1_000_000_000 as f64 / jitosol_price as f64;
            let lamports = (amount as f64 * rate) as u64;

            invoke(
                &solana_program::system_instruction::transfer(
                    &ctx.accounts.signer.key(),
                    &ctx.accounts.pool_token_receiver_account_wsol.key(),
                    lamports + minimum_rent,
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
            solana_program::program::invoke_signed(
                &spl_token::instruction::sync_native(
                    &spl_token::ID,
                    &ctx.accounts.pool_token_receiver_account_wsol.key(),
                )?,
                &[ctx
                    .accounts
                    .pool_token_receiver_account_wsol
                    .to_account_info()],
                &signer,
            )
            .unwrap();
        }
        {
            let rate = 1_000_000_000 as f64 / jitosol_price as f64;
            let lamports = (amount as f64 * rate) as u64;
            // fee of 1/1000
            let amount = lamports * 999 / 1000;
            // repay obligatino liquidity

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
                    &spl_token::id(),
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
}
