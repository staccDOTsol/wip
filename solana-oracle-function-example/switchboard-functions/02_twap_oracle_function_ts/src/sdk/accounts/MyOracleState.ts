import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface MyOracleStateFields {
  bump: number
  btc: types.OracleDataFields
  usdc: types.OracleDataFields
  eth: types.OracleDataFields
  sol: types.OracleDataFields
  doge: types.OracleDataFields
}

export interface MyOracleStateJSON {
  bump: number
  btc: types.OracleDataJSON
  usdc: types.OracleDataJSON
  eth: types.OracleDataJSON
  sol: types.OracleDataJSON
  doge: types.OracleDataJSON
}

export class MyOracleState {
  readonly bump: number
  readonly btc: types.OracleData
  readonly usdc: types.OracleData
  readonly eth: types.OracleData
  readonly sol: types.OracleData
  readonly doge: types.OracleData

  static readonly discriminator = Buffer.from([
    125, 13, 208, 149, 124, 13, 235, 167,
  ])

  static readonly layout = borsh.struct([
    borsh.u8("bump"),
    types.OracleData.layout("btc"),
    types.OracleData.layout("usdc"),
    types.OracleData.layout("eth"),
    types.OracleData.layout("sol"),
    types.OracleData.layout("doge"),
  ])

  constructor(fields: MyOracleStateFields) {
    this.bump = fields.bump
    this.btc = new types.OracleData({ ...fields.btc })
    this.usdc = new types.OracleData({ ...fields.usdc })
    this.eth = new types.OracleData({ ...fields.eth })
    this.sol = new types.OracleData({ ...fields.sol })
    this.doge = new types.OracleData({ ...fields.doge })
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<MyOracleState | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<MyOracleState | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): MyOracleState {
    if (!data.slice(0, 8).equals(MyOracleState.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = MyOracleState.layout.decode(data.slice(8))

    return new MyOracleState({
      bump: dec.bump,
      btc: types.OracleData.fromDecoded(dec.btc),
      usdc: types.OracleData.fromDecoded(dec.usdc),
      eth: types.OracleData.fromDecoded(dec.eth),
      sol: types.OracleData.fromDecoded(dec.sol),
      doge: types.OracleData.fromDecoded(dec.doge),
    })
  }

  toJSON(): MyOracleStateJSON {
    return {
      bump: this.bump,
      btc: this.btc.toJSON(),
      usdc: this.usdc.toJSON(),
      eth: this.eth.toJSON(),
      sol: this.sol.toJSON(),
      doge: this.doge.toJSON(),
    }
  }

  static fromJSON(obj: MyOracleStateJSON): MyOracleState {
    return new MyOracleState({
      bump: obj.bump,
      btc: types.OracleData.fromJSON(obj.btc),
      usdc: types.OracleData.fromJSON(obj.usdc),
      eth: types.OracleData.fromJSON(obj.eth),
      sol: types.OracleData.fromJSON(obj.sol),
      doge: types.OracleData.fromJSON(obj.doge),
    })
  }
}
