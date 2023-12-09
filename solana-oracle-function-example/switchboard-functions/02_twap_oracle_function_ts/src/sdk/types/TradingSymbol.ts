import { PublicKey } from "@solana/web3.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js"; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "."; // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh";

export interface UnknownJSON {
  kind: "Unknown";
}

export class Unknown {
  static readonly discriminator = 0;
  static readonly kind = "Unknown";
  readonly discriminator = 0;
  readonly kind = "Unknown";

  toJSON(): UnknownJSON {
    return {
      kind: "Unknown",
    };
  }

  toEncodable() {
    return {
      Unknown: {},
    };
  }
}

export interface BtcJSON {
  kind: "Btc";
}

export class Btc {
  static readonly discriminator = 1;
  static readonly kind = "Btc";
  readonly discriminator = 1;
  readonly kind = "Btc";

  toJSON(): BtcJSON {
    return {
      kind: "Btc",
    };
  }

  toEncodable() {
    return {
      Btc: {},
    };
  }
}

export interface UsdcJSON {
  kind: "Usdc";
}

export class Usdc {
  static readonly discriminator = 2;
  static readonly kind = "Usdc";
  readonly discriminator = 2;
  readonly kind = "Usdc";

  toJSON(): UsdcJSON {
    return {
      kind: "Usdc",
    };
  }

  toEncodable() {
    return {
      Usdc: {},
    };
  }
}

export interface EthJSON {
  kind: "Eth";
}

export class Eth {
  static readonly discriminator = 3;
  static readonly kind = "Eth";
  readonly discriminator = 3;
  readonly kind = "Eth";

  toJSON(): EthJSON {
    return {
      kind: "Eth",
    };
  }

  toEncodable() {
    return {
      Eth: {},
    };
  }
}

export interface SolJSON {
  kind: "Sol";
}

export class Sol {
  static readonly discriminator = 4;
  static readonly kind = "Sol";
  readonly discriminator = 4;
  readonly kind = "Sol";

  toJSON(): SolJSON {
    return {
      kind: "Sol",
    };
  }

  toEncodable() {
    return {
      Sol: {},
    };
  }
}

export interface DogeJSON {
  kind: "Doge";
}

export class Doge {
  static readonly discriminator = 5;
  static readonly kind = "Doge";
  readonly discriminator = 5;
  readonly kind = "Doge";

  toJSON(): DogeJSON {
    return {
      kind: "Doge",
    };
  }

  toEncodable() {
    return {
      Doge: {},
    };
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.TradingSymbolKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object");
  }

  if ("Unknown" in obj) {
    return new Unknown();
  }
  if ("Btc" in obj) {
    return new Btc();
  }
  if ("Usdc" in obj) {
    return new Usdc();
  }
  if ("Eth" in obj) {
    return new Eth();
  }
  if ("Sol" in obj) {
    return new Sol();
  }
  if ("Doge" in obj) {
    return new Doge();
  }

  throw new Error("Invalid enum object");
}

export function fromJSON(
  obj: types.TradingSymbolJSON
): types.TradingSymbolKind {
  switch (obj.kind) {
    case "Unknown": {
      return new Unknown();
    }
    case "Btc": {
      return new Btc();
    }
    case "Usdc": {
      return new Usdc();
    }
    case "Eth": {
      return new Eth();
    }
    case "Sol": {
      return new Sol();
    }
    case "Doge": {
      return new Doge();
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "Unknown"),
    borsh.struct([], "Btc"),
    borsh.struct([], "Usdc"),
    borsh.struct([], "Eth"),
    borsh.struct([], "Sol"),
    borsh.struct([], "Doge"),
  ]);
  if (property !== undefined) {
    return ret.replicate(property);
  }
  return ret;
}
