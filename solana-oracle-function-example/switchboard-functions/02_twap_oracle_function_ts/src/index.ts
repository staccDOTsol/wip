import { Program } from "@coral-xyz/anchor";
import idl from "./idl.json";
import { FunctionRunner } from "@switchboard-xyz/solana.js/runner";
import { Binance } from "./binance";
import { TwapOracle } from "./types";
import { TransactionInstruction } from "@solana/web3.js";

async function main() {
  console.log("[DEBUG] Entry:", new Date().toISOString());
  const runner = await FunctionRunner.create();
  console.log("[DEBUG] Loaded Runner:", new Date().toISOString());

  const program: Program<TwapOracle> = new Program(
    JSON.parse(JSON.stringify(idl)),
    "3NKUtPKboaQN4MwY3nyULBesFaW7hHsXFrBTVjbn2nBr",
    runner.program.provider
  );

  const binance = await Binance.fetch();
  const refreshOraclesIxn: TransactionInstruction = await binance.toInstruction(
    runner,
    program
  );
  console.log("[DEBUG] Fetched Binance Data:", new Date().toISOString());

  await runner.emit([refreshOraclesIxn]);
}

// run switchboard function
main().catch((err) => {
  console.error(err);
  process.exit(-1);
});
