pub mod instructions;
use std::borrow::BorrowMut;
use std::fmt;
use std::str::FromStr;

pub use instructions::*;
use solana_program::borsh0_10::get_instance_packed_len;
use solana_program::program_memory::sol_memcmp;
use solana_program::program_pack::Pack;
pub use anchor_lang::prelude::*;
use num_derive::{ToPrimitive, FromPrimitive};
use num_traits::FromPrimitive;
use solana_program::program_pack::Sealed;
use solana_program::pubkey::PUBKEY_BYTES;
use solana_program::stake::state::Lockup;
use spl_pod::primitives::{PodU64, PodU32};
use spl_stake_pool::{WITHDRAWAL_BASELINE_FEE, MAX_WITHDRAWAL_FEE_INCREASE};
use spl_stake_pool::big_vec::BigVec;
use spl_stake_pool::error::StakePoolError;
use spl_token::state::AccountState;
use spl_token_2022::extension::{StateWithExtensions, ExtensionType};
pub use switchboard_solana::*;

pub use bytemuck;
pub use bytemuck::{Pod, Zeroable};

/// Seed for deposit authority seed
const AUTHORITY_DEPOSIT: &[u8] = b"deposit";

/// Seed for withdraw authority seed
const AUTHORITY_WITHDRAW: &[u8] = b"withdraw";

/// Seed for transient stake account
const TRANSIENT_STAKE_SEED_PREFIX: &[u8] = b"transient";

/// Seed for ephemeral stake account
const EPHEMERAL_STAKE_SEED_PREFIX: &[u8] = b"ephemeral";



declare_id!("5CohttpA8Bm3KSii7QRMFJsXBhy3wn4LXb5JN4fTZ546");

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
    pub fn init_mrgn_fi_pda(ctx: Context<InitMrgnFiPda>, bump: u8) -> anchor_lang::Result<()> {
        InitMrgnFiPda::init_mrgn_fi_pda(ctx, bump)
    }

    pub fn request(ctx: Context<Request>, keyhash: [u8; 32], bump: u8) -> anchor_lang::Result<()> {
        Request::request(ctx, keyhash, bump)
    }
    pub fn create_seeded_account(
        ctx: Context<CreateSeededAccount>,
        params: CreateSeededAccountParams,
    ) -> anchor_lang::Result<()> {
        CreateSeededAccount::create_seeded_account(ctx, params)
    }
    pub fn init_obligation_account(
        ctx: Context<CreateSeededAccount>,
        params: CreateSeededAccountParams,
    ) -> anchor_lang::Result<()> {
        CreateSeededAccount::init_obligation_account(ctx, params)
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> anchor_lang::Result<()> {
        Deposit::deposit(ctx, amount)
    }
    pub fn seed(ctx: Context<Seed>, seed: u32) -> anchor_lang::Result<()> {
        Seed::seed(ctx, seed)
    }

    pub fn reveal(ctx: Context<Reveal>, pubkey: Pubkey) -> anchor_lang::Result<()> {
        Reveal::reveal(ctx, pubkey)
    }
}

#[error_code]
pub enum SbError {
    RequestAlreadySeeded,
    RequestAlreadyRevealed,
    KeyVerifyFailed,
}

/// Enum representing the account type managed by the program
#[derive(Clone, Debug, Default, PartialEq,  AnchorDeserialize, AnchorSerialize)]
pub enum AccountType {
    /// If the account has not been initialized, the enum will be 0
    #[default]
    Uninitialized,
    /// Stake pool
    StakePool,
    /// Validator stake list
    ValidatorList,
}

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq,  AnchorDeserialize, AnchorSerialize)]
pub struct StakePool {
    /// Account type, must be StakePool currently
    pub account_type: AccountType,

    /// Manager authority, allows for updating the staker, manager, and fee
    /// account
    pub manager: Pubkey,

    /// Staker authority, allows for adding and removing validators, and
    /// managing stake distribution
    pub staker: Pubkey,

    /// Stake deposit authority
    ///
    /// If a depositor pubkey is specified on initialization, then deposits must
    /// be signed by this authority. If no deposit authority is specified,
    /// then the stake pool will default to the result of:
    /// `Pubkey::find_program_address(
    ///     &[&stake_pool_address.as_ref(), b"deposit"],
    ///     program_id,
    /// )`
    pub stake_deposit_authority: Pubkey,

    /// Stake withdrawal authority bump seed
    /// for `create_program_address(&[state::StakePool account, "withdrawal"])`
    pub stake_withdraw_bump_seed: u8,

    /// Validator stake list storage account
    pub validator_list: Pubkey,

    /// Reserve stake account, holds deactivated stake
    pub reserve_stake: Pubkey,

