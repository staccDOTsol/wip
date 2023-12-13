//! Instruction types

use crate::state::{LendingMarketMetadata, ReserveType};
use crate::{
    error::LendingError,
    state::{RateLimiterConfig, ReserveConfig, ReserveFees},
};
use bytemuck::bytes_of;

use num_traits::FromPrimitive;
use solana_program::system_program;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    sysvar,
};
use std::{convert::TryInto, mem::size_of};

/// Instructions supported by the lending program.
#[derive(Clone, Debug, PartialEq, Eq)]
// #[allow(clippy::large_enum_variant)]
pub enum LendingInstruction {
    // 0
    /// Initializes a new lending market.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Lending market account - uninitialized.
    ///   1. `[]` Rent sysvar.
    ///   2. `[]` Token program id.
    ///   3. `[]` Oracle program id.
    ///   4. `[]` Switchboard Oracle program id.
    InitLendingMarket {
        /// Owner authority which can add new reserves
        owner: Pubkey,
        /// Currency market prices are quoted in
        /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or SPL token mint pubkey
        quote_currency: [u8; 32],
    },

    // 1
    /// Sets the new owner of a lending market.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Lending market account.
    ///   1. `[signer]` Current owner.
    SetLendingMarketOwnerAndConfig {
        /// The new owner
        new_owner: Pubkey,
        /// The new config
        rate_limiter_config: RateLimiterConfig,
        /// whitelist liquidator
        whitelisted_liquidator: Option<Pubkey>,
        /// The risk authority
        risk_authority: Pubkey,
    },

    // 2
    /// Initializes a new lending market reserve.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account - uninitialized.
    ///   2. `[writable]` Reserve account - uninitialized.
    ///   3. `[]` Reserve liquidity SPL Token mint.
    ///   4. `[writable]` Reserve liquidity supply SPL Token account - uninitialized.
    ///   5. `[writable]` Reserve liquidity fee receiver - uninitialized.
    ///   6. `[writable]` Reserve collateral SPL Token mint - uninitialized.
    ///   7 `[writable]` Reserve collateral token supply - uninitialized.
    ///   8. `[]` Pyth product account.
    ///   9. `[]` Pyth price account.
    ///             This will be used as the reserve liquidity oracle account.
    ///   10. `[]` Switchboard price feed account. used as a backup oracle
    ///   11 `[]` Lending market account.
    ///   12 `[]` Derived lending market authority.
    ///   13 `[signer]` Lending market owner.
    ///   14 `[signer]` User transfer authority ($authority).
    ///   15 `[]` Clock sysvar (optional, will be removed soon).
    ///   16 `[]` Rent sysvar.
    ///   17 `[]` Token program id.
    InitReserve {
        /// Initial amount of liquidity to deposit into the new reserve
        liquidity_amount: u64,
        /// Reserve configuration values
        config: ReserveConfig,
    },

    // 3
    /// Accrue interest and update market price of liquidity on a reserve.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Reserve account.
    ///   1. `[]` Pyth Reserve liquidity oracle account.
    ///             Must be the Pyth price account specified at InitReserve.
    ///   2. `[]` Switchboard Reserve liquidity oracle account.
    ///             Must be the Switchboard price feed account specified at InitReserve.
    ///   3. `[]` Clock sysvar (optional, will be removed soon).
    RefreshReserve,

    // 4
    /// Deposit liquidity into a reserve in exchange for collateral. Collateral represents a share
    /// of the reserve liquidity pool.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account.
    ///   2. `[writable]` Reserve account.
    ///   3. `[writable]` Reserve liquidity supply SPL Token account.
    ///   4. `[writable]` Reserve collateral SPL Token mint.
    ///   5. `[]` Lending market account.
    ///   6. `[]` Derived lending market authority.
    ///   7. `[signer]` User transfer authority ($authority).
    ///   8. `[]` Clock sysvar (optional, will be removed soon).
    ///   9. `[]` Token program id.
    DepositReserveLiquidity {
        /// Amount of liquidity to deposit in exchange for collateral tokens
        liquidity_amount: u64,
    },

    // 5
    /// Redeem collateral from a reserve in exchange for liquidity.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source collateral token account.
    ///                     $authority can transfer $collateral_amount.
    ///   1. `[writable]` Destination liquidity token account.
    ///   2. `[writable]` Reserve account.
    ///   3. `[writable]` Reserve collateral SPL Token mint.
    ///   4. `[writable]` Reserve liquidity supply SPL Token account.
    ///   5. `[]` Lending market account.
    ///   6. `[]` Derived lending market authority.
    ///   7. `[signer]` User transfer authority ($authority).
    ///   8. `[]` Clock sysvar (optional, will be removed soon).
    ///   9. `[]` Token program id.
    RedeemReserveCollateral {
        /// Amount of collateral tokens to redeem in exchange for liquidity
        collateral_amount: u64,
    },

    // 6
    /// Initializes a new lending market obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation account - uninitialized.
    ///   1. `[]` Lending market account.
    ///   2. `[signer]` Obligation owner.
    ///   3. `[]` Clock sysvar (optional, will be removed soon).
    ///   4. `[]` Rent sysvar.
    ///   5. `[]` Token program id.
    InitObligation,

    // 7
    /// Refresh an obligation's accrued interest and collateral and liquidity prices. Requires
    /// refreshed reserves, as all obligation collateral deposit reserves in order, followed by all
    /// liquidity borrow reserves in order.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Obligation account.
    ///   1. `[]` Clock sysvar (optional, will be removed soon).
    ///   .. `[]` Collateral deposit reserve accounts - refreshed, all, in order.
    ///   .. `[]` Liquidity borrow reserve accounts - refreshed, all, in order.
    RefreshObligation,

    // 8
    /// Deposit collateral to an obligation.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source collateral token account.
    ///                     Minted by deposit reserve collateral mint.
    ///                     $authority can transfer $collateral_amount.
    ///   1. `[writable]` Destination deposit reserve collateral supply SPL Token account.
    ///   2. `[writable]` Deposit reserve account.
    ///   3. `[writable]` Obligation account.
    ///   4. `[]` Lending market account.
    ///   5. `[signer]` Obligation owner.
    ///   6. `[signer]` User transfer authority ($authority).
    ///   7. `[]` Clock sysvar (optional, will be removed soon).
    ///   8. `[]` Token program id.
    DepositObligationCollateral {
        /// Amount of collateral tokens to deposit
        collateral_amount: u64,
    },

