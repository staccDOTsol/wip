import React, { useEffect, useState } from 'react';
import { SolendMarket,createmObligationAddress, depositReserveLiquidityInstruction,borrowObligationLiquidityInstruction, refreshObligationInstruction, refreshReserveInstruction} from '@solendprotocol/solend-sdk'
import { theIdl } from './theIdl';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
// Import other necessary libraries and dependencies
import {  AddressLookupTableProgram, ComputeBudgetInstruction, ComputeBudgetProgram, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey,  SYSVAR_CLOCK_PUBKEY,  SYSVAR_RENT_PUBKEY,  SYSVAR_STAKE_HISTORY_PUBKEY,  SystemProgram,  Transaction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { AccountType, getConfig, MarginfiClient} from "@mrgnlabs/marginfi-client-v2";
import { NATIVE_MINT, TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, createInitializeAccountInstruction, createSyncNativeInstruction, shortenAddress, parseOracleSetup } from "@mrgnlabs/mrgn-common";
import { MINT_SIZE,  TOKEN_2022_PROGRAM_ID, createTransferCheckedInstruction, getAssociatedTokenAddress, getMinimumBalanceForRentExemptAccount, syncNative } from '@solana/spl-token'
import { WalletConnectButton, WalletDisconnectButton, WalletModalButton, WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import * as solanaStakePool from '@solana/spl-stake-pool';
import { Marinade, MarinadeConfig, Wallet } from '@marinade.finance/marinade-ts-sdk'

import '@solana/wallet-adapter-react-ui/styles.css';
import { MarinadeFinanceProgram } from '@marinade.finance/marinade-ts-sdk/dist/src/programs/marinade-finance-program';
import { STAKE_PROGRAM_ID, SYSTEM_PROGRAM_ID } from '@marinade.finance/marinade-ts-sdk/dist/src/util';
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { BN } from 'bn.js';
import { set } from '@coral-xyz/anchor/dist/cjs/utils/features';
const hydra =new PublicKey("2bxwkKqwzkvwUqj3xYs4Rpmo1ncPcA1TedAPzTXN1yHu")
const hydraHostFeeAccount = new PublicKey("5WbQEkwrUUAxLeueYFzBP7y6qQgYXXrQxWrAQ32wPuAn")
const hydraReferrer = new PublicKey("942fhtBTB6Xb66ZoJMTexVuXyVUfhYfGvUGGCxx9owiA")
let winwin = new PublicKey('JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm')
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
  program,
  winnerWinnerChickumDinner
){
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
    [Buffer.from("jarezi"),
    winnerWinnerChickumDinner.toBuffer()],
    new PublicKey('Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d')
  );
  const derivedInputs = await deriveInputs(marginfi_pda, fundingAddress, seed, SOLEND_PROGRAM_ID)
  

return [
  await program.methods.createSeededAccount({
  seed: seed,
  lamports: new BN(lamports),
  space: new BN(space),
  bump: new BN(bump),
})
.accounts({
  program: marginfi_pda,
  winnerWinnerChickumDinner,
  from: fundingAddress,
  to: derivedInputs.toPubkey,
  base:  marginfi_pda,
  owner: derivedInputs.owner,
  systemProgram: SystemProgram.programId,
  lendingMarket: lendingMarketPubkey,
  solendSdk: SOLEND_PROGRAM_ID,
  tokenProgram: TOKEN_PROGRAM_ID,
  rent: SYSVAR_RENT_PUBKEY
})
.instruction()]
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

async function getMintByAuthority(connection, marginfi_pda) {

 //COption<Pubkey>, // sizeof // Pubkey
  const filters = [
    // Only get Mint accounts
    {
        dataSize: MINT_SIZE,
    },
    // Only get Mint accounts with the given Mint authority
    {
        memcmp: {
            offset: 4,//size of u32 = 4
            bytes: marginfi_pda.toBase58(),
        },
    },
];

  const encodedAccounts = await connection.getProgramAccounts(TOKEN_2022_PROGRAM_ID, {
      filters,
  });

console.log(encodedAccounts)
  const accounts = encodedAccounts.map(({ pubkey, account }) => {

      return {
          pubkey,
      };
  });

  console.log(accounts);
  return accounts[0].pubkey;
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
    pub stake_pool_bsol: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub stake_pool_withdraw_authority_bsol: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_stake_account_bsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub manager_fee_account_bsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_bsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK:
    pub stake_pool_program_bsol: AccountInfo<'info>,
    #[account(init_if_needed,
        payer = signer,
        token::authority = marginfi_pda,
        token::mint = pool_mint_bsol
        
    )]
    pub pool_token_receiver_account_bsol: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, token::authority = referrer,
        token::mint = pool_mint_bsol,
        payer = signer,
        
        
    )]
    pub referrer_token_account_bsol: Box<Account<'info, TokenAccount>>,
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
  [Buffer.from("jarezi"),
  winwin.toBuffer()],
    new PublicKey('Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d')
);