    /// Pool Mint
    pub pool_mint: Pubkey,

    /// Manager fee account
    pub manager_fee_account: Pubkey,

    /// Pool token program id
    pub token_program_id: Pubkey,

    /// Total stake under management.
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub total_lamports: u64,

    /// Total supply of pool tokens (should always match the supply in the Pool
    /// Mint)
    pub pool_token_supply: u64,

    /// Last epoch the `total_lamports` field was updated
    pub last_update_epoch: u64,

    /// Lockup that all stakes in the pool must have
    pub lockup: Lockup,

    /// Fee taken as a proportion of rewards each epoch
    pub epoch_fee: Fee,

    /// Fee for next epoch
    pub next_epoch_fee: FutureEpoch<Fee>,

    /// Preferred deposit validator vote account pubkey
    pub preferred_deposit_validator_vote_address: Option<Pubkey>,

    /// Preferred withdraw validator vote account pubkey
    pub preferred_withdraw_validator_vote_address: Option<Pubkey>,

    /// Fee assessed on stake deposits
    pub stake_deposit_fee: Fee,

    /// Fee assessed on withdrawals
    pub stake_withdrawal_fee: Fee,

    /// Future stake withdrawal fee, to be set for the following epoch
    pub next_stake_withdrawal_fee: FutureEpoch<Fee>,

    /// Fees paid out to referrers on referred stake deposits.
    /// Expressed as a percentage (0 - 100) of deposit fees.
    /// i.e. `stake_deposit_fee`% of stake deposited is collected as deposit
    /// fees for every deposit and `stake_referral_fee`% of the collected
    /// stake deposit fees is paid out to the referrer
    pub stake_referral_fee: u8,

    /// Toggles whether the `DepositSol` instruction requires a signature from
    /// this `sol_deposit_authority`
    pub sol_deposit_authority: Option<Pubkey>,

    /// Fee assessed on SOL deposits
    pub sol_deposit_fee: Fee,

    /// Fees paid out to referrers on referred SOL deposits.
    /// Expressed as a percentage (0 - 100) of SOL deposit fees.
    /// i.e. `sol_deposit_fee`% of SOL deposited is collected as deposit fees
    /// for every deposit and `sol_referral_fee`% of the collected SOL
    /// deposit fees is paid out to the referrer
    pub sol_referral_fee: u8,

    /// Toggles whether the `WithdrawSol` instruction requires a signature from
    /// the `deposit_authority`
    pub sol_withdraw_authority: Option<Pubkey>,

    /// Fee assessed on SOL withdrawals
    pub sol_withdrawal_fee: Fee,

    /// Future SOL withdrawal fee, to be set for the following epoch
    pub next_sol_withdrawal_fee: FutureEpoch<Fee>,

    /// Last epoch's total pool tokens, used only for APR estimation
    pub last_epoch_pool_token_supply: u64,

    /// Last epoch's total lamports, used only for APR estimation
    pub last_epoch_total_lamports: u64,
}

impl<T> Default for FutureEpoch<T> {
    fn default() -> Self {
        Self::None
    }
}
impl<T> FutureEpoch<T> {
    /// Create a new value to be unlocked in a two epochs
    pub fn new(value: T) -> Self {
        Self::Two(value)
    }
}
impl<T: Clone> FutureEpoch<T> {
    /// Update the epoch, to be done after `get`ting the underlying value
    pub fn update_epoch(&mut self) {
        match self {
            Self::None => {}
            Self::One(_) => {
                // The value has waited its last epoch
                *self = Self::None;
            }
            // The value still has to wait one more epoch after this
            Self::Two(v) => {
                *self = Self::One(v.clone());
            }
        }
    }

    /// Get the value if it's ready, which is only at `One` epoch remaining
    pub fn get(&self) -> Option<&T> {
        match self {
            Self::None | Self::Two(_) => None,
            Self::One(v) => Some(v),
        }
    }
}
impl<T> From<FutureEpoch<T>> for Option<T> {
    fn from(v: FutureEpoch<T>) -> Option<T> {
        match v {
            FutureEpoch::None => None,
            FutureEpoch::One(inner) | FutureEpoch::Two(inner) => Some(inner),
        }
    }
}

/// Storage list for all validator stake accounts in the pool.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq,  AnchorDeserialize, AnchorSerialize)]
pub struct ValidatorList {
    /// Data outside of the validator list, separated out for cheaper
    /// deserializations
    pub header: ValidatorListHeader,

    /// List of stake info for each validator in the pool
    pub validators: Vec<ValidatorStakeInfo>,
}

/// Helper type to deserialize just the start of a ValidatorList
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq,  AnchorDeserialize, AnchorSerialize)]
pub struct ValidatorListHeader {
    /// Account type, must be ValidatorList currently
    pub account_type: AccountType,

