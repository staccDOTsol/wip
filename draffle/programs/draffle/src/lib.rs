pub mod randomness_tools;
pub mod recent_blockhashes;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::token::Token;
use anchor_spl::token::{self, Mint, TokenAccount};
use std::cell::{Ref, RefMut};
use std::convert::TryFrom;
use basic_oracle::{MyProgramState, PROGRAM_SEED};
use spl_stake_pool::instruction::{deposit_sol, withdraw_sol};

pub const ENTRANTS_SIZE: u32 = 5000;
pub const TIME_BUFFER: i64 = 20;

#[cfg(not(feature = "production"))]
pub const PROTOCOL_FEE_BPS: u128 = 10;

#[cfg(feature = "production")]
pub const PROTOCOL_FEE_BPS: u128 = 0;

pub mod treasury {
    use super::*;
    // Replace with your treasury, this is the default treasury for testing purposes
    declare_id!("Treasury11111111111111111111111111111111112");
}

#[cfg(not(feature = "production"))]
declare_id!("dRaFFLe111111111111111111111111111111111112");

#[cfg(feature = "production")]
declare_id!("dRafA7ymQiLKjR5dmmdZC9RPX4EQUjqYFB3mWokRuDs");

#[program]
pub mod draffle {
    use basic_oracle::invoke_signed;

    use super::*;

    pub fn create_raffle(
        ctx: Context<CreateRaffle>,
        ticket_price: u64,
        max_entrants: u32,
    ) -> Result<()> {
        let raffle = &mut ctx.accounts.raffle;

        raffle.bump = *ctx.bumps.get("raffle").unwrap();
        raffle.creator = *ctx.accounts.creator.key;
        raffle.total_prizes = 0;
        raffle.claimed_prizes = 0;
        raffle.randomness = None;
        // end_timetamp = now + a day
        let end_timestamp = Clock::get()?.unix_timestamp + 86400;
        raffle.end_timestamp = end_timestamp;
        raffle.ticket_price = ticket_price;
        raffle.entrants = ctx.accounts.entrants.key();
        raffle.initial_bsol_price = ctx.accounts.program.load()?.bsol_price as u64;
        let entrants = &mut ctx.accounts.entrants;
        entrants.total = 0;
        entrants.max = max_entrants;

        // Verify that we have enough space for max entrants
        require!(
            entrants.to_account_info().data_len()
                >= Entrants::BASE_SIZE + 32 * max_entrants as usize,
            RaffleError::EntrantsAccountTooSmallForMaxEntrants
        );

        Ok(())
    }

