import { SwitchboardProgram, loadKeypair } from "@switchboard-xyz/solana.js";
import * as anchor from "@coral-xyz/anchor";
import { UsdyUsdOracle } from "../target/types/usdy_usd_oracle";
import dotenv from "dotenv";
import * as mplMetadata from '@metaplex-foundation/mpl-token-metadata'
import { loadDefaultQueue } from "./utils";
import fs from 'fs'
import { Keypair, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
dotenv.config();

(async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(
    process.argv.length > 2
      ? new anchor.AnchorProvider(
          provider.connection,
          new anchor.Wallet(loadKeypair(process.argv[2])),
          {}
        )
      : provider
  );

  const payer = (provider.wallet as anchor.Wallet).payer;
  console.log(`PAYER: ${payer.publicKey}`);

  let program = new anchor.Program(
    await anchor.Program.fetchIdl(
      new PublicKey("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d"),
      provider
    ) as anchor.Idl,
    new PublicKey("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d"),
    provider
  );
  console.log(`PROGRAM: ${program.programId}`);

  const switchboardProgram = await SwitchboardProgram.fromProvider(provider);

  const [programStatePubkey, b1] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("USDY_USDC_ORACLE_V2")],
    program.programId
  );
  console.log(`PROGRAM_STATE: ${programStatePubkey}`);

  const [oraclePubkey, b2] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ORACLE_USDY_SEED_V2")],
    program.programId
  );
  const [marginfiPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("jarezi"),
    (new PublicKey("JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm")).toBuffer()],
    program.programId
  );
  
  console.log(`ORACLE_PUBKEY: ${oraclePubkey}`);

  const attestationQueueAccount = await loadDefaultQueue(switchboardProgram);
  console.log(`ATTESTATION_QUEUE: ${attestationQueueAccount.publicKey}`);

  // Create the instructions to initialize our Switchboard Function
  const [functionAccount, functionInit] =
    await attestationQueueAccount.createFunctionInstruction(payer.publicKey, {
      container: `${process.env.DOCKERHUB_ORGANIZATION ?? "switchboardlabs"}/${
        process.env.DOCKERHUB_CONTAINER_NAME ?? "solana-lst-oracle-function"
      }`,
      version: `${process.env.DOCKERHUB_CONTAINER_VERSION ?? "latest"}`, // TODO: set to 'latest' after testing
    });
  console.log(`SWITCHBOARD_FUNCTION: ${functionAccount.publicKey}`);
  const marginfiPdaSwitchboard = PublicKey.findProgramAddressSync(
    [Buffer.from("jarezi"),
    marginfiPda.toBuffer()],
    new PublicKey("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d")
  )[0];
  const fake_mint = Keypair.generate();

  const metadata =  PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"),
    new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").toBuffer(),
    new PublicKey("GHQGAo8M5K5gry4qVMGGvWEGHvuvnNvtSMidkpFdhwR4").toBuffer()],
    new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
  )[0];

  const signature = await program.methods
    .update(b1, b2)
    .accounts({
      oracle: oraclePubkey,
      program: programStatePubkey,
      authority: payer.publicKey,
      payer: payer.publicKey,
      switchboardFunction: new PublicKey("HrRcNQe9qUbxR8YNh3tLPYvBCVtM3qdZ14zPJUEJQa3Z"),
      systemProgram : anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log(`[TX] initialize: ${signature}`);
  const metadataed = await program.methods.setJareziMintMetadata("(Real) Stacc Token", "staccs4eva", "https://gist.githubusercontent.com/staccDOTsol/58acbf7d006904bcb1c2dd5597ace2f2/raw/e20f03a60eb288b37efbac1fcec9cf31b8601f53/bool.json")
  .accounts({
    /*
                #[account(mut,
        constraint = marginfi_pda.authority == authority.key(),
        seeds = [SEED_PREFIX, marginfi_pda.thewinnerog.as_ref()],
        bump
    )]
    pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub jarezi_mint: Box<InterfaceAccount<'info, anchor_spl::token_interface::Mint>>,
    #[account(mut)]
    /// CHECK:
    pub metadata: AccountInfo<'info>,
    pub token_program_2022: Program<'info, Token2022>,
    /// CHECK:
    pub mpl_token_metadata_program: AccountInfo<'info>,
    /// CHECK:
    pub sysvar_instructions: AccountInfo<'info>,*/
    marginfiPda: marginfiPda,
    authority: payer.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    winnerWinnerChickumDinner: new PublicKey("JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm"),
    jareziMint: new PublicKey("GHQGAo8M5K5gry4qVMGGvWEGHvuvnNvtSMidkpFdhwR4"),
    metadata: metadata,
    tokenProgram2022: new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
    mplTokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
    sysvarInstructions: new PublicKey("Sysvar1nstructions1111111111111111111111111"),
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    fakeMint : fake_mint.publicKey,
  }).signers([fake_mint])
  .rpc();

    const functioned = await program.methods.setFunction()
    .accounts({
      
      /*
       pub marginfi_pda: Box<Account<'info, MarginFiPda>>,
    /// CHECK:
    pub winner_winner_chickum_dinner: AccountInfo<'info>,
    /// CHECK: 
    
    /// CHECK:
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,

    pub authority: Signer<'info>, */
      marginfiPda: marginfiPda,
      marginfiPdaSwitchboard: marginfiPdaSwitchboard,
      winnerWinnerChickumDinner: new PublicKey("JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm"),
      switchboardFunction: new PublicKey("D887WegE3scoL4fFHkePFLC5z19ikDrnX1GzcVztfnwv"),
      authority: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    
    
  console.log(`[TX] initialize: ${functioned}`);
  /*
  const signature = await program.methods
    .initialize()
    .accounts({
      oracle: oraclePubkey,
      program: programStatePubkey,
      authority: payer.publicKey,
      payer: payer.publicKey,
      switchboardFunction: functionAccount.publicKey,
    })
    .signers([...functionInit.signers])
    .preInstructions([...functionInit.ixns])
    .rpc();

  console.log(`[TX] initialize: ${signature}`);*/
})();