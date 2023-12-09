import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "."; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh";

export interface OracleDataBorshFields {
  oracleTimestamp: BN;
  price: BN;
  volume1hr: BN;
  volume24hr: BN;
  twap1hr: BN;
  twap24hr: BN;
}

export interface OracleDataBorshJSON {
  oracleTimestamp: string;
  price: string;
  volume1hr: string;
  volume24hr: string;
  twap1hr: string;
  twap24hr: string;
}

export class OracleDataBorsh {
  readonly oracleTimestamp: BN;
  readonly price: BN;
  readonly volume1hr: BN;
  readonly volume24hr: BN;
  readonly twap1hr: BN;
  readonly twap24hr: BN;

  constructor(fields: OracleDataBorshFields) {
    this.oracleTimestamp = fields.oracleTimestamp;
    this.price = fields.price;
    this.volume1hr = fields.volume1hr;
    this.volume24hr = fields.volume24hr;
    this.twap1hr = fields.twap1hr;
    this.twap24hr = fields.twap24hr;
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.i64("oracleTimestamp"),
        borsh.i128("price"),
        borsh.i128("volume1hr"),
        borsh.i128("volume24hr"),
        borsh.i128("twap1hr"),
        borsh.i128("twap24hr"),
      ],
      property
    );
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new OracleDataBorsh({
      oracleTimestamp: obj.oracleTimestamp,
      price: obj.price,
      volume1hr: obj.volume1hr,
      volume24hr: obj.volume24hr,
      twap1hr: obj.twap1hr,
      twap24hr: obj.twap24hr,
    });
  }

  static toEncodable(fields: OracleDataBorshFields) {
    return {
      oracleTimestamp: fields.oracleTimestamp,
      price: fields.price,
      volume1hr: fields.volume1hr,
      volume24hr: fields.volume24hr,
      twap1hr: fields.twap1hr,
      twap24hr: fields.twap24hr,
    };
  }

  toJSON(): OracleDataBorshJSON {
    return {
      oracleTimestamp: this.oracleTimestamp.toString(),
      price: this.price.toString(),
      volume1hr: this.volume1hr.toString(),
      volume24hr: this.volume24hr.toString(),
      twap1hr: this.twap1hr.toString(),
      twap24hr: this.twap24hr.toString(),
    };
  }

  static fromJSON(obj: OracleDataBorshJSON): OracleDataBorsh {
    return new OracleDataBorsh({
      oracleTimestamp: new BN(obj.oracleTimestamp),
      price: new BN(obj.price),
      volume1hr: new BN(obj.volume1hr),
      volume24hr: new BN(obj.volume24hr),
      twap1hr: new BN(obj.twap1hr),
      twap24hr: new BN(obj.twap24hr),
    });
  }

  toEncodable() {
    return OracleDataBorsh.toEncodable(this);
  }
}
