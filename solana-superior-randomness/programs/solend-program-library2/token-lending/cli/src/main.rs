use lending_state::SolendState;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcProgramAccountsConfig, RpcSendTransactionConfig};
use solana_client::{rpc_config::RpcAccountInfoConfig, rpc_filter::RpcFilterType};
use solana_sdk::{commitment_config::CommitmentLevel, compute_budget::ComputeBudgetInstruction};
use solend_program::{
    instruction::set_lending_market_owner_and_config,
    state::{validate_reserve_config, RateLimiterConfig},
};
use solend_sdk::{
    instruction::{
        liquidate_obligation_and_redeem_reserve_collateral, redeem_reserve_collateral,
        refresh_obligation, refresh_reserve,
    },
    state::Obligation,
    state::ReserveType,
};

mod lending_state;

use {
    clap::{
        crate_description, crate_name, crate_version, value_t, App, AppSettings, Arg, ArgMatches,
        SubCommand,
    },
    solana_clap_utils::{
        fee_payer::fee_payer_arg,
        input_parsers::{keypair_of, pubkey_of, value_of},
        input_validators::{is_amount, is_keypair, is_parsable, is_pubkey, is_url},
        keypair::signer_from_path,
    },
    solana_client::rpc_client::RpcClient,
    solana_program::{
        message::Message, native_token::lamports_to_sol, program_pack::Pack, pubkey::Pubkey,
    },
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    },
    solend_sdk::{
        self,
        instruction::{init_lending_market, init_reserve, update_reserve_config},
        math::WAD,
        state::{LendingMarket, Reserve, ReserveConfig, ReserveFees},
    },
    spl_token::{
        amount_to_ui_amount,
        instruction::{approve, revoke},
        state::{Account as Token, Mint},
        ui_amount_to_amount,
    },
    std::{borrow::Borrow, process::exit, str::FromStr},
    system_instruction::create_account,
};

use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;

struct Config {
    rpc_client: RpcClient,
    fee_payer: Box<dyn Signer>,
    lending_program_id: Pubkey,
    verbose: bool,
    dry_run: bool,
}

/// Reserve config with optional fields
struct PartialReserveConfig {
    /// Optimal utilization rate, as a percentage
    pub optimal_utilization_rate: Option<u8>,
    /// max utilization rate, as a percentage
    pub max_utilization_rate: Option<u8>,
    /// Target ratio of the value of borrows to deposits, as a percentage
    /// 0 if use as collateral is disabled
    pub loan_to_value_ratio: Option<u8>,
    /// Bonus a liquidator gets when repaying part of an unhealthy obligation, as a percentage
    pub liquidation_bonus: Option<u8>,
    /// Maximum bonus a liquidator gets when repaying part of an unhealthy obligation, as a percentage
    pub max_liquidation_bonus: Option<u8>,
    /// Loan to value ratio at which an obligation can be liquidated, as a percentage
    pub liquidation_threshold: Option<u8>,
    /// Loan to value ratio at which an obligation can be liquidated for the maximum bonus, as a percentage
    pub max_liquidation_threshold: Option<u8>,
    /// Min borrow APY
    pub min_borrow_rate: Option<u8>,
    /// Optimal (utilization) borrow APY
    pub optimal_borrow_rate: Option<u8>,
    /// Max borrow APY
    pub max_borrow_rate: Option<u8>,
    /// Supermax borrow apr
    pub super_max_borrow_rate: Option<u64>,
    /// Program owner fees assessed, separate from gains due to interest accrual
    pub fees: PartialReserveFees,
    /// Deposit limit
    pub deposit_limit: Option<u64>,
    /// Borrow limit
    pub borrow_limit: Option<u64>,
    /// Liquidity fee receiver
    pub fee_receiver: Option<Pubkey>,
    /// Cut of the liquidation bonus that the protocol receives, in deca bps
    pub protocol_liquidation_fee: Option<u8>,
    /// Protocol take rate is the amount borrowed interest protocol recieves, as a percentage  
    pub protocol_take_rate: Option<u8>,
    /// Rate Limiter's max window size
    pub rate_limiter_window_duration: Option<u64>,
    /// Rate Limiter's max outflow per window
    pub rate_limiter_max_outflow: Option<u64>,
    /// Added borrow weight in basis points
    pub added_borrow_weight_bps: Option<u64>,
    /// Type of the reseerve (Regular, Isolated)
    pub reserve_type: Option<ReserveType>,
}

/// Reserve Fees with optional fields
struct PartialReserveFees {
    pub borrow_fee_wad: Option<u64>,
    /// Fee for flash loan, expressed as a Wad.
    /// 0.3% (Aave flash loan fee) = 3_000_000_000_000_000
    pub flash_loan_fee_wad: Option<u64>,
    /// Amount of fee going to host account, if provided in liquidate and repay
    pub host_fee_percentage: Option<u8>,
}

type Error = Box<dyn std::error::Error>;
type CommandResult = Result<(), Error>;

const PYTH_PROGRAM_ID: &str = "gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s";
// const SWITCHBOARD_PROGRAM_ID: &str = "DtmE9D2CSB4L5D6A15mraeEjrGMm6auWVzgaD8hK2tZM";
const SWITCHBOARD_PROGRAM_ID_DEV: &str = "7azgmy1pFXHikv36q1zZASvFq5vFa39TT9NweVugKKTU";