let jareziMint = await  getMintByAuthority(connection, marginfi_pda);
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
  let pdaAccount = await program.account.marginFiPda.fetch(marginfi_pda)
maybe = await getAssociatedTokenAddress(
  jareziMint,
  wallet.publicKey,
  true,
  TOKEN_2022_PROGRAM_ID
)
associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      wallet.publicKey,
      jareziMint,
      TOKEN_2022_PROGRAM_ID
      );
      tx.add(ixx)
  }

let wsolPrice = client.getOraclePriceByBank(bank1.address);
let bsolPrice = client.getOraclePriceByBank(bank2.address);
let jitoPrice = client.getOraclePriceByBank(bank3.address);

const derivedInputs = await deriveInputs(marginfi_pda, wallet.publicKey, pdaAccount.seededSeed, SystemProgram.programId)
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())

const market = await SolendMarket.initialize(connection, "production")
console.log(market)
let obligationAddress
let ixx 
const reserve = market.reserves.find((r) => r.config.liquidityToken.mint == NATIVE_MINT.toBase58())
const reservebsol = market.reserves.find((r) => r.config.liquidityToken.mint == mint2.toBase58())

const {
  seed2,
  lamports,
  space,
} = {
  lamports: 9938880,
  space: 1300,
  seed2: market.config.address.toString().slice(0, 32),
}
var derivedInputs2 = await deriveInputs(marginfi_pda, wallet.publicKey, seed2, SOLEND_PROGRAM_ID)
obligationAddress = derivedInputs2.toPubkey

console.log(obligationAddress.toBase58())

    await market.loadReserves()
    console.log(reserve.config)
console.log(market)
const userCollateralAccountAddress = await getAssociatedTokenAddress(
  new PublicKey(reservebsol.config.collateralMintAddress),
  marginfi_pda,
  true
);
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
    let sig1 = await client.provider.sendAndConfirm(tx, [], {skipPreflight: true, commitment: 'confirmed'})
    console.log(sig1);
}



