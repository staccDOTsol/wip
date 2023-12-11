import React, { useEffect, useState } from 'react';
import { SolendMarket,createmObligationAddress, depositReserveLiquidityInstruction,borrowObligationLiquidityInstruction, refreshObligationInstruction, refreshReserveInstruction} from '@solendprotocol/solend-sdk'
import { theIdl } from './theIdl';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
// Import other necessary libraries and dependencies
import {  AddressLookupTableProgram, ComputeBudgetInstruction, ComputeBudgetProgram, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey,  SYSVAR_CLOCK_PUBKEY,  SYSVAR_RENT_PUBKEY,  SystemProgram,  Transaction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { AccountType, getConfig, MarginfiClient} from "@mrgnlabs/marginfi-client-v2";
import { NATIVE_MINT, TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, createInitializeAccountInstruction, createSyncNativeInstruction, getMinimumBalanceForRentExemptAccount, shortenAddress, parseOracleSetup } from "@mrgnlabs/mrgn-common";
import { ACCOUNT_SIZE, TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddress } from '@solana/spl-token'
import { WalletConnectButton, WalletDisconnectButton, WalletModalButton } from '@solana/wallet-adapter-react-ui';
import * as solanaStakePool from '@solana/spl-stake-pool';
import { Marinade, MarinadeConfig, Wallet } from '@marinade.finance/marinade-ts-sdk'

import '@solana/wallet-adapter-react-ui/styles.css';
import { MarinadeFinanceProgram } from '@marinade.finance/marinade-ts-sdk/dist/src/programs/marinade-finance-program';
import { SYSTEM_PROGRAM_ID } from '@marinade.finance/marinade-ts-sdk/dist/src/util';
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { BN } from 'bn.js';

async function deriveObligationAddressFromWalletAndSeed(
  walletAddress,
  lendingMarketPubkey,
) {
  return PublicKey.createWithSeed(
    walletAddress,
    lendingMarketPubkey.toString().slice(0, 32),
    SOLEND_PROGRAM_ID
  )
}
async function createObligationAccount(
  fundingAddress,
  walletAddress,
  lendingMarketPubkey,
  program
){
  const newAccountPubkey = await deriveObligationAddressFromWalletAndSeed(
    walletAddress,
    lendingMarketPubkey
  )

  const {
    seed,
    lamports,
    space,
  } = {
    lamports: 9938880,
    space: 1300,
    seed: lendingMarketPubkey.toString().slice(0, 32),
  }

  const [marginfi_pda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("marginfi")],
    new PublicKey('GQQ5gDjd1vYKk257qJLJmrsTkiNZQZjC8btN5SHfhpNL')
  );
  const derivedInputs = await deriveInputs(marginfi_pda, fundingAddress, seed, SOLEND_PROGRAM_ID)
try {
  await program.methods.createSeededAccount({
    seed: seed,
    lamports: new BN(lamports),
    space: new BN(space),
    bump: new BN(bump),
  })
  .accounts({
    program: marginfi_pda,
    from: derivedInputs.fromPubkey,
    to: derivedInputs.toPubkey,
    base:  derivedInputs.base,
    owner: derivedInputs.owner,
    systemProgram: SystemProgram.programId,
    lendingMarket: lendingMarketPubkey,
    solendSdk: SOLEND_PROGRAM_ID,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY
  })
  .rpc({skipPreflight: true, commitment: 'confirmed'})
}
catch (err){
  console.log(err)
}
await program.methods.initObligationAccount({
  seed: seed,
  lamports: new BN(lamports),
  space: new BN(space),
  bump: new BN(bump),
})
.accounts({
  program: marginfi_pda,
  from: derivedInputs.fromPubkey,
  to: derivedInputs.toPubkey,
  base:  derivedInputs.base,
  owner: derivedInputs.owner,
  systemProgram: SystemProgram.programId,
  lendingMarket: lendingMarketPubkey,
  solendSdk: SOLEND_PROGRAM_ID,
  tokenProgram: TOKEN_PROGRAM_ID,
  rent: SYSVAR_RENT_PUBKEY
})
.rpc({skipPreflight: true, commitment: 'confirmed'})
  return SystemProgram.createAccountWithSeed({
    basePubkey: walletAddress,
    fromPubkey: fundingAddress,
    newAccountPubkey,
    programId: SOLEND_PROGRAM_ID,
    seed,
    lamports,
    space,
  })
}
const SOLEND_PROGRAM_ID = new PublicKey("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo")

// import wallet adapter react ui css
const LOOKUP_TABLE_ADDRESS = new PublicKey("37pFNGm75HgEZz32xezCZp945Fac4g3UkRFqSdWJ6N3n") // await createLookupTable()

