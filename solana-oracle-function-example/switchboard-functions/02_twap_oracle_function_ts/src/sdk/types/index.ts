import * as TradingSymbol from "./TradingSymbol"

export { OracleData } from "./OracleData"
export type { OracleDataFields, OracleDataJSON } from "./OracleData"
export { OracleDataBorsh } from "./OracleDataBorsh"
export type {
  OracleDataBorshFields,
  OracleDataBorshJSON,
} from "./OracleDataBorsh"
export { OracleDataWithTradingSymbol } from "./OracleDataWithTradingSymbol"
export type {
  OracleDataWithTradingSymbolFields,
  OracleDataWithTradingSymbolJSON,
} from "./OracleDataWithTradingSymbol"
export { RefreshOraclesParams } from "./RefreshOraclesParams"
export type {
  RefreshOraclesParamsFields,
  RefreshOraclesParamsJSON,
} from "./RefreshOraclesParams"
export { TradingSymbol }

export type TradingSymbolKind =
  | TradingSymbol.Unknown
  | TradingSymbol.Btc
  | TradingSymbol.Usdc
  | TradingSymbol.Eth
  | TradingSymbol.Sol
  | TradingSymbol.Doge
export type TradingSymbolJSON =
  | TradingSymbol.UnknownJSON
  | TradingSymbol.BtcJSON
  | TradingSymbol.UsdcJSON
  | TradingSymbol.EthJSON
  | TradingSymbol.SolJSON
  | TradingSymbol.DogeJSON