const [oracle, _bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("ORACLE_USDY_SEED_V2")],
  program.programId
);
console.log(wsolPrice.price.toNumber())
console.log(
  new BN (amount * 10 ** 9 ),
 new BN( bsolPrice.price.toNumber() / wsolPrice.price.toNumber() * 10 ** 9),
  new BN(jitoPrice.price.toNumber() / wsolPrice.price.toNumber() * 10 ** 9))
  let ix = await program.methods.deposit(
   new BN (amount * 10 ** 9 ))
   .accounts({
    signer: wallet.publicKey,
    marginfiPda : marginfi_pda,
    winnerWinnerChickumDinner: pdaAccount.winnerWinnerChickumDinner,
    poolTokenReceiverAccount: await getAssociatedTokenAddress(
      mint2,
      marginfi_pda,
      true
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
    liquidityVault: new PublicKey( reservebsol.config.liquidityAddress),
    marginfiBankWsol: new PublicKey(reserve.config.address),
    poolTokenReceiverAccountWsol: await getAssociatedTokenAddress(
      mint1,
      marginfi_pda,
      true
    ),
    liquidityVaultWsol:new PublicKey( reserve.config.liquidityAddress),
    poolMintWsol: mint1,
    stakePoolWithdrawAuthorityWsol: new PublicKey(reserve.config.liquidityFeeReceiverAddress),
    jareziMint: jareziMint,
    jareziTokenAccount: await getAssociatedTokenAddress(
      jareziMint,
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
    pythOracle2: new PublicKey(reserve.config.pythOracle),
    switchboardOracle2: new PublicKey(reserve.config.switchboardOracle),
    clock: SYSVAR_CLOCK_PUBKEY,
    stakeHistory: SYSVAR_STAKE_HISTORY_PUBKEY,
    stakeProgram: STAKE_PROGRAM_ID,

    rent: SYSVAR_RENT_PUBKEY,
    oracle,
    hydra,
    hydraReferrer,
    hydraHostFeeAccount,
    
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
    instructions: [computeBudgetIx, ...tx.instructions, ix]
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
async function mainUy(connection, wallet, amount, program) {

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
    pub stake_pool_bsol: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: no validation, for educational purpose only
    pub stake_pool_withdraw_authority_bsol: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_stake_account_bsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub manager_fee_account_bsol: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_mint_bsol: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK:
    pub stake_pool_program_bsol: AccountInfo<'info>,
    #[account(init_if_needed,
        payer = signer,
        token::authority = marginfi_pda,
        token::mint = pool_mint_bsol
        
    )]
    pub pool_token_receiver_account_bsol: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, token::authority = referrer,
        token::mint = pool_mint_bsol,
        payer = signer,
        
        
    )]
    pub referrer_token_account_bsol: Box<Account<'info, TokenAccount>>,
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

  [Buffer.from("jarezi"),
  winwin.toBuffer()],
    new PublicKey('Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d')
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
  let pdaAccount = await program.account.marginFiPda.fetch(marginfi_pda)

let jareziMint = await  getMintByAuthority(connection, marginfi_pda);
maybe = await getAssociatedTokenAddress(
  jareziMint,
  wallet.publicKey,
  true,
  TOKEN_2022_PROGRAM_ID
)
associatedTokenAccount = await connection.getAccountInfo(maybe);
  if (!associatedTokenAccount) {
    let ixx = await createAssociatedTokenAccountInstruction(wallet.publicKey,
      maybe,
      wallet.publicKey,
      jareziMint,
      TOKEN_2022_PROGRAM_ID
      );
      tx.add(ixx)
  }

let wsolPrice = client.getOraclePriceByBank(bank1.address);
let bsolPrice = client.getOraclePriceByBank(bank2.address);
let jitoPrice = client.getOraclePriceByBank(bank3.address);

const derivedInputs = await deriveInputs(marginfi_pda, wallet.publicKey, pdaAccount.seededSeed, SystemProgram.programId)
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())
console.log(derivedInputs.toPubkey.toBase58())

const market = await SolendMarket.initialize(connection, "production")
console.log(market)
let obligationAddress
let ixx 
const reserve = market.reserves.find((r) => r.config.liquidityToken.mint == NATIVE_MINT.toBase58())
const reservebsol = market.reserves.find((r) => r.config.liquidityToken.mint == mint2.toBase58())
 
const {
  seed2,
  lamports,
  space,
} = {
  lamports: 9938880,
  space: 1300,
  seed2: market.config.address.toString().slice(0, 32),
}
var derivedInputs2 = await deriveInputs(marginfi_pda, wallet.publicKey, seed2, SOLEND_PROGRAM_ID)
obligationAddress = derivedInputs2.toPubkey
console.log(obligationAddress.toBase58())

    await market.loadReserves()
    console.log(reserve.config)
console.log(market)
const userCollateralAccountAddress = await getAssociatedTokenAddress(
  new PublicKey(reservebsol.config.collateralMintAddress),
  marginfi_pda,
  true
);
  
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
    let sig1 = await client.provider.sendAndConfirm(tx, [], {skipPreflight: true, commitment: 'confirmed'})
    console.log(sig1);
}