async function deriveInputs(base, wallet, seed, programId) {
  // Generate a new Keypair for the 'from' account (funding account)
  const fromPubkey = wallet;
//  let nft_address =     format!("{}", ctx.accounts.nft_address)[..32].to_string();

  // The base account (can be the same as the from account or different)

  // Generate the 'to' account (new account) public key
  const toPubkey = await PublicKey.createWithSeed(base, seed, programId);
  const [pda, bump] = PublicKey.findProgramAddressSync([base.toBuffer(), Buffer.from(seed)], programId)
  console.log(pda.toBase58())
  console.log(bump)
  // Specify lamports (e.g., funding the account with 1 SOL)
  const lamports = 0.01 * LAMPORTS_PER_SOL; // 1 SOL in lamports

  // Specify space in bytes for the account data
  const space = 0; // For example, 1024 bytes

  // The owner program's public key
  const owner = programId

  return {
      fromPubkey,
      toPubkey,
      base: base,
      seed,
      lamports,
      space,
      owner,
  };
}
async function mainY(connection, wallet, amount, program) {
  

  const config = await getConfig("production");
  const client = await MarginfiClient.fetch(config, wallet, connection);
  
  let mint1 = NATIVE_MINT;
  const bank1 = client.getBankByMint(mint1);
  if (!bank1) throw Error(`${mint1.toBase58()} bank not found`);
  const bankLabel2 = "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1";
  let mint2 = new PublicKey(bankLabel2);
  const bank2 = client.getBankByMint(mint2);
  const bankLabel3 = "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn";
  let mint3 = new PublicKey(bankLabel3);
  const bank3 = client.getBankByMint(mint3);
  const bankLabel4 = "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So";
  let mint4 = new PublicKey(bankLabel4);
  const bank4 = client.getBankByMint(mint4);
  const bankLabel5 = "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj";
  let mint5 = new PublicKey(bankLabel5);
  const bank5 = client.getBankByMint(mint5);
let index1, index2 

  let c = 0
  for (var bankmap of client.banks){
    if (bankmap[1].address.equals(bank2.address)){
      index2 = c
    }
    if (bankmap[1].address.equals(bank1.address)){
      index1 = c
    }
    c++
  }
  console.log(index1, index2)

  if (!bank2) throw Error(`${bankLabel2} bank not found`);

  if (!bank3) throw Error(`${bankLabel3} bank not found`);

  if (!bank4) throw Error(`${bankLabel4} bank not found`);

  if (!bank5) throw Error(`${bankLabel5} bank not found`);
  let fundingAccount = await getAssociatedTokenAddress(
    NATIVE_MINT,
    wallet.publicKey
  )


  const bsolPayload = {
    stakePool: new PublicKey("stk9ApL5HeVAwPLr3TLhDXdZS8ptVu7zp6ov8HFDuMi"),
    withdrawAuthority: new PublicKey("6WecYymEARvjG5ZyqkrVQ6YkhPfujNzWpSPwNKXHCbV2"),
    reserveStake: new PublicKey("rsrxDvYUXjH1RQj2Ke36LNZEVqGztATxFkqNukERqFT"),
    fundingAccount: wallet.publicKey,
    destinationPoolAccount: await getAssociatedTokenAddress(
      mint2,
      wallet.publicKey
    ),
    managerFeeAccount: new PublicKey("Dpo148tVGewDPyh2FkGV18gouWctbdX2fHJopJGe9xv1"),
    referralPoolAccount:  await getAssociatedTokenAddress(
      mint2,
      new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
    ),
    poolMint: mint2,
    lamports: amount * 10 ** 9,
  };
  const jitoSolPayload = {
    stakePool: new PublicKey("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb"),
    withdrawAuthority: new PublicKey("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS"),
    reserveStake: new PublicKey("BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL"),
    fundingAccount: wallet.publicKey,
    destinationPoolAccount: await getAssociatedTokenAddress(
      mint3,
      wallet.publicKey
    ),
    managerFeeAccount: new PublicKey("feeeFLLsam6xZJFc6UQFrHqkvVt4jfmVvi2BRLkUZ4i"),
    referralPoolAccount:  await getAssociatedTokenAddress(
      mint3,
      new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
    ),
    poolMint: mint3,
    lamports: (amount * (1-0.008) * 0.74) * 10 ** 9,
  };
  let fundingAccountBalance = await connection.getBalance(fundingAccount)
  console.log(fundingAccountBalance)

  /*
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,
        seeds = [SEED_PREFIX],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    #[account(init_if_needed,
        payer = signer,
        token::authority = marginfi_pda,
        token::mint = pool_mint,
    )]
    pub pool_token_receiver_account: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = Pubkey::from_str("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6").unwrap())]
    /// CHECK: no validation, for educational purpose only
    pub referrer : AccountInfo<'info>,
    #[account(init_if_needed, token::authority = referrer,
        token::mint = pool_mint,
        payer = signer,
    )]
    pub referrer_token_account: Box<Account<'info, TokenAccount>>,
  

    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub stake_pool: AccountInfo<'info>,
    /// CHECK: Checked by CPI to Spl Stake Program
    pub stake_pool_withdraw_authority: AccountInfo<'info>,
    /// CHECK: Checked by CPI to Spl Stake Program
    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub reserve_stake_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Checked by CPI to Spl Stake Program
    pub manager_fee_account: AccountInfo<'info>,
    #[account(mut)]
    pub pool_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    pub stake_pool_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
     /// CHECK: no validation, for educational purpose only
     pub marginfi_program: AccountInfo<'info>,

     /// CHECK: no validation, for educational purpose only
     #[account(mut)]
     pub marginfi_account: AccountInfo<'info>,
     /// CHECK: no validation, for educational purpose only
     #[account(mut)]
     pub marginfi_bank: Box<Account<'info, TokenAccount>>,
     #[account(mut)]
     /// CHECK: no validation, for educational purpose only
     pub liquidity_vault: Box<Account<'info, TokenAccount>>,

     /// CHECK: no validation, for educational purpose only
     #[account(mut)]
     pub marginfi_group: AccountInfo<'info>,
     /// CHECK: no validation, for educational purpose only
     #[account(mut)]
     pub funding_account: AccountInfo<'info>,

     /// CHECK: no validation, for educational purpose only
     #[account(mut)]
     pub token_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub marginfi_bank_wsol: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed,
        payer = signer,
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
    pub reserve_stake_account_jitosol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub manager_fee_account_jitosol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_jitosol: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK:
    pub stake_pool_program_jitosol: AccountInfo<'info>,
    #[account(init_if_needed,
        payer = signer,
        token::authority = marginfi_pda,
        token::mint = pool_mint_jitosol
        
    )]
    pub pool_token_receiver_account_jitosol: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, token::authority = referrer,
        token::mint = pool_mint_jitosol,
        payer = signer,
        
        
    )]
    pub referrer_token_account_jitosol: Box<Account<'info, TokenAccount>>,
    /// CHECK: Checked by CPI to Spl Stake Program
    pub stake_pool_withdraw_authority_wsol: AccountInfo<'info>,
    /// CHECK: Checked by CPI to Spl Stake Program
    pub bank_liquidity_vault_authority_wsol: AccountInfo<'info>,
    
    pub jarezi_mint: Box<InterfaceAccount<'info, anchor_spl::token_interface::Mint>>,
    #[account(init_if_needed,
        payer = signer,
        token::authority = signer,
        token::mint = jarezi_mint,
        token::token_program = token_program_2022
    )]
    pub jarezi_token_account: Box<InterfaceAccount<'info, anchor_spl::token_interface::TokenAccount>>,
    pub token_program_2022: Program<'info, Token2022>,

}
*/
const [marginfi_pda, bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("marginfi")],
  new PublicKey('GQQ5gDjd1vYKk257qJLJmrsTkiNZQZjC8btN5SHfhpNL')
);
let marginfiAccount = new PublicKey("9mYyaKmfjJsaAAM6StZUy2JGg5vWTJjkNWfhuaxs4Ct2")
let tx = new Transaction()
let maybe = await getAssociatedTokenAddress(
  mint2,
  marginfi_pda,
  true
)
let associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      marginfi_pda,
      mint2
      );
      tx.add(ixx)
  }
