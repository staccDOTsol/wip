export type CustomError =
  | InvalidAuthority
  | ArrayOverflow
  | StaleData
  | InvalidTrustedSigner
  | InvalidMrEnclave
  | InvalidSymbol
  | IncorrectSwitchboardFunction
  | InvalidSwitchboardFunction
  | FunctionValidationFailed

export class InvalidAuthority extends Error {
  static readonly code = 6000
  readonly code = 6000
  readonly name = "InvalidAuthority"
  readonly msg = "Invalid authority account"

  constructor(readonly logs?: string[]) {
    super("6000: Invalid authority account")
  }
}

export class ArrayOverflow extends Error {
  static readonly code = 6001
  readonly code = 6001
  readonly name = "ArrayOverflow"
  readonly msg = "Array overflow"

  constructor(readonly logs?: string[]) {
    super("6001: Array overflow")
  }
}

export class StaleData extends Error {
  static readonly code = 6002
  readonly code = 6002
  readonly name = "StaleData"
  readonly msg = "Stale data"

  constructor(readonly logs?: string[]) {
    super("6002: Stale data")
  }
}

export class InvalidTrustedSigner extends Error {
  static readonly code = 6003
  readonly code = 6003
  readonly name = "InvalidTrustedSigner"
  readonly msg = "Invalid trusted signer"

  constructor(readonly logs?: string[]) {
    super("6003: Invalid trusted signer")
  }
}

export class InvalidMrEnclave extends Error {
  static readonly code = 6004
  readonly code = 6004
  readonly name = "InvalidMrEnclave"
  readonly msg = "Invalid MRENCLAVE"

  constructor(readonly logs?: string[]) {
    super("6004: Invalid MRENCLAVE")
  }
}

export class InvalidSymbol extends Error {
  static readonly code = 6005
  readonly code = 6005
  readonly name = "InvalidSymbol"
  readonly msg = "Failed to find a valid trading symbol for this price"

  constructor(readonly logs?: string[]) {
    super("6005: Failed to find a valid trading symbol for this price")
  }
}

export class IncorrectSwitchboardFunction extends Error {
  static readonly code = 6006
  readonly code = 6006
  readonly name = "IncorrectSwitchboardFunction"
  readonly msg = "FunctionAccount pubkey did not match program_state.function"

  constructor(readonly logs?: string[]) {
    super("6006: FunctionAccount pubkey did not match program_state.function")
  }
}

export class InvalidSwitchboardFunction extends Error {
  static readonly code = 6007
  readonly code = 6007
  readonly name = "InvalidSwitchboardFunction"
  readonly msg = "FunctionAccount pubkey did not match program_state.function"

  constructor(readonly logs?: string[]) {
    super("6007: FunctionAccount pubkey did not match program_state.function")
  }
}

export class FunctionValidationFailed extends Error {
  static readonly code = 6008
  readonly code = 6008
  readonly name = "FunctionValidationFailed"
  readonly msg = "FunctionAccount was not validated successfully"

  constructor(readonly logs?: string[]) {
    super("6008: FunctionAccount was not validated successfully")
  }
}

export function fromCode(code: number, logs?: string[]): CustomError | null {
  switch (code) {
    case 6000:
      return new InvalidAuthority(logs)
    case 6001:
      return new ArrayOverflow(logs)
    case 6002:
      return new StaleData(logs)
    case 6003:
      return new InvalidTrustedSigner(logs)
    case 6004:
      return new InvalidMrEnclave(logs)
    case 6005:
      return new InvalidSymbol(logs)
    case 6006:
      return new IncorrectSwitchboardFunction(logs)
    case 6007:
      return new InvalidSwitchboardFunction(logs)
    case 6008:
      return new FunctionValidationFailed(logs)
  }

  return null
}