const [oracle, _bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("ORACLE_USDY_SEED_V2")],
  program.programId
);
console.log(wsolPrice.price.toNumber())
console.log(
  new BN (amount * 10 ** 9 ),
 new BN( bsolPrice.priceRealtime.toNumber() / wsolPrice.priceRealtime.toNumber() * 10 ** 9),
  new BN(jitoPrice.priceRealtime.toNumber() / wsolPrice.priceRealtime.toNumber() * 10 ** 9))
  let ix = await program.methods.withdraw(
   new BN (amount * 10 ** 9 ))
   .accounts({
    signer: wallet.publicKey,
    marginfiPda : marginfi_pda,
    winnerWinnerChickumDinner: pdaAccount.winnerWinnerChickumDinner,
    poolTokenReceiverAccount: await getAssociatedTokenAddress(
      mint2,
      marginfi_pda,
      true
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
    liquidityVault: new PublicKey( reservebsol.config.liquidityAddress),
    marginfiBankWsol: new PublicKey(reserve.config.address),
    poolTokenReceiverAccountWsol: await getAssociatedTokenAddress(
      mint1,
      marginfi_pda,
      true
    ),
    liquidityVaultWsol:new PublicKey( reserve.config.liquidityAddress),
    poolMintWsol: mint1,
    stakePoolWithdrawAuthorityWsol: new PublicKey(reserve.config.liquidityFeeReceiverAddress),
    jareziMint: jareziMint,
    jareziTokenAccount: await getAssociatedTokenAddress(
      jareziMint,
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
    pythOracle2: new PublicKey(reserve.config.pythOracle),
    switchboardOracle2: new PublicKey(reserve.config.switchboardOracle),
    
    clock: SYSVAR_CLOCK_PUBKEY,
    stakeHistory: SYSVAR_STAKE_HISTORY_PUBKEY,
    stakeProgram: STAKE_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
    oracle,
    hydra,
    hydraReferrer,
    hydraHostFeeAccount,
    
  })
  
  .instruction();
  console.log(ix)
  console.log(...bank2.config.oracleKeys)
  console.log(...bank1.config.oracleKeys)

  

let lookupTable = (await connection.getAddressLookupTable(LOOKUP_TABLE_ADDRESS)).value;
let lookupTable2 = (await connection.getAddressLookupTable(new PublicKey(market.config.lookupTableAddress))).value;
let latestBlockhash = await connection.getLatestBlockhash();

const computeBudgetIx =await ComputeBudgetProgram.setComputeUnitLimit(
  {
    units: 200000000,
  })
  try {
  const messageWithLookupTable = new TransactionMessage({
    payerKey: wallet.publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions: [computeBudgetIx, ...tx.instructions, ix]
}).compileToV0Message([lookupTable, lookupTable2]); // 👈 NOTE: We DO include the lookup table
const transactionWithLookupTable = new VersionedTransaction(messageWithLookupTable);

  const sigs = await client.provider.sendAndConfirm(transactionWithLookupTable, [], {
    skipPreflight: true, commitment: 'recent'})

console.log(sigs)
} catch (err){
  console.log(err)
  return 'uhoh'
}

return 'Unyield successful'

  }
  
function SolanaComponent() {
  const wallet = useWallet();
  const [output, setOutput] = useState('');
  const [amount, setAmount] = useState(1);
  const [program, setProgram] = useState();
  const [provider, setProvider] = useState();
  const [seed, setSeed] = useState('@staccoverflow');
  const [winnerWinnerChickumDinner, setWinnerWinnerChickumDinner] = useState(winwin);
  const [kickback, setKickback] = useState(100);
const [foldedOut, setFoldedOut] = useState(false);

const [userTokenBalance, setUserTokenBalance] = useState(0);
const [tokenTVL, setTokenTVL] = useState(0);
const [tokenTVL2, setTokenTVL2] = useState(0);


const tokenMintAddress = 'GHQGAo8M5K5gry4qVMGGvWEGHvuvnNvtSMidkpFdhwR4';
const tvlTokenAccount = 'E2iGSzRyerDwtaAzarEoMz5hapFiAgkceTfhvHxbqbd8';

useEffect(() => {
  const fetchBalances = async () => {
    if (wallet.connected) {
      // Fetch User's Token Balance
      try {
      const userTokenAccount = await getAssociatedTokenAddress(
        new PublicKey(tokenMintAddress),
        wallet.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID
      );
      
      const userTokenAccountInfo = await connection.getTokenAccountBalance(
        userTokenAccount
      );
      if (userTokenAccountInfo) {
        const userBalance = userTokenAccountInfo.value.uiAmount; // Replace with correct parsing
        setUserTokenBalance(userBalance);
      }
      } 
      catch (err){

      }
      const tokenBal = await connection.getTokenSupply(
        new PublicKey(tokenMintAddress)
      );
      if (tokenBal) {
        setTokenTVL2(tokenBal.value.uiAmount);
      }
      // Fetch TVL of Token Account
      const tvlAccountInfo = await connection.getTokenAccountBalance(
        new PublicKey(tvlTokenAccount)
      );
      if (tvlAccountInfo) {
        const tvlBalance = tvlAccountInfo.value.uiAmount; // Replace with correct parsing
       
        setTokenTVL(tvlBalance);
      }
    }
  };

  fetchBalances();
}, [wallet.connected]);

const handleFoldout = () => {
  setFoldedOut(!foldedOut)
}

  // Implement the handleYield and handleUnyield functions
  // These functions will use the wallet to send transactions
  const connection = new Connection("https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718");
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
const handleInited = async () => {
  console.log(seed, wallet, program, winnerWinnerChickumDinner, kickback)
  initIt(seed, wallet, program, winnerWinnerChickumDinner, kickback)
}
  const handleUnyield = async () => {
    let answer = await mainUy(connection, wallet, amount, program);
    setOutput(answer);
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
          new PublicKey('Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d'),
          provider
        ),

        new PublicKey('Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d'),
        provider
      )
          console.log(program)
      setProgram(program);
    }
    initIt()
    } else {
      setOutput('Disconnected');
    }
  }, [wallet.connected]);
  async function initIt(seed, wallet, program, winnerWinnerChickumDinner, kickback){
    
    const market = await SolendMarket.initialize(connection, "production")
    console.log(market)
    const [marginfi_pda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("jarezi"),
      winnerWinnerChickumDinner.toBuffer()],
      
      new PublicKey('Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d')
    );
  const config = await getConfig("production");

  const {
    seed2,
    lamports,
    space,
  } = {
    lamports: 9938880,
    space: 1300,
    seed2: market.config.address.toString().slice(0, 32),
  }

  var derivedInputs = await deriveInputs(marginfi_pda, wallet.publicKey, seed2, SOLEND_PROGRAM_ID)
  const client = await MarginfiClient.fetch(config, wallet, connection);
    const jarezi_mint = Keypair.generate();
  try {

    const bankLabel2 = "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1";
    let mint2 = new PublicKey(bankLabel2);
  var derivedInputs = await deriveInputs(marginfi_pda, wallet.publicKey, seed2, SOLEND_PROGRAM_ID)
  const reservebsol = market.reserves.find((r) => r.config.liquidityToken.mint == mint2.toBase58())
  const reserve = market.reserves.find((r) => r.config.liquidityToken.mint == NATIVE_MINT.toBase58())

  const userCollateralAccountAddress = await getAssociatedTokenAddress(
    new PublicKey(reservebsol.config.collateralMintAddress),
    marginfi_pda,
    true
  );
  const userCollateralAccountAddress2 = await getAssociatedTokenAddress(
    new PublicKey(reserve.config.collateralMintAddress),
    marginfi_pda,
    true
  );



 let inited1 =  await program.methods.initMrgnFiPda(bump, new BN(kickback * 10_000), seed, seed2).accounts({
    marginfiPda: marginfi_pda,
    winnerWinnerChickumDinner: winnerWinnerChickumDinner,
    authority: wallet.publicKey,
    marginfiGroup: client.groupAddress,
    marginfiProgram: new PublicKey('MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA'),
    jareziMint: jarezi_mint.publicKey,
    tokenProgram2022: TOKEN_2022_PROGRAM_ID,

  to: derivedInputs.toPubkey,
  base:  derivedInputs.base,
  owner: derivedInputs.owner,
  systemProgram: SystemProgram.programId,
  lendingMarket: new PublicKey(market.config.address),
  solendSdk: SOLEND_PROGRAM_ID,
  tokenProgram: TOKEN_PROGRAM_ID,
  rent: SYSVAR_RENT_PUBKEY,
    marginfiBankWsol: new PublicKey(reservebsol.config.address),
    poolTokenReceiverAccountWsol: await getAssociatedTokenAddress(
      mint2,
      marginfi_pda,
      true
    ),
    marginfiBankWsol2: new PublicKey(reserve.config.address),
    liquidityVaultWsol:new PublicKey( reservebsol.config.liquidityAddress),
    poolMintWsol: mint2,
    stakePoolWithdrawAuthorityWsol: new PublicKey(reservebsol.config.liquidityFeeReceiverAddress),

    stakePoolWithdrawAuthorityWsol2: new PublicKey(reserve.config.liquidityFeeReceiverAddress),
    lendingMarketPubkey: new PublicKey(market.config.address),
    lendingMarketAuthorityPubkey: new PublicKey(market.config.authorityAddress),
    reserveCollateralMintPubkey:        new PublicKey(reservebsol.config.collateralMintAddress),
    userCollateralPubkey: new PublicKey(reservebsol.config.collateralSupplyAddress),

    destinationDepositCollateralPubkey: userCollateralAccountAddress,

    reserveCollateralMintPubkey2:        new PublicKey(reserve.config.collateralMintAddress),
    destinationDepositCollateralPubkey2: userCollateralAccountAddress2,


    poolTokenReceiverAccountWsol2: await getAssociatedTokenAddress(
      NATIVE_MINT,
      marginfi_pda,
      true
    ),
    liquidityVaultWsol2:new PublicKey( reserve.config.liquidityAddress),
    poolMintWsol2: NATIVE_MINT,
    pythOracle: new PublicKey(reservebsol.config.pythOracle),
    switchboardOracle: new PublicKey(reservebsol.config.switchboardOracle),
    pythOracle2: new PublicKey(reserve.config.pythOracle),
    switchboardOracle2: new PublicKey(reserve.config.switchboardOracle),

  })
  .instruction()

  var derivedInputs = await deriveInputs(marginfi_pda, wallet.publicKey, seed, SystemProgram.programId)
  console.log(derivedInputs.toPubkey.toBase58())
  console.log(derivedInputs.toPubkey.toBase58())
  console.log(derivedInputs.toPubkey.toBase58())
  console.log(derivedInputs.toPubkey.toBase58())
  console.log(derivedInputs.toPubkey.toBase58())
  console.log(derivedInputs.toPubkey.toBase58())
  console.log(derivedInputs.toPubkey.toBase58())

  console.log({
    seed: derivedInputs.seed,
    lamports: derivedInputs.lamports,
    space: derivedInputs.space,
  })
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
        winnerWinnerChickumDinner: winnerWinnerChickumDinner,
        from: wallet.publicKey,
        to: derivedInputs.toPubkey,
        base:  derivedInputs.base,
        owner: derivedInputs.owner,
        systemProgram: SystemProgram.programId,
        lendingMarket: new PublicKey(market.config.address),
        solendSdk: SOLEND_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY
      })
      .instruction()
      
  let tx = new Transaction()
  console.log(marginfi_pda.toBase58())
  let ata = await getAssociatedTokenAddress(
    NATIVE_MINT,
    marginfi_pda,
    true
  )
  let ataAccount = await connection.getAccountInfo(ata);
  tx = new Transaction()
  tx.add(createAssociatedTokenAccountInstruction(
    wallet.publicKey,
    userCollateralAccountAddress2,
    marginfi_pda,
    new PublicKey(reserve.config.collateralMintAddress)  ),
    createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      await getAssociatedTokenAddress(
        NATIVE_MINT,
        marginfi_pda,
        true
      ),
      marginfi_pda,
      NATIVE_MINT
    ),
    SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: await getAssociatedTokenAddress(
        NATIVE_MINT,
        marginfi_pda,
        true
      ),
      lamports: 6660,
    }))
  tx.feePayer = wallet.publicKey
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  try {
    await provider.sendAndConfirm(tx, [], {skipPreflight: true, commitment: 'confirmed'})
    }
    catch (err){
      console.log(err)
    }
      tx = new Transaction()
  //tx.add()
  tx.feePayer = wallet.publicKey
  tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  try {
  await provider.sendAndConfirm(tx, [], {skipPreflight: true, commitment: 'confirmed'})
  }
  catch (err){
    console.log(err)
  }
  
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
    instructions: [createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      userCollateralAccountAddress,
      marginfi_pda,
      new PublicKey(reservebsol.config.collateralMintAddress)  ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        await getAssociatedTokenAddress(
          mint2,
          marginfi_pda,
          true
        ),
        marginfi_pda,
        mint2
      ),
      createTransferCheckedInstruction(
        await getAssociatedTokenAddress(
          mint2,
          wallet.publicKey,
        ),
        mint2, 
        await getAssociatedTokenAddress(
          mint2,
          marginfi_pda,true
        ),
        wallet.publicKey,
        666,
        9
      ),computeBudgetIx, inited1, createdSeeded]
}).compileToV0Message([lookupTable, lookupTable2]); // 👈 NOTE: We DO include the lookup table
const transactionWithLookupTable = new VersionedTransaction(messageWithLookupTable);

  const sigs = await client.provider.sendAndConfirm(transactionWithLookupTable, [jarezi_mint], {
    skipPreflight: true, commitment: 'recent'})