    pub fn add_prize(ctx: Context<AddPrize>, prize_index: u32, amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        let raffle = &mut ctx.accounts.raffle;

        require!(
            clock.unix_timestamp < raffle.end_timestamp,
            RaffleError::RaffleEnded
        );
        require_eq!(
            prize_index,
            raffle.total_prizes,
            RaffleError::InvalidPrizeIndex
        );
        require_neq!(amount, 0, RaffleError::NoPrize);

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.prize.to_account_info(),
                    authority: ctx.accounts.creator.to_account_info(),
                },
            ),
            amount,
        )?;

        raffle.total_prizes = raffle
            .total_prizes
            .checked_add(1)
            .ok_or(RaffleError::InvalidCalculation)?;

        Ok(())
    }

    pub fn buy_tickets(ctx: Context<BuyTickets>, amount: u32) -> Result<()> {
        let clock = Clock::get()?;
        let raffle = &mut ctx.accounts.raffle;
        let entrants = &mut ctx.accounts.entrants;

        let membership_key = &mut ctx.accounts.membership_key;
        let membership_key_account_info = membership_key.clone().to_account_info();
        require!(
            clock.unix_timestamp < raffle.end_timestamp,
            RaffleError::RaffleEnded
        );

        let bsol_price = ctx.accounts.program.load()?.bsol_price;

        let shares = amount
            .checked_mul(bsol_price as u32)
            .ok_or(RaffleError::InvalidCalculation)?;
        membership_key.clone().total_bsol_price = membership_key.clone().total_bsol_price
            .checked_add(bsol_price as u64)
            .ok_or(RaffleError::InvalidCalculation)?;
        membership_key.clone().count = membership_key.clone().count
            .checked_add(1)
            .ok_or(RaffleError::InvalidCalculation)?;
        membership_key.clone().average_bsol_price = membership_key.clone().total_bsol_price
            .checked_div(membership_key.clone().count)
            .ok_or(RaffleError::InvalidCalculation)?;
        raffle.total_staked_shares = raffle
            .total_staked_shares
            .checked_add(shares as u128)
            .ok_or(RaffleError::InvalidCalculation)?;
        let total = entrants.total;
        let entrants_account_info = entrants.clone().to_account_info();
        for i in 0..amount {
            membership_key.clone().totals.push(total as u64 +i as u64);
            entrants.append_entrant(
                entrants_account_info.data.borrow_mut(),
                ctx.accounts.buyer_token_account.owner,
                bsol_price as u128,
                total + i as u32,
            )?;
        }

        let lamport_snapshot = &ctx.accounts.proceeds.to_account_info().lamports();
        let ix = deposit_sol(
            &ctx.accounts.stake_pool_program.key(),
            &ctx.accounts.stake_pool.key(),
            &ctx.accounts.stake_pool_withdraw_authority.key(),
            &ctx.accounts.reserve_stake_account.key(),
            &ctx.accounts.proceeds.key(),
            &ctx.accounts.purchaser_token_account.key(),
            &ctx.accounts.manager_fee_account.key(),
            &ctx.accounts.referrer_pool_tokens_account.key(),
            &ctx.accounts.pool_mint.key(),
            &ctx.accounts.token_program.key(),
            *lamport_snapshot
        );
        
        invoke_signed(&ix, &[
            ctx.accounts.stake_pool_program.to_account_info().clone(),
            ctx.accounts.stake_pool.to_account_info().clone(),
            ctx.accounts.stake_pool_withdraw_authority.to_account_info().clone(),
            ctx.accounts.reserve_stake_account.to_account_info().clone(),
            ctx.accounts.proceeds.to_account_info().clone(),
            ctx.accounts.purchaser_token_account.to_account_info().clone(),
            ctx.accounts.manager_fee_account.to_account_info().clone(),
            ctx.accounts.referrer_pool_tokens_account.to_account_info().clone(),
            ctx.accounts.pool_mint.to_account_info().clone(),
            ctx.accounts.token_program.to_account_info().clone(),
        ],
        &[&[
            b"raffle".as_ref(),
            raffle.clone().entrants.as_ref(),
            &[raffle.bump],
        ]])?;

        msg!("Total entrants: {}", entrants.total);

        Ok(())
    }
    pub fn withdraw_tickets(ctx: Context<BuyTickets>) -> Result<()> {
        let clock = Clock::get()?;
        let raffle = &mut ctx.accounts.raffle;
        let entrants = &mut ctx.accounts.entrants;
        let membership_key = &mut ctx.accounts.membership_key;
        let entrants_account_info = entrants.clone().to_account_info();
       
       let total_staked_shares = raffle
       .total_staked_shares;
        let bsol_price = ctx.accounts.program.load()?.bsol_price;
        let average_bsol_price = membership_key.clone().average_bsol_price;
        membership_key.clone().total_bsol_price = 0;
        membership_key.clone().count = 0;
        membership_key.clone().average_bsol_price = 0;
        
        let prize_amount = membership_key.clone().shares
            .checked_mul(average_bsol_price as u64)
            .ok_or(RaffleError::InvalidCalculation)?
            .checked_div(bsol_price as u64)
            .ok_or(RaffleError::InvalidCalculation)?;

            
        let ix = withdraw_sol(
            &ctx.accounts.stake_pool_program.key(),
            &ctx.accounts.stake_pool.key(),
            &ctx.accounts.stake_pool_withdraw_authority.key(),
            &raffle.key(),
            &ctx.accounts.proceeds.key(),
            &ctx.accounts.reserve_stake_account.key(),
            &ctx.accounts.buyer_transfer_authority.key(),
            &ctx.accounts.manager_fee_account.key(),
            &ctx.accounts.pool_mint.key(),
            &ctx.accounts.token_program.key(),
            prize_amount as u64
        );
        
        invoke_signed(&ix, &[
            ctx.accounts.stake_pool_program.to_account_info().clone(),
            ctx.accounts.stake_pool.to_account_info().clone(),
            ctx.accounts.stake_pool_withdraw_authority.to_account_info().clone(),
            ctx.accounts.reserve_stake_account.to_account_info().clone(),
            ctx.accounts.proceeds.to_account_info().clone(),
            ctx.accounts.purchaser_token_account.to_account_info().clone(),
            ctx.accounts.manager_fee_account.to_account_info().clone(),
            ctx.accounts.referrer_pool_tokens_account.to_account_info().clone(),
            ctx.accounts.pool_mint.to_account_info().clone(),
            ctx.accounts.token_program.to_account_info().clone(),
        ],
        &[&[
            b"raffle".as_ref(),
            raffle.clone().entrants.as_ref(),
            &[raffle.bump],
        ]])?;
        drop(membership_key);
        let cloned = ctx.accounts.membership_key.clone();
        // iterate through totals and withdraw entrant
        let token_owner = ctx.accounts.buyer_token_account.owner;
        let vec_iterator = cloned.totals.iter();
        vec_iterator
        .for_each(|&i| {
            entrants.withdraw_entrant(
                entrants_account_info.data.borrow_mut(),
                token_owner,
                bsol_price as u128,
                i as u32,
            ).unwrap();

        });

        let cloned_mut = ctx.accounts.membership_key.clone();
        cloned_mut.clone().totals = Vec::new();
        raffle.total_staked_shares = total_staked_shares
            .checked_sub(prize_amount as u128)
            .ok_or(RaffleError::InvalidCalculation)?;
        
        msg!("Total entrants: {}", entrants.total);

        Ok(())
    }
    pub fn reveal_winners(ctx: Context<RevealWinners>) -> Result<()> {
        let clock = Clock::get()?;
        let raffle = &mut ctx.accounts.raffle;

        let end_timestamp_with_buffer = raffle
            .end_timestamp
            .checked_add(TIME_BUFFER)
            .ok_or(RaffleError::InvalidCalculation)?;
        require!(
            clock.unix_timestamp > end_timestamp_with_buffer,
            RaffleError::RaffleStillRunning
        );

        let randomness =
            recent_blockhashes::last_blockhash_accessor(&ctx.accounts.recent_blockhashes)?;

        match raffle.randomness {
            Some(_) => return Err(RaffleError::WinnersAlreadyDrawn.into()),
            None => raffle.randomness = Some(randomness),
        }

        Ok(())
    }

    pub fn claim_prize(
        ctx: Context<ClaimPrize>,
        prize_index: u32,
        ticket_index: u32,
    ) -> Result<()> {
        let raffle_account_info = ctx.accounts.raffle.to_account_info();
        let raffle = &mut ctx.accounts.raffle;

        let randomness = match raffle.randomness {
            Some(randomness) => randomness,
            None => return Err(RaffleError::WinnerNotDrawn.into()),
        };

        let entrants = &ctx.accounts.entrants;

        // When total number of entrants is zero we bypass the winner check and verify the "winner_token_account" belongs to the raffle creator,
        if entrants.total == 0 {
            require_keys_eq!(
                ctx.accounts.winner_token_account.key(),
                raffle.creator,
                RaffleError::OnlyCreatorCanClaimNoEntrantRafflePrizes
            );
            msg!(
                "Raffle creator claiming prize {} of no entrant raffle",
                prize_index
            );
        } else {
            let winner_rand = randomness_tools::expand(randomness, prize_index);
            let winner_index = winner_rand % entrants.total;

            msg!(
                "Ticket {} attempts claiming prize {} (winner is {})",
                ticket_index,
                prize_index,
                winner_index
            );
            msg!("{} {}", winner_rand, winner_index);

            require_eq!(ticket_index, winner_index, RaffleError::TicketHasNotWon);
            let entrant_for_ticket = Entrants::get_entrant(
                ctx.accounts.entrants.to_account_info().data.borrow(),
                ticket_index as usize,
            );
            require_keys_eq!(
                ctx.accounts.winner_token_account.key(),
                entrant_for_ticket,
                RaffleError::TokenAccountNotOwnedByWinner
            );
        }

        let bsol_price = ctx.accounts.program.load()?.bsol_price;
        let total_shares = raffle.total_staked_shares;
        let initial_bsol_price = raffle.initial_bsol_price;
        let prize_amount = total_shares
            .checked_mul(bsol_price as u128)
            .ok_or(RaffleError::InvalidCalculation)?
            .checked_div(initial_bsol_price as u128)
            .ok_or(RaffleError::InvalidCalculation)?;

        // we want to only award the yield and keep the principal in the treasury
        
        
        
            
        let ix = withdraw_sol(
            &ctx.accounts.stake_pool_program.key(),
            &ctx.accounts.stake_pool.key(),
            &ctx.accounts.stake_pool_withdraw_authority.key(),
            &raffle.key(),
            &ctx.accounts.proceeds.key(),
            &ctx.accounts.reserve_stake_account.key(),
            &ctx.accounts.winner_token_account.key(),
            &ctx.accounts.manager_fee_account.key(),
            &ctx.accounts.pool_mint.key(),
            &ctx.accounts.token_program.key(),
            prize_amount as u64
        );
        
        invoke_signed(&ix, &[
            ctx.accounts.stake_pool_program.to_account_info().clone(),
            ctx.accounts.stake_pool.to_account_info().clone(),
            ctx.accounts.stake_pool_withdraw_authority.to_account_info().clone(),
            ctx.accounts.reserve_stake_account.to_account_info().clone(),
            ctx.accounts.proceeds.to_account_info().clone(),
            raffle.to_account_info().clone(),
            ctx.accounts.winner_token_account.to_account_info().clone(),
            ctx.accounts.manager_fee_account.to_account_info().clone(),
            ctx.accounts.referrer_pool_tokens_account.to_account_info().clone(),
            ctx.accounts.pool_mint.to_account_info().clone(),
            ctx.accounts.token_program.to_account_info().clone(),
        ],
        &[&[
            b"raffle".as_ref(),
            raffle.clone().entrants.as_ref(),
            &[raffle.bump],
        ]])?;
        raffle.claimed_prizes = raffle
            .claimed_prizes
            .checked_add(1)
            .ok_or(RaffleError::InvalidCalculation)?;

        Ok(())
    }

    pub fn collect_proceeds<'info>(
        ctx: Context<'_, '_, '_, 'info, CollectProceeds<'info>>,
    ) -> Result<()> {
        let raffle = &ctx.accounts.raffle;

        require!(raffle.randomness.is_some(), RaffleError::WinnerNotDrawn);

        let proceeds_amount = ctx.accounts.proceeds.amount;
        let protocol_fee_amount = u64::try_from(
            (proceeds_amount as u128)
                .checked_mul(PROTOCOL_FEE_BPS)
                .ok_or(RaffleError::InvalidCalculation)?,
        )
        .map_err(|_| RaffleError::InvalidCalculation)?
        .checked_div(10_000)
        .ok_or(RaffleError::InvalidCalculation)?;
        let creator_proceeds = proceeds_amount
            .checked_sub(protocol_fee_amount)
            .ok_or(RaffleError::InvalidCalculation)?;

        let bump = raffle.bump;

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.proceeds.to_account_info(),
                    to: ctx.accounts.creator_proceeds.to_account_info(),
                    authority: ctx.accounts.raffle.to_account_info(),
                },
                &[&[b"raffle".as_ref(), raffle.entrants.as_ref(), &[bump]]],
            ),
            creator_proceeds,
        )?;

        if PROTOCOL_FEE_BPS > 0 {
            let mut remaining_accounts_iter = ctx.remaining_accounts.iter();
            let treasury_token_account: Account<TokenAccount> =
                Account::try_from(&remaining_accounts_iter.next().unwrap())?;
            require_keys_eq!(
                treasury_token_account.owner,
                treasury::ID,
                RaffleError::InvalidTreasuryTokenAccountOwner
            );

            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    token::Transfer {
                        from: ctx.accounts.proceeds.to_account_info(),
                        to: treasury_token_account.to_account_info(),
                        authority: ctx.accounts.raffle.to_account_info(),
                    },
                    &[&[b"raffle".as_ref(), raffle.entrants.as_ref(), &[bump]]],
                ),
                protocol_fee_amount,
            )?;
        }

        Ok(())
    }

    pub fn close_entrants(ctx: Context<CloseEntrants>) -> Result<()> {
        let raffle = &ctx.accounts.raffle;
        require!(
            raffle.claimed_prizes == raffle.total_prizes,
            RaffleError::UnclaimedPrizes
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(
        init,
        seeds = [b"raffle".as_ref(), entrants.key().as_ref()],
        bump,
        payer = creator,
        space = 8 + 32 + 4 + 4 + 4 + 32 + 8 + 8 + 32 + 8,
    )]
    pub raffle: Account<'info, Raffle>,
    #[account(zero)]
    pub entrants: Account<'info, Entrants>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        seeds = [raffle.key().as_ref(), b"proceeds"],
        bump,
        payer = creator,
        token::mint = proceeds_mint,
        token::authority = raffle,
    )]
    pub proceeds: Account<'info, TokenAccount>,
    pub proceeds_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
    )]
    pub program: AccountLoader<'info, MyProgramState>,
}