maybe = await getAssociatedTokenAddress(
  mint2,
  new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
)
associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6"),
      mint2
      );
      tx.add(ixx)
  }
maybe = await getAssociatedTokenAddress(
  mint1,
  marginfi_pda,
  true
)
associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      marginfi_pda,
      mint1
      );
      tx.add(ixx)
  }
maybe = await getAssociatedTokenAddress(
  
  mint3,
  marginfi_pda,
  true
)
associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      marginfi_pda,
      mint3
      );
      tx.add(ixx)
  } 
maybe = await getAssociatedTokenAddress(
  
  mint3,
  new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
)
associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6"),
      mint3
      );
      tx.add(ixx)

  }
maybe = await getAssociatedTokenAddress(
  new PublicKey('5tR1kBz9gFxuZ5ZSRbchi96xhTerpSyRVwphUfBajC3L'),
  wallet.publicKey,
  true,
  TOKEN_2022_PROGRAM_ID
)
associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      wallet.publicKey,
      new PublicKey('5tR1kBz9gFxuZ5ZSRbchi96xhTerpSyRVwphUfBajC3L'),
      TOKEN_2022_PROGRAM_ID
      );
      tx.add(ixx)
  }

if (tx.instructions.length > 0){
  tx.feePayer = wallet.publicKey
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  let sig1 = await client.provider.sendAndConfirm(tx, [], {skipPreflight: false, commitment: 'confirmed'})
  console.log(sig1);
}
let wsolPrice = client.getOraclePriceByBank(bank1.address);
let bsolPrice = client.getOraclePriceByBank(bank2.address);
let jitoPrice = client.getOraclePriceByBank(bank3.address);