    /// Maximum allowable number of validators
    pub max_validators: u32,
}

/// Status of the stake account in the validator list, for accounting
#[derive(
    ToPrimitive,
    FromPrimitive,
    Copy,
    Clone,
    Debug,
    PartialEq,
    AnchorDeserialize,
    AnchorSerialize
)]
pub enum StakeStatus {
    /// Stake account is active, there may be a transient stake as well
    Active,
    /// Only transient stake account exists, when a transient stake is
    /// deactivating during validator removal
    DeactivatingTransient,
    /// No more validator stake accounts exist, entry ready for removal during
    /// `UpdateStakePoolBalance`
    ReadyForRemoval,
    /// Only the validator stake account is deactivating, no transient stake
    /// account exists
    DeactivatingValidator,
    /// Both the transient and validator stake account are deactivating, when
    /// a validator is removed with a transient stake active
    DeactivatingAll,
}
/// Wrapper struct that can be `Pod`, containing a byte that *should* be a valid
/// `StakeStatus` underneath.
#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Pod,
    Zeroable,
    AnchorDeserialize,
    AnchorSerialize,
)]
pub struct PodStakeStatus(u8);
/// Withdrawal type, figured out during process_withdraw_stake
#[derive(Debug, PartialEq)]
pub(crate) enum StakeWithdrawSource {
    /// Some of an active stake account, but not all
    Active,
    /// Some of a transient stake account
    Transient,
    /// Take a whole validator stake account
    ValidatorRemoval,
}

/// Information about a validator in the pool
///
/// NOTE: ORDER IS VERY IMPORTANT HERE, PLEASE DO NOT RE-ORDER THE FIELDS UNLESS
/// THERE'S AN EXTREMELY GOOD REASON.
///
/// To save on BPF instructions, the serialized bytes are reinterpreted with a
/// bytemuck transmute, which means that this structure cannot have any
/// undeclared alignment-padding in its representation.
#[repr(C)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Pod,
    Zeroable,
    AnchorDeserialize,
    AnchorSerialize
)]
pub struct ValidatorStakeInfo {
    /// Amount of lamports on the validator stake account, including rent
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub active_stake_lamports: PodU64,

    /// Amount of transient stake delegated to this validator
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub transient_stake_lamports: PodU64,

    /// Last epoch the active and transient stake lamports fields were updated
    pub last_update_epoch: PodU64,

    /// Transient account seed suffix, used to derive the transient stake
    /// account address
    pub transient_seed_suffix: PodU64,

    /// Unused space, initially meant to specify the end of seed suffixes
    pub unused: PodU32,

    /// Validator account seed suffix
    pub validator_seed_suffix: PodU32, // really `Option<NonZeroU32>` so 0 is `None`

    /// Status of the validator stake account
    pub status: PodStakeStatus,

    /// Validator vote account address
    pub vote_account_address: Pubkey,
}
/// Wrapper type that "counts down" epochs, which is Borsh-compatible with the
/// native `Option`
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum FutureEpoch<T> {
    /// Nothing is set
    None,
    /// Value is ready after the next epoch boundary
    One(T),
    /// Value is ready after two epoch boundaries
    Two(T),
}

/// Fee rate as a ratio, minted on `UpdateStakePoolBalance` as a proportion of
/// the rewards
/// If either the numerator or the denominator is 0, the fee is considered to be
/// 0
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub struct Fee {
    /// denominator of the fee ratio
    pub denominator: u64,
    /// numerator of the fee ratio
    pub numerator: u64,
}


/// The type of fees that can be set on the stake pool
#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum FeeType {
    /// Referral fees for SOL deposits
    SolReferral(u8),
    /// Referral fees for stake deposits
    StakeReferral(u8),
    /// Management fee paid per epoch
    Epoch(Fee),
    /// Stake withdrawal fee
    StakeWithdrawal(Fee),
    /// Deposit fee for SOL deposits
    SolDeposit(Fee),
    /// Deposit fee for stake deposits
    StakeDeposit(Fee),
    /// SOL withdrawal fee
    SolWithdrawal(Fee),
}


impl anchor_lang::AccountDeserialize for StakePool {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        let mut data = buf.borrow_mut();
        let stake_pool = StakePool::deserialize(&mut data)?;
        Ok(stake_pool)
    }
}
impl anchor_lang::AccountSerialize for StakePool {
    
    fn try_serialize<W: std::io::Write>(&self, buf: &mut W) -> Result<()> {
        let mut data = buf.borrow_mut();
        StakePool::serialize(self, &mut data)?;
        Ok(())
    }
}
impl anchor_lang::Owner for StakePool {
    fn owner() -> Pubkey {
        Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap()
    }
}