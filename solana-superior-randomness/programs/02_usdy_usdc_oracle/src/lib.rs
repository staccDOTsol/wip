
#![allow(clippy::result_large_err)]
pub mod instructions;

pub mod models;
pub use models::*;

pub const PROGRAM_SEED: &[u8] = b"USDY_USDC_ORACLE_V2";

pub const ORACLE_SEED: &[u8] = b"ORACLE_USDY_SEED_V2";

pub use instructions::*;
pub use switchboard_solana::*;
declare_id!("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d");

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct RequestAccountData {
    pub bump: u8,
    pub pubkey_hash: [u8; 32],
    pub switchboard_request: Pubkey,
    pub seed: u32,
    pub blockhash: [u8; 32],
    pub result: [u8; 32],
    pub request_timestamp: i64,
    pub seed_timestamp: i64,
    pub reveal_timestamp: i64,
}

#[program]
pub mod superior_randomness {

    use super::*;
    pub fn init_mrgn_fi_pda(ctx: Context<InitMrgnFiPda>, bump: u8, kickback: u64, seeded_seed: String, seed2: String) -> anchor_lang::Result<()> {
        InitMrgnFiPda::init_mrgn_fi_pda(ctx, bump, kickback, seeded_seed, seed2)
    }

    pub fn set_jarezi_mint_metadata(ctx: Context<SetMetadata>, name: String, symbol: String, uri: String) -> anchor_lang::Result<()> {
        InitMrgnFiPda::set_jarezi_mint_metadata(ctx, name, symbol, uri)
    }

    pub fn create_seeded_account(
        ctx: Context<CreateSeededAccount>,
        params: CreateSeededAccountParams,
    ) -> anchor_lang::Result<()> {
        CreateSeededAccount::create_seeded_account(ctx, params)
    }
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        Deposit::deposit(ctx, amount)
    }
    pub fn withdraw(
        ctx: Context<Deposit>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        Deposit::withdraw(ctx, amount)
    }
    pub fn set_function(
        ctx: Context<SetFunction>,
    ) -> anchor_lang::Result<()> {
        Deposit::set_function(ctx)
    }
    pub fn set_winner_winner_chickum_dinner(
        ctx: Context<SetWinner>,
    ) -> anchor_lang::Result<()> {
        Deposit::set_winner_winner_chickum_dinner(ctx)
    }
    pub fn winner_winner_chickum_dinner_distribute(
        ctx: Context<Winner>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        Deposit::winner_winner_chickum_dinner_distribute(ctx, amount)
    }

    pub fn initialize(ctx: Context<Initialize>, bump: u8, bump2: u8) -> anchor_lang::Result<()> {
        let program = &mut ctx.accounts.program.load_init()?;
        program.bump = bump;
        program.authority = ctx.accounts.authority.key();

        // Optionally set the switchboard_function if provided
        if let Some(switchboard_function) = ctx.accounts.switchboard_function.as_ref() {
            program.switchboard_function = switchboard_function.key();
        }

        let oracle = &mut ctx.accounts.oracle.load_init()?;
        oracle.bump = bump2;

        Ok(())
    }


    pub fn update(ctx: Context<Initialize>, bump: u8, bump2: u8) -> anchor_lang::Result<()> {
        let program = &mut ctx.accounts.program.load_mut()?;
        program.bump = bump;
        program.authority = ctx.accounts.authority.key();

        // Optionally set the switchboard_function if provided
        if let Some(switchboard_function) = ctx.accounts.switchboard_function.as_ref() {
            program.switchboard_function = switchboard_function.key();
        }

        let oracle = &mut ctx.accounts.oracle.load_init()?;
        oracle.bump = bump2;

        Ok(())
    }

    pub fn refresh_oracles(
        ctx: Context<RefreshOracles>,
        params: RefreshOraclesParams,
    ) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        msg!("saving oracle data");
        oracle.save_rows(&params.rows)?;
        msg!("${}", {oracle.jitosol_sol.mean});
        msg!("${}", {oracle.jitosol_sol.median});
        msg!("{}%", {oracle.jitosol_sol.std});
        msg!("${}", {oracle.bsol_sol.mean});
        msg!("${}", {oracle.bsol_sol.median});
        msg!("{}%", {oracle.bsol_sol.std});
        
        Ok(())
    }
    pub fn trigger_function(ctx: Context<TriggerFunction>) -> anchor_lang::Result<()> {
        FunctionTrigger {
            function: ctx.accounts.switchboard_function.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            attestation_queue: ctx.accounts.attestation_queue.to_account_info(),
        }
        .invoke(ctx.accounts.attestation_program.clone())?;
        Ok(())
    }
}
// Program: Solana TWAP Oracle
// This Solana program will allow you to peridoically relay information from EtherPrices to your
// program and store in an account. When a user interacts with our program they will reference
// the price from the previous push.
// - initialize:        Initializes the program and creates the accounts.
// - set_function:      Sets the Switchboard Function for our program. This is the only function
//                      allowed to push data to our program.
// - refresh_oracle:    This is the instruction our Switchboard Function will emit to update
//                      our oracle prices.
// - trigger_function:  Our Switchboard Function will be configured to push data on a pre-defined
//                      schedule. This instruction will allow us to manually request a new price
//                      from the off-chain oracles.


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        space = 8 + std::mem::size_of::<MyProgramState>(),
        payer = payer,
        seeds = [PROGRAM_SEED],
        bump
    )]
    pub program: AccountLoader<'info, MyProgramState>,

    #[account(
        init,
        space = 8 + std::mem::size_of::<MyOracleState>(),
        payer = payer,
        seeds = [ORACLE_SEED],
        bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,

    pub authority: Signer<'info>,

    pub switchboard_function: Option<AccountLoader<'info, FunctionAccountData>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(params: RefreshOraclesParams)] // rpc parameters hint