#[derive(Accounts)]
#[instruction(prize_index: u32)]
pub struct AddPrize<'info> {
    #[account(mut, has_one = creator)]
    pub raffle: Account<'info, Raffle>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [raffle.key().as_ref(), b"prize", &prize_index.to_le_bytes()],
        bump,
        payer = creator,
        token::mint = prize_mint,
        token::authority = raffle,
    )]
    pub prize: Account<'info, TokenAccount>,
    pub prize_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyTickets<'info> {
    #[account(has_one = entrants)]
    pub raffle: Account<'info, Raffle>,
    #[account(mut)]
    pub entrants: Account<'info, Entrants>,
    #[account(
        mut,
        seeds = [raffle.key().as_ref(), b"proceeds"],
        bump,
    )]
    pub proceeds: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_transfer_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
    )]
    pub program: AccountLoader<'info, MyProgramState>,
    #[account(init,
        payer = authority,
        space = 8 + std::mem::size_of::<TokenAccount>(),
    )]
    pub purchaser_token_account: Account<'info, TokenAccount>,
    #[account(mut)]

    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub mint: Account<'info, Mint>,
    /// CHECK:
    pub stake_pool_program: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK:
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK:
    pub stake_pool_withdraw_authority: AccountInfo<'info>,
    #[account(mut)]
    pub manager_fee_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub referrer_pool_tokens_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK:
    pub reserve_stake_account: UncheckedAccount<'info>,
    pub pool_mint : Account<'info, Mint>,
    #[account(init_if_needed,
        seeds = [b"member", buyer_transfer_authority.key().as_ref()],
        bump,
        payer = buyer_transfer_authority,
        space = 8 + std::mem::size_of::<MembershipKey>(),
    )]
    pub membership_key: Account<'info, MembershipKey>,
}

