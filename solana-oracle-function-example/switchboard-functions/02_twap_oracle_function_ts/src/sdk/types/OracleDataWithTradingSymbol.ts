import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "."; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh";

export interface OracleDataWithTradingSymbolFields {
  symbol: types.TradingSymbolKind;
  data: types.OracleDataBorshFields;
}

export interface OracleDataWithTradingSymbolJSON {
  symbol: types.TradingSymbolJSON;
  data: types.OracleDataBorshJSON;
}

export class OracleDataWithTradingSymbol {
  readonly symbol: types.TradingSymbolKind;
  readonly data: types.OracleDataBorsh;

  constructor(fields: OracleDataWithTradingSymbolFields) {
    this.symbol = fields.symbol;
    this.data = new types.OracleDataBorsh({ ...fields.data });
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        types.TradingSymbol.layout("symbol"),
        types.OracleDataBorsh.layout("data"),
      ],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new OracleDataWithTradingSymbol({
      symbol: types.TradingSymbol.fromDecoded(obj.symbol),
      data: types.OracleDataBorsh.fromDecoded(obj.data),
    });
  }

  static toEncodable(fields: OracleDataWithTradingSymbolFields) {
    return {
      symbol: fields.symbol.toEncodable(),
      data: types.OracleDataBorsh.toEncodable(fields.data),
    };
  }

  toJSON(): OracleDataWithTradingSymbolJSON {
    return {
      symbol: this.symbol.toJSON(),
      data: this.data.toJSON(),
    };
  }

  static fromJSON(
    obj: OracleDataWithTradingSymbolJSON
  ): OracleDataWithTradingSymbol {
    return new OracleDataWithTradingSymbol({
      symbol: types.TradingSymbol.fromJSON(obj.symbol),
      data: types.OracleDataBorsh.fromJSON(obj.data),
    });
  }

  toEncodable() {
    return OracleDataWithTradingSymbol.toEncodable(this);
  }
}
