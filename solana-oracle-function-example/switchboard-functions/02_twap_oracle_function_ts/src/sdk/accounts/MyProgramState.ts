import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface MyProgramStateFields {
  bump: number
  authority: PublicKey
  switchboardFunction: PublicKey
}

export interface MyProgramStateJSON {
  bump: number
  authority: string
  switchboardFunction: string
}

export class MyProgramState {
  readonly bump: number
  readonly authority: PublicKey
  readonly switchboardFunction: PublicKey

  static readonly discriminator = Buffer.from([
    211, 175, 180, 162, 150, 149, 108, 225,
  ])

  static readonly layout = borsh.struct([
    borsh.u8("bump"),
    borsh.publicKey("authority"),
    borsh.publicKey("switchboardFunction"),
  ])

  constructor(fields: MyProgramStateFields) {
    this.bump = fields.bump
    this.authority = fields.authority
    this.switchboardFunction = fields.switchboardFunction
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<MyProgramState | null> {
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
  ): Promise<Array<MyProgramState | null>> {
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

  static decode(data: Buffer): MyProgramState {
    if (!data.slice(0, 8).equals(MyProgramState.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = MyProgramState.layout.decode(data.slice(8))

    return new MyProgramState({
      bump: dec.bump,
      authority: dec.authority,
      switchboardFunction: dec.switchboardFunction,
    })
  }

  toJSON(): MyProgramStateJSON {
    return {
      bump: this.bump,
      authority: this.authority.toString(),
      switchboardFunction: this.switchboardFunction.toString(),
    }
  }

  static fromJSON(obj: MyProgramStateJSON): MyProgramState {
    return new MyProgramState({
      bump: obj.bump,
      authority: new PublicKey(obj.authority),
      switchboardFunction: new PublicKey(obj.switchboardFunction),
    })
  }
}