#[derive(Accounts)]
pub struct RevealWinners<'info> {
    #[account(mut)]
    pub raffle: Account<'info, Raffle>,
    /// CHECK: sysvar address check is hardcoded, we want to avoid the default deserialization
    #[account(address = sysvar::recent_blockhashes::ID)]
    pub recent_blockhashes: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(prize_index: u32)]
pub struct ClaimPrize<'info> {
    #[account(mut, has_one = entrants)]
    pub raffle: Account<'info, Raffle>,
    pub entrants: Account<'info, Entrants>,
    #[account(
        mut,
        seeds = [raffle.key().as_ref(), b"prize", &prize_index.to_le_bytes()],
        bump,
    )]
    pub prize: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK:
    pub winner_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = [PROGRAM_SEED],
        bump = program.load()?.bump,
    )]
    pub program: AccountLoader<'info, MyProgramState>,
    /// CHECK:
    pub stake_pool_program: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK:
    pub stake_pool: UncheckedAccount<'info>,
    /// CHECK:
    pub stake_pool_withdraw_authority: AccountInfo<'info>,
    #[account(mut)]
    pub manager_fee_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub referrer_pool_tokens_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK:
    pub reserve_stake_account: UncheckedAccount<'info>,
    pub pool_mint : Account<'info, Mint>,
    #[account(
        mut,
        seeds = [raffle.key().as_ref(), b"proceeds"],
        bump
    )]
    pub proceeds: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct CollectProceeds<'info> {
    #[account(has_one = creator)]
    pub raffle: Account<'info, Raffle>,
    #[account(
        mut,
        seeds = [raffle.key().as_ref(), b"proceeds"],
        bump
    )]
    pub proceeds: Account<'info, TokenAccount>,
    pub creator: Signer<'info>,
    #[account(
        mut,
        constraint = creator_proceeds.owner == creator.key()
    )]
    pub creator_proceeds: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseEntrants<'info> {
    #[account(has_one = creator, has_one = entrants)]
    pub raffle: Account<'info, Raffle>,
    #[account(mut, close = creator)]
    pub entrants: Account<'info, Entrants>,
    pub creator: Signer<'info>,
}

