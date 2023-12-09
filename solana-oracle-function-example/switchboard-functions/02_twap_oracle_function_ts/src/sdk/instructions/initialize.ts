import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface InitializeAccounts {
  program: PublicKey
  oracle: PublicKey
  authority: PublicKey
  switchboardFunction: PublicKey
  payer: PublicKey
  systemProgram: PublicKey
}

export function initialize(
  accounts: InitializeAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.program, isSigner: false, isWritable: true },
    { pubkey: accounts.oracle, isSigner: false, isWritable: true },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    {
      pubkey: accounts.switchboardFunction,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.payer, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([175, 175, 109, 31, 13, 152, 155, 237])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