pub struct RefreshOracles<'info> {
    // We need this to validate that the Switchboard Function passed to our program
    // is the expected one.
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
       has_one = switchboard_function
    )]
    pub program: AccountLoader<'info, MyProgramState>,

    #[account(
        mut,
        seeds = [ORACLE_SEED],
        bump = oracle.load()?.bump
    )]
    pub oracle: AccountLoader<'info, MyOracleState>,

    // We use this to verify the functions enclave state was verified successfully
   #[account(
    constraint =
                switchboard_function.load()?.validate(
                &enclave_signer.to_account_info()
            )? @ USDY_USDC_ORACLEError::FunctionValidationFailed     
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    pub enclave_signer: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RefreshOraclesParams {
    pub rows: Vec<OracleDataWithTradingSymbol>,
}

#[derive(Accounts)]
pub struct TriggerFunction<'info> {
    // We need this to validate that the Switchboard Function passed to our program
    // is the expected one.
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = switchboard_function
    )]
    pub program: AccountLoader<'info, MyProgramState>,

    #[account(mut,
        has_one = authority,
        has_one = attestation_queue,
        owner = attestation_program.key()
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    pub authority: Signer<'info>,

    pub attestation_queue: AccountLoader<'info, AttestationQueueAccountData>,

    /// CHECK: address is explicit
    #[account(address = SWITCHBOARD_ATTESTATION_PROGRAM_ID)]
    pub attestation_program: AccountInfo<'info>,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum USDY_USDC_ORACLEError {
    #[msg("Invalid authority account")]
    InvalidAuthority,
    #[msg("Array overflow")]
    ArrayOverflow,
    #[msg("Stale data")]
    StaleData,
    #[msg("Invalid trusted signer")]
    InvalidTrustedSigner,
    #[msg("Invalid MRENCLAVE")]
    InvalidMrEnclave,
    #[msg("Failed to find a valid trading symbol for this price")]
    InvalidSymbol,
    #[msg("FunctionAccount pubkey did not match program_state.function")]
    IncorrectSwitchboardFunction,
    #[msg("FunctionAccount pubkey did not match program_state.function")]
    InvalidSwitchboardFunction,
    #[msg("FunctionAccount was not validated successfully")]
    FunctionValidationFailed,

    RequestAlreadySeeded,
    RequestAlreadyRevealed,
    KeyVerifyFailed,
}