    // 9
    /// Withdraw collateral from an obligation. Requires a refreshed obligation and reserve.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source withdraw reserve collateral supply SPL Token account.
    ///   1. `[writable]` Destination collateral token account.
    ///                     Minted by withdraw reserve collateral mint.
    ///   2. `[]` Withdraw reserve account - refreshed.
    ///   3. `[writable]` Obligation account - refreshed.
    ///   4. `[]` Lending market account.
    ///   5. `[]` Derived lending market authority.
    ///   6. `[signer]` Obligation owner.
    ///   7. `[]` Clock sysvar (optional, will be removed soon).
    ///   8. `[]` Token program id.
    WithdrawObligationCollateral {
        /// Amount of collateral tokens to withdraw - u64::MAX for up to 100% of deposited amount
        collateral_amount: u64,
    },

    // 10
    /// Borrow liquidity from a reserve by depositing collateral tokens. Requires a refreshed
    /// obligation and reserve.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source borrow reserve liquidity supply SPL Token account.
    ///   1. `[writable]` Destination liquidity token account.
    ///                     Minted by borrow reserve liquidity mint.
    ///   2. `[writable]` Borrow reserve account - refreshed.
    ///   3. `[writable]` Borrow reserve liquidity fee receiver account.
    ///                     Must be the fee account specified at InitReserve.
    ///   4. `[writable]` Obligation account - refreshed.
    ///   5. `[]` Lending market account.
    ///   6. `[]` Derived lending market authority.
    ///   7. `[signer]` Obligation owner.
    ///   8. `[]` Clock sysvar (optional, will be removed soon).
    ///   9. `[]` Token program id.
    ///   10 `[optional, writable]` Host fee receiver account.
    BorrowObligationLiquidity {
        /// Amount of liquidity to borrow - u64::MAX for 100% of borrowing power
        liquidity_amount: u64,
        // @TODO: slippage constraint - https://git.io/JmV67
    },

    // 11
    /// Repay borrowed liquidity to a reserve. Requires a refreshed obligation and reserve.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     Minted by repay reserve liquidity mint.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination repay reserve liquidity supply SPL Token account.
    ///   2. `[writable]` Repay reserve account - refreshed.
    ///   3. `[writable]` Obligation account - refreshed.
    ///   4. `[]` Lending market account.
    ///   5. `[signer]` User transfer authority ($authority).
    ///   6. `[]` Clock sysvar (optional, will be removed soon).
    ///   7. `[]` Token program id.
    RepayObligationLiquidity {
        /// Amount of liquidity to repay - u64::MAX for 100% of borrowed amount
        liquidity_amount: u64,
    },

    // 12
    /// Repay borrowed liquidity to a reserve to receive collateral at a discount from an unhealthy
    /// obligation. Requires a refreshed obligation and reserves.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     Minted by repay reserve liquidity mint.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account.
    ///                     Minted by withdraw reserve collateral mint.
    ///   2. `[writable]` Repay reserve account - refreshed.
    ///   3. `[writable]` Repay reserve liquidity supply SPL Token account.
    ///   4. `[]` Withdraw reserve account - refreshed.
    ///   5. `[writable]` Withdraw reserve collateral supply SPL Token account.
    ///   6. `[writable]` Obligation account - refreshed.
    ///   7. `[]` Lending market account.
    ///   8. `[]` Derived lending market authority.
    ///   9. `[signer]` User transfer authority ($authority).
    ///   10 `[]` Clock sysvar (optional, will be removed soon).
    ///   11 `[]` Token program id.
    LiquidateObligation {
        /// Amount of liquidity to repay - u64::MAX for up to 100% of borrowed amount
        liquidity_amount: u64,
    },

    // 13
    /// This instruction is now deprecated. Use FlashBorrowReserveLiquidity instead.
    /// Make a flash loan.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     Minted by reserve liquidity mint.
    ///                     Must match the reserve liquidity supply.
    ///   1. `[writable]` Destination liquidity token account.
    ///                     Minted by reserve liquidity mint.
    ///   2. `[writable]` Reserve account.
    ///   3. `[writable]` Flash loan fee receiver account.
    ///                     Must match the reserve liquidity fee receiver.
    ///   4. `[writable]` Host fee receiver.
    ///   5. `[]` Lending market account.
    ///   6. `[]` Derived lending market authority.
    ///   7. `[]` Token program id.
    ///   8. `[]` Flash loan receiver program id.
    ///             Must implement an instruction that has tag of 0 and a signature of `(amount: u64)`
    ///             This instruction must return the amount to the source liquidity account.
    ///   .. `[any]` Additional accounts expected by the receiving program's `ReceiveFlashLoan` instruction.
    ///
    ///   The flash loan receiver program that is to be invoked should contain an instruction with
    ///   tag `0` and accept the total amount (including fee) that needs to be returned back after
    ///   its execution has completed.
    ///
    ///   Flash loan receiver should have an instruction with the following signature:
    ///
    ///   0. `[writable]` Source liquidity (matching the destination from above).
    ///   1. `[writable]` Destination liquidity (matching the source from above).
    ///   2. `[]` Token program id
    ///   .. `[any]` Additional accounts provided to the lending program's `FlashLoan` instruction above.
    ///   ReceiveFlashLoan {
    ///       // Amount that must be repaid by the receiver program
    ///       amount: u64
    ///   }
    FlashLoan {
        /// The amount that is to be borrowed - u64::MAX for up to 100% of available liquidity
        amount: u64,
    },

    // 14
    /// Combines DepositReserveLiquidity and DepositObligationCollateral
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account.
    ///   2. `[writable]` Reserve account.
    ///   3. `[writable]` Reserve liquidity supply SPL Token account.
    ///   4. `[writable]` Reserve collateral SPL Token mint.
    ///   5. `[]` Lending market account.
    ///   6. `[]` Derived lending market authority.
    ///   7. `[writable]` Destination deposit reserve collateral supply SPL Token account.
    ///   8. `[writable]` Obligation account.
    ///   9. `[signer]` Obligation owner.
    ///   10 `[]` Pyth price oracle account.
    ///   11 `[]` Switchboard price feed oracle account.
    ///   12 `[signer]` User transfer authority ($authority).
    ///   13 `[]` Clock sysvar (optional, will be removed soon).
    ///   14 `[]` Token program id.
    DepositReserveLiquidityAndObligationCollateral {
        /// Amount of liquidity to deposit in exchange
        liquidity_amount: u64,
    },

    // 15
    /// Combines WithdrawObligationCollateral and RedeemReserveCollateral
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source withdraw reserve collateral supply SPL Token account.
    ///   1. `[writable]` Destination collateral token account.
    ///                     Minted by withdraw reserve collateral mint.
    ///   2. `[writable]` Withdraw reserve account - refreshed.
    ///   3. `[writable]` Obligation account - refreshed.
    ///   4. `[]` Lending market account.
    ///   5. `[]` Derived lending market authority.
    ///   6. `[writable]` User liquidity token account.
    ///   7. `[writable]` Reserve collateral SPL Token mint.
    ///   8. `[writable]` Reserve liquidity supply SPL Token account.
    ///   9. `[signer]` Obligation owner
    ///   10 `[signer]` User transfer authority ($authority).
    ///   11. `[]` Clock sysvar (optional, will be removed soon).
    ///   12. `[]` Token program id.
    WithdrawObligationCollateralAndRedeemReserveCollateral {
        /// liquidity_amount is the amount of collateral tokens to withdraw
        collateral_amount: u64,
    },

