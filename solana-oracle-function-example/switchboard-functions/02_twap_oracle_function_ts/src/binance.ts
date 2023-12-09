import { BN, Program } from "@coral-xyz/anchor";
import { OracleDataBorsh, TradingSymbol, TradingSymbolJSON } from "./sdk/types";
import { BasicOracle } from "./types";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { FunctionRunner } from "@switchboard-xyz/solana.js/runner";
import fetch from "node-fetch";
import { Big, BigUtils } from "@switchboard-xyz/common";

interface Ticker {
  symbol: string; // BTCUSDT
  priceChange: string;
  priceChangePercent: string;
  weightedAvgPrice: string;
  openPrice: string;
  highPrice: string;
  lowPrice: string;
  lastPrice: string;
  volume: string;
  quoteVolume: string;
  openTime: number; // ms
  closeTime: number; // ms
}

// interface OracleDataBorsh {
//   oracleTimestamp: BN;
//   price: BN;
//   volume1hr: BN;
//   volume24hr: BN;
//   twap1hr: BN;
//   twap24hr: BN;
// }

// type TradingSymbol =
//   | { unknown: {} }
//   | { btc: {} }
//   | { usdc: {} }
//   | { eth: {} }
//   | { sol: {} }
//   | { doge: {} };

export function convertSymbol(symbol: string): TradingSymbolJSON {
  switch (symbol) {
    case "BTCUSDT":
      return TradingSymbol.Btc;
    case "USDCUSDT":
      return TradingSymbol.Usdc;
    case "ETHUSDT":
      return TradingSymbol.Eth;
    case "SOLUSDT":
      return TradingSymbol.Sol;
    case "DOGEUSDT":
      return TradingSymbol.Doge;
    default:
      return TradingSymbol.Unknown;
  }
}

export function convertSymbolEnum(symbol: string) {
  switch (symbol) {
    case "BTCUSDT":
      return { btc: {} };
    case "USDCUSDT":
      return { usdc: {} };
    case "ETHUSDT":
      return { eth: {} };
    case "SOLUSDT":
      return { sol: {} };
    case "DOGEUSDT":
      return { doge: {} };
    default:
      return { unknown: {} };
  }
}

class IndexData {
  constructor(
    readonly symbol: string,
    readonly hr: Ticker,
    readonly d: Ticker
  ) {}

  toOracleDataBorsh(timestamp: number): OracleDataBorsh {
    return new OracleDataBorsh({
      oracleTimestamp: new BN(timestamp),
      price: formatToNineDecimalPrecision(this.hr.lastPrice),
      volume1hr: formatToNineDecimalPrecision(this.hr.volume),
      volume24hr: formatToNineDecimalPrecision(this.d.volume),
      twap1hr: formatToNineDecimalPrecision(this.hr.weightedAvgPrice),
      twap24hr: formatToNineDecimalPrecision(this.d.weightedAvgPrice),
    });
  }
}

export class Binance {
  private fetchTimestamp = Math.floor(Date.now() / 1000);
  constructor(
    public readonly btcUsdt: IndexData,
    public readonly usdcUsdt: IndexData,
    public readonly ethUsdt: IndexData,
    public readonly solUsdt: IndexData,
    public readonly dogeUsdt: IndexData
  ) {}

  public static async fetch(): Promise<Binance> {
    const symbols = ["BTCUSDT", "USDCUSDT", "ETHUSDT", "SOLUSDT", "DOGEUSDT"];

    const oneHrUrl = `https://api.binance.com/api/v3/ticker?symbols=[${symbols
      .map((s) => `\"${s}\"`)
      .join(",")}]&windowSize=1h`;
    console.log(`Fetching 1hr data from ${oneHrUrl}`);
    const tickers1HrResponse = await fetch(oneHrUrl);
    if (!tickers1HrResponse.ok) {
      throw new Error(
        `Failed to fetch tickers for the 1hr interval, Status=${tickers1HrResponse.status}`
      );
    }
    const tickers1Hr: Ticker[] = await tickers1HrResponse.json();

    if (tickers1Hr.length !== symbols.length) {
      throw new Error(`Mismatch in number of tickers`);
    }

    const twentyFourHourUrl = `https://api.binance.com/api/v3/ticker?symbols=[${symbols
      .map((s) => `\"${s}\"`)
      .join(",")}]&windowSize=1h`;
    console.log(`Fetching 24hr data from ${twentyFourHourUrl}`);
    const tickers24HrResponse = await fetch(twentyFourHourUrl);
    if (!tickers24HrResponse.ok) {
      throw new Error(
        `Failed to fetch tickers for the 1d interval, Status=${tickers24HrResponse.status}`
      );
    }
    const tickers1d: Ticker[] = await tickers24HrResponse.json();
    if (tickers1d.length !== symbols.length) {
      throw new Error(`Mismatch in number of tickers`);
    }

    const data: IndexData[] = symbols.map(
      (s) =>
        new IndexData(
          s,
          tickers1Hr.find((t) => t.symbol === s)!,
          tickers1d.find((t) => t.symbol === s)!
        )
    );

    return new Binance(data[0], data[1], data[2], data[3], data[4]);
  }

  async toInstruction(
    runner: FunctionRunner,
    program: Program<BasicOracle>
  ): Promise<TransactionInstruction> {
    return await program.methods
      .refreshOracles({
        rows: [
          {
            symbol: {
              btc: {},
            },
            data: this.btcUsdt.toOracleDataBorsh(this.fetchTimestamp),
          },
          {
            symbol: {
              usdc: {},
            },
            data: this.usdcUsdt.toOracleDataBorsh(this.fetchTimestamp),
          },
          {
            symbol: {
              eth: {},
            },
            data: this.ethUsdt.toOracleDataBorsh(this.fetchTimestamp),
          },
        ],
      })
      .accounts({
        program: PublicKey.findProgramAddressSync(
          [Buffer.from("BASICORACLE")],
          program.programId
        )[0],
        oracle: PublicKey.findProgramAddressSync(
          [Buffer.from("ORACLE_V1_SEED")],
          program.programId
        )[0],
        switchboardFunction: runner.functionKey,
        enclaveSigner: runner.signer,
      })
      .instruction();
  }
}

/**
 * Formats the input string to nine decimal precision and returns a BN object.
 * @param input - The input string to be formatted.
 * @returns A BN object with the formatted input string.
 */
function formatToNineDecimalPrecision(input: string): BN {
  const big = new Big(input);
  const scale = BigUtils.safePow(new Big(10), 9);

  const fixed = BigUtils.safeMul(big, scale);
  const trimmed = fixed.round(0);
  return new BN(trimmed.toString());
}
