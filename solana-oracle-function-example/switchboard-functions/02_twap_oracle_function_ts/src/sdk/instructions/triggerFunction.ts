import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface TriggerFunctionAccounts {
  program: PublicKey
  switchboardFunction: PublicKey
  authority: PublicKey
  attestationQueue: PublicKey
  attestationProgram: PublicKey
}

export function triggerFunction(
  accounts: TriggerFunctionAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.program, isSigner: false, isWritable: false },
    {
      pubkey: accounts.switchboardFunction,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.authority, isSigner: true, isWritable: false },
    { pubkey: accounts.attestationQueue, isSigner: false, isWritable: false },
    { pubkey: accounts.attestationProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([99, 146, 122, 231, 78, 155, 69, 111])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
