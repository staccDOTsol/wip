// eslint-disable-next-line node/no-unpublished-import
import type { TaskRunnerOracle } from "../target/types/task_runner_oracle";

import type { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { sleep } from "@switchboard-xyz/common";
import type { FunctionAccount } from "@switchboard-xyz/solana.js";
import {
  SwitchboardWallet,
  attestationTypes,
} from "@switchboard-xyz/solana.js";
import { type BootstrappedAttestationQueue } from "@switchboard-xyz/solana.js";
import { assert } from "chai";
import { MRENCLAVE, setupTest } from "./utils";

const unixTimestamp = () => Math.floor(Date.now() / 1000);

describe("task_runner_oracle", () => {
  let switchboard: BootstrappedAttestationQueue;
  let wallet: SwitchboardWallet;
  let functionAccount: FunctionAccount;

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .TaskRunnerOracle as Program<TaskRunnerOracle>;

  console.log(`ProgramID: ${program.programId}`);

  const payer = (program.provider as anchor.AnchorProvider).publicKey;

  const programStatePubkey = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("TASKRUNNERORACLE")],
    program.programId
  )[0];
  console.log(`programStatePubkey: ${programStatePubkey}`);

  const oracleKeypair = Keypair.generate();
  console.log(`oraclePubkey: ${oracleKeypair.publicKey}`);

  const firstFeedIpfsHash = Buffer.from(
    "MyFirstDataFeed-IpfsHash".padEnd(32, " ")
  ).slice(0, 32);

  before(async () => {
    switchboard = await setupTest(program.provider as anchor.AnchorProvider);
    console.log(`Attestation Queue: ${switchboard.attestationQueue.publicKey}`);

    [wallet] = await SwitchboardWallet.create(
      switchboard.program,
      switchboard.attestationQueue.publicKey,
      payer,
      "TaskRunnerOracleWallet",
      16
    );

    console.log(`wallet: ${wallet.publicKey}`);

    // Kind of annoying, the function is a PDA derived from the payer and recentSlot
    // So we need to wait for the recentSlot to tick over before we can create the function.
    // We could manually provide the incremented recent slot, but this is easier.
    await sleep(5000);

    [functionAccount] =
      await switchboard.attestationQueue.account.createFunction(
        {
          name: "test function",
          metadata: "this function handles XYZ for my protocol",
          schedule: "15 * * * * *",
          container: "switchboardlabs/basic-oracle-function",
          version: "latest",
          mrEnclave: MRENCLAVE,
          authority: programStatePubkey,
        },
        wallet,
        { skipPreflight: true }
      );
    console.log(`functionAccount: ${functionAccount.publicKey}`);
  });

  it("Is initialized!", async () => {
    const space = program.account.myOracleState.size;
    console.log(`Oracle Account Size: ${space}`);

    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        program: programStatePubkey,
        oracle: oracleKeypair.publicKey,
        authority: payer,
        payer: payer,
        switchboardFunction: functionAccount.publicKey,
      })
      .signers([oracleKeypair])
      .preInstructions([
        // oracle account is a keypair account so theres no PDA size restrictions
        SystemProgram.createAccount({
          fromPubkey: payer,
          newAccountPubkey: oracleKeypair.publicKey,
          lamports:
            await program.provider.connection.getMinimumBalanceForRentExemption(
              space
            ),
          space,
          programId: program.programId,
        }),
      ])
      .rpc()
      .catch((err) => {
        console.error(err);
        throw err;
      });
    console.log(`[TX] initialize: ${tx}`);
  });

  it("Adds a data feed", async () => {
    const tx = await program.methods
      .addFeed({
        idx: 0,
        name: Buffer.from("First Feed").slice(0, 32),
        ipfsHash: firstFeedIpfsHash,
        updateInterval: 10,
      })
      .accounts({
        program: programStatePubkey,
        oracle: oracleKeypair.publicKey,
        authority: payer,
      })
      .rpc();
    console.log(`[TX] add_feed: ${tx}`);

    const oracleState = await program.account.myOracleState.fetch(
      oracleKeypair.publicKey
    );
    assert(
      Buffer.compare(
        Buffer.from(oracleState.feeds[0].ipfsHash),
        firstFeedIpfsHash
      ) === 0
    );
  });

  it("Saves a result to our data feed", async () => {
    const preOracleState = await program.account.myOracleState.fetch(
      oracleKeypair.publicKey
    );

    const securedSigner = anchor.web3.Keypair.generate();

    const rewardAddress =
      await switchboard.program.mint.getOrCreateAssociatedUser(payer);

    const functionState = await functionAccount.loadData();

    // TODO: generate function verify ixn
    const functionVerifyIxn = attestationTypes.functionVerify(
      switchboard.program,
      {
        params: {
          observedTime: new anchor.BN(unixTimestamp()),
          nextAllowedTimestamp: new anchor.BN(unixTimestamp() + 100),
          isFailure: false,
          mrEnclave: Array.from(MRENCLAVE),
        },
      },
      {
        function: functionAccount.publicKey,
        functionEnclaveSigner: securedSigner.publicKey,
        verifier: switchboard.verifier.publicKey,
        verifierSigner: switchboard.verifier.signer.publicKey,
        attestationQueue: switchboard.attestationQueue.publicKey,
        escrowWallet: functionState.escrowWallet,
        escrowTokenWallet: functionState.escrowTokenWallet,
        receiver: rewardAddress,
        verifierPermission: switchboard.verifier.permissionAccount.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }
    );

    const tx = await program.methods
      .saveFeedResult({
        idx: 0,
        ipfsHash: firstFeedIpfsHash,
        result: new anchor.BN(10),
      })
      .accounts({
        program: programStatePubkey,
        oracle: oracleKeypair.publicKey,
        switchboardFunction: functionAccount.publicKey,
        enclaveSigner: securedSigner.publicKey,
      })
      .preInstructions([functionVerifyIxn])
      .signers([switchboard.verifier.signer, securedSigner])
      .rpc({ skipPreflight: true });

    console.log(`[TX] save_result: ${tx}`);

    await sleep(1000);
    const oracleState = await program.account.myOracleState.fetch(
      oracleKeypair.publicKey
    );
    const firstFeed = oracleState.feeds[0];
    const firstValue: { value: anchor.BN; timestamp: anchor.BN } =
      firstFeed.history[firstFeed.historyIdx - 1];

    assert(firstValue.value.eq(new anchor.BN(10)), "Data Feed result mismatch");
  });

  // // TODO: remove data feed
  it("Removes a data feed", async () => {
    const preOracleState = await program.account.myOracleState.fetch(
      oracleKeypair.publicKey
    );

    const tx = await program.methods
      // .removeFeed(0, Buffer.from(firstFeedIpfsHash))
      .removeFeed(0, null)
      .accounts({
        program: programStatePubkey,
        oracle: oracleKeypair.publicKey,
        authority: payer,
      })
      .rpc();
    console.log(`[TX] remove_feed: ${tx}`);

    await sleep(1000);

    const oracleState = await program.account.myOracleState.fetch(
      oracleKeypair.publicKey
    );
    const firstFeed = oracleState.feeds[0];
    assert(
      Buffer.compare(
        Buffer.from(firstFeed.ipfsHash),
        Buffer.from(new Uint8Array(32))
      ) === 0,
      "Data Feed not removed"
    );
  });
});