fn main() {
    solana_logger::setup_with_default("solana=info");

    let default_lending_program_id: &str = &solend_sdk::solend_mainnet::id().to_string();

    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg({
            let arg = Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::with_name("json_rpc_url")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .validator(is_url)
                .help("JSON RPC URL for the cluster.  Default from the configuration file."),
        )
        .arg(
            fee_payer_arg()
                .short("p")
                .global(true)
        )
        .arg(
            Arg::with_name("lending_program_id")
                .long("program")
                .validator(is_pubkey)
                .value_name("PUBKEY")
                .takes_value(true)
                .required(true)
                .default_value(default_lending_program_id)
                .help("Lending program ID"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::with_name("dry_run")
                .long("dry-run")
                .takes_value(false)
                .global(true)
                .help("Simulate transaction instead of executing"),
        )
        .subcommand(
            SubCommand::with_name("view-reserve")
                .about("View reserve")
                .arg(
                    Arg::with_name("reserve")
                        .long("reserve")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("reserve pubkey"),
                )
        )
        .subcommand(
            SubCommand::with_name("view-market")
                .about("View market")
                .arg(
                    Arg::with_name("market")
                        .long("market")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("market pubkey"),
                )
        )
        .subcommand(
            SubCommand::with_name("view-all-markets")
                .about("View all markets")
        )
        .subcommand(
            SubCommand::with_name("view-obligation")
                .about("View obligation")
                .arg(
                    Arg::with_name("obligation")
                        .long("obligation")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("obligation pubkey"),
                )
        )
        .subcommand(
            SubCommand::with_name("create-market")
                .about("Create a new lending market")
                .arg(
                    Arg::with_name("lending_market_owner")
                        .long("market-owner")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Owner that can add reserves to the market"),
                )
                .arg(
                    Arg::with_name("oracle_program_id")
                        .long("oracle")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .default_value(PYTH_PROGRAM_ID)
                        .help("Oracle (Pyth) program ID for quoting market prices"),
                )
                .arg(
                    Arg::with_name("switchboard_oracle_program_id")
                        .long("switchboard-oracle")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .default_value(SWITCHBOARD_PROGRAM_ID_DEV)
                        .help("Oracle (switchboard) program ID for quoting market prices"),
                )
                .arg(
                    Arg::with_name("quote_currency")
                        .long("quote")
                        .value_name("STRING")
                        .takes_value(true)
                        .required(true)
                        .default_value("USD")
                        .help("Currency market prices are quoted in"),
                ),
        )
        .subcommand(
            SubCommand::with_name("liquidate-obligation")
                .about("Liquidate Obligation and redeem reserve collateral")
                // @TODO: use is_valid_signer
                .arg(
                    Arg::with_name("obligation")
                        .long("obligation")
                        .value_name("OBLIGATION_PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("obligation pubkey"),
                )
                .arg(
                    Arg::with_name("repay-reserve")
                        .long("repay-reserve")
                        .value_name("RESERVE_PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("repay reserve"),
                )
                .arg(
                    Arg::with_name("source-liquidity")
                        .long("source-liquidity")
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Token account that repays the obligation's debt"),
                )
                .arg(
                    Arg::with_name("withdraw-reserve")
                        .long("withdraw-reserve")
                        .value_name("RESERVE_PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("withdraw reserve"),
                )
                .arg(
                    Arg::with_name("liquidity-amount")
                        .long("liquidity-amount")
                        .value_name("AMOUNT")
                        .takes_value(true)
                        .required(true)
                        .help("amount of tokens to repay"),
                )
        )
        .subcommand(
            SubCommand::with_name("withdraw-collateral")
                .about("Withdraw obligation collateral")
                // @TODO: use is_valid_signer
                .arg(
                    Arg::with_name("obligation")
                        .long("obligation")
                        .value_name("OBLIGATION_PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("obligation pubkey"),
                )
                .arg(
                    Arg::with_name("withdraw-reserve")
                        .long("withdraw-reserve")
                        .value_name("RESERVE_PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("reserve that you want to withdraw ctokens from"),
                )
                .arg(
                    Arg::with_name("collateral-amount")
                        .long("withdraw-amount")
                        .value_name("AMOUNT")
                        .takes_value(true)
                        .required(true)
                        .help("amount of ctokens to withdraw"),
                )
        )
        .subcommand(
            SubCommand::with_name("redeem-collateral")
                .about("Redeem ctokens for tokens")
                // @TODO: use is_valid_signer
                .arg(
                    Arg::with_name("redeem-reserve")
                        .long("redeem-reserve")
                        .value_name("RESERVE_PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("reserve pubkey"),
                )
                .arg(
                    Arg::with_name("collateral-amount")
                        .long("redeem-amount")
                        .value_name("AMOUNT")
                        .takes_value(true)
                        .required(true)
                        .help("amount of ctokens to redeem"),
                )
        )
        .subcommand(
            SubCommand::with_name("add-reserve")
                .about("Add a reserve to a lending market")
                // @TODO: use is_valid_signer
                .arg(
                    Arg::with_name("lending_market_owner")
                        .long("market-owner")
                        .validator(is_keypair)
                        .value_name("KEYPAIR")
                        .takes_value(true)
                        .required(true)
                        .help("Owner of the lending market"),
                )
                // @TODO: use is_valid_signer
                .arg(
                    Arg::with_name("source_liquidity_owner")
                        .long("source-owner")
                        .validator(is_keypair)
                        .value_name("KEYPAIR")
                        .takes_value(true)
                        .required(true)
                        .help("Owner of the SPL Token account to deposit initial liquidity from"),
                )
                .arg(
                    Arg::with_name("lending_market")
                        .long("market")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Lending market address"),
                )
                .arg(
                    Arg::with_name("source_liquidity")
                        .long("source")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("SPL Token account to deposit initial liquidity from"),
                )
                // @TODO: use is_amount_or_all
                .arg(
                    Arg::with_name("liquidity_amount")
                        .long("amount")
                        .validator(is_amount)
                        .value_name("DECIMAL_AMOUNT")
                        .takes_value(true)
                        .required(true)
                        .help("Initial amount of liquidity to deposit into the new reserve"),
                )
                .arg(
                    Arg::with_name("pyth_product")
                        .long("pyth-product")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Pyth product account: https://pyth.network/developers/consumers/accounts"),
                )
                .arg(
                    Arg::with_name("pyth_price")
                        .long("pyth-price")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Pyth price account: https://pyth.network/developers/consumers/accounts"),
                )
                .arg(
                    Arg::with_name("switchboard_feed")
                        .long("switchboard-feed")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Switchboard price feed account: https://switchboard.xyz/#/explorer"),
                )
                .arg(
                    Arg::with_name("optimal_utilization_rate")
                        .long("optimal-utilization-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("80")
                        .help("Optimal utilization rate: [0, 100]"),
                )
                .arg(
                    Arg::with_name("max_utilization_rate")
                        .long("max-utilization-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Max utilization rate: [0, 100]"),
                )
                .arg(
                    Arg::with_name("loan_to_value_ratio")
                        .long("loan-to-value-ratio")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("50")
                        .help("Target ratio of the value of borrows to deposits: [0, 100)"),
                )
                .arg(
                    Arg::with_name("liquidation_bonus")
                        .long("liquidation-bonus")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("5")
                        .help("Bonus a liquidator gets when repaying part of an unhealthy obligation: [0, 100]"),
                )
                .arg(
                    Arg::with_name("max_liquidation_bonus")
                        .long("max-liquidation-bonus")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Maximum bonus a liquidator gets when repaying part of an unhealthy obligation: [0, 100]"),
                )
                .arg(
                    Arg::with_name("liquidation_threshold")
                        .long("liquidation-threshold")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("55")
                        .help("Loan to value ratio at which an obligation can be liquidated: (LTV, 100]"),
                )
                .arg(
                    Arg::with_name("max_liquidation_threshold")
                        .long("max-liquidation-threshold")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Loan to value ratio at which an obligation can be liquidated for the max bonus: (liquidation_threshold, 100]"),
                )
                .arg(
                    Arg::with_name("min_borrow_rate")
                        .long("min-borrow-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("0")
                        .help("Min borrow APY: min <= optimal <= max"),
                )
                .arg(
                    Arg::with_name("optimal_borrow_rate")
                        .long("optimal-borrow-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("4")
                        .help("Optimal (utilization) borrow APY: min <= optimal <= max"),
                )
                .arg(
                    Arg::with_name("max_borrow_rate")
                        .long("max-borrow-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("30")
                        .help("Max borrow APY: min <= optimal <= max"),
                )
                .arg(
                    Arg::with_name("super_max_borrow_rate")
                        .long("super-max-borrow-rate")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("super max borrow APY: min <= optimal <= max <= super_max"),
                )
                .arg(
                    Arg::with_name("borrow_fee")
                        .long("borrow-fee")
                        .validator(is_parsable::<f64>)
                        .value_name("DECIMAL_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("0.0001")
                        .help("Fee assessed on borrow, expressed as a percentage: [0, 1)"),
                )
                .arg(
                    Arg::with_name("flash_loan_fee")
                        .long("flash-loan-fee")
                        .validator(is_parsable::<f64>)
                        .value_name("DECIMAL_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value(".003")
                        .help("Fee assessed for flash loans, expressed as a percentage: [0, 1)"),
                )
                .arg(
                    Arg::with_name("host_fee_percentage")
                        .long("host-fee-percentage")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(true)
                        .default_value("20")
                        .help("Amount of fee going to host account: [0, 100]"),
                )
                .arg(
                    Arg::with_name("protocol_liquidation_fee")
                        .long("protocol-liquidation-fee")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .default_value("30")
                        .help("Amount of liquidation bonus going to fee receiver: [0, 100]"),
                )
                .arg(
                    Arg::with_name("protocol_take_rate")
                        .long("protocol-take-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Amount of interest spread going to fee receiver: [0, 100]"),
                )
                .arg(
                    Arg::with_name("deposit_limit")
                        .long("deposit-limit")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(true)
                        .default_value("18446744073709551615")
                        .help("Deposit limit"),
                )
                .arg(
                    Arg::with_name("borrow_limit")
                        .long("borrow-limit")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(true)
                        .default_value("18446744073709551615")
                        .help("Borrow limit"),
                )
                .arg(
                    Arg::with_name("added_borrow_weight_bps")
                        .long("added-borrow-weight-bps")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(true)
                        .help("Added borrow weight bps"),
                )
                .arg(
                    Arg::with_name("reserve_type")
                        .long("reserve-type")
                        .validator(is_parsable::<ReserveType>)
                        .value_name("RESERVE_TYPE")
                        .takes_value(true)
                        .required(false)
                        .help("Reserve type"),
                )
        )
        .subcommand(
            SubCommand::with_name("set-lending-market-owner-and-config")
                .about("Set lending market owner and config")
                .arg(
                    Arg::with_name("lending_market_owner")
                        .long("lending-market-owner")
                        .validator(is_keypair)
                        .value_name("KEYPAIR")
                        .takes_value(true)
                        .required(true)
                        .help("Owner of the lending market"),
                )
                .arg(
                    Arg::with_name("lending_market")
                        .long("market")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Lending market address"),
                )
                .arg(
                    Arg::with_name("new_lending_market_owner")
                        .long("new-lending-market-owner")
                        .validator(is_keypair)
                        .value_name("KEYPAIR")
                        .takes_value(true)
                        .required(false)
                        .help("Owner of the lending market"),
                )
                .arg(
                    Arg::with_name("rate_limiter_window_duration")
                        .long("rate-limiter-window-duration")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("Rate Limiter Window Duration in Slots"),
                )
                .arg(
                    Arg::with_name("rate_limiter_max_outflow")
                        .long("rate-limiter-max-outflow")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("Rate Limiter max outflow denominated in dollars within 1 window"),
                )
                .arg(
                    Arg::with_name("whitelisted_liquidator")
                        .long("whitelisted-liquidator")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .help("Whitelisted liquidator address"),
                )
                .arg(
                    Arg::with_name("risk_authority")
                        .long("risk-authority")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(false)
                        .help("Risk authority address"),
                )
        )
        .subcommand(
            SubCommand::with_name("update-reserve")
                .about("Update a reserve config")
                .arg(
                    Arg::with_name("reserve")
                        .long("reserve")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Reserve address"),
                )
                .arg(
                    Arg::with_name("lending_market_owner")
                        .long("market-owner")
                        .validator(is_keypair)
                        .value_name("KEYPAIR")
                        .takes_value(true)
                        .required(true)
                        .help("Owner of the lending market"),
                )
                // @TODO: use is_valid_signer
                .arg(
                    Arg::with_name("lending_market")
                        .long("market")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(true)
                        .help("Lending market address"),
                )
                .arg(
                    Arg::with_name("optimal_utilization_rate")
                        .long("optimal-utilization-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Optimal utilization rate: [0, 100]"),
                )
                .arg(
                    Arg::with_name("max_utilization_rate")
                        .long("max-utilization-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Max utilization rate: [0, 100]"),
                )
                .arg(
                    Arg::with_name("loan_to_value_ratio")
                        .long("loan-to-value-ratio")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Target ratio of the value of borrows to deposits: [0, 100)"),
                )
                .arg(
                    Arg::with_name("liquidation_bonus")
                        .long("liquidation-bonus")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Bonus a liquidator gets when repaying part of an unhealthy obligation: [0, 100]"),
                )
                .arg(
                    Arg::with_name("max_liquidation_bonus")
                        .long("max-liquidation-bonus")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Maximum bonus a liquidator gets when repaying part of an unhealthy obligation: [0, 100]"),
                )
                .arg(
                    Arg::with_name("liquidation_threshold")
                        .long("liquidation-threshold")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Loan to value ratio at which an obligation can be liquidated: (LTV, 100]"),
                )
                .arg(
                    Arg::with_name("max_liquidation_threshold")
                        .long("max-liquidation-threshold")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Loan to value ratio at which an obligation can be liquidated for the max bonus: (liquidation_threshold, 100]"),
                )
                .arg(
                    Arg::with_name("min_borrow_rate")
                        .long("min-borrow-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Min borrow APY: min <= optimal <= max"),
                )
                .arg(
                    Arg::with_name("optimal_borrow_rate")
                        .long("optimal-borrow-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Optimal (utilization) borrow APY: min <= optimal <= max"),
                )
                .arg(
                    Arg::with_name("max_borrow_rate")
                        .long("max-borrow-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Max borrow APY: min <= optimal <= max"),
                )
                .arg(
                    Arg::with_name("super_max_borrow_rate")
                        .long("super-max-borrow-rate")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("super max borrow APY: min <= optimal <= max <= super_max"),
                )
                .arg(
                    Arg::with_name("borrow_fee")
                        .long("borrow-fee")
                        .validator(is_parsable::<f64>)
                        .value_name("DECIMAL_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Fee assessed on borrow, expressed as a percentage: [0, 1)"),
                )
                .arg(
                    Arg::with_name("flash_loan_fee")
                        .long("flash-loan-fee")
                        .validator(is_parsable::<f64>)
                        .value_name("DECIMAL_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Fee assessed for flash loans, expressed as a percentage: [0, 1)"),
                )
                .arg(
                    Arg::with_name("host_fee_percentage")
                        .long("host-fee-percentage")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Amount of fee going to host account: [0, 100]"),
                )
                .arg(
                    Arg::with_name("protocol_liquidation_fee")
                        .long("protocol-liquidation-fee")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Amount of liquidation bonus going to fee receiver: [0, 100]"),
                )
                .arg(
                    Arg::with_name("protocol_take_rate")
                        .long("protocol-take-rate")
                        .validator(is_parsable::<u8>)
                        .value_name("INTEGER_PERCENT")
                        .takes_value(true)
                        .required(false)
                        .help("Amount of interest spread going to fee receiver: [0, 100]"),
                )
                .arg(
                    Arg::with_name("deposit_limit")
                        .long("deposit-limit")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("Deposit Limit"),
                )
                .arg(
                    Arg::with_name("borrow_limit")
                        .long("borrow-limit")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("Borrow Limit"),
                )
                .arg(
                    Arg::with_name("fee_receiver")
                        .long("fee-receiver")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(false)
                        .help("Fee receiver address"),
                )
                .arg(
                    Arg::with_name("pyth_product")
                        .long("pyth-product")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(false)
                        .help("Pyth product account: https://pyth.network/developers/consumers/accounts"),
                )
                .arg(
                    Arg::with_name("pyth_price")
                        .long("pyth-price")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(false)
                        .help("Pyth price account: https://pyth.network/developers/consumers/accounts"),
                )
                .arg(
                    Arg::with_name("switchboard_feed")
                        .long("switchboard-feed")
                        .validator(is_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .required(false)
                        .help("Switchboard price feed account: https://switchboard.xyz/#/explorer"),
                )
                .arg(
                    Arg::with_name("rate_limiter_window_duration")
                        .long("rate-limiter-window-duration")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("Rate Limiter Window Duration in Slots"),
                )
                .arg(
                    Arg::with_name("rate_limiter_max_outflow")
                        .long("rate-limiter-max-outflow")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("Rate Limiter max outflow of token amounts within 1 window"),
                )
                .arg(
                    Arg::with_name("added_borrow_weight_bps")
                        .long("added-borrow-weight-bps")
                        .validator(is_parsable::<u64>)
                        .value_name("INTEGER")
                        .takes_value(true)
                        .required(false)
                        .help("Added borrow weight in basis points"),
                )
                .arg(
                    Arg::with_name("reserve_type")
                        .long("reserve-type")
                        .validator(is_parsable::<ReserveType>)
                        .value_name("RESERVE_TYPE")
                        .takes_value(true)
                        .required(false)
                        .help("Reserve type"),
                )
        )
        .get_matches();

    let mut wallet_manager = None;
    let mut config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };
        let json_rpc_url = value_t!(matches, "json_rpc_url", String)
            .unwrap_or_else(|_| cli_config.json_rpc_url.clone());

        let fee_payer = signer_from_path(
            &matches,
            matches
                .value_of("fee_payer")
                .unwrap_or(&cli_config.keypair_path),
            "fee_payer",
            &mut wallet_manager,
        )
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            exit(1);
        });

        let lending_program_id = pubkey_of(&matches, "lending_program_id").unwrap();
        let verbose = matches.is_present("verbose");
        let dry_run = matches.is_present("dry_run");

        Config {
            rpc_client: RpcClient::new_with_commitment(json_rpc_url, CommitmentConfig::confirmed()),
            fee_payer,
            lending_program_id,
            verbose,
            dry_run,
        }
    };

    let _ = match matches.subcommand() {
        ("view-reserve", Some(arg_matches)) => {
            let reserve = pubkey_of(arg_matches, "reserve").unwrap();
            let data = config.rpc_client.get_account_data(&reserve).unwrap();
            print!("{:#?}", Reserve::unpack(&data));

            Ok(())
        }
        ("view-market", Some(arg_matches)) => {
            let market = pubkey_of(arg_matches, "market").unwrap();
            let data = config.rpc_client.get_account_data(&market).unwrap();
            print!("{:#?}", LendingMarket::unpack(&data));

            Ok(())
        }
        ("view-obligation", Some(arg_matches)) => {
            let obligation = pubkey_of(arg_matches, "obligation").unwrap();
            let data = config.rpc_client.get_account_data(&obligation).unwrap();
            print!("{:#?}", Obligation::unpack(&data));

            Ok(())
        }
        ("view-all-markets", Some(_arg_matches)) => {
            let accounts = config
                .rpc_client
                .get_program_accounts_with_config(
                    &config.lending_program_id,
                    RpcProgramAccountsConfig {
                        filters: Some(vec![RpcFilterType::DataSize(LendingMarket::LEN as u64)]),
                        account_config: RpcAccountInfoConfig {
                            encoding: Some(UiAccountEncoding::Base64Zstd),
                            ..RpcAccountInfoConfig::default()
                        },
                        with_context: Some(false),
                    },
                )
                .unwrap();

            for (address, _) in accounts {
                println!("{}", address);
            }

            Ok(())
        }
        ("create-market", Some(arg_matches)) => {
            let lending_market_owner = pubkey_of(arg_matches, "lending_market_owner").unwrap();
            let quote_currency = quote_currency_of(arg_matches, "quote_currency").unwrap();
            let oracle_program_id = pubkey_of(arg_matches, "oracle_program_id").unwrap();
            let switchboard_oracle_program_id =
                pubkey_of(arg_matches, "switchboard_oracle_program_id").unwrap();

            command_create_lending_market(
                &config,
                lending_market_owner,
                quote_currency,
                oracle_program_id,
                switchboard_oracle_program_id,
            )
        }
        ("liquidate-obligation", Some(arg_matches)) => {
            let obligation = pubkey_of(arg_matches, "obligation").unwrap();
            let repay_reserve = pubkey_of(arg_matches, "repay-reserve").unwrap();
            let source_liquidity = pubkey_of(arg_matches, "source-liquidity").unwrap();
            let withdraw_reserve = pubkey_of(arg_matches, "withdraw-reserve").unwrap();
            let liquidity_amount = value_of(arg_matches, "liquidity-amount").unwrap();

            command_liquidate_obligation(
                &config,
                obligation,
                repay_reserve,
                source_liquidity,
                withdraw_reserve,
                liquidity_amount,
            )
        }
        ("withdraw-collateral", Some(arg_matches)) => {
            let obligation = pubkey_of(arg_matches, "obligation").unwrap();
            let withdraw_reserve = pubkey_of(arg_matches, "withdraw-reserve").unwrap();
            let collateral_amount = value_of(arg_matches, "collateral-amount").unwrap();

            command_withdraw_collateral(&config, obligation, withdraw_reserve, collateral_amount)
        }
        ("redeem-collateral", Some(arg_matches)) => {
            let redeem_reserve = pubkey_of(arg_matches, "redeem-reserve").unwrap();
            let collateral_amount = value_of(arg_matches, "collateral-amount").unwrap();

            command_redeem_collateral(&config, &redeem_reserve, collateral_amount)
        }
        ("add-reserve", Some(arg_matches)) => {
            let lending_market_owner_keypair =
                keypair_of(arg_matches, "lending_market_owner").unwrap();
            let source_liquidity_owner_keypair =
                keypair_of(arg_matches, "source_liquidity_owner").unwrap();
            let lending_market_pubkey = pubkey_of(arg_matches, "lending_market").unwrap();
            let source_liquidity_pubkey = pubkey_of(arg_matches, "source_liquidity").unwrap();
            let ui_amount = value_of(arg_matches, "liquidity_amount").unwrap();
            let pyth_product_pubkey = pubkey_of(arg_matches, "pyth_product").unwrap();
            let pyth_price_pubkey = pubkey_of(arg_matches, "pyth_price").unwrap();
            let switchboard_feed_pubkey = pubkey_of(arg_matches, "switchboard_feed").unwrap();
            let optimal_utilization_rate =
                value_of(arg_matches, "optimal_utilization_rate").unwrap();
            let max_utilization_rate = value_of(arg_matches, "max_utilization_rate").unwrap();
            let loan_to_value_ratio = value_of(arg_matches, "loan_to_value_ratio").unwrap();
            let liquidation_bonus = value_of(arg_matches, "liquidation_bonus").unwrap();
            let max_liquidation_bonus = value_of(arg_matches, "max_liquidation_bonus").unwrap();
            let liquidation_threshold = value_of(arg_matches, "liquidation_threshold").unwrap();
            let max_liquidation_threshold =
                value_of(arg_matches, "max_liquidation_threshold").unwrap();
            let min_borrow_rate = value_of(arg_matches, "min_borrow_rate").unwrap();
            let optimal_borrow_rate = value_of(arg_matches, "optimal_borrow_rate").unwrap();
            let max_borrow_rate = value_of(arg_matches, "max_borrow_rate").unwrap();
            let super_max_borrow_rate = value_of(arg_matches, "super_max_borrow_rate").unwrap();
            let borrow_fee = value_of::<f64>(arg_matches, "borrow_fee").unwrap();
            let flash_loan_fee = value_of::<f64>(arg_matches, "flash_loan_fee").unwrap();
            let host_fee_percentage = value_of(arg_matches, "host_fee_percentage").unwrap();
            let deposit_limit = value_of(arg_matches, "deposit_limit").unwrap();
            let borrow_limit = value_of(arg_matches, "borrow_limit").unwrap();

            let added_borrow_weight_bps = value_of(arg_matches, "added_borrow_weight_bps").unwrap();
            let reserve_type = value_of(arg_matches, "reserve_type").unwrap();

            let borrow_fee_wad = (borrow_fee * WAD as f64) as u64;
            let flash_loan_fee_wad = (flash_loan_fee * WAD as f64) as u64;

            let liquidity_fee_receiver_keypair = Keypair::new();
            let protocol_liquidation_fee =
                value_of(arg_matches, "protocol_liquidation_fee").unwrap();
            let protocol_take_rate = value_of(arg_matches, "protocol_take_rate").unwrap();

            let source_liquidity_account = config
                .rpc_client
                .get_account(&source_liquidity_pubkey)
                .unwrap();
            let source_liquidity =
                Token::unpack_from_slice(source_liquidity_account.data.borrow()).unwrap();
            let source_liquidity_mint_account = config
                .rpc_client
                .get_account(&source_liquidity.mint)
                .unwrap();
            let source_liquidity_mint =
                Mint::unpack_from_slice(source_liquidity_mint_account.data.borrow()).unwrap();

            let liquidity_amount = ui_amount_to_amount(ui_amount, source_liquidity_mint.decimals);
            let deposit_limit = ui_amount_to_amount(deposit_limit, source_liquidity_mint.decimals);
            let borrow_limit = ui_amount_to_amount(borrow_limit, source_liquidity_mint.decimals);

            command_add_reserve(
                &mut config,
                liquidity_amount,
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
                    fees: ReserveFees {
                        borrow_fee_wad,
                        flash_loan_fee_wad,
                        host_fee_percentage,
                    },
                    deposit_limit,
                    borrow_limit,
                    fee_receiver: liquidity_fee_receiver_keypair.pubkey(),
                    protocol_liquidation_fee,
                    protocol_take_rate,
                    added_borrow_weight_bps,
                    reserve_type,
                },
                source_liquidity_pubkey,
                source_liquidity_owner_keypair,
                lending_market_pubkey,
                lending_market_owner_keypair,
                pyth_product_pubkey,
                pyth_price_pubkey,
                switchboard_feed_pubkey,
                liquidity_fee_receiver_keypair,
                source_liquidity,
            )
        }
        ("set-lending-market-owner-and-config", Some(arg_matches)) => {
            let lending_market_owner_keypair =
                keypair_of(arg_matches, "lending_market_owner").unwrap();
            let lending_market_pubkey = pubkey_of(arg_matches, "lending_market").unwrap();
            let new_lending_market_owner_keypair =
                keypair_of(arg_matches, "new_lending_market_owner");
            let rate_limiter_window_duration =
                value_of(arg_matches, "rate_limiter_window_duration");
            let rate_limiter_max_outflow = value_of(arg_matches, "rate_limiter_max_outflow");
            let whitelisted_liquidator_pubkey = pubkey_of(arg_matches, "whitelisted_liquidator");
            let risk_authority_pubkey = pubkey_of(arg_matches, "risk_authority").unwrap();
            command_set_lending_market_owner_and_config(
                &mut config,
                lending_market_pubkey,
                lending_market_owner_keypair,
                new_lending_market_owner_keypair,
                rate_limiter_window_duration,
                rate_limiter_max_outflow,
                whitelisted_liquidator_pubkey,
                risk_authority_pubkey,
            )
        }
        ("update-reserve", Some(arg_matches)) => {
            let reserve_pubkey = pubkey_of(arg_matches, "reserve").unwrap();
            let lending_market_owner_keypair =
                keypair_of(arg_matches, "lending_market_owner").unwrap();
            let lending_market_pubkey = pubkey_of(arg_matches, "lending_market").unwrap();
            let optimal_utilization_rate = value_of(arg_matches, "optimal_utilization_rate");
            let max_utilization_rate = value_of(arg_matches, "max_utilization_rate");
            let loan_to_value_ratio = value_of(arg_matches, "loan_to_value_ratio");
            let liquidation_bonus = value_of(arg_matches, "liquidation_bonus");
            let max_liquidation_bonus = value_of(arg_matches, "max_liquidation_bonus");
            let liquidation_threshold = value_of(arg_matches, "liquidation_threshold");
            let max_liquidation_threshold = value_of(arg_matches, "max_liquidation_threshold");
            let min_borrow_rate = value_of(arg_matches, "min_borrow_rate");
            let optimal_borrow_rate = value_of(arg_matches, "optimal_borrow_rate");
            let max_borrow_rate = value_of(arg_matches, "max_borrow_rate");
            let super_max_borrow_rate = value_of(arg_matches, "super_max_borrow_rate");
            let borrow_fee = value_of::<f64>(arg_matches, "borrow_fee");
            let flash_loan_fee = value_of::<f64>(arg_matches, "flash_loan_fee");
            let host_fee_percentage = value_of(arg_matches, "host_fee_percentage");
            let deposit_limit = value_of(arg_matches, "deposit_limit");
            let borrow_limit = value_of(arg_matches, "borrow_limit");
            let fee_receiver = pubkey_of(arg_matches, "fee_receiver");
            let protocol_liquidation_fee = value_of(arg_matches, "protocol_liquidation_fee");
            let protocol_take_rate = value_of(arg_matches, "protocol_take_rate");
            let pyth_product_pubkey = pubkey_of(arg_matches, "pyth_product");
            let pyth_price_pubkey = pubkey_of(arg_matches, "pyth_price");
            let switchboard_feed_pubkey = pubkey_of(arg_matches, "switchboard_feed");
            let rate_limiter_window_duration =
                value_of(arg_matches, "rate_limiter_window_duration");
            let rate_limiter_max_outflow = value_of(arg_matches, "rate_limiter_max_outflow");
            let added_borrow_weight_bps = value_of(arg_matches, "added_borrow_weight_bps");
            let reserve_type = value_of(arg_matches, "reserve_type");

            let borrow_fee_wad = borrow_fee.map(|fee| (fee * WAD as f64) as u64);
            let flash_loan_fee_wad = flash_loan_fee.map(|fee| (fee * WAD as f64) as u64);

            command_update_reserve(
                &mut config,
                PartialReserveConfig {
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
                    fees: PartialReserveFees {
                        borrow_fee_wad,
                        flash_loan_fee_wad,
                        host_fee_percentage,
                    },
                    deposit_limit,
                    borrow_limit,
                    fee_receiver,
                    protocol_liquidation_fee,
                    protocol_take_rate,
                    rate_limiter_window_duration,
                    rate_limiter_max_outflow,
                    added_borrow_weight_bps,
                    reserve_type,
                },
                pyth_product_pubkey,
                pyth_price_pubkey,
                switchboard_feed_pubkey,
                reserve_pubkey,
                lending_market_pubkey,
                lending_market_owner_keypair,
            )
        }
        _ => unreachable!(),
    }
    .map_err(|err| {
        eprintln!("{}", err);
        exit(1);
    });
}

// COMMANDS

fn command_create_lending_market(
    config: &Config,
    lending_market_owner: Pubkey,
    quote_currency: [u8; 32],
    oracle_program_id: Pubkey,
    switchboard_oracle_program_id: Pubkey,
) -> CommandResult {
    let lending_market_keypair = Keypair::new();
    println!(
        "Creating lending market {}",
        lending_market_keypair.pubkey()
    );

    let lending_market_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(LendingMarket::LEN)?;

    let recent_blockhash = config.rpc_client.get_latest_blockhash()?;

    let message = Message::new_with_blockhash(
        &[
            // Account for the lending market
            create_account(
                &config.fee_payer.pubkey(),
                &lending_market_keypair.pubkey(),
                lending_market_balance,
                LendingMarket::LEN as u64,
                &config.lending_program_id,
            ),
            // Initialize lending market account
            init_lending_market(
                config.lending_program_id,
                lending_market_owner,
                quote_currency,
                lending_market_keypair.pubkey(),
                oracle_program_id,
                switchboard_oracle_program_id,
            ),
        ],
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );

    check_fee_payer_balance(
        config,
        lending_market_balance + config.rpc_client.get_fee_for_message(&message)?,
    )?;

    let transaction = Transaction::new(
        &vec![config.fee_payer.as_ref(), &lending_market_keypair],
        message,
        recent_blockhash,
    );
    send_transaction(config, transaction)?;

    let lending_market_pubkey = lending_market_keypair.pubkey();
    let lending_market_account = config.rpc_client.get_account(&lending_market_pubkey)?;
    let lending_market = LendingMarket::unpack_from_slice(lending_market_account.data.borrow())?;
    let authority_signer_seeds = &[lending_market_pubkey.as_ref(), &[lending_market.bump_seed]];
    println!(
        "Authority Address {}",
        Pubkey::create_program_address(authority_signer_seeds, &config.lending_program_id)?,
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn command_redeem_collateral(
    config: &Config,
    redeem_reserve_pubkey: &Pubkey,
    collateral_amount: u64,
) -> CommandResult {
    let redeem_reserve = {
        let data = config
            .rpc_client
            .get_account(redeem_reserve_pubkey)
            .unwrap();
        Reserve::unpack(&data.data).unwrap()
    };

    let source_ata =
        get_or_create_associated_token_address(config, &redeem_reserve.collateral.mint_pubkey);
    let dest_ata =
        get_or_create_associated_token_address(config, &redeem_reserve.liquidity.mint_pubkey);

    let recent_blockhash = config.rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new(
        &vec![config.fee_payer.as_ref()],
        Message::new_with_blockhash(
            &[redeem_reserve_collateral(
                config.lending_program_id,
                collateral_amount,
                source_ata,
                dest_ata,
                *redeem_reserve_pubkey,
                redeem_reserve.collateral.mint_pubkey,
                redeem_reserve.liquidity.supply_pubkey,
                redeem_reserve.lending_market,
                config.fee_payer.pubkey(),
            )],
            Some(&config.fee_payer.pubkey()),
            &recent_blockhash,
        ),
        recent_blockhash,
    );

    send_transaction(config, transaction)?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn command_withdraw_collateral(
    config: &Config,
    obligation_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    collateral_amount: u64,
) -> CommandResult {
    let solend_state = SolendState::new(
        config.lending_program_id,
        obligation_pubkey,
        &config.rpc_client,
    );

    let withdraw_reserve = solend_state
        .find_reserve_by_key(withdraw_reserve_pubkey)
        .unwrap();

    // make atas
    get_or_create_associated_token_address(config, &withdraw_reserve.collateral.mint_pubkey);

    let instructions = solend_state.withdraw(&withdraw_reserve_pubkey, collateral_amount);
    let recent_blockhash = config.rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new(
        &vec![config.fee_payer.as_ref()],
        Message::new_with_blockhash(
            &instructions,
            Some(&config.fee_payer.pubkey()),
            &recent_blockhash,
        ),
        recent_blockhash,
    );

    send_transaction(config, transaction)?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn command_liquidate_obligation(
    config: &Config,
    obligation_pubkey: Pubkey,
    repay_reserve_pubkey: Pubkey,
    source_liquidity_pubkey: Pubkey,
    withdraw_reserve_pubkey: Pubkey,
    liquidity_amount: u64,
) -> CommandResult {
    let obligation_state = {
        let data = config.rpc_client.get_account(&obligation_pubkey)?;
        Obligation::unpack(&data.data)?
    };

    // get reserve pubkeys
    let reserve_pubkeys = {
        let mut r = Vec::new();
        r.extend(obligation_state.deposits.iter().map(|d| d.deposit_reserve));
        r.extend(obligation_state.borrows.iter().map(|b| b.borrow_reserve));
        r
    };

    // get reserve accounts
    let reserves: Vec<(Pubkey, Reserve)> = config
        .rpc_client
        .get_multiple_accounts(&reserve_pubkeys)?
        .into_iter()
        .zip(reserve_pubkeys.iter())
        .map(|(account, pubkey)| (*pubkey, Reserve::unpack(&account.unwrap().data).unwrap()))
        .collect();

    assert!(reserve_pubkeys.len() == reserves.len());

    // find repay, withdraw reserve states
    let withdraw_reserve_state = reserves
        .iter()
        .find_map(|(pubkey, reserve)| {
            if withdraw_reserve_pubkey == *pubkey {
                Some(reserve)
            } else {
                None
            }
        })
        .unwrap();
    let repay_reserve_state = reserves
        .iter()
        .find_map(|(pubkey, reserve)| {
            if repay_reserve_pubkey == *pubkey {
                Some(reserve)
            } else {
                None
            }
        })
        .unwrap();

    // make sure atas exist. if they don't, create them.
    let required_mints = [
        withdraw_reserve_state.collateral.mint_pubkey,
        withdraw_reserve_state.liquidity.mint_pubkey,
    ];

    for mint in required_mints {
        get_or_create_associated_token_address(config, &mint);
    }

    let destination_collateral_pubkey = get_associated_token_address(
        &config.fee_payer.pubkey(),
        &withdraw_reserve_state.collateral.mint_pubkey,
    );
    let destination_liquidity_pubkey = get_associated_token_address(
        &config.fee_payer.pubkey(),
        &withdraw_reserve_state.liquidity.mint_pubkey,
    );

    let mut instructions = vec![ComputeBudgetInstruction::set_compute_unit_price(30101)];

    // refresh all reserves
    instructions.extend(reserves.iter().map(|(pubkey, reserve)| {
        refresh_reserve(
            config.lending_program_id,
            *pubkey,
            reserve.liquidity.pyth_oracle_pubkey,
            reserve.liquidity.switchboard_oracle_pubkey,
        )
    }));

    // refresh obligation
    instructions.push(refresh_obligation(
        config.lending_program_id,
        obligation_pubkey,
        reserve_pubkeys,
    ));

    instructions.push(liquidate_obligation_and_redeem_reserve_collateral(
        config.lending_program_id,
        liquidity_amount,
        source_liquidity_pubkey,
        destination_collateral_pubkey,
        destination_liquidity_pubkey,
        repay_reserve_pubkey,
        repay_reserve_state.liquidity.supply_pubkey,
        withdraw_reserve_pubkey,
        withdraw_reserve_state.collateral.mint_pubkey,
        withdraw_reserve_state.collateral.supply_pubkey,
        withdraw_reserve_state.liquidity.supply_pubkey,
        withdraw_reserve_state.config.fee_receiver,
        obligation_pubkey,
        obligation_state.lending_market,
        config.fee_payer.pubkey(),
    ));

    let recent_blockhash = config.rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new(
        &vec![config.fee_payer.as_ref()],
        Message::new_with_blockhash(
            &instructions,
            Some(&config.fee_payer.pubkey()),
            &recent_blockhash,
        ),
        recent_blockhash,
    );

    send_transaction(config, transaction)?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn command_add_reserve(
    config: &mut Config,
    liquidity_amount: u64,
    reserve_config: ReserveConfig,
    source_liquidity_pubkey: Pubkey,
    source_liquidity_owner_keypair: Keypair,
    lending_market_pubkey: Pubkey,
    lending_market_owner_keypair: Keypair,
    pyth_product_pubkey: Pubkey,
    pyth_price_pubkey: Pubkey,
    switchboard_feed_pubkey: Pubkey,
    liquidity_fee_receiver_keypair: Keypair,
    source_liquidity: Token,
) -> CommandResult {
    let reserve_keypair = Keypair::new();
    let collateral_mint_keypair = Keypair::new();
    let collateral_supply_keypair = Keypair::new();
    let liquidity_supply_keypair = Keypair::new();
    let user_collateral_keypair = Keypair::new();
    let user_transfer_authority_keypair = Keypair::new();

    println!("Adding reserve {}", reserve_keypair.pubkey());
    if config.verbose {
        println!(
            "Adding collateral mint {}",
            collateral_mint_keypair.pubkey()
        );
        println!(
            "Adding collateral supply {}",
            collateral_supply_keypair.pubkey()
        );
        println!(
            "Adding liquidity supply {}",
            liquidity_supply_keypair.pubkey()
        );
        println!(
            "Adding liquidity fee receiver {}",
            liquidity_fee_receiver_keypair.pubkey()
        );
        println!(
            "Adding user collateral {}",
            user_collateral_keypair.pubkey()
        );
        println!(
            "Adding user transfer authority {}",
            user_transfer_authority_keypair.pubkey()
        );
    }

    let reserve_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(Reserve::LEN)?;
    let collateral_mint_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    let token_account_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(Token::LEN)?;
    let collateral_supply_balance = token_account_balance;
    let user_collateral_balance = token_account_balance;
    let liquidity_supply_balance = token_account_balance;
    let liquidity_fee_receiver_balance = token_account_balance;

    let total_balance = reserve_balance
        + collateral_mint_balance
        + collateral_supply_balance
        + user_collateral_balance
        + liquidity_supply_balance
        + liquidity_fee_receiver_balance;
    let recent_blockhash = config.rpc_client.get_latest_blockhash()?;

    let message_1 = Message::new_with_blockhash(
        &[
            create_account(
                &config.fee_payer.pubkey(),
                &reserve_keypair.pubkey(),
                reserve_balance,
                Reserve::LEN as u64,
                &config.lending_program_id,
            ),
            create_account(
                &config.fee_payer.pubkey(),
                &collateral_mint_keypair.pubkey(),
                collateral_mint_balance,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            create_account(
                &config.fee_payer.pubkey(),
                &collateral_supply_keypair.pubkey(),
                collateral_supply_balance,
                Token::LEN as u64,
                &spl_token::id(),
            ),
            create_account(
                &config.fee_payer.pubkey(),
                &user_collateral_keypair.pubkey(),
                user_collateral_balance,
                Token::LEN as u64,
                &spl_token::id(),
            ),
        ],
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );

    let message_2 = Message::new_with_blockhash(
        &[
            create_account(
                &config.fee_payer.pubkey(),
                &liquidity_supply_keypair.pubkey(),
                liquidity_supply_balance,
                Token::LEN as u64,
                &spl_token::id(),
            ),
            create_account(
                &config.fee_payer.pubkey(),
                &liquidity_fee_receiver_keypair.pubkey(),
                liquidity_fee_receiver_balance,
                Token::LEN as u64,
                &spl_token::id(),
            ),
        ],
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );

    let message_3 = Message::new_with_blockhash(
        &[
            approve(
                &spl_token::id(),
                &source_liquidity_pubkey,
                &user_transfer_authority_keypair.pubkey(),
                &source_liquidity_owner_keypair.pubkey(),
                &[],
                liquidity_amount,
            )
            .unwrap(),
            init_reserve(
                config.lending_program_id,
                liquidity_amount,
                reserve_config,
                source_liquidity_pubkey,
                user_collateral_keypair.pubkey(),
                reserve_keypair.pubkey(),
                source_liquidity.mint,
                liquidity_supply_keypair.pubkey(),
                collateral_mint_keypair.pubkey(),
                collateral_supply_keypair.pubkey(),
                pyth_product_pubkey,
                pyth_price_pubkey,
                switchboard_feed_pubkey,
                lending_market_pubkey,
                lending_market_owner_keypair.pubkey(),
                user_transfer_authority_keypair.pubkey(),
            ),
            revoke(
                &spl_token::id(),
                &source_liquidity_pubkey,
                &source_liquidity_owner_keypair.pubkey(),
                &[],
            )
            .unwrap(),
        ],
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );

    check_fee_payer_balance(
        config,
        total_balance
            + config.rpc_client.get_fee_for_message(&message_1)?
            + config.rpc_client.get_fee_for_message(&message_2)?
            + config.rpc_client.get_fee_for_message(&message_3)?,
    )?;

    let transaction_1 = Transaction::new(
        &vec![
            config.fee_payer.as_ref(),
            &reserve_keypair,
            &collateral_mint_keypair,
            &collateral_supply_keypair,
            &user_collateral_keypair,
        ],
        message_1,
        recent_blockhash,
    );
    send_transaction(config, transaction_1)?;
    let transaction_2 = Transaction::new(
        &vec![
            config.fee_payer.as_ref(),
            &liquidity_supply_keypair,
            &liquidity_fee_receiver_keypair,
        ],
        message_2,
        recent_blockhash,
    );
    send_transaction(config, transaction_2)?;
    let transaction_3 = Transaction::new(
        &vec![
            config.fee_payer.as_ref(),
            &source_liquidity_owner_keypair,
            &lending_market_owner_keypair,
            &user_transfer_authority_keypair,
        ],
        message_3,
        recent_blockhash,
    );
    send_transaction(config, transaction_3)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn command_set_lending_market_owner_and_config(
    config: &mut Config,
    lending_market_pubkey: Pubkey,
    lending_market_owner_keypair: Keypair,
    new_lending_market_owner_keypair: Option<Keypair>,
    rate_limiter_window_duration: Option<u64>,
    rate_limiter_max_outflow: Option<u64>,
    whitelisted_liquidator_pubkey: Option<Pubkey>,
    risk_authority_pubkey: Pubkey,
) -> CommandResult {
    let lending_market_info = config.rpc_client.get_account(&lending_market_pubkey)?;
    let lending_market = LendingMarket::unpack_from_slice(lending_market_info.data.borrow())?;
    println!("{:#?}", lending_market);

    let recent_blockhash = config.rpc_client.get_latest_blockhash()?;
    let message = Message::new_with_blockhash(
        &[set_lending_market_owner_and_config(
            config.lending_program_id,
            lending_market_pubkey,
            lending_market_owner_keypair.pubkey(),
            if let Some(owner) = new_lending_market_owner_keypair {
                owner.pubkey()
            } else {
                lending_market.owner
            },
            RateLimiterConfig {
                window_duration: rate_limiter_window_duration
                    .unwrap_or(lending_market.rate_limiter.config.window_duration),
                max_outflow: rate_limiter_max_outflow
                    .unwrap_or(lending_market.rate_limiter.config.max_outflow),
            },
            whitelisted_liquidator_pubkey,
            risk_authority_pubkey,
        )],
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );

    let transaction = Transaction::new(
        &vec![config.fee_payer.as_ref(), &lending_market_owner_keypair],
        message,
        recent_blockhash,
    );

    send_transaction(config, transaction)?;
    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::unnecessary_unwrap)]
fn command_update_reserve(
    config: &mut Config,
    reserve_config: PartialReserveConfig,
    pyth_product_pubkey: Option<Pubkey>,
    pyth_price_pubkey: Option<Pubkey>,
    switchboard_feed_pubkey: Option<Pubkey>,
    reserve_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    lending_market_owner_keypair: Keypair,
) -> CommandResult {
    let reserve_info = config.rpc_client.get_account(&reserve_pubkey)?;
    let mut reserve = Reserve::unpack_from_slice(reserve_info.data.borrow())?;
    println!("Reserve: {:#?}", reserve);
    let mut no_change = true;
    if reserve_config.optimal_utilization_rate.is_some()
        && reserve.config.optimal_utilization_rate
            != reserve_config.optimal_utilization_rate.unwrap()
    {
        no_change = false;
        println!(
            "Updating optimal_utilization_rate from {} to {}",
            reserve.config.optimal_utilization_rate,
            reserve_config.optimal_utilization_rate.unwrap(),
        );
        reserve.config.optimal_utilization_rate = reserve_config.optimal_utilization_rate.unwrap();
    }

    if reserve_config.max_utilization_rate.is_some()
        && reserve.config.max_utilization_rate != reserve_config.max_utilization_rate.unwrap()
    {
        no_change = false;
        println!(
            "Updating max_utilization_rate from {} to {}",
            reserve.config.max_utilization_rate,
            reserve_config.max_utilization_rate.unwrap(),
        );
        reserve.config.max_utilization_rate = reserve_config.max_utilization_rate.unwrap();
    }

    if reserve_config.loan_to_value_ratio.is_some()
        && reserve.config.loan_to_value_ratio != reserve_config.loan_to_value_ratio.unwrap()
    {
        no_change = false;
        println!(
            "Updating loan_to_value_ratio from {} to {}",
            reserve.config.loan_to_value_ratio,
            reserve_config.loan_to_value_ratio.unwrap(),
        );
        reserve.config.loan_to_value_ratio = reserve_config.loan_to_value_ratio.unwrap();
    }

    if reserve_config.liquidation_bonus.is_some()
        && reserve.config.liquidation_bonus != reserve_config.liquidation_bonus.unwrap()
    {
        no_change = false;
        println!(
            "Updating liquidation_bonus from {} to {}",
            reserve.config.liquidation_bonus,
            reserve_config.liquidation_bonus.unwrap(),
        );
        reserve.config.liquidation_bonus = reserve_config.liquidation_bonus.unwrap();
    }

    if reserve_config.max_liquidation_bonus.is_some()
        && reserve.config.max_liquidation_bonus != reserve_config.max_liquidation_bonus.unwrap()
    {
        no_change = false;
        println!(
            "Updating max_liquidation_bonus from {} to {}",
            reserve.config.max_liquidation_bonus,
            reserve_config.max_liquidation_bonus.unwrap(),
        );
        reserve.config.max_liquidation_bonus = reserve_config.max_liquidation_bonus.unwrap();
    }

    if reserve_config.liquidation_threshold.is_some()
        && reserve.config.liquidation_threshold != reserve_config.liquidation_threshold.unwrap()
    {
        no_change = false;
        println!(
            "Updating liquidation_threshold from {} to {}",
            reserve.config.liquidation_threshold,
            reserve_config.liquidation_threshold.unwrap(),
        );
        reserve.config.liquidation_threshold = reserve_config.liquidation_threshold.unwrap();
    }

    if reserve_config.max_liquidation_threshold.is_some()
        && reserve.config.max_liquidation_threshold
            != reserve_config.max_liquidation_threshold.unwrap()
    {
        no_change = false;
        println!(
            "Updating max_liquidation_threshold from {} to {}",
            reserve.config.max_liquidation_threshold,
            reserve_config.max_liquidation_threshold.unwrap(),
        );
        reserve.config.max_liquidation_threshold =
            reserve_config.max_liquidation_threshold.unwrap();
    }

    if reserve_config.min_borrow_rate.is_some()
        && reserve.config.min_borrow_rate != reserve_config.min_borrow_rate.unwrap()
    {
        no_change = false;
        println!(
            "Updating min_borrow_rate from {} to {}",
            reserve.config.min_borrow_rate,
            reserve_config.min_borrow_rate.unwrap(),
        );
        reserve.config.min_borrow_rate = reserve_config.min_borrow_rate.unwrap();
    }

    if reserve_config.optimal_borrow_rate.is_some()
        && reserve.config.optimal_borrow_rate != reserve_config.optimal_borrow_rate.unwrap()
    {
        no_change = false;
        println!(
            "Updating optimal_borrow_rate from {} to {}",
            reserve.config.optimal_borrow_rate,
            reserve_config.optimal_borrow_rate.unwrap(),
        );
        reserve.config.optimal_borrow_rate = reserve_config.optimal_borrow_rate.unwrap();
    }

    if reserve_config.max_borrow_rate.is_some()
        && reserve.config.max_borrow_rate != reserve_config.max_borrow_rate.unwrap()
    {
        no_change = false;
        println!(
            "Updating max_borrow_rate from {} to {}",
            reserve.config.max_borrow_rate,
            reserve_config.max_borrow_rate.unwrap(),
        );
        reserve.config.max_borrow_rate = reserve_config.max_borrow_rate.unwrap();
    }

    if reserve_config.super_max_borrow_rate.is_some()
        && reserve.config.super_max_borrow_rate != reserve_config.super_max_borrow_rate.unwrap()
    {
        no_change = false;
        println!(
            "Updating super_max_borrow_rate from {} to {}",
            reserve.config.super_max_borrow_rate,
            reserve_config.super_max_borrow_rate.unwrap(),
        );
        reserve.config.super_max_borrow_rate = reserve_config.super_max_borrow_rate.unwrap();
    }

    if reserve_config.fees.borrow_fee_wad.is_some()
        && reserve.config.fees.borrow_fee_wad != reserve_config.fees.borrow_fee_wad.unwrap()
    {
        no_change = false;
        println!(
            "Updating borrow_fee_wad from {} to {}",
            reserve.config.fees.borrow_fee_wad,
            reserve_config.fees.borrow_fee_wad.unwrap(),
        );
        reserve.config.fees.borrow_fee_wad = reserve_config.fees.borrow_fee_wad.unwrap();
    }

    if reserve_config.fees.flash_loan_fee_wad.is_some()
        && reserve.config.fees.flash_loan_fee_wad != reserve_config.fees.flash_loan_fee_wad.unwrap()
    {
        no_change = false;
        println!(
            "Updating flash_loan_fee_wad from {} to {}",
            reserve.config.fees.flash_loan_fee_wad,
            reserve_config.fees.flash_loan_fee_wad.unwrap(),
        );
        reserve.config.fees.flash_loan_fee_wad = reserve_config.fees.flash_loan_fee_wad.unwrap();
    }

    if reserve_config.fees.host_fee_percentage.is_some()
        && reserve.config.fees.host_fee_percentage
            != reserve_config.fees.host_fee_percentage.unwrap()
    {
        no_change = false;
        println!(
            "Updating host_fee_percentage from {} to {}",
            reserve.config.fees.host_fee_percentage,
            reserve_config.fees.host_fee_percentage.unwrap(),
        );
        reserve.config.fees.host_fee_percentage = reserve_config.fees.host_fee_percentage.unwrap();
    }

    if reserve_config.deposit_limit.is_some()
        && reserve.config.deposit_limit != reserve_config.deposit_limit.unwrap()
    {
        no_change = false;
        println!(
            "Updating deposit_limit from {} to {}",
            amount_to_ui_amount(
                reserve.config.deposit_limit,
                reserve.liquidity.mint_decimals
            ),
            reserve_config.deposit_limit.unwrap(),
        );
        reserve.config.deposit_limit = ui_amount_to_amount(
            reserve_config.deposit_limit.unwrap() as f64,
            reserve.liquidity.mint_decimals,
        )
    }

    if reserve_config.borrow_limit.is_some()
        && reserve.config.borrow_limit != reserve_config.borrow_limit.unwrap()
    {
        no_change = false;
        println!(
            "Updating borrow_limit from {} to {}",
            amount_to_ui_amount(reserve.config.borrow_limit, reserve.liquidity.mint_decimals),
            reserve_config.borrow_limit.unwrap(),
        );
        reserve.config.borrow_limit = ui_amount_to_amount(
            reserve_config.borrow_limit.unwrap() as f64,
            reserve.liquidity.mint_decimals,
        )
    }

    if reserve_config.fee_receiver.is_some()
        && reserve.config.fee_receiver != reserve_config.fee_receiver.unwrap()
    {
        no_change = false;
        println!(
            "Updating fee_receiver from {} to {}",
            reserve.config.fee_receiver,
            reserve_config.fee_receiver.unwrap(),
        );
        reserve.config.fee_receiver = reserve_config.fee_receiver.unwrap();
    }

    if reserve_config.protocol_liquidation_fee.is_some()
        && reserve.config.protocol_liquidation_fee
            != reserve_config.protocol_liquidation_fee.unwrap()
    {
        no_change = false;
        println!(
            "Updating protocol_liquidation_fee from {} to {}",
            reserve.config.protocol_liquidation_fee,
            reserve_config.protocol_liquidation_fee.unwrap(),
        );
        reserve.config.protocol_liquidation_fee = reserve_config.protocol_liquidation_fee.unwrap();
    }

    if reserve_config.protocol_take_rate.is_some()
        && reserve.config.protocol_take_rate != reserve_config.protocol_take_rate.unwrap()
    {
        no_change = false;
        println!(
            "Updating protocol_take_rate from {} to {}",
            reserve.config.protocol_take_rate,
            reserve_config.protocol_take_rate.unwrap(),
        );
        reserve.config.protocol_take_rate = reserve_config.protocol_take_rate.unwrap();
    }

    let mut new_pyth_product_pubkey = solend_sdk::NULL_PUBKEY;
    if pyth_price_pubkey.is_some() {
        no_change = false;
        println!(
            "Updating pyth oracle pubkey from {} to {}",
            reserve.liquidity.pyth_oracle_pubkey,
            pyth_price_pubkey.unwrap(),
        );
        reserve.liquidity.pyth_oracle_pubkey = pyth_price_pubkey.unwrap();
        new_pyth_product_pubkey = pyth_product_pubkey.unwrap();
    }

    if switchboard_feed_pubkey.is_some() {
        no_change = false;
        println!(
            "Updating switchboard_oracle_pubkey {} to {}",
            reserve.liquidity.switchboard_oracle_pubkey,
            switchboard_feed_pubkey.unwrap(),
        );
        reserve.liquidity.switchboard_oracle_pubkey = switchboard_feed_pubkey.unwrap();
    }

    if reserve_config.rate_limiter_window_duration.is_some()
        && reserve.rate_limiter.config.window_duration
            != reserve_config.rate_limiter_window_duration.unwrap()
    {
        no_change = false;
        println!(
            "Updating rate_limiter_window_duration from {} to {}",
            reserve.rate_limiter.config.window_duration,
            reserve_config.rate_limiter_window_duration.unwrap(),
        );
        reserve.rate_limiter.config.window_duration =
            reserve_config.rate_limiter_window_duration.unwrap();
    }

    if reserve_config.rate_limiter_max_outflow.is_some()
        && reserve.rate_limiter.config.max_outflow
            != reserve_config.rate_limiter_max_outflow.unwrap()
    {
        no_change = false;
        println!(
            "Updating rate_limiter_max_outflow from {} to {}",
            reserve.rate_limiter.config.max_outflow,
            reserve_config.rate_limiter_max_outflow.unwrap(),
        );
        reserve.rate_limiter.config.max_outflow = reserve_config.rate_limiter_max_outflow.unwrap();
    }

    if reserve_config.added_borrow_weight_bps.is_some()
        && reserve.config.added_borrow_weight_bps != reserve_config.added_borrow_weight_bps.unwrap()
    {
        no_change = false;
        println!(
            "Updating added_borrow_weight_bps from {} to {}",
            reserve.config.added_borrow_weight_bps,
            reserve_config.added_borrow_weight_bps.unwrap(),
        );
        reserve.config.added_borrow_weight_bps = reserve_config.added_borrow_weight_bps.unwrap();
    }

    if reserve_config.reserve_type.is_some()
        && reserve.config.reserve_type != reserve_config.reserve_type.unwrap()
    {
        no_change = false;
        println!(
            "Updating reserve_type from {:?} to {:?}",
            reserve.config.reserve_type,
            reserve_config.reserve_type.unwrap(),
        );
        reserve.config.reserve_type = reserve_config.reserve_type.unwrap();
    }

    if validate_reserve_config(reserve.config).is_err() {
        println!("Error: invalid reserve config");
        return Err("Error: invalid reserve config".into());
    }

    if no_change {
        println!("No changes made for reserve {}", reserve_pubkey);
        return Ok(());
    }

    let recent_blockhash = config.rpc_client.get_latest_blockhash()?;

    let message = Message::new_with_blockhash(
        &[update_reserve_config(
            config.lending_program_id,
            reserve.config,
            RateLimiterConfig {
                window_duration: reserve.rate_limiter.config.window_duration,
                max_outflow: reserve.rate_limiter.config.max_outflow,
            },
            reserve_pubkey,
            lending_market_pubkey,
            lending_market_owner_keypair.pubkey(),
            new_pyth_product_pubkey,
            reserve.liquidity.pyth_oracle_pubkey,
            reserve.liquidity.switchboard_oracle_pubkey,
        )],
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );

    let transaction = Transaction::new(
        &vec![config.fee_payer.as_ref(), &lending_market_owner_keypair],
        message,
        recent_blockhash,
    );

    send_transaction(config, transaction)?;
    Ok(())
}

// HELPERS

fn check_fee_payer_balance(config: &Config, required_balance: u64) -> Result<(), Error> {
    let balance = config.rpc_client.get_balance(&config.fee_payer.pubkey())?;
    if balance < required_balance {
        Err(format!(
            "Fee payer, {}, has insufficient balance: {} required, {} available",
            config.fee_payer.pubkey(),
            lamports_to_sol(required_balance),
            lamports_to_sol(balance)
        )
        .into())
    } else {
        Ok(())
    }
}

fn send_transaction(
    config: &Config,
    transaction: Transaction,
) -> solana_client::client_error::Result<()> {
    if config.dry_run {
        let result = config.rpc_client.simulate_transaction(&transaction)?;
        println!("Simulate result: {:?}", result);
    } else {
        let signature = config
            .rpc_client
            .send_and_confirm_transaction_with_spinner_and_config(
                &transaction,
                CommitmentConfig::confirmed(),
                RpcSendTransactionConfig {
                    preflight_commitment: Some(CommitmentLevel::Processed),
                    skip_preflight: true,
                    encoding: None,
                    max_retries: None,
                    min_context_slot: None,
                },
            )?;
        println!("Signature: {}", signature);
    }
    Ok(())
}

fn quote_currency_of(matches: &ArgMatches<'_>, name: &str) -> Option<[u8; 32]> {
    if let Some(value) = matches.value_of(name) {
        if value == "USD" {
            Some(*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0")
        } else if value.len() <= 32 {
            let mut bytes32 = [0u8; 32];
            bytes32[0..value.len()].clone_from_slice(value.as_bytes());
            Some(bytes32)
        } else {
            Some(Pubkey::from_str(value).unwrap().to_bytes())
        }
    } else {
        None
    }
}

fn get_or_create_associated_token_address(config: &Config, mint: &Pubkey) -> Pubkey {
    let ata = get_associated_token_address(&config.fee_payer.pubkey(), mint);

    if config.rpc_client.get_account(&ata).is_err() {
        println!("Creating ATA for mint {:?}", mint);

        let recent_blockhash = config.rpc_client.get_latest_blockhash().unwrap();
        let transaction = Transaction::new(
            &vec![config.fee_payer.as_ref()],
            Message::new_with_blockhash(
                &[create_associated_token_account(
                    &config.fee_payer.pubkey(),
                    &config.fee_payer.pubkey(),
                    mint,
                    &spl_associated_token_account::id(),
                )],
                Some(&config.fee_payer.pubkey()),
                &recent_blockhash,
            ),
            recent_blockhash,
        );

        send_transaction(config, transaction).unwrap();
    }

    ata
}