    // 16
    /// Updates a reserves config and a reserve price oracle pubkeys
    ///
    /// Accounts expected by this instruction:
    ///
    ///   1. `[writable]` Reserve account - refreshed
    ///   2 `[]` Lending market account.
    ///   3 `[]` Derived lending market authority.
    ///   4 `[signer]` Lending market owner.
    ///   5 `[]` Pyth product key.
    ///   6 `[]` Pyth price key.
    ///   7 `[]` Switchboard key.
    UpdateReserveConfig {
        /// Reserve config to update to
        config: ReserveConfig,
        /// Rate limiter config
        rate_limiter_config: RateLimiterConfig,
    },

    // 17
    /// Repay borrowed liquidity to a reserve to receive collateral at a discount from an unhealthy
    /// obligation. Requires a refreshed obligation and reserves.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     Minted by repay reserve liquidity mint.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination collateral token account.
    ///                     Minted by withdraw reserve collateral mint.
    ///   2. `[writable]` Destination liquidity token account.
    ///   3. `[writable]` Repay reserve account - refreshed.
    ///   4. `[writable]` Repay reserve liquidity supply SPL Token account.
    ///   5. `[writable]` Withdraw reserve account - refreshed.
    ///   6. `[writable]` Withdraw reserve collateral SPL Token mint.
    ///   7. `[writable]` Withdraw reserve collateral supply SPL Token account.
    ///   8. `[writable]` Withdraw reserve liquidity supply SPL Token account.
    ///   9. `[writable]` Withdraw reserve liquidity fee receiver account.
    ///   10 `[writable]` Obligation account - refreshed.
    ///   11 `[]` Lending market account.
    ///   12 `[]` Derived lending market authority.
    ///   13 `[signer]` User transfer authority ($authority).
    ///   14 `[]` Token program id.
    LiquidateObligationAndRedeemReserveCollateral {
        /// Amount of liquidity to repay - u64::MAX for up to 100% of borrowed amount
        liquidity_amount: u64,
    },

    // 18
    ///   0. `[writable]` Reserve account.
    ///   1. `[writable]` Borrow reserve liquidity fee receiver account.
    ///                     Must be the fee account specified at InitReserve.
    ///   2. `[writable]` Reserve liquidity supply SPL Token account.
    ///   3. `[]` Lending market account.
    ///   4. `[]` Derived lending market authority.
    ///   5. `[]` Token program id.
    RedeemFees,

    // 19
    /// Flash borrow reserve liquidity
    //
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///   1. `[writable]` Destination liquidity token account.
    ///   2. `[writable]` Reserve account.
    ///   3. `[]` Lending market account.
    ///   4. `[]` Derived lending market authority.
    ///   5. `[]` Instructions sysvar.
    ///   6. `[]` Token program id.
    ///   7. `[]` Clock sysvar (optional, will be removed soon).
    FlashBorrowReserveLiquidity {
        /// Amount of liquidity to flash borrow
        liquidity_amount: u64,
    },

    // 20
    /// Flash repay reserve liquidity
    //
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Source liquidity token account.
    ///                     $authority can transfer $liquidity_amount.
    ///   1. `[writable]` Destination liquidity token account.
    ///   2. `[writable]` Flash loan fee receiver account.
    ///                     Must match the reserve liquidity fee receiver.
    ///   3. `[writable]` Host fee receiver.
    ///   4. `[writable]` Reserve account.
    ///   5. `[]` Lending market account.
    ///   6. `[signer]` User transfer authority ($authority).
    ///   7. `[]` Instructions sysvar.
    ///   8. `[]` Token program id.
    FlashRepayReserveLiquidity {
        /// Amount of liquidity to flash repay
        liquidity_amount: u64,
        /// Index of FlashBorrowReserveLiquidity instruction
        borrow_instruction_index: u8,
    },

    // 21
    /// Forgive Debt
    ///
    /// Accounts expected by this instruction:
    ///  0. `[writable]` Obligation account - refreshed.
    ///  1. `[writable]` Reserve account - refreshed.
    ///  2. `[]` Lending Market account.
    ///  3. `[signer]` Lending Market owner.
    ForgiveDebt {
        /// Amount of debt to forgive
        liquidity_amount: u64,
    },

    // 22
    /// UpdateMarketMetadata
    ///
    /// Accounts expected by this instruction:
    /// 0. `[]` Lending market account.
    /// 1. `[signer]` Lending market owner.
    /// 2. `[writable]` Lending market metadata account.
    /// Must be a pda with seeds [lending_market, "MetaData"]
    /// 3. `[]` System program
    UpdateMarketMetadata,
}

impl LendingInstruction {
    /// Unpacks a byte buffer into a [LendingInstruction](enum.LendingInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(LendingError::InstructionUnpackError)?;
        Ok(match tag {
            0 => {
                let (owner, rest) = Self::unpack_pubkey(rest)?;
                let (quote_currency, _rest) = Self::unpack_bytes32(rest)?;
                Self::InitLendingMarket {
                    owner,
                    quote_currency: *quote_currency,
                }
            }
            1 => {
                let (new_owner, rest) = Self::unpack_pubkey(rest)?;
                let (window_duration, rest) = Self::unpack_u64(rest)?;
                let (max_outflow, rest) = Self::unpack_u64(rest)?;
                let (whitelisted_liquidator, rest) = match Self::unpack_u8(rest)? {
                    (0, rest) => (None, rest),
                    (1, rest) => {
                        let (pubkey, rest) = Self::unpack_pubkey(rest)?;
                        (Some(pubkey), rest)
                    }
                    _ => return Err(LendingError::InstructionUnpackError.into()),
                };

                let (risk_authority, _rest) = Self::unpack_pubkey(rest)?;
                Self::SetLendingMarketOwnerAndConfig {
                    new_owner,
                    rate_limiter_config: RateLimiterConfig {
                        window_duration,
                        max_outflow,
                    },
                    whitelisted_liquidator,
                    risk_authority,
                }
            }
            2 => {
                let (liquidity_amount, rest) = Self::unpack_u64(rest)?;
                let (optimal_utilization_rate, rest) = Self::unpack_u8(rest)?;
                let (max_utilization_rate, rest) = Self::unpack_u8(rest)?;
                let (loan_to_value_ratio, rest) = Self::unpack_u8(rest)?;
                let (liquidation_bonus, rest) = Self::unpack_u8(rest)?;
                let (liquidation_threshold, rest) = Self::unpack_u8(rest)?;
                let (min_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (optimal_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (max_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (super_max_borrow_rate, rest) = Self::unpack_u64(rest)?;
                let (borrow_fee_wad, rest) = Self::unpack_u64(rest)?;
                let (flash_loan_fee_wad, rest) = Self::unpack_u64(rest)?;
                let (host_fee_percentage, rest) = Self::unpack_u8(rest)?;
                let (deposit_limit, rest) = Self::unpack_u64(rest)?;
                let (borrow_limit, rest) = Self::unpack_u64(rest)?;
                let (fee_receiver, rest) = Self::unpack_pubkey(rest)?;
                let (protocol_liquidation_fee, rest) = Self::unpack_u8(rest)?;
                let (protocol_take_rate, rest) = Self::unpack_u8(rest)?;
                let (added_borrow_weight_bps, rest) = Self::unpack_u64(rest)?;
                let (asset_type, rest) = Self::unpack_u8(rest)?;
                let (max_liquidation_bonus, rest) = Self::unpack_u8(rest)?;
                let (max_liquidation_threshold, _rest) = Self::unpack_u8(rest)?;
                Self::InitReserve {
                    liquidity_amount,
                    config: ReserveConfig {
                        optimal_utilization_rate,
                        max_utilization_rate,
                        loan_to_value_ratio,
                        liquidation_bonus,
                        max_liquidation_bonus,
                        liquidation_threshold,
                        max_liquidation_threshold,
                        min_borrow_rate,
                        optimal_borrow_rate,
                        max_borrow_rate,
                        super_max_borrow_rate,
                        fees: ReserveFees {
                            borrow_fee_wad,
                            flash_loan_fee_wad,
                            host_fee_percentage,
                        },
                        deposit_limit,
                        borrow_limit,
                        fee_receiver,
                        protocol_liquidation_fee,
                        protocol_take_rate,
                        added_borrow_weight_bps,
                        reserve_type: ReserveType::from_u8(asset_type).unwrap(),
                    },
                }
            }
            3 => Self::RefreshReserve,
            4 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::DepositReserveLiquidity { liquidity_amount }
            }
            5 => {
                let (collateral_amount, _rest) = Self::unpack_u64(rest)?;
                Self::RedeemReserveCollateral { collateral_amount }
            }
            6 => Self::InitObligation,
            7 => Self::RefreshObligation,
            8 => {
                let (collateral_amount, _rest) = Self::unpack_u64(rest)?;
                Self::DepositObligationCollateral { collateral_amount }
            }
            9 => {
                let (collateral_amount, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawObligationCollateral { collateral_amount }
            }
            10 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::BorrowObligationLiquidity { liquidity_amount }
            }
            11 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::RepayObligationLiquidity { liquidity_amount }
            }
            12 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::LiquidateObligation { liquidity_amount }
            }
            13 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::FlashLoan { amount }
            }
            14 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::DepositReserveLiquidityAndObligationCollateral { liquidity_amount }
            }
            15 => {
                let (collateral_amount, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawObligationCollateralAndRedeemReserveCollateral { collateral_amount }
            }
            16 => {
                let (optimal_utilization_rate, rest) = Self::unpack_u8(rest)?;
                let (max_utilization_rate, rest) = Self::unpack_u8(rest)?;
                let (loan_to_value_ratio, rest) = Self::unpack_u8(rest)?;
                let (liquidation_bonus, rest) = Self::unpack_u8(rest)?;
                let (liquidation_threshold, rest) = Self::unpack_u8(rest)?;
                let (min_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (optimal_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (max_borrow_rate, rest) = Self::unpack_u8(rest)?;
                let (super_max_borrow_rate, rest) = Self::unpack_u64(rest)?;
                let (borrow_fee_wad, rest) = Self::unpack_u64(rest)?;
                let (flash_loan_fee_wad, rest) = Self::unpack_u64(rest)?;
                let (host_fee_percentage, rest) = Self::unpack_u8(rest)?;
                let (deposit_limit, rest) = Self::unpack_u64(rest)?;
                let (borrow_limit, rest) = Self::unpack_u64(rest)?;
                let (fee_receiver, rest) = Self::unpack_pubkey(rest)?;
                let (protocol_liquidation_fee, rest) = Self::unpack_u8(rest)?;
                let (protocol_take_rate, rest) = Self::unpack_u8(rest)?;
                let (added_borrow_weight_bps, rest) = Self::unpack_u64(rest)?;
                let (asset_type, rest) = Self::unpack_u8(rest)?;
                let (max_liquidation_bonus, rest) = Self::unpack_u8(rest)?;
                let (max_liquidation_threshold, rest) = Self::unpack_u8(rest)?;
                let (window_duration, rest) = Self::unpack_u64(rest)?;
                let (max_outflow, _rest) = Self::unpack_u64(rest)?;

                Self::UpdateReserveConfig {
                    config: ReserveConfig {
                        optimal_utilization_rate,
                        max_utilization_rate,
                        loan_to_value_ratio,
                        liquidation_bonus,
                        max_liquidation_bonus,
                        liquidation_threshold,
                        max_liquidation_threshold,
                        min_borrow_rate,
                        optimal_borrow_rate,
                        max_borrow_rate,
                        super_max_borrow_rate,
                        fees: ReserveFees {
                            borrow_fee_wad,
                            flash_loan_fee_wad,
                            host_fee_percentage,
                        },
                        deposit_limit,
                        borrow_limit,
                        fee_receiver,
                        protocol_liquidation_fee,
                        protocol_take_rate,
                        added_borrow_weight_bps,
                        reserve_type: ReserveType::from_u8(asset_type).unwrap(),
                    },
                    rate_limiter_config: RateLimiterConfig {
                        window_duration,
                        max_outflow,
                    },
                }
            }
            17 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::LiquidateObligationAndRedeemReserveCollateral { liquidity_amount }
            }
            18 => Self::RedeemFees,
            19 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::FlashBorrowReserveLiquidity { liquidity_amount }
            }
            20 => {
                let (liquidity_amount, rest) = Self::unpack_u64(rest)?;
                let (borrow_instruction_index, _rest) = Self::unpack_u8(rest)?;
                Self::FlashRepayReserveLiquidity {
                    liquidity_amount,
                    borrow_instruction_index,
                }
            }
            21 => {
                let (liquidity_amount, _rest) = Self::unpack_u64(rest)?;
                Self::ForgiveDebt { liquidity_amount }
            }
            22 => Self::UpdateMarketMetadata,
            _ => {
                msg!("Instruction cannot be unpacked");
                return Err(LendingError::InstructionUnpackError.into());
            }
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < 8 {
            msg!("u64 cannot be unpacked");
            return Err(LendingError::InstructionUnpackError.into());
        }
        let (bytes, rest) = input.split_at(8);
        let value = bytes
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(LendingError::InstructionUnpackError)?;
        Ok((value, rest))
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.is_empty() {
            msg!("u8 cannot be unpacked");
            return Err(LendingError::InstructionUnpackError.into());
        }
        let (bytes, rest) = input.split_at(1);
        let value = bytes
            .get(..1)
            .and_then(|slice| slice.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(LendingError::InstructionUnpackError)?;
        Ok((value, rest))
    }

    fn unpack_bytes32(input: &[u8]) -> Result<(&[u8; 32], &[u8]), ProgramError> {
        if input.len() < 32 {
            msg!("32 bytes cannot be unpacked");
            return Err(LendingError::InstructionUnpackError.into());
        }
        let (bytes, rest) = input.split_at(32);
        Ok((
            bytes
                .try_into()
                .map_err(|_| LendingError::InstructionUnpackError)?,
            rest,
        ))
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < PUBKEY_BYTES {
            msg!("Pubkey cannot be unpacked");
            return Err(LendingError::InstructionUnpackError.into());
        }
        let (key, rest) = input.split_at(PUBKEY_BYTES);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    }

    /// Packs a [LendingInstruction](enum.LendingInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match *self {
            Self::InitLendingMarket {
                owner,
                quote_currency,
            } => {
                buf.push(0);
                buf.extend_from_slice(owner.as_ref());
                buf.extend_from_slice(quote_currency.as_ref());
            }
            Self::SetLendingMarketOwnerAndConfig {
                new_owner,
                rate_limiter_config: config,
                whitelisted_liquidator,
                risk_authority,
            } => {
                buf.push(1);
                buf.extend_from_slice(new_owner.as_ref());
                buf.extend_from_slice(&config.window_duration.to_le_bytes());
                buf.extend_from_slice(&config.max_outflow.to_le_bytes());
                match whitelisted_liquidator {
                    Some(liquidator) => {
                        buf.push(1);
                        buf.extend_from_slice(liquidator.as_ref());
                    }
                    None => {
                        buf.push(0);
                    }
                };
                buf.extend_from_slice(risk_authority.as_ref());
            }
            Self::InitReserve {
                liquidity_amount,
                config:
                    ReserveConfig {
                        optimal_utilization_rate,
                        max_utilization_rate,
                        loan_to_value_ratio,
                        liquidation_bonus,
                        max_liquidation_bonus,
                        liquidation_threshold,
                        max_liquidation_threshold,
                        min_borrow_rate,
                        optimal_borrow_rate,
                        max_borrow_rate,
                        super_max_borrow_rate,
                        fees:
                            ReserveFees {
                                borrow_fee_wad,
                                flash_loan_fee_wad,
                                host_fee_percentage,
                            },
                        deposit_limit,
                        borrow_limit,
                        fee_receiver,
                        protocol_liquidation_fee,
                        protocol_take_rate,
                        added_borrow_weight_bps: borrow_weight_bps,
                        reserve_type: asset_type,
                    },
            } => {
                buf.push(2);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&optimal_utilization_rate.to_le_bytes());
                buf.extend_from_slice(&max_utilization_rate.to_le_bytes());
                buf.extend_from_slice(&loan_to_value_ratio.to_le_bytes());
                buf.extend_from_slice(&liquidation_bonus.to_le_bytes());
                buf.extend_from_slice(&liquidation_threshold.to_le_bytes());
                buf.extend_from_slice(&min_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&optimal_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&max_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&super_max_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&borrow_fee_wad.to_le_bytes());
                buf.extend_from_slice(&flash_loan_fee_wad.to_le_bytes());
                buf.extend_from_slice(&host_fee_percentage.to_le_bytes());
                buf.extend_from_slice(&deposit_limit.to_le_bytes());
                buf.extend_from_slice(&borrow_limit.to_le_bytes());
                buf.extend_from_slice(&fee_receiver.to_bytes());
                buf.extend_from_slice(&protocol_liquidation_fee.to_le_bytes());
                buf.extend_from_slice(&protocol_take_rate.to_le_bytes());
                buf.extend_from_slice(&borrow_weight_bps.to_le_bytes());
                buf.extend_from_slice(&(asset_type as u8).to_le_bytes());
                buf.extend_from_slice(&max_liquidation_bonus.to_le_bytes());
                buf.extend_from_slice(&max_liquidation_threshold.to_le_bytes());
            }
            Self::RefreshReserve => {
                buf.push(3);
            }
            Self::DepositReserveLiquidity { liquidity_amount } => {
                buf.push(4);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::RedeemReserveCollateral { collateral_amount } => {
                buf.push(5);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::InitObligation => {
                buf.push(6);
            }
            Self::RefreshObligation => {
                buf.push(7);
            }
            Self::DepositObligationCollateral { collateral_amount } => {
                buf.push(8);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::WithdrawObligationCollateral { collateral_amount } => {
                buf.push(9);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::BorrowObligationLiquidity { liquidity_amount } => {
                buf.push(10);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::RepayObligationLiquidity { liquidity_amount } => {
                buf.push(11);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::LiquidateObligation { liquidity_amount } => {
                buf.push(12);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::FlashLoan { amount } => {
                buf.push(13);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::DepositReserveLiquidityAndObligationCollateral { liquidity_amount } => {
                buf.push(14);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::WithdrawObligationCollateralAndRedeemReserveCollateral { collateral_amount } => {
                buf.push(15);
                buf.extend_from_slice(&collateral_amount.to_le_bytes());
            }
            Self::UpdateReserveConfig {
                config,
                rate_limiter_config,
            } => {
                buf.push(16);
                buf.extend_from_slice(&config.optimal_utilization_rate.to_le_bytes());
                buf.extend_from_slice(&config.max_utilization_rate.to_le_bytes());
                buf.extend_from_slice(&config.loan_to_value_ratio.to_le_bytes());
                buf.extend_from_slice(&config.liquidation_bonus.to_le_bytes());
                buf.extend_from_slice(&config.liquidation_threshold.to_le_bytes());
                buf.extend_from_slice(&config.min_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&config.optimal_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&config.max_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&config.super_max_borrow_rate.to_le_bytes());
                buf.extend_from_slice(&config.fees.borrow_fee_wad.to_le_bytes());
                buf.extend_from_slice(&config.fees.flash_loan_fee_wad.to_le_bytes());
                buf.extend_from_slice(&config.fees.host_fee_percentage.to_le_bytes());
                buf.extend_from_slice(&config.deposit_limit.to_le_bytes());
                buf.extend_from_slice(&config.borrow_limit.to_le_bytes());
                buf.extend_from_slice(&config.fee_receiver.to_bytes());
                buf.extend_from_slice(&config.protocol_liquidation_fee.to_le_bytes());
                buf.extend_from_slice(&config.protocol_take_rate.to_le_bytes());
                buf.extend_from_slice(&config.added_borrow_weight_bps.to_le_bytes());
                buf.extend_from_slice(&(config.reserve_type as u8).to_le_bytes());
                buf.extend_from_slice(&config.max_liquidation_bonus.to_le_bytes());
                buf.extend_from_slice(&config.max_liquidation_threshold.to_le_bytes());
                buf.extend_from_slice(&rate_limiter_config.window_duration.to_le_bytes());
                buf.extend_from_slice(&rate_limiter_config.max_outflow.to_le_bytes());
            }
            Self::LiquidateObligationAndRedeemReserveCollateral { liquidity_amount } => {
                buf.push(17);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::RedeemFees {} => {
                buf.push(18);
            }
            Self::FlashBorrowReserveLiquidity { liquidity_amount } => {
                buf.push(19);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            Self::FlashRepayReserveLiquidity {
                liquidity_amount,
                borrow_instruction_index,
            } => {
                buf.push(20);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&borrow_instruction_index.to_le_bytes());
            }
            Self::ForgiveDebt { liquidity_amount } => {
                buf.push(21);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
            }
            // special handling for this instruction, bc the instruction is too big to deserialize
            Self::UpdateMarketMetadata => {}
        }
        buf
    }
}

/// Creates an 'InitLendingMarket' instruction.
pub fn init_lending_market(
    program_id: Pubkey,
    owner: Pubkey,
    quote_currency: [u8; 32],
    lending_market_pubkey: Pubkey,
    oracle_program_id: Pubkey,
    switchboard_oracle_program_id: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(lending_market_pubkey, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(oracle_program_id, false),
            AccountMeta::new_readonly(switchboard_oracle_program_id, false),
        ],
        data: LendingInstruction::InitLendingMarket {
            owner,
            quote_currency,
        }
        .pack(),
    }
}

/// Creates a 'SetLendingMarketOwner' instruction.
pub fn set_lending_market_owner_and_config(
    program_id: Pubkey,
    lending_market_pubkey: Pubkey,
    lending_market_owner: Pubkey,
    new_owner: Pubkey,
    rate_limiter_config: RateLimiterConfig,
    whitelisted_liquidator: Option<Pubkey>,
    risk_authority: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_owner, true),
        ],
        data: LendingInstruction::SetLendingMarketOwnerAndConfig {
            new_owner,
            rate_limiter_config,
            whitelisted_liquidator,
            risk_authority,
        }
        .pack(),
    }
}

/// Creates an 'InitReserve' instruction.
#[allow(clippy::too_many_arguments)]
pub fn init_reserve(
    program_id: Pubkey,
    liquidity_amount: u64,
    config: ReserveConfig,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_mint_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    reserve_collateral_supply_pubkey: Pubkey,
    pyth_product_pubkey: Pubkey,
    pyth_price_pubkey: Pubkey,
    switchboard_feed_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    lending_market_owner_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    let accounts = vec![
        AccountMeta::new(source_liquidity_pubkey, false),
        AccountMeta::new(destination_collateral_pubkey, false),
        AccountMeta::new(reserve_pubkey, false),
        AccountMeta::new_readonly(reserve_liquidity_mint_pubkey, false),
        AccountMeta::new(reserve_liquidity_supply_pubkey, false),
        AccountMeta::new(config.fee_receiver, false),
        AccountMeta::new(reserve_collateral_mint_pubkey, false),
        AccountMeta::new(reserve_collateral_supply_pubkey, false),
        AccountMeta::new_readonly(pyth_product_pubkey, false),
        AccountMeta::new_readonly(pyth_price_pubkey, false),
        AccountMeta::new_readonly(switchboard_feed_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new_readonly(lending_market_owner_pubkey, true),
        AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::InitReserve {
            liquidity_amount,
            config,
        }
        .pack(),
    }
}

/// Creates a `RefreshReserve` instruction
pub fn refresh_reserve(
    program_id: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_pyth_oracle_pubkey: Pubkey,
    reserve_liquidity_switchboard_oracle_pubkey: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(reserve_pubkey, false),
        AccountMeta::new_readonly(reserve_liquidity_pyth_oracle_pubkey, false),
        AccountMeta::new_readonly(reserve_liquidity_switchboard_oracle_pubkey, false),
    ];
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::RefreshReserve.pack(),
    }
}

/// Creates a 'DepositReserveLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn deposit_reserve_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::DepositReserveLiquidity { liquidity_amount }.pack(),
    }
}

/// Creates a 'RedeemReserveCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn redeem_reserve_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    source_collateral_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_collateral_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::RedeemReserveCollateral { collateral_amount }.pack(),
    }
}

/// Creates an 'InitObligation' instruction.
#[allow(clippy::too_many_arguments)]
pub fn init_obligation(
    program_id: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    obligation_owner_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(obligation_owner_pubkey, true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::InitObligation.pack(),
    }
}

/// Creates a 'RefreshObligation' instruction.
#[allow(clippy::too_many_arguments)]
pub fn refresh_obligation(
    program_id: Pubkey,
    obligation_pubkey: Pubkey,
    reserve_pubkeys: Vec<Pubkey>,
) -> Instruction {
    let mut accounts = vec![AccountMeta::new(obligation_pubkey, false)];
    accounts.extend(
        reserve_pubkeys
            .into_iter()
            .map(|pubkey| AccountMeta::new_readonly(pubkey, false)),
    );
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::RefreshObligation.pack(),
    }
}

/// Creates a 'DepositObligationCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn deposit_obligation_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    source_collateral_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    deposit_reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    obligation_owner_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_collateral_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(deposit_reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(obligation_owner_pubkey, true),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::DepositObligationCollateral { collateral_amount }.pack(),
    }
}

/// Creates a 'DepositReserveLiquidityAndObligationCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn deposit_reserve_liquidity_and_obligation_collateral(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    user_collateral_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    destination_deposit_collateral_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    obligation_owner_pubkey: Pubkey,
    reserve_liquidity_pyth_oracle_pubkey: Pubkey,
    reserve_liquidity_switchboard_oracle_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(user_collateral_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new(destination_deposit_collateral_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(obligation_owner_pubkey, true),
            AccountMeta::new_readonly(reserve_liquidity_pyth_oracle_pubkey, false),
            AccountMeta::new_readonly(reserve_liquidity_switchboard_oracle_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::DepositReserveLiquidityAndObligationCollateral {
            liquidity_amount,
        }
        .pack(),
    }
}

/// Creates a 'WithdrawObligationCollateralAndRedeemReserveCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    source_collateral_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    obligation_owner_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_collateral_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(withdraw_reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new_readonly(obligation_owner_pubkey, true),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral {
            collateral_amount,
        }
        .pack(),
    }
}