console.log(sigs)
} catch (err){
  console.log(err)
  return 'uhoh'
}
      }
     catch (err){
      console.log(err)
      console.log('already initialized... skipping')
     }

    return 'Init successful'
  }

  return (
    <div>
  {wallet.connected ? (
    <div className="container connected-container">
    <label className="App-header">Amount</label><br/>
        
      <div className="yield-section">
        <button
          className="retro-button"
          onClick={handleYield}
          disabled={!wallet.connected}
        >
          Yield
        </button>
        <div className="output">{output}</div>
      </div>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
      <div className="unyield-section">
        <button
          className="retro-button"
          onClick={handleUnyield}
          disabled={!wallet.connected}
        >
          Unyield
        </button>
        <div>
        </div>
      </div>      <WalletMultiButton /> 
      <div>
        <p>Your Token Balance: {userTokenBalance}</p>
        <p>Total Supply Tokens: {tokenTVL2}</p>
      </div>


      <button 
          className="retro-button" onClick={handleFoldout}>Wanna manage your own magick yielding treasury?</button>

      {foldedOut && 
    <div className="container connected-container">
      <div className="yield-section">
        <p>Your own unique seed for your LST Staccer Machine:</p><br/>
        <input
          type="text"
          value={seed}
          maxLength={32}
          onChange={(e) => setSeed(e.target.value)}
        />

<p >Proceeds from your LST Staccer Machine go to the address - recommend a https://docs.metaplex.com/programs/hydra/quick-start Hydra Fanout Wallet:</p><br/>
        <input
          type="text"
          value={winnerWinnerChickumDinner}
          onChange={(e) => setWinnerWinnerChickumDinner(new PublicKey(e.target.value))}
        />

<p>The amount of yield from your LST Staccer to automagically kickback to the deposit00rs:</p><br/>
        <input
          type="number"
          value={kickback}
          onChange={(e) => setKickback(e.target.value)}
        />
        <br />
        <br />
        <button 
          className="retro-button" onClick={handleInited}>Init</button>
      </div><ul>
    <li>Experience DeFi innovation with our One-Click MegaYield Button. Stake to bSOL, borrow SOL, then stake to bSOL, compounding your yield efficiently and effortlessly.</li>
    <li>Revolutionize your staking strategy: Stake to bSOL, borrow SOL, and then stake to bSOL for maximized and compounded returns.</li>
    <li>Introducing a new era of fundraising: Stake to bSOL, borrow SOL, stake to bSOL, and issue tokens based on bSOL held, all without risking investor capital.</li>
    <li>Transform your approach to yield farming: Stake to bSOL, borrow SOL, stake to bSOL, and enjoy the benefits of a simplified, yet powerful, yield strategy.</li>
    <li>Unlock the full potential of your assets. Stake to bSOL, borrow SOL, then stake to bSOL, simplifying your investment and maximizing returns.</li>
    <li>Empower your projects with our innovative fundraising model. Stake to bSOL, borrow SOL, stake to bSOL, and issue tokens, ensuring safety for your supporters' investments.</li>
    <li>Step into the future of yield farming with our method that focuses on SOL, offering a streamlined and profitable staking experience.</li>
    <li>Embrace the new wave of risk-free fundraising. Stake to bSOL, borrow SOL, stake to bSOL, and issue tokens, keeping investor funds secure and growing.</li>
    <li>Take control of your DeFi journey. Stake to bSOL, borrow SOL, stake to bSOL, and issue tokens for a direct path to higher returns.</li>
    <li>One-Click MegaYield: A simple yet powerful tool for DeFi, enabling you to stake to bSOL, borrow SOL, and stake to bSOL for optimal returns.</li>
    <li>Build a sustainable support system for your projects. Stake to bSOL, borrow SOL, stake to bSOL, and issue tokens, all while ensuring zero risk to your supporters.</li>
    <li>Our platform redefines yield farming: Stake to bSOL, borrow SOL, stake to bSOL, and issue tokens, making earning on your crypto easy and efficient.</li>
</ul>
</div>
        }
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
      <h3 className="App-header">What it **does** do (connect && find out):</h3>
      <WalletMultiButton /> 
      <h3 className="App-header">How to use it:</h3>
      <ul>
        <li>Yield stakes bsol, deposits bsol to mrgnfi, borrows sol, stakes bsol.</li>
        <li>Unyield unstakes bsol, repays sol, unstakes bsol.</li>
        <li>It's a one click megayield button. ATOW bsol yield is 6.471% and bsol, which you re-stake 76% of, is 6.969%. You will yield 1 * 6.471% + 0.74 * 6.969% = 11.76744% per $ deposited.</li>
      </ul>
    </div>
  )}
</div>

  );
}

export default SolanaComponent;