#[account]
#[derive(Debug)]
pub struct Raffle {
    pub bump: u8,
    pub creator: Pubkey,
    pub total_prizes: u32,
    pub claimed_prizes: u32,
    pub randomness: Option<[u8; 32]>,
    pub end_timestamp: i64,
    pub ticket_price: u64,
    pub entrants: Pubkey,
    pub total_staked_shares: u128,
    pub initial_bsol_price: u64,
}


#[account]
pub struct MembershipKey {
    pub shares: u64,
    pub average_bsol_price: u64,
    pub total_bsol_price: u64,
    pub count: u64,
    pub totals: Vec<u64>,
}
#[account]
pub struct Entrants {
    pub total: u32,
    pub max: u32,
    // Entrants array of length max
    // pub entrants: [Pubkey; max],
}

impl Entrants {
    /// The size of entrants excluding the entrants array
    const BASE_SIZE: usize = 8 + 4 + 4;

    pub fn get_entrant(entrants_data: Ref<&mut [u8]>, index: usize) -> Pubkey {
        let mut start_index = Entrants::BASE_SIZE + ((32 + 8 + 4) * index);
        let mut winner = Pubkey::default();
        while winner == Pubkey::default() {
            start_index += 32 + 8 + 4;
            winner = Pubkey::new(&entrants_data[start_index..start_index + 32])
        }
        winner
    }