const derivedInputs = await deriveInputs(marginfi_pda, wallet.publicKey, "robot001", SystemProgram.programId)

const market = await SolendMarket.initialize(connection, "production")
let obligationAddress
let ixx 
const reserve = market.reserves.find((r) => r.config.liquidityToken.mint == NATIVE_MINT.toBase58())
const reservebsol = market.reserves.find((r) => r.config.liquidityToken.mint == mint2.toBase58())
  obligationAddress = await deriveObligationAddressFromWalletAndSeed(
    marginfi_pda,
    new PublicKey(market.config.address)
  )

console.log(obligationAddress.toBase58())

    await market.loadReserves()
    console.log(reserve.config)
console.log(market)
let obRefresh = await refreshObligationInstruction(
  obligationAddress,
  [],
  [],
  SOLEND_PROGRAM_ID,
)
let refresh = await refreshReserveInstruction (
  new PublicKey(reserve.config.address),
  SOLEND_PROGRAM_ID,
  new PublicKey(reserve.config.pythOracle )
)
let refresh_bsol = await refreshReserveInstruction (
  new PublicKey(reservebsol.config.address),
  SOLEND_PROGRAM_ID,
  new PublicKey(reservebsol.config.pythOracle )
)
const userCollateralAccountAddress = await getAssociatedTokenAddress(
  new PublicKey(reservebsol.config.collateralMintAddress),
  marginfi_pda,
  true
);
var instruction = solanaStakePool.StakePoolInstruction.depositSol(bsolPayload);
  
const userCollateralAccount = await connection.getAccountInfo(userCollateralAccountAddress);
if (!userCollateralAccount) {
  ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
    userCollateralAccountAddress,
    marginfi_pda,
    new PublicKey(reservebsol.config.collateralMintAddress)
    );
    let tx = new Transaction()
    tx.add(ixx)
    tx.feePayer = wallet.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    let sig1 = await client.provider.sendAndConfirm(tx, [], {skipPreflight: false, commitment: 'confirmed'})
    console.log(sig1);
}



