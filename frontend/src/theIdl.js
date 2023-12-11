export const theIdl = {
    "version": "0.1.0",
    "name": "superior_randomness",
    "instructions": [
      {
        "name": "initMrgnFiPda",
        "accounts": [
          {
            "name": "marginfiPda",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "authority",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "marginfiAccount",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "marginfiGroup",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "marginfiProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "jareziMint",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "tokenProgram2022",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      },
      {
        "name": "request",
        "accounts": [
          {
            "name": "payer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "req",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "authority",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "switchboard",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "switchboardState",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "switchboardAttestationQueue",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "switchboardFunction",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "switchboardRequest",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "switchboardRequestEscrow",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "switchboardMint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "associatedTokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "keyhash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      },
      {
        "name": "createSeededAccount",
        "accounts": [
          {
            "name": "from",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "to",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "base",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "owner",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "program",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "lendingMarket",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "solendProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "rent",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "params",
            "type": {
              "defined": "CreateSeededAccountParams"
            }
          }
        ]
      },
      {
        "name": "initObligationAccount",
        "accounts": [
          {
            "name": "from",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "to",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "base",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "owner",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "program",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "lendingMarket",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "solendProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "rent",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "params",
            "type": {
              "defined": "CreateSeededAccountParams"
            }
          }
        ]
      },
      {
        "name": "deposit",
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "marginfiPda",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolTokenReceiverAccount",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "referrer",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "referrerTokenAccount",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakePool",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakePoolWithdrawAuthority",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "reserveStakeAccount",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "managerFeeAccount",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolMint",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakePoolProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "tokenProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "marginfiBank",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "liquidityVault",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "fundingAccount",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "marginfiBankWsol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolTokenReceiverAccountWsol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "liquidityVaultWsol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolMintWsol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakePoolJitosol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakePoolWithdrawAuthorityJitosol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "reserveStakeAccountJitosol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "managerFeeAccountJitosol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolMintJitosol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "poolTokenReceiverAccountJitosol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "referrerTokenAccountJitosol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "stakePoolWithdrawAuthorityWsol",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "bankLiquidityVaultAuthorityWsol",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "jareziMint",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "jareziTokenAccount",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "tokenProgram2022",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "to",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "obligationPubkey",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "lendingMarketPubkey",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "solendProgram",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "lendingMarketAuthorityPubkey",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "userCollateralPubkey",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "reserveCollateralMintPubkey",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "destinationDepositCollateralPubkey",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "pythOracle",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "switchboardOracle",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "clock",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "rate",
            "type": "u64"
          },
          {
            "name": "rateJito",
            "type": "u64"
          }
        ]
      },
      {
        "name": "seed",
        "accounts": [
          {
            "name": "req",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "switchboardFunction",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "switchboardRequest",
            "isMut": false,
            "isSigner": false
          },
          {
            "name": "enclaveSigner",
            "isMut": false,
            "isSigner": true
          },
          {
            "name": "recentBlockhashes",
            "isMut": false,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "seed",
            "type": "u32"
          }
        ]
      },
      {
        "name": "reveal",
        "accounts": [
          {
            "name": "req",
            "isMut": true,
            "isSigner": false
          }
        ],
        "args": [
          {
            "name": "pubkey",
            "type": "publicKey"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "MarginFiPda",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "bump",
              "type": "u8"
            },
            {
              "name": "authority",
              "type": "publicKey"
            }
          ]
        }
      },
      {
        "name": "RequestAccountData",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "bump",
              "type": "u8"
            },
            {
              "name": "pubkeyHash",
              "type": {
                "array": [
                  "u8",
                  32
                ]
              }
            },
            {
              "name": "switchboardRequest",
              "type": "publicKey"
            },
            {
              "name": "seed",
              "type": "u32"
            },
            {
              "name": "blockhash",
              "type": {
                "array": [
                  "u8",
                  32
                ]
              }
            },
            {
              "name": "result",
              "type": {
                "array": [
                  "u8",
                  32
                ]
              }
            },
            {
              "name": "requestTimestamp",
              "type": "i64"
            },
            {
              "name": "seedTimestamp",
              "type": "i64"
            },
            {
              "name": "revealTimestamp",
              "type": "i64"
            }
          ]
        }
      }
    ],
    "types": [
      {
        "name": "U192",
        "docs": [
          "A 192-bit unsigned integer"
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "data",
              "type": {
                "array": [
                  "u8",
                  24
                ]
              }
            }
          ]
        }
      },
      {
        "name": "LastUpdate",
        "docs": [
          "Last update state"
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "slot",
              "docs": [
                "Last slot when updated"
              ],
              "type": {
                "defined": "Slot"
              }
            },
            {
              "name": "stale",
              "docs": [
                "True when marked stale, false when slot updated"
              ],
              "type": "bool"
            }
          ]
        }
      },
      {
        "name": "RateLimiter",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "config",
              "docs": [
                "configuration parameters"
              ],
              "type": {
                "defined": "RateLimiterConfig"
              }
            },
            {
              "name": "prevQty",
              "docs": [
                "prev qty is the sum of all outflows from [window_start - config.window_duration, window_start)"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "windowStart",
              "docs": [
                "window_start is the start of the current window"
              ],
              "type": {
                "defined": "Slot"
              }
            },
            {
              "name": "curQty",
              "docs": [
                "cur qty is the sum of all outflows from [window_start, window_start + config.window_duration)"
              ],
              "type": {
                "defined": "U192"
              }
            }
          ]
        }
      },
      {
        "name": "RateLimiterConfig",
        "docs": [
          "Lending market configuration parameters"
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "windowDuration",
              "docs": [
                "Rate limiter window size in slots"
              ],
              "type": "u64"
            },
            {
              "name": "maxOutflow",
              "docs": [
                "Rate limiter param. Max outflow of tokens in a window"
              ],
              "type": "u64"
            }
          ]
        }
      },
      {
        "name": "LendingMarket",
        "docs": [
          "Lending market state"
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "version",
              "docs": [
                "Version of lending market"
              ],
              "type": "u8"
            },
            {
              "name": "bumpSeed",
              "docs": [
                "Bump seed for derived authority address"
              ],
              "type": "u8"
            },
            {
              "name": "owner",
              "docs": [
                "Owner authority which can add new reserves"
              ],
              "type": "publicKey"
            },
            {
              "name": "quoteCurrency",
              "docs": [
                "Currency market prices are quoted in",
                "e.g. \"USD\" null padded (`*b\"USD\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\\0\"`) or a SPL token mint pubkey"
              ],
              "type": {
                "array": [
                  "u8",
                  32
                ]
              }
            },
            {
              "name": "tokenProgramId",
              "docs": [
                "Token program id"
              ],
              "type": "publicKey"
            },
            {
              "name": "oracleProgramId",
              "docs": [
                "Oracle (Pyth) program id"
              ],
              "type": "publicKey"
            },
            {
              "name": "switchboardOracleProgramId",
              "docs": [
                "Oracle (Switchboard) program id"
              ],
              "type": "publicKey"
            },
            {
              "name": "rateLimiter",
              "docs": [
                "Outflow rate limiter denominated in dollars"
              ],
              "type": {
                "defined": "RateLimiter"
              }
            },
            {
              "name": "whitelistedLiquidator",
              "docs": [
                "whitelisted liquidator"
              ],
              "type": {
                "option": "publicKey"
              }
            },
            {
              "name": "riskAuthority",
              "docs": [
                "risk authority (additional pubkey used for setting params)"
              ],
              "type": "publicKey"
            }
          ]
        }
      },
      {
        "name": "ObligationCollateral",
        "docs": [
          "Obligation collateral state"
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "depositReserve",
              "docs": [
                "Reserve collateral is deposited to"
              ],
              "type": "publicKey"
            },
            {
              "name": "depositedAmount",
              "docs": [
                "Amount of collateral deposited"
              ],
              "type": "u64"
            },
            {
              "name": "marketValue",
              "docs": [
                "Collateral market value in quote currency"
              ],
              "type": {
                "defined": "U192"
              }
            }
          ]
        }
      },
      {
        "name": "ObligationLiquidity",
        "docs": [
          "Obligation liquidity state"
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "borrowReserve",
              "docs": [
                "Reserve liquidity is borrowed from"
              ],
              "type": "publicKey"
            },
            {
              "name": "cumulativeBorrowRateWads",
              "docs": [
                "Borrow rate used for calculating interest"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "borrowedAmountWads",
              "docs": [
                "Amount of liquidity borrowed plus interest"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "marketValue",
              "docs": [
                "Liquidity market value in quote currency"
              ],
              "type": {
                "defined": "U192"
              }
            }
          ]
        }
      },
      {
        "name": "Obligation",
        "docs": [
          "Lending market obligation state"
        ],
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "version",
              "docs": [
                "Version of the struct"
              ],
              "type": "u8"
            },
            {
              "name": "lastUpdate",
              "docs": [
                "Last update to collateral, liquidity, or their market values"
              ],
              "type": {
                "defined": "LastUpdate"
              }
            },
            {
              "name": "lendingMarket",
              "docs": [
                "Lending market address"
              ],
              "type": "publicKey"
            },
            {
              "name": "owner",
              "docs": [
                "Owner authority which can borrow liquidity"
              ],
              "type": "publicKey"
            },
            {
              "name": "deposits",
              "docs": [
                "Deposited collateral for the obligation, unique by deposit reserve address"
              ],
              "type": {
                "vec": {
                  "defined": "ObligationCollateral"
                }
              }
            },
            {
              "name": "borrows",
              "docs": [
                "Borrowed liquidity for the obligation, unique by borrow reserve address"
              ],
              "type": {
                "vec": {
                  "defined": "ObligationLiquidity"
                }
              }
            },
            {
              "name": "depositedValue",
              "docs": [
                "Market value of deposits"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "borrowedValue",
              "docs": [
                "Risk-adjusted market value of borrows.",
                "ie sum(b.borrowed_amount * b.current_spot_price * b.borrow_weight for b in borrows)"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "borrowedValueUpperBound",
              "docs": [
                "Risk-adjusted upper bound market value of borrows.",
                "ie sum(b.borrowed_amount * max(b.current_spot_price, b.smoothed_price) * b.borrow_weight for b in borrows)"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "allowedBorrowValue",
              "docs": [
                "The maximum open borrow value.",
                "ie sum(d.deposited_amount * d.ltv * min(d.current_spot_price, d.smoothed_price) for d in deposits)",
                "if borrowed_value_upper_bound >= allowed_borrow_value, then the obligation is unhealthy and",
                "borrows and withdraws are disabled."
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "unhealthyBorrowValue",
              "docs": [
                "The dangerous borrow value at the weighted average liquidation threshold.",
                "ie sum(d.deposited_amount * d.liquidation_threshold * d.current_spot_price for d in deposits)",
                "if borrowed_value >= unhealthy_borrow_value, the obligation can be liquidated"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "superUnhealthyBorrowValue",
              "docs": [
                "ie sum(d.deposited_amount * d.max_liquidation_threshold * d.current_spot_price for d in",
                "deposits). This field is used to calculate the liquidator bonus.",
                "An obligation with a borrowed value >= super_unhealthy_borrow_value is eligible for the max",
                "bonus"
              ],
              "type": {
                "defined": "U192"
              }
            },
            {
              "name": "borrowingIsolatedAsset",
              "docs": [
                "True if the obligation is currently borrowing an isolated tier asset"
              ],
              "type": "bool"
            }
          ]
        }
      },
      {
        "name": "CreateSeededAccountParams",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "seed",
              "type": "string"
            },
            {
              "name": "lamports",
              "type": "u64"
            },
            {
              "name": "space",
              "type": "u64"
            },
            {
              "name": "bump",
              "type": "u8"
            }
          ]
        }
      },
      {
        "name": "Slot",
        "docs": [
          "The unit of time given to a leader for encoding a block.",
          "",
          "It is some some number of _ticks_ long."
        ],
        "type": {
          "kind": "alias",
          "value": "u64"
        }
      }
    ],
    "events": [
      {
        "name": "RequestSeededEvent",
        "fields": [
          {
            "name": "request",
            "type": "publicKey",
            "index": false
          },
          {
            "name": "seed",
            "type": "u32",
            "index": false
          }
        ]
      }
    ],
    "errors": [
      {
        "code": 6000,
        "name": "RequestAlreadySeeded"
      },
      {
        "code": 6001,
        "name": "RequestAlreadyRevealed"
      },
      {
        "code": 6002,
        "name": "KeyVerifyFailed"
      }
    ]
  }