import { SwitchboardProgram, loadKeypair } from "@switchboard-xyz/solana.js";
import * as anchor from "@coral-xyz/anchor";
import { BasicOracle } from "../target/types/basic_oracle";
import dotenv from "dotenv";
import { loadDefaultQueue } from "./utils";
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

  const program: anchor.Program<BasicOracle> = anchor.workspace.BasicOracle;
  console.log(`PROGRAM: ${program.programId}`);

  const switchboardProgram = await SwitchboardProgram.fromProvider(provider);

  const [programStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("BASICORACLE")],
    program.programId
  );
  console.log(`PROGRAM_STATE: ${programStatePubkey}`);

  const [oraclePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ORACLE_V1_SEED")],
    program.programId
  );
  console.log(`ORACLE_PUBKEY: ${oraclePubkey}`);

  try {
    const programState = await program.account.myProgramState.fetch(
      programStatePubkey
    );
    console.log(`Program state already initialized`);
    console.log(
      `PROGRAM_STATE: \n${JSON.stringify(programState, undefined, 2)}`
    );
    return;

    // Account already initialized
  } catch (error) {
    if (!`${error}`.includes("Account does not exist or has no data")) {
      throw error;
    }
  }

  const attestationQueueAccount = await loadDefaultQueue(switchboardProgram);
  console.log(`ATTESTATION_QUEUE: ${attestationQueueAccount.publicKey}`);

  // Create the instructions to initialize our Switchboard Function
  const [functionAccount, functionInit] =
    await attestationQueueAccount.createFunctionInstruction(payer.publicKey, {
      schedule: "15 * * * * *",
      container: `${process.env.DOCKERHUB_ORGANIZATION ?? "switchboardlabs"}/${
        process.env.DOCKERHUB_CONTAINER_NAME ?? "solana-basic-oracle-function"
      }`,
      version: `${process.env.DOCKERHUB_CONTAINER_VERSION ?? "typescript"}`, // TODO: set to 'latest' after testing
    });
  console.log(`SWITCHBOARD_FUNCTION: ${functionAccount.publicKey}`);

  const signature = await program.methods
    .initialize()
    .accounts({
      program: programStatePubkey,
      oracle: oraclePubkey,
      authority: payer.publicKey,
      switchboardFunction: functionAccount.publicKey,
    })
    .signers([...functionInit.signers])
    .preInstructions([...functionInit.ixns])
    .rpc();

  console.log(`[TX] initialize: ${signature}`);
})();
