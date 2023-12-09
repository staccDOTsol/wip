import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "."; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh";

export interface RefreshOraclesParamsFields {
  rows: Array<types.OracleDataWithTradingSymbolFields>;
}

export interface RefreshOraclesParamsJSON {
  rows: Array<types.OracleDataWithTradingSymbolJSON>;
}

export class RefreshOraclesParams {
  readonly rows: Array<types.OracleDataWithTradingSymbol>;

  constructor(fields: RefreshOraclesParamsFields) {
    this.rows = fields.rows.map(
      (item) => new types.OracleDataWithTradingSymbol({ ...item })
    );
  }

  static layout(property?: string) {
    return borsh.struct(
      [borsh.vec(types.OracleDataWithTradingSymbol.layout(), "rows")],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new RefreshOraclesParams({
      rows: obj.rows.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.OracleDataWithTradingSymbol.fromDecoded(item)
      ),
    });
  }

  static toEncodable(fields: RefreshOraclesParamsFields) {
    return {
      rows: fields.rows.map((item) =>
        types.OracleDataWithTradingSymbol.toEncodable(item)
      ),
    };
  }

  toJSON(): RefreshOraclesParamsJSON {
    return {
      rows: this.rows.map((item) => item.toJSON()),
    };
  }

  static fromJSON(obj: RefreshOraclesParamsJSON): RefreshOraclesParams {
    return new RefreshOraclesParams({
      rows: obj.rows.map((item) =>
        types.OracleDataWithTradingSymbol.fromJSON(item)
      ),
    });
  }

  toEncodable() {
    return RefreshOraclesParams.toEncodable(this);
  }
}