    fn append_entrant(
        &mut self,
        mut entrants_data: RefMut<&mut [u8]>,
        entrant: Pubkey, // 32
        shares: u128, // 8
        total: u32, // 4
    ) -> Result<()> {
        let current_index = Entrants::BASE_SIZE + ((32 + 8 + 4) * self.total) as usize;
        let entrant_slice: &mut [u8] = &mut entrants_data[current_index..current_index + 32 + 8 + 4];

        entrant_slice.copy_from_slice(&entrant.to_bytes());
        entrant_slice[32..].copy_from_slice(&shares.to_le_bytes());
        entrant_slice[32 + 8..].copy_from_slice(&self.total.to_le_bytes());
        self.total += 1;

        Ok(())
    }


    fn withdraw_entrant(
        &mut self,
        mut entrants_data: RefMut<&mut [u8]>,
        entrant: Pubkey, // 32
        shares: u128, // 8
        total: u32, // 4
    ) -> Result<()> {
        let current_index = Entrants::BASE_SIZE + ((32 + 8 + 4) * total) as usize;
        let entrant_slice: &mut [u8] = &mut entrants_data[current_index..current_index + 32 + 8 + 4];

        entrant_slice.copy_from_slice(&Pubkey::default().to_bytes());
        entrant_slice[32..].copy_from_slice(&u128::default().to_le_bytes());
        entrant_slice[32 + 8..].copy_from_slice(&u32::default().to_le_bytes());
        self.total -= 1;

        Ok(())
    }
}

#[error_code]
pub enum RaffleError {
    #[msg("Entrants account too small for max entrants")]
    EntrantsAccountTooSmallForMaxEntrants,
    #[msg("Raffle has ended")]
    RaffleEnded,
    #[msg("Invalid prize index")]
    InvalidPrizeIndex,
    #[msg("No prize")]
    NoPrize,
    #[msg("Invalid calculation")]
    InvalidCalculation,
    #[msg("Raffle is still running")]
    RaffleStillRunning,
    #[msg("Winner already drawn")]
    WinnersAlreadyDrawn,
    #[msg("Winner not drawn")]
    WinnerNotDrawn,
    #[msg("Invalid revealed data")]
    InvalidRevealedData,
    #[msg("Ticket account not owned by winner")]
    TokenAccountNotOwnedByWinner,
    #[msg("Ticket has not won")]
    TicketHasNotWon,
    #[msg("Unclaimed prizes")]
    UnclaimedPrizes,
    #[msg("Invalid recent blockhashes")]
    InvalidRecentBlockhashes,
    #[msg("Only the creator can calin no entrant raffle prizes")]
    OnlyCreatorCanClaimNoEntrantRafflePrizes,
    #[msg("Invalid treasury token account owner")]
    InvalidTreasuryTokenAccountOwner,
}
