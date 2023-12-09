import { Connection, clusterApiUrl } from "@solana/web3.js";
import { toDateTimeString } from "@switchboard-xyz/common";
import { SwitchboardProgram } from "@switchboard-xyz/solana.js";
import { FunctionRunner } from "@switchboard-xyz/solana.js/runner";
import {
  OracleJob,
  TaskRunner,
  serializeOracleJob,
  receiptSuccess,
  TaskRunnerClients,
} from "@switchboard-xyz/task-runner";

const myOracleJob: OracleJob = serializeOracleJob({
  tasks: [{ valueTask: { value: 5 } }],
});

async function main() {
  console.log(`${toDateTimeString(new Date())} Entered main() ...`);
  const runner = FunctionRunner.create();
  console.log(`${toDateTimeString(new Date())} Loaded TaskRunner ...`);
  // const program: Program<BasicOracle> = new Program(
  //   JSON.parse(JSON.stringify(idl)),
  //   "ATSP5Zo8Nqv74kz1W6GYxeE76vgRkiBmxjex73uZ4Pes",
  //   runner.provider
  // );

  const switchboardProgram = await SwitchboardProgram.fromConnection(
    runner.connection
  );
  const mainnetConnection =
    (runner as any).env.cluster === "mainnet-beta"
      ? runner.connection
      : new Connection(clusterApiUrl("mainnet-beta"));

  const taskRunnerClients = new TaskRunnerClients(
    runner.program,
    mainnetConnection,
    undefined
  );

  const taskRunner = new TaskRunner(
    switchboardProgram,
    clusterApiUrl("mainnet-beta"),
    taskRunnerClients
  );

  const receipt = await taskRunner.perform("myOracleJob", myOracleJob);
  if (!receiptSuccess(receipt)) {
    throw receipt.error; // TODO: handle this better and report an error
  }

  console.log(`${toDateTimeString(new Date())} Result: ${receipt.result}`);

  await runner.emit([]);
}

// run switchboard function
main().catch((err) => {
  console.error(err);
  throw err;
});