/// Creates a 'WithdrawObligationCollateral' instruction.
#[allow(clippy::too_many_arguments)]
pub fn withdraw_obligation_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    source_collateral_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    obligation_owner_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_collateral_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new_readonly(withdraw_reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(obligation_owner_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::WithdrawObligationCollateral { collateral_amount }.pack(),
    }
}

/// Creates a 'BorrowObligationLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn borrow_obligation_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    borrow_reserve_pubkey: Pubkey,
    borrow_reserve_liquidity_fee_receiver_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    obligation_owner_pubkey: Pubkey,
    host_fee_receiver_pubkey: Option<Pubkey>,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    let mut accounts = vec![
        AccountMeta::new(source_liquidity_pubkey, false),
        AccountMeta::new(destination_liquidity_pubkey, false),
        AccountMeta::new(borrow_reserve_pubkey, false),
        AccountMeta::new(borrow_reserve_liquidity_fee_receiver_pubkey, false),
        AccountMeta::new(obligation_pubkey, false),
        AccountMeta::new(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new_readonly(obligation_owner_pubkey, true),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    if let Some(host_fee_receiver_pubkey) = host_fee_receiver_pubkey {
        accounts.push(AccountMeta::new(host_fee_receiver_pubkey, false));
    }
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::BorrowObligationLiquidity { liquidity_amount }.pack(),
    }
}

