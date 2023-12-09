import { SwitchboardProgram, loadKeypair } from "@switchboard-xyz/solana.js";
import * as anchor from "@coral-xyz/anchor";
import { BasicOracle } from "../target/types/basic_oracle";
import dotenv from "dotenv";
import { sleep } from "@switchboard-xyz/common";
import { PublicKey } from "@solana/web3.js";
dotenv.config();

(async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program: anchor.Program<BasicOracle> = anchor.workspace.BasicOracle;
  console.log(`PROGRAM: ${program.programId}`);

  const [programStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("BASICORACLE")],
    program.programId
  );
  console.log(`PROGRAM_STATE: ${programStatePubkey}`);
  const programState = await program.account.myProgramState.fetch(
    programStatePubkey
  );

  const [oraclePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ORACLE_V1_SEED")],
    program.programId
  );
  console.log(`ORACLE_PUBKEY: ${oraclePubkey}`);

  let oracleState = await program.account.myOracleState.fetch(oraclePubkey);
  let lastFetched: number = Date.now();
  while (true) {
    await sleep(5000);
    oracleState = await program.account.myOracleState.fetch(oraclePubkey);
    displayOracleState(oraclePubkey, oracleState as any); // apparently doesnt like _# syntax
  }
})();

interface OracleState {
  bump: number;
  btc: OracleData;
  usdc: OracleData;
  eth: OracleData;
  sol: OracleData;
  doge: OracleData;
}
interface OracleData {
  oracleTimestamp: anchor.BN;
  price: anchor.BN;
  volume1Hr: anchor.BN;
  volume24Hr: anchor.BN;
  twap1Hr: anchor.BN;
  twap24Hr: anchor.BN;
}
function displayOracleState(pubkey: PublicKey, oracleState: OracleState) {
  console.clear();
  console.log(`## Oracle (${pubkey})`);
  displaySymbol(oracleState.btc, "btc");
  displaySymbol(oracleState.eth, "eth");
}

function displaySymbol(data: OracleData, symbol: string) {
  console.log(` > ${symbol.toUpperCase()} / USD`);
  console.log(`\tPrice: ${data.price}`);
  console.log(`\tTWAP 1hr / 1d: ${data.twap1Hr} / ${data.twap24Hr}`);
  console.log(`\tVolume 1hr / 1d: ${data.volume1Hr} / ${data.volume24Hr}`);
}
