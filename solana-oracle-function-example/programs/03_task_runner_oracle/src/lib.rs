#![allow(clippy::result_large_err)]
// Program: Solana Basic Oracle
// This Solana program will allow you to peridoically relay information from Binance to your
// program and store in an account. When a user interacts with our program they will reference
// the price from the previous push.
// - initialize:        Initializes the program and creates the accounts.
pub use switchboard_solana::prelude::*;

pub mod models;
pub use models::*;

declare_id!("FAqBpxJsdsAntjDv9iAb3WwL2F7PYaJLbm5B97kgbNep");

/// The seed used to derive the global program state PDA.
pub const PROGRAM_SEED: &[u8] = b"TASKRUNNERORACLE";

/// The maximum number of data feeds the oracle can support.
pub const MAX_NUM_DATA_FEEDS: usize = 1000;

// Store 100 samples for each data feed, along with their updated timestamp
pub const DATA_FEED_HISTORY_SIZE: usize = 100;

// Store 9 digits of precision for each data feed sample.
// Fixed precision makes it easiser to manage and scale.
// Dynamic scales = unpredictable gas/compute costs.
pub const DATA_FEED_PRECISION: usize = 9;

#[program]
pub mod task_runner_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        let program = &mut ctx.accounts.program.load_init()?;
        program.bump = *ctx.bumps.get("program").unwrap();
        program.authority = ctx.accounts.authority.key();
        program.oracle = ctx.accounts.oracle.key();

        // Optionally set the switchboard_function if provided
        program.switchboard_function = ctx.accounts.switchboard_function.key();

        ctx.accounts.oracle.load_init()?;
        // oracle.max_feeds = MAX_NUM_DATA_FEEDS as u32;

        Ok(())
    }

    pub fn add_feed(ctx: Context<AddFeed>, params: AddFeedParams) -> anchor_lang::Result<()> {
        // Params validation
        if params.name.len() > 32 {
            return Err(error!(OracleError::ArrayOverflow));
        }
        // IPFS hash is always required to add a feed
        if params.ipfs_hash.len() != 32 {
            return Err(error!(OracleError::ArrayOverflow));
        }

        let mut name = [0u8; 32];
        name[0..params.name.len()].clone_from_slice(&params.name);

        let mut ipfs_hash = [0u8; 32];
        ipfs_hash.clone_from_slice(&params.ipfs_hash);

        let oracle = &mut ctx.accounts.oracle.load_mut()?;

        // let idx: usize = if let Some(idx) = params.idx {
        //     idx as usize
        // } else {
        //     oracle.num_feeds as usize + 1
        // };

        let idx: usize = params.idx as usize;

        if idx > MAX_NUM_DATA_FEEDS {
            return Err(error!(OracleError::ArrayOverflow));
        }

        if oracle.feeds[idx].ipfs_hash != [0u8; 32] {
            return Err(error!(OracleError::DataFeedExistsAtIdx));
        }

        oracle.feeds[idx] = DataFeed {
            name,
            ipfs_hash,
            update_interval: params.update_interval,
            history_idx: 0,
            history: [Default::default(); DATA_FEED_HISTORY_SIZE],
        };

        Ok(())
    }

    pub fn remove_feed(
        ctx: Context<RemoveFeed>,
        idx: u32,
        ipfs_hash: Option<Vec<u8>>,
    ) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;

        if let Some(ipfs_hash_vec) = ipfs_hash.as_ref() {
            if ipfs_hash_vec.len() != 32 {
                return Err(error!(OracleError::ArrayOverflow));
            }
            let mut ipfs_hash = [0u8; 32];
            ipfs_hash.clone_from_slice(ipfs_hash_vec);

            if oracle.feeds[idx as usize].ipfs_hash != ipfs_hash {
                return Err(error!(OracleError::DataFeedHashMismatch));
            }
        } else if oracle.feeds[idx as usize].ipfs_hash == [0u8; 32] {
            return Err(error!(OracleError::DataFeedMissingAtIdx));
        };

        oracle.feeds[idx as usize] = Default::default();

        Ok(())
    }

    pub fn save_feed_result(
        ctx: Context<SaveFeedResult>,
        params: SaveFeedResultParams,
    ) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;

        oracle.save_result(params.idx as usize, params.result, params.ipfs_hash)?;

        msg!(
            "Data feed {} updated with result {}",
            params.idx,
            params.result
        );

        // msg!("Sanity check: {}", {
        //     oracle.feeds[params.idx as usize].history[0].value
        // },);

        Ok(())
    }

    pub fn save_feed_results(
        ctx: Context<SaveFeedResult>,
        params: Vec<SaveFeedResultParams>,
    ) -> anchor_lang::Result<()> {
        let oracle = &mut ctx.accounts.oracle.load_mut()?;
        for save_result in params {
            match oracle.save_result(
                save_result.idx as usize,
                save_result.result,
                save_result.ipfs_hash,
            ) {
                Ok(_) => {}
                Err(e) => {
                    msg!(
                        "failed to save feed result for idx {}: {:?}",
                        save_result.idx,
                        e
                    );
                }
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<MyProgramState>(),
        payer = payer,
        seeds = [PROGRAM_SEED],
        bump
    )]
    pub program: AccountLoader<'info, MyProgramState>,

    #[account(zero)]
    pub oracle: AccountLoader<'info, MyOracleState>,

    pub authority: Signer<'info>,

    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AddFeedParams {
    pub idx: u32,
    pub name: Vec<u8>,
    pub ipfs_hash: Vec<u8>,
    pub update_interval: u32,
}

#[derive(Accounts)]
#[instruction(params: AddFeedParams)] // rpc parameters hint
pub struct AddFeed<'info> {
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = authority,
        has_one = oracle,
    )]
    pub program: AccountLoader<'info, MyProgramState>,

    #[account(mut)]
    pub oracle: AccountLoader<'info, MyOracleState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveFeed<'info> {
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = authority,
        has_one = oracle,
    )]
    pub program: AccountLoader<'info, MyProgramState>,

    #[account(mut)]
    pub oracle: AccountLoader<'info, MyOracleState>,

    pub authority: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SaveFeedResultParams {
    pub idx: u32,
    pub ipfs_hash: Option<Vec<u8>>,
    pub result: u64,
}

#[derive(Accounts)]
#[instruction(params: SaveFeedResultParams)] // rpc parameters hint
pub struct SaveFeedResult<'info> {
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
        has_one = switchboard_function,
        has_one = oracle,
    )]
    pub program: AccountLoader<'info, MyProgramState>,

    #[account(mut)]
    pub oracle: AccountLoader<'info, MyOracleState>,

    // We use this to verify the functions enclave state was verified successfully
    #[account(
        constraint =
                switchboard_function.load()?.validate(
                &enclave_signer.to_account_info()
            )? @ OracleError::FunctionValidationFailed
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    pub enclave_signer: Signer<'info>,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum OracleError {
    #[msg("Invalid authority account")]
    InvalidAuthority,
    #[msg("Array overflow")]
    ArrayOverflow,
    #[msg("Data feed already exists at the provided idx.")]
    DataFeedExistsAtIdx,
    #[msg("Data feed does not exist at the provided idx.")]
    DataFeedMissingAtIdx,
    #[msg("Data feed at given idx has an unexpected ipfs hash")]
    DataFeedHashMismatch,
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
}