console.log(wsolPrice.price)
console.log(
  new BN (amount * 10 ** 9 ),
 new BN( bsolPayload.price * 10 ** 9).div(new BN(wsolPrice.price* 10 ** 9)),
   new BN(jitoPrice.price* 10 ** 9).div(new BN(wsolPrice.price* 10 ** 9)))
  let ix = await program.methods.deposit(
   new BN (amount * 10 ** 9 ),
  new BN( bsolPayload.price * 10 ** 9).div(new BN(wsolPrice.price* 10 ** 9)),
    new BN(jitoPrice.price* 10 ** 9).div(new BN(wsolPrice.price* 10 ** 9)))
   .accounts({
    signer: wallet.publicKey,
    marginfiPda : marginfi_pda,
    poolTokenReceiverAccount: await getAssociatedTokenAddress(
      mint2,
      marginfi_pda,
      true
    ),
    referrerTokenAccount: await getAssociatedTokenAddress(
      mint2,
      new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
    ),
    stakePool: new PublicKey("stk9ApL5HeVAwPLr3TLhDXdZS8ptVu7zp6ov8HFDuMi"),
    stakePoolWithdrawAuthority: bsolPayload.withdrawAuthority,
    reserveStakeAccount: bsolPayload.reserveStake,
    managerFeeAccount: bsolPayload.managerFeeAccount,
    poolMint: bsolPayload.poolMint,
    stakePoolProgram:new PublicKey("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy"),
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    marginfiBank: new PublicKey(reservebsol.config.address),
    marginfiBankJito: new PublicKey(market.reserves.find((r) => r.config.liquidityToken.mint == mint3.toBase58()).config.address),
    liquidityVault: new PublicKey( reservebsol.config.liquidityAddress),
    marginfiBankWsol: new PublicKey(reserve.config.address),
    poolTokenReceiverAccountWsol: await getAssociatedTokenAddress(
      mint1,
      marginfi_pda,
      true
    ),
    liquidityVaultWsol:new PublicKey( reserve.config.liquidityAddress),
    poolMintWsol: mint1,
    stakePoolJitosol: jitoSolPayload.stakePool,
    stakePoolWithdrawAuthorityJitosol: jitoSolPayload.withdrawAuthority,
    reserveStakeAccountJitosol: jitoSolPayload.reserveStake,
    managerFeeAccountJitosol: jitoSolPayload.managerFeeAccount,
    poolMintJitosol: jitoSolPayload.poolMint,
    poolTokenReceiverAccountJitosol: await getAssociatedTokenAddress(
      mint3,
      marginfi_pda,
      true
    ),
    referrerTokenAccountJitosol: await getAssociatedTokenAddress(
      mint3,
      new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
    ),
    stakePoolWithdrawAuthorityWsol: new PublicKey(reserve.config.liquidityFeeReceiverAddress),
    bankLiquidityVaultAuthorityWsol:  bsolPayload.withdrawAuthority,
    jareziMint: new PublicKey('5tR1kBz9gFxuZ5ZSRbchi96xhTerpSyRVwphUfBajC3L'),
    jareziTokenAccount: await getAssociatedTokenAddress(
      new PublicKey('5tR1kBz9gFxuZ5ZSRbchi96xhTerpSyRVwphUfBajC3L'),
      wallet.publicKey,
      true,
      TOKEN_2022_PROGRAM_ID
    ),
    tokenProgram2022: TOKEN_2022_PROGRAM_ID,
    to: derivedInputs.toPubkey,
    obligationPubkey: obligationAddress,
    lendingMarketPubkey: new PublicKey(market.config.address),
    solendSdk: SOLEND_PROGRAM_ID,
    lendingMarketAuthorityPubkey: new PublicKey(market.config.authorityAddress),
    userCollateralPubkey: userCollateralAccountAddress,
    reserveCollateralMintPubkey:        new PublicKey(reservebsol.config.collateralMintAddress),
    destinationDepositCollateralPubkey: new PublicKey(reservebsol.config.collateralSupplyAddress),
    pythOracle: new PublicKey(reservebsol.config.pythOracle),
    switchboardOracle: new PublicKey(reservebsol.config.switchboardOracle),
  })
  
  .instruction();
  console.log(ix)
  console.log(...bank2.config.oracleKeys)
  console.log(...bank1.config.oracleKeys)

  
const depositBsolToReserve = await depositReserveLiquidityInstruction(
  amount / Number(bsolPrice.price) * 10 ** 9,
  bsolPayload.destinationPoolAccount,
  new PublicKey(reservebsol.config.collateralSupplyAddress),
  new PublicKey( reservebsol.config.address),
  new PublicKey(reservebsol.config.liquidityAddress),
  new PublicKey(reservebsol.config.collateralMintAddress),
  new PublicKey(market.config.address),
  new PublicKey(market.config.authorityAddress),
wallet.publicKey,
SOLEND_PROGRAM_ID);

let lookupTable = (await connection.getAddressLookupTable(LOOKUP_TABLE_ADDRESS)).value;
let lookupTable2 = (await connection.getAddressLookupTable(new PublicKey(market.config.lookupTableAddress))).value;
let latestBlockhash = await connection.getLatestBlockhash();

const computeBudgetIx =await ComputeBudgetProgram.setComputeUnitLimit(
  {
    units: 100000000,
  })
  try {
  const messageWithLookupTable = new TransactionMessage({
    payerKey: wallet.publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions: [computeBudgetIx, obRefresh, refresh, refresh_bsol, ix]
}).compileToV0Message([lookupTable, lookupTable2]); // 👈 NOTE: We DO include the lookup table
const transactionWithLookupTable = new VersionedTransaction(messageWithLookupTable);

  const sigs = await client.provider.sendAndConfirm(transactionWithLookupTable, [], {
    skipPreflight: true, commitment: 'recent'})

console.log(sigs)
} catch (err){
  console.log(err)
  return 'uhoh'
}

return 'Yield successful'

}
async function mainUy(connection, wallet, amount) {

    const config = await getConfig("production");
    const client = await MarginfiClient.fetch(config, wallet, connection);
  
    const programAddresses = await client.getAllProgramAccountAddresses(AccountType.MarginfiGroup);
    console.log(programAddresses.map((key) => key.toBase58()));
  
    // const marginfiAccount = await MarginfiAccount.fetch(
    //   "H9rVGRzqZJC2gJ9ysgVDq1AnwLurdipVz94f4yy9igan",
    //   client
    // );
  
    let marginfiAccount = (await client.getMarginfiAccountsForAuthority(wallet.publicKey))[0];
    try {
      const marginFiAccountInfo = await connection.getAccountInfo(marginfiAccount.address);
     } catch (err){
          marginfiAccount = await client.createMarginfiAccount();
      }
      try {
        let wrappedBalance = await connection.getTokenAccountBalance(await getAssociatedTokenAddress(
          NATIVE_MINT,
          wallet.publicKey
        ))
        console.log(wrappedBalance)
        } catch (err){
          let ata = await getAssociatedTokenAddress(
            NATIVE_MINT,
            wallet.publicKey
          )
          
        }

    let mint1 = NATIVE_MINT;
    const bank1 = client.getBankByMint(mint1);
    if (!bank1) throw Error(`${mint1.toBase58()} bank not found`);
    const bankLabel2 = "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1";
    let mint2 = new PublicKey(bankLabel2);
    const bank2 = client.getBankByMint(mint2);
    const bankLabel3 = "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn";
    let mint3 = new PublicKey(bankLabel3);
    const bank3 = client.getBankByMint(mint3);
    const bankLabel4 = "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So";
    let mint4 = new PublicKey(bankLabel4);
    const bank4 = client.getBankByMint(mint4);
    const bankLabel5 = "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj";
    let mint5 = new PublicKey(bankLabel5);
    const bank5 = client.getBankByMint(mint5);
  
    if (!bank2) throw Error(`${bankLabel2} bank not found`);
  
    if (!bank3) throw Error(`${bankLabel3} bank not found`);
  
    if (!bank4) throw Error(`${bankLabel4} bank not found`);
  
    if (!bank5) throw Error(`${bankLabel5} bank not found`);
    let tx1 = new Transaction()
    for (const mint of [mint3,mint2,mint4,mint5]){//mint2
      let associatedToken = await getAssociatedTokenAddress(
        mint,
        wallet.publicKey
      );
      let associatedTokenAccount = await connection.getAccountInfo(associatedToken);
      if (!associatedTokenAccount) {
        let ix = await createAssociatedTokenAccountInstruction(wallet.publicKey,
          associatedToken,
          wallet.publicKey,
          mint
          );
          tx1.add(ix)
      }
    }
    if (tx1.instructions.length > 0){
      tx1.feePayer = wallet.publicKey
      tx1.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
      //let sig1 = await client.provider.sendAndConfirm(tx1)
      //console.log(sig1);
    }
    let fundingAccount = await getAssociatedTokenAddress(
      NATIVE_MINT,
      wallet.publicKey
    )
    const jitoBalance = parseInt((await connection.getTokenAccountBalance(await getAssociatedTokenAddress(
      mint3,
      wallet.publicKey
    ))).value.amount)
    const jitoSolPayload = {
      stakePool: new PublicKey("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb"),
      withdrawAuthority: new PublicKey("6iQKfEyhr3bZMotVkW6beNZz5CPAkiwvgV2CTje9pVSS"),
      reserveStake: new PublicKey("BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL"),
      destinationSystemAccount: wallet.publicKey,
      sourceTransferAuthority: wallet.publicKey,
      sourcePoolAccount: await getAssociatedTokenAddress(
        mint3,
        wallet.publicKey
      ),
      managerFeeAccount: new PublicKey("feeeFLLsam6xZJFc6UQFrHqkvVt4jfmVvi2BRLkUZ4i"),
      referralPoolAccount:  await getAssociatedTokenAddress(
        mint3,
        new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
      ),
      poolMint: mint3,
      poolTokens: jitoBalance
    };
    let fundingAccountBalance = await connection.getBalance(fundingAccount)
    console.log(fundingAccountBalance)
    const instruction = solanaStakePool.StakePoolInstruction.withdrawSol(jitoSolPayload);
    console.log(instruction)

    let solbal = 0

    await marginfiAccount.reload();
    marginfiAccount.activeBalances.forEach(async (balance) => {
      const bank = client.getBankByPk(balance.bankPk);
      const oraclePrice = 1
      const { assets, liabilities } = balance.getUsdValueWithPriceBias(bank, oraclePrice);
      if (liabilities > assets){
      solbal += parseFloat(liabilities)
      }
     
    })
    let ixs = []
    console.log(solbal)
    let repay =   [{instructions:[], keys: []}]
    if (solbal > 0){
      repay = await marginfiAccount.makeRepayIx(solbal, bank1.address)
    }
    ixs.push(
      ...[{instructions:[...tx1.instructions], keys: []}], // accounts
      await marginfiAccount.makeDepositIx(jitoBalance / 10 ** 9 , bank3.address), // deposit jito
      // repay sol
      repay,
      await marginfiAccount.makeWithdrawIx(jitoBalance / 10 ** 9 * (1-0.008) * (1/0.74), bank2.address)) // unstake bsol
      console.log(...ixs)
    let lookupTable = (await connection.getAddressLookupTable(LOOKUP_TABLE_ADDRESS)).value;
  let latestBlockhash = await connection.getLatestBlockhash();
  let instructions = []
  let first = true 
  for (const ix of ixs){
    try {
    for (var i of ix.instructions){
      if (i.keys.length == 0 && first){
        first = false 
      }
      else if (i.keys.length == 0 && !first){
        continue
      }
      instructions.push(i)
    }
  } catch (err){

  }
  }

  try {
  const messageWithLookupTable = new TransactionMessage({
    payerKey: wallet.publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions
}).compileToV0Message([lookupTable]); // 👈 NOTE: We DO include the lookup table
const transactionWithLookupTable = new VersionedTransaction(messageWithLookupTable);
  const sigs = await client.provider.sendAndConfirm(transactionWithLookupTable, [], {commitment: 'confirmed'})

console.log(sigs)
} catch (err){
  console.log(err)
  alert('single address lookuptable tx failed.. continuing with many tx. next time you megayield it will be one tx')

  for (const ix of ixs){
    try {
    let tx = new Transaction().add(...ix.instructions);
    tx.feePayer = wallet.publicKey;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    let sig = await client.provider.sendAndConfirm(tx);
    console.log(sig);
    } catch (err){

    }
  }
}

    
const bsolBalance =  parseInt((await connection.getTokenAccountBalance(await getAssociatedTokenAddress(
  mint2,
  wallet.publicKey
))).value.amount) 
const bsolPayload = {
  stakePool: new PublicKey("stk9ApL5HeVAwPLr3TLhDXdZS8ptVu7zp6ov8HFDuMi"),
  withdrawAuthority: new PublicKey("6WecYymEARvjG5ZyqkrVQ6YkhPfujNzWpSPwNKXHCbV2"),
  reserveStake: new PublicKey("rsrxDvYUXjH1RQj2Ke36LNZEVqGztATxFkqNukERqFT"),

  destinationSystemAccount: wallet.publicKey,
  sourceTransferAuthority: wallet.publicKey,
  sourcePoolAccount: await getAssociatedTokenAddress(
    mint2,
    wallet.publicKey
  ),
  
  managerFeeAccount: new PublicKey("Dpo148tVGewDPyh2FkGV18gouWctbdX2fHJopJGe9xv1"),
  referralPoolAccount:  await getAssociatedTokenAddress(
    mint2,
    new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6")
  ),
  poolMint: mint2,
  poolTokens: bsolBalance
};
const instruction2 = solanaStakePool.StakePoolInstruction.withdrawSol(bsolPayload);
console.log(instruction2)
let bsolTx = new Transaction().add(instruction2);
bsolTx.feePayer = wallet.publicKey;
bsolTx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
let sig = await client.provider.sendAndConfirm(bsolTx);
console.log(sig);
    await marginfiAccount.reload();
    marginfiAccount.activeBalances.forEach(async (balance) => {
      const bank = client.getBankByPk(balance.bankPk);
      const oraclePrice = client.getOraclePriceByBank(bank.address);
      const { assets, liabilities } = balance.getUsdValueWithPriceBias(bank, oraclePrice);
     
      console.log(
        `Balance for ${shortenAddress(bank.mint)} (${shortenAddress(
          balance.bankPk
        )}) deposits: ${assets}, borrows: ${liabilities}`
      );
    });
  }
  