/// Creates a `RepayObligationLiquidity` instruction
#[allow(clippy::too_many_arguments)]
pub fn repay_obligation_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    repay_reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(repay_reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::RepayObligationLiquidity { liquidity_amount }.pack(),
    }
}

/// Creates a `LiquidateObligation` instruction
#[allow(clippy::too_many_arguments)]
pub fn liquidate_obligation(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    repay_reserve_pubkey: Pubkey,
    repay_reserve_liquidity_supply_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    withdraw_reserve_collateral_supply_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(repay_reserve_pubkey, false),
            AccountMeta::new(repay_reserve_liquidity_supply_pubkey, false),
            AccountMeta::new_readonly(withdraw_reserve_pubkey, false),
            AccountMeta::new(withdraw_reserve_collateral_supply_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::LiquidateObligation { liquidity_amount }.pack(),
    }
}

/// Creates an 'UpdateReserveConfig' instruction.
#[allow(clippy::too_many_arguments)]
pub fn update_reserve_config(
    program_id: Pubkey,
    config: ReserveConfig,
    rate_limiter_config: RateLimiterConfig,
    reserve_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    lending_market_owner_pubkey: Pubkey,
    pyth_product_pubkey: Pubkey,
    pyth_price_pubkey: Pubkey,
    switchboard_feed_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    let accounts = vec![
        AccountMeta::new(reserve_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new_readonly(lending_market_owner_pubkey, true),
        AccountMeta::new_readonly(pyth_product_pubkey, false),
        AccountMeta::new_readonly(pyth_price_pubkey, false),
        AccountMeta::new_readonly(switchboard_feed_pubkey, false),
    ];
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::UpdateReserveConfig {
            config,
            rate_limiter_config,
        }
        .pack(),
    }
}

/// Creates a `LiquidateObligationAndRedeemReserveCollateral` instruction
#[allow(clippy::too_many_arguments)]
pub fn liquidate_obligation_and_redeem_reserve_collateral(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_collateral_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    repay_reserve_pubkey: Pubkey,
    repay_reserve_liquidity_supply_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    withdraw_reserve_collateral_mint_pubkey: Pubkey,
    withdraw_reserve_collateral_supply_pubkey: Pubkey,
    withdraw_reserve_liquidity_supply_pubkey: Pubkey,
    withdraw_reserve_liquidity_fee_receiver_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_collateral_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(repay_reserve_pubkey, false),
            AccountMeta::new(repay_reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(withdraw_reserve_pubkey, false),
            AccountMeta::new(withdraw_reserve_collateral_mint_pubkey, false),
            AccountMeta::new(withdraw_reserve_collateral_supply_pubkey, false),
            AccountMeta::new(withdraw_reserve_liquidity_supply_pubkey, false),
            AccountMeta::new(withdraw_reserve_liquidity_fee_receiver_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::LiquidateObligationAndRedeemReserveCollateral {
            liquidity_amount,
        }
        .pack(),
    }
}

/// Creates a `RedeemFees` instruction
pub fn redeem_fees(
    program_id: Pubkey,
    reserve_pubkey: Pubkey,
    reserve_liquidity_fee_receiver_pubkey: Pubkey,
    reserve_supply_liquidity_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );
    let accounts = vec![
        AccountMeta::new(reserve_pubkey, false),
        AccountMeta::new(reserve_liquidity_fee_receiver_pubkey, false),
        AccountMeta::new(reserve_supply_liquidity_pubkey, false),
        AccountMeta::new_readonly(lending_market_pubkey, false),
        AccountMeta::new_readonly(lending_market_authority_pubkey, false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];
    Instruction {
        program_id,
        accounts,
        data: LendingInstruction::RedeemFees.pack(),
    }
}

/// Creates a 'FlashBorrowReserveLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn flash_borrow_reserve_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    source_liquidity_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::FlashBorrowReserveLiquidity { liquidity_amount }.pack(),
    }
}

/// Creates a 'FlashRepayReserveLiquidity' instruction.
#[allow(clippy::too_many_arguments)]
pub fn flash_repay_reserve_liquidity(
    program_id: Pubkey,
    liquidity_amount: u64,
    borrow_instruction_index: u8,
    source_liquidity_pubkey: Pubkey,
    destination_liquidity_pubkey: Pubkey,
    reserve_liquidity_fee_receiver_pubkey: Pubkey,
    host_fee_receiver_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source_liquidity_pubkey, false),
            AccountMeta::new(destination_liquidity_pubkey, false),
            AccountMeta::new(reserve_liquidity_fee_receiver_pubkey, false),
            AccountMeta::new(host_fee_receiver_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::FlashRepayReserveLiquidity {
            liquidity_amount,
            borrow_instruction_index,
        }
        .pack(),
    }
}

/// Creates a `ForgiveDebt` instruction
pub fn forgive_debt(
    program_id: Pubkey,
    liquidity_amount: u64,
    reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    lending_market_owner: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_owner, true),
        ],
        data: LendingInstruction::ForgiveDebt { liquidity_amount }.pack(),
    }
}

