import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface RefreshOraclesArgs {
  params: types.RefreshOraclesParamsFields
}

export interface RefreshOraclesAccounts {
  program: PublicKey
  oracle: PublicKey
  switchboardFunction: PublicKey
  enclaveSigner: PublicKey
}

export const layout = borsh.struct([
  types.RefreshOraclesParams.layout("params"),
])

export function refreshOracles(
  args: RefreshOraclesArgs,
  accounts: RefreshOraclesAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.program, isSigner: false, isWritable: false },
    { pubkey: accounts.oracle, isSigner: false, isWritable: true },
    {
      pubkey: accounts.switchboardFunction,
      isSigner: false,
      isWritable: false,
    },
    { pubkey: accounts.enclaveSigner, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([20, 216, 246, 51, 72, 226, 207, 160])
  const buffer = Buffer.alloc(1000)
  const len = layout.encode(
    {
      params: types.RefreshOraclesParams.toEncodable(args.params),
    },
    buffer
  )
  const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len)
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