function SolanaComponent() {
  const wallet = useWallet();
  const [output, setOutput] = useState('');
  const [amount, setAmount] = useState(1);
  const [program, setProgram] = useState();
  const [provider, setProvider] = useState();
  // Implement the handleYield and handleUnyield functions
  // These functions will use the wallet to send transactions
  const connection = new Connection("https://shallow-sharai-fast-mainnet.helius-rpc.com/");
  const handleYield = async () => {
    // Implement the logic for the 'yield' functionality
    // For example, you can call the main function here
    try {
      let answer = await mainY(connection, wallet, amount, program);
      setOutput(answer);
    } catch (error) {
      console.error(error);
      setOutput('Error in Yield');
    }
  };

  const handleUnyield = async () => {
    await mainUy(connection, wallet, amount);
    setOutput('Unyield clicked');
  };
  useEffect(() => {
    if (wallet.connected) {
      setOutput('Connected');
      const provider = new AnchorProvider(connection, wallet, {
        commitment: 'confirmed',
      });
      setProvider(provider);
      async function initIt(){
      const program = new Program(
        await Program.fetchIdl(
          new PublicKey('GQQ5gDjd1vYKk257qJLJmrsTkiNZQZjC8btN5SHfhpNL'),
          provider
        ),

        new PublicKey('GQQ5gDjd1vYKk257qJLJmrsTkiNZQZjC8btN5SHfhpNL'),
        provider
      )
          console.log(program)
      setProgram(program);
/*
      const market = await SolendMarket.initialize(connection, "production")
      const [marginfi_pda, bump] = PublicKey.findProgramAddressSync(
        [Buffer.from("marginfi")],
        new PublicKey('GQQ5gDjd1vYKk257qJLJmrsTkiNZQZjC8btN5SHfhpNL')
      );
      createObligationAccount(
        wallet.publicKey,
        marginfi_pda,
        new PublicKey(market.config.address),
        program
      )
      
      const derivedInputs = await deriveInputs(marginfi_pda, wallet.publicKey, "robot001", SystemProgram.programId)

      console.log({
        seed: derivedInputs.seed,
        lamports: derivedInputs.lamports,
        space: derivedInputs.space,
      })
    const config = await getConfig("production");

    const client = await MarginfiClient.fetch(config, wallet, connection);
      const jarezi_mint = Keypair.generate();
    let marginfi_account = Keypair.generate();
   let inited1 =  await program.methods.initMrgnFiPda(bump).accounts({
      marginfiPda: marginfi_pda,
      authority: wallet.publicKey,
      marginfiAccount: marginfi_account.publicKey,
      marginfiGroup: client.groupAddress,
      marginfiProgram: new PublicKey('MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA'),
      jareziMint: jarezi_mint.publicKey,
      tokenProgram2022: TOKEN_2022_PROGRAM_ID
    }).signers([ jarezi_mint, marginfi_account ])
    .instruction()

      let createdSeeded = await program.methods.createSeededAccount(
        {
          seed: derivedInputs.seed,
          lamports: new BN(derivedInputs.lamports),
          space: new BN(derivedInputs.space),
          bump: new BN(derivedInputs.bump),
        }
        )
        .accounts({
          program: marginfi_pda,
          from: derivedInputs.fromPubkey,
          to: derivedInputs.toPubkey,
          base:  derivedInputs.base,
          owner: derivedInputs.owner,
          systemProgram: SystemProgram.programId,
          lendingMarket: new PublicKey(market.config.address),
          solendSdk: SOLEND_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY
        })
        .signers([ jarezi_mint, marginfi_account ])
        .preInstructions([inited1])
        .rpc({skipPreflight: true})
     */
    }
    initIt()
    } else {
      setOutput('Disconnected');
    }
  }, [wallet.connected]);
  async function initIt(){
    /*
    
#[derive(Accounts)]
pub struct InitMrgnFiPda<'info> {
    #[account(init,
        seeds = [SEED_PREFIX],

        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<MarginFiPda>(),
    )]
    pub marginfi_pda: Account<'info, MarginFiPda>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: no validation, for educational purpose only
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
    pub jarezi_mint: Box<Account<'info, Mint>>,
    pub token_program_2022: Program<'info, Token>,
*/
    }
  return (
    <div>
  {wallet.connected ? (
    <div className="container connected-container">
      <div className="yield-section">
        <h2 className="App-header">Yield</h2>
        <label className="App-header">Amount to Yield</label>
        <br />
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
        <button
          className="retro-button"
          onClick={handleYield}
          disabled={!wallet.connected}
        >
          Yield
        </button>
        <div className="output">{output}</div>
      </div>
      <div className="unyield-section">
        <h2 className="App-header">Unyield</h2>
        <button
          className="retro-button"
          onClick={handleUnyield}
          disabled={!wallet.connected}
        >
          Unyield
        </button>
        <div>
        </div>
      </div>
    </div>
  ) : (
    <div className="container disconnected-container">
      <h3 className="App-header">What it **does not** do:</h3>
      <ul>
        <li>Serve as a flotation device</li>
        <li>Cure the blues</li>
        <li>Take out the garbage</li>
        <li>Fly</li>
      </ul>
      <h3 className="App-header">What it **does** do:</h3>
      <ul>
        <li>Yield stakes bsol, deposits bsol to mrgnfi, borrows sol, stakes jitosol.</li>
        <li>Unyield unstakes jitosol, repays sol, unstakes jitosol.</li>
        <li>It's a one click megayield button. ATOW bsol yield is 6.471% and jitosol, which you re-stake 76% of, is 6.969%. You will yield 1 * 6.471% + 0.74 * 6.969% = 11.76744% per $ deposited.</li>
      </ul>
     
    </div>
  )}
</div>

  );
}

export default SolanaComponent;