/// Creates a `UpdateMarketMetadata` instruction
pub fn update_market_metadata(
    program_id: Pubkey,
    mut metadata: LendingMarketMetadata,
    lending_market_pubkey: Pubkey,
    lending_market_owner: Pubkey,
) -> Instruction {
    let (lending_market_metadata_pubkey, bump_seed) = Pubkey::find_program_address(
        &[
            &lending_market_pubkey.to_bytes()[..PUBKEY_BYTES],
            b"MetaData",
        ],
        &program_id,
    );

    metadata.bump_seed = bump_seed;

    let mut data = [0u8; 1 + std::mem::size_of::<LendingMarketMetadata>()];
    data[0] = 22;
    data[1..].copy_from_slice(bytes_of(&metadata));

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new(lending_market_owner, true),
            AccountMeta::new(lending_market_metadata_pubkey, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: data.to_vec(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn pack_and_unpack_instructions() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            {
                let instruction = LendingInstruction::InitLendingMarket {
                    owner: Pubkey::new_unique(),
                    quote_currency: [rng.gen::<u8>(); 32],
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // set lending market owner and config
            {
                let instruction = LendingInstruction::SetLendingMarketOwnerAndConfig {
                    new_owner: Pubkey::new_unique(),
                    rate_limiter_config: RateLimiterConfig {
                        window_duration: rng.gen::<u64>(),
                        max_outflow: rng.gen::<u64>(),
                    },
                    whitelisted_liquidator: if rng.gen_bool(0.5) {
                        None
                    } else {
                        Some(Pubkey::new_unique())
                    },
                    risk_authority: Pubkey::new_unique(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            {
                let instruction = LendingInstruction::InitReserve {
                    liquidity_amount: rng.gen::<u64>(),
                    config: ReserveConfig {
                        optimal_utilization_rate: rng.gen::<u8>(),
                        max_utilization_rate: rng.gen::<u8>(),
                        loan_to_value_ratio: rng.gen::<u8>(),
                        liquidation_bonus: rng.gen::<u8>(),
                        max_liquidation_bonus: rng.gen::<u8>(),
                        liquidation_threshold: rng.gen::<u8>(),
                        max_liquidation_threshold: rng.gen::<u8>(),
                        min_borrow_rate: rng.gen::<u8>(),
                        optimal_borrow_rate: rng.gen::<u8>(),
                        max_borrow_rate: rng.gen::<u8>(),
                        super_max_borrow_rate: rng.gen::<u64>(),
                        fees: ReserveFees {
                            borrow_fee_wad: rng.gen::<u64>(),
                            flash_loan_fee_wad: rng.gen::<u64>(),
                            host_fee_percentage: rng.gen::<u8>(),
                        },
                        deposit_limit: rng.gen::<u64>(),
                        borrow_limit: rng.gen::<u64>(),
                        fee_receiver: Pubkey::new_unique(),
                        protocol_liquidation_fee: rng.gen::<u8>(),
                        protocol_take_rate: rng.gen::<u8>(),
                        added_borrow_weight_bps: rng.gen::<u64>(),
                        reserve_type: ReserveType::from_u8(rng.gen::<u8>() % 2).unwrap(),
                    },
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // refresh reserve
            {
                let instruction = LendingInstruction::RefreshReserve;
                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // deposit reserve liquidity
            {
                let instruction = LendingInstruction::DepositReserveLiquidity {
                    liquidity_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // redeem reserve collateral
            {
                let instruction = LendingInstruction::RedeemReserveCollateral {
                    collateral_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // init obligation
            {
                let instruction = LendingInstruction::InitObligation;
                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // refresh obligation
            {
                let instruction = LendingInstruction::RefreshObligation;
                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // deposit obligation collateral
            {
                let instruction = LendingInstruction::DepositObligationCollateral {
                    collateral_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // borrow obligation liquidity
            {
                let instruction = LendingInstruction::BorrowObligationLiquidity {
                    liquidity_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // repay obligation liquidity
            {
                let instruction = LendingInstruction::RepayObligationLiquidity {
                    liquidity_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // liquidate obligation
            {
                let instruction = LendingInstruction::LiquidateObligation {
                    liquidity_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // flash loan
            {
                let instruction = LendingInstruction::FlashLoan {
                    amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // deposit reserve liquidity and obligation collateral
            {
                let instruction =
                    LendingInstruction::DepositReserveLiquidityAndObligationCollateral {
                        liquidity_amount: rng.gen::<u64>(),
                    };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // withdraw obligation collateral and redeem reserve collateral
            {
                let instruction =
                    LendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral {
                        collateral_amount: rng.gen::<u64>(),
                    };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // update reserve config
            {
                let instruction = LendingInstruction::UpdateReserveConfig {
                    config: ReserveConfig {
                        optimal_utilization_rate: rng.gen::<u8>(),
                        max_utilization_rate: rng.gen::<u8>(),
                        loan_to_value_ratio: rng.gen::<u8>(),
                        liquidation_bonus: rng.gen::<u8>(),
                        max_liquidation_bonus: rng.gen::<u8>(),
                        liquidation_threshold: rng.gen::<u8>(),
                        max_liquidation_threshold: rng.gen::<u8>(),
                        min_borrow_rate: rng.gen::<u8>(),
                        optimal_borrow_rate: rng.gen::<u8>(),
                        max_borrow_rate: rng.gen::<u8>(),
                        super_max_borrow_rate: rng.gen::<u64>(),
                        fees: ReserveFees {
                            borrow_fee_wad: rng.gen::<u64>(),
                            flash_loan_fee_wad: rng.gen::<u64>(),
                            host_fee_percentage: rng.gen::<u8>(),
                        },
                        deposit_limit: rng.gen::<u64>(),
                        borrow_limit: rng.gen::<u64>(),
                        fee_receiver: Pubkey::new_unique(),
                        protocol_liquidation_fee: rng.gen::<u8>(),
                        protocol_take_rate: rng.gen::<u8>(),
                        added_borrow_weight_bps: rng.gen::<u64>(),
                        reserve_type: ReserveType::from_u8(rng.gen::<u8>() % 2).unwrap(),
                    },
                    rate_limiter_config: RateLimiterConfig {
                        window_duration: rng.gen::<u64>(),
                        max_outflow: rng.gen::<u64>(),
                    },
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // liquidate obligation and redeem reserve collateral
            {
                let instruction =
                    LendingInstruction::LiquidateObligationAndRedeemReserveCollateral {
                        liquidity_amount: rng.gen::<u64>(),
                    };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // redeem fees
            {
                let instruction = LendingInstruction::RedeemFees;

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // flash borrow reserve liquidity
            {
                let instruction = LendingInstruction::FlashBorrowReserveLiquidity {
                    liquidity_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // flash repay reserve liquidity
            {
                let instruction = LendingInstruction::FlashRepayReserveLiquidity {
                    liquidity_amount: rng.gen::<u64>(),
                    borrow_instruction_index: rng.gen::<u8>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }

            // forgive debt
            {
                let instruction = LendingInstruction::ForgiveDebt {
                    liquidity_amount: rng.gen::<u64>(),
                };

                let packed = instruction.pack();
                let unpacked = LendingInstruction::unpack(&packed).unwrap();
                assert_eq!(instruction, unpacked);
            }
        }
    }
}
