import { FunctionRunner } from "@switchboard-xyz/solana.js/runner";
import { Program, web3 } from "@coral-xyz/anchor";
import { BasicOracle, IDL } from "./types.ts";

async function main() {
  console.log("Starting function ...");
  const runner = await FunctionRunner.create();

  const program: Program<BasicOracle> = new Program(
    IDL,
    "DApMSLHYpnXB4qk71vbZS8og4w31hg8Dkr14coaRFANb",
    runner.program.provider
  );

  const response = await fetch(
    `https://api.coingecko.com/api/v3/simple/price?ids=blazestake-staked-sol&vs_currencies=usd`
  );
  if (!response.ok) {
    throw new Error(`Failed to fetch BSOL price`);
  }

  const price: { "blazestake-staked-sol": { usd: number } } = await response.json();

  const ixn: web3.TransactionInstruction = await program.methods
    .updatePrice(price["blazestake-staked-sol"].usd)
    .accounts({
      program: web3.PublicKey.findProgramAddressSync(
        [Buffer.from("BASICORACLE")],
        program.programId
      )[0],
      switchboardFunction: runner.functionKey,
      enclaveSigner: runner.signer,
    })
    .instruction();

  await runner.emit([ixn]);
}

// run switchboard function
main()
  .then()
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
