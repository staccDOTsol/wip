import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SetFunctionAccounts {
  program: PublicKey
  authority: PublicKey
  switchboardFunction: PublicKey
}

export function setFunction(
  accounts: SetFunctionAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.program, isSigner: false, isWritable: true },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    {
      pubkey: accounts.switchboardFunction,
      isSigner: false,
      isWritable: false,
    },
  ]
  const identifier = Buffer.from([8, 149, 20, 25, 204, 244, 39, 222])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
