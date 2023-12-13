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
                    "name": "solendSdk",
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
                    "name": "solendSdk",
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
                    "name": "marginfiBankJito",
                    "isMut": true,
                    "isSigner": false
                  },
                  {
                    "name": "liquidityVault",
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
                    "isMut": true,
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
                    "name": "solendSdk",
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
                    "name": "pythOracle2",
                    "isMut": false,
                    "isSigner": false
                  },
                  {
                    "name": "switchboardOracle2",
                    "isMut": false,
                    "isSigner": false
                  },
                  {
                    "name": "rent",
                    "isMut": false,
                    "isSigner": false,
                    "isOptional": true
                  }
                ],
                "args": [
                  {
                    "name": "amount",
                    "type": "u64"
                  },
                  {
                    "name": "bsolPrice",
                    "type": "u64"
                  },
                  {
                    "name": "jitosolPrice",
                    "type": "u64"
                  }
                ]
              },
              {
                "name": "withdraw",
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
                    "name": "marginfiBankJito",
                    "isMut": true,
                    "isSigner": false
                  },
                  {
                    "name": "liquidityVault",
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
                    "isMut": true,
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
                    "name": "solendSdk",
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
                    "name": "pythOracle2",
                    "isMut": false,
                    "isSigner": false
                  },
                  {
                    "name": "switchboardOracle2",
                    "isMut": false,
                    "isSigner": false
                  },
                  {
                    "name": "rent",
                    "isMut": false,
                    "isSigner": false,
                    "isOptional": true
                  }
                ],
                "args": [
                  {
                    "name": "amount",
                    "type": "u64"
                  },
                  {
                    "name": "bsolPrice",
                    "type": "u64"
                  },
                  {
                    "name": "jitosolPrice",
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
                "name": "StakePool",
                "docs": [
                  "Initialized program details."
                ],
                "type": {
                  "kind": "struct",
                  "fields": [
                    {
                      "name": "accountType",
                      "docs": [
                        "Account type, must be StakePool currently"
                      ],
                      "type": {
                        "defined": "AccountType"
                      }
                    },
                    {
                      "name": "manager",
                      "docs": [
                        "Manager authority, allows for updating the staker, manager, and fee",
                        "account"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "staker",
                      "docs": [
                        "Staker authority, allows for adding and removing validators, and",
                        "managing stake distribution"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "stakeDepositAuthority",
                      "docs": [
                        "Stake deposit authority",
                        "",
                        "If a depositor pubkey is specified on initialization, then deposits must",
                        "be signed by this authority. If no deposit authority is specified,",
                        "then the stake pool will default to the result of:",
                        "`Pubkey::find_program_address(",
                        "&[&stake_pool_address.as_ref(), b\"deposit\"],",
                        "program_id,",
                        ")`"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "stakeWithdrawBumpSeed",
                      "docs": [
                        "Stake withdrawal authority bump seed",
                        "for `create_program_address(&[state::StakePool account, \"withdrawal\"])`"
                      ],
                      "type": "u8"
                    },
                    {
                      "name": "validatorList",
                      "docs": [
                        "Validator stake list storage account"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "reserveStake",
                      "docs": [
                        "Reserve stake account, holds deactivated stake"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "poolMint",
                      "docs": [
                        "Pool Mint"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "managerFeeAccount",
                      "docs": [
                        "Manager fee account"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "tokenProgramId",
                      "docs": [
                        "Pool token program id"
                      ],
                      "type": "publicKey"
                    },
                    {
                      "name": "totalLamports",
                      "docs": [
                        "Total stake under management.",
                        "Note that if `last_update_epoch` does not match the current epoch then",
                        "this field may not be accurate"
                      ],
                      "type": "u64"
                    },
                    {
                      "name": "poolTokenSupply",
                      "docs": [
                        "Total supply of pool tokens (should always match the supply in the Pool",
                        "Mint)"
                      ],
                      "type": "u64"
                    },
                    {
                      "name": "lastUpdateEpoch",
                      "docs": [
                        "Last epoch the `total_lamports` field was updated"
                      ],
                      "type": "u64"
                    },
                    {
                      "name": "lockup",
                      "docs": [
                        "Lockup that all stakes in the pool must have"
                      ],
                      "type": {
                        "defined": "Lockup"
                      }
                    },
                    {
                      "name": "epochFee",
                      "docs": [
                        "Fee taken as a proportion of rewards each epoch"
                      ],
                      "type": {
                        "defined": "Fee"
                      }
                    },
                    {
                      "name": "nextEpochFee",
                      "docs": [
                        "Fee for next epoch"
                      ],
                      "type": {
                        "defined": "FutureEpoch<Fee>"
                      }
                    },
                    {
                      "name": "preferredDepositValidatorVoteAddress",
                      "docs": [
                        "Preferred deposit validator vote account pubkey"
                      ],
                      "type": {
                        "option": "publicKey"
                      }
                    },
                    {
                      "name": "preferredWithdrawValidatorVoteAddress",
                      "docs": [
                        "Preferred withdraw validator vote account pubkey"
                      ],
                      "type": {
                        "option": "publicKey"
                      }
                    },
                    {
                      "name": "stakeDepositFee",
                      "docs": [
                        "Fee assessed on stake deposits"
                      ],
                      "type": {
                        "defined": "Fee"
                      }
                    },
                    {
                      "name": "stakeWithdrawalFee",
                      "docs": [
                        "Fee assessed on withdrawals"
                      ],
                      "type": {
                        "defined": "Fee"
                      }
                    },
                    {
                      "name": "nextStakeWithdrawalFee",
                      "docs": [
                        "Future stake withdrawal fee, to be set for the following epoch"
                      ],
                      "type": {
                        "defined": "FutureEpoch<Fee>"
                      }
                    },
                    {
                      "name": "stakeReferralFee",
                      "docs": [
                        "Fees paid out to referrers on referred stake deposits.",
                        "Expressed as a percentage (0 - 100) of deposit fees.",
                        "i.e. `stake_deposit_fee`% of stake deposited is collected as deposit",
                        "fees for every deposit and `stake_referral_fee`% of the collected",
                        "stake deposit fees is paid out to the referrer"
                      ],
                      "type": "u8"
                    },
                    {
                      "name": "solDepositAuthority",
                      "docs": [
                        "Toggles whether the `DepositSol` instruction requires a signature from",
                        "this `sol_deposit_authority`"
                      ],
                      "type": {
                        "option": "publicKey"
                      }
                    },
                    {
                      "name": "solDepositFee",
                      "docs": [
                        "Fee assessed on SOL deposits"
                      ],
                      "type": {
                        "defined": "Fee"
                      }
                    },
                    {
                      "name": "solReferralFee",
                      "docs": [
                        "Fees paid out to referrers on referred SOL deposits.",
                        "Expressed as a percentage (0 - 100) of SOL deposit fees.",
                        "i.e. `sol_deposit_fee`% of SOL deposited is collected as deposit fees",
                        "for every deposit and `sol_referral_fee`% of the collected SOL",
                        "deposit fees is paid out to the referrer"
                      ],
                      "type": "u8"
                    },
                    {
                      "name": "solWithdrawAuthority",
                      "docs": [
                        "Toggles whether the `WithdrawSol` instruction requires a signature from",
                        "the `deposit_authority`"
                      ],
                      "type": {
                        "option": "publicKey"
                      }
                    },
                    {
                      "name": "solWithdrawalFee",
                      "docs": [
                        "Fee assessed on SOL withdrawals"
                      ],
                      "type": {
                        "defined": "Fee"
                      }
                    },
                    {
                      "name": "nextSolWithdrawalFee",
                      "docs": [
                        "Future SOL withdrawal fee, to be set for the following epoch"
                      ],
                      "type": {
                        "defined": "FutureEpoch<Fee>"
                      }
                    },
                    {
                      "name": "lastEpochPoolTokenSupply",
                      "docs": [
                        "Last epoch's total pool tokens, used only for APR estimation"
                      ],
                      "type": "u64"
                    },
                    {
                      "name": "lastEpochTotalLamports",
                      "docs": [
                        "Last epoch's total lamports, used only for APR estimation"
                      ],
                      "type": "u64"
                    }
                  ]
                }
              },
              {
                "name": "ValidatorList",
                "docs": [
                  "Storage list for all validator stake accounts in the pool."
                ],
                "type": {
                  "kind": "struct",
                  "fields": [
                    {
                      "name": "header",
                      "docs": [
                        "Data outside of the validator list, separated out for cheaper",
                        "deserializations"
                      ],
                      "type": {
                        "defined": "ValidatorListHeader"
                      }
                    },
                    {
                      "name": "validators",
                      "docs": [
                        "List of stake info for each validator in the pool"
                      ],
                      "type": {
                        "vec": {
                          "defined": "ValidatorStakeInfo"
                        }
                      }
                    }
                  ]
                }
              },
              {
                "name": "ValidatorListHeader",
                "docs": [
                  "Helper type to deserialize just the start of a ValidatorList"
                ],
                "type": {
                  "kind": "struct",
                  "fields": [
                    {
                      "name": "accountType",
                      "docs": [
                        "Account type, must be ValidatorList currently"
                      ],
                      "type": {
                        "defined": "AccountType"
                      }
                    },
                    {
                      "name": "maxValidators",
                      "docs": [
                        "Maximum allowable number of validators"
                      ],
                      "type": "u32"
                    }
                  ]
                }
              },
              {
                "name": "ValidatorStakeInfo",
                "docs": [
                  "Information about a validator in the pool",
                  "",
                  "NOTE: ORDER IS VERY IMPORTANT HERE, PLEASE DO NOT RE-ORDER THE FIELDS UNLESS",
                  "THERE'S AN EXTREMELY GOOD REASON.",
                  "",
                  "To save on BPF instructions, the serialized bytes are reinterpreted with a",
                  "bytemuck transmute, which means that this structure cannot have any",
                  "undeclared alignment-padding in its representation."
                ],
                "type": {
                  "kind": "struct",
                  "fields": [
                    {
                      "name": "activeStakeLamports",
                      "docs": [
                        "Amount of lamports on the validator stake account, including rent",
                        "",
                        "Note that if `last_update_epoch` does not match the current epoch then",
                        "this field may not be accurate"
                      ],
                      "type": {
                        "defined": "PodU64"
                      }
                    },
                    {
                      "name": "transientStakeLamports",
                      "docs": [
                        "Amount of transient stake delegated to this validator",
                        "",
                        "Note that if `last_update_epoch` does not match the current epoch then",
                        "this field may not be accurate"
                      ],
                      "type": {
                        "defined": "PodU64"
                      }
                    },
                    {
                      "name": "lastUpdateEpoch",
                      "docs": [
                        "Last epoch the active and transient stake lamports fields were updated"
                      ],
                      "type": {
                        "defined": "PodU64"
                      }
                    },
                    {
                      "name": "transientSeedSuffix",
                      "docs": [
                        "Transient account seed suffix, used to derive the transient stake",
                        "account address"
                      ],
                      "type": {
                        "defined": "PodU64"
                      }
                    },
                    {
                      "name": "unused",
                      "docs": [
                        "Unused space, initially meant to specify the end of seed suffixes"
                      ],
                      "type": {
                        "defined": "PodU32"
                      }
                    },
                    {
                      "name": "validatorSeedSuffix",
                      "docs": [
                        "Validator account seed suffix"
                      ],
                      "type": {
                        "defined": "PodU32"
                      }
                    },
                    {
                      "name": "status",
                      "docs": [
                        "Status of the validator stake account"
                      ],
                      "type": {
                        "defined": "PodStakeStatus"
                      }
                    },
                    {
                      "name": "voteAccountAddress",
                      "docs": [
                        "Validator vote account address"
                      ],
                      "type": "publicKey"
                    }
                  ]
                }
              },
              {
                "name": "Fee",
                "docs": [
                  "Fee rate as a ratio, minted on `UpdateStakePoolBalance` as a proportion of",
                  "the rewards",
                  "If either the numerator or the denominator is 0, the fee is considered to be",
                  "0"
                ],
                "type": {
                  "kind": "struct",
                  "fields": [
                    {
                      "name": "denominator",
                      "docs": [
                        "denominator of the fee ratio"
                      ],
                      "type": "u64"
                    },
                    {
                      "name": "numerator",
                      "docs": [
                        "numerator of the fee ratio"
                      ],
                      "type": "u64"
                    }
                  ]
                }
              },
              {
                "name": "AccountType",
                "docs": [
                  "Enum representing the account type managed by the program"
                ],
                "type": {
                  "kind": "enum",
                  "variants": [
                    {
                      "name": "Uninitialized"
                    },
                    {
                      "name": "StakePool"
                    },
                    {
                      "name": "ValidatorList"
                    }
                  ]
                }
              },
              {
                "name": "StakeStatus",
                "docs": [
                  "Status of the stake account in the validator list, for accounting"
                ],
                "type": {
                  "kind": "enum",
                  "variants": [
                    {
                      "name": "Active"
                    },
                    {
                      "name": "DeactivatingTransient"
                    },
                    {
                      "name": "ReadyForRemoval"
                    },
                    {
                      "name": "DeactivatingValidator"
                    },
                    {
                      "name": "DeactivatingAll"
                    }
                  ]
                }
              },
              {
                "name": "FutureEpoch",
                "docs": [
                  "Wrapper type that \"counts down\" epochs, which is Borsh-compatible with the",
                  "native `Option`"
                ],
                "type": {
                  "kind": "enum",
                  "variants": [
                    {
                      "name": "None"
                    },
                    {
                      "name": "One",
                      "fields": [
                        {
                          "defined": "T"
                        }
                      ]
                    },
                    {
                      "name": "Two",
                      "fields": [
                        {
                          "defined": "T"
                        }
                      ]
                    }
                  ]
                }
              },
              {
                "name": "FeeType",
                "docs": [
                  "The type of fees that can be set on the stake pool"
                ],
                "type": {
                  "kind": "enum",
                  "variants": [
                    {
                      "name": "SolReferral",
                      "fields": [
                        "u8"
                      ]
                    },
                    {
                      "name": "StakeReferral",
                      "fields": [
                        "u8"
                      ]
                    },
                    {
                      "name": "Epoch",
                      "fields": [
                        {
                          "defined": "Fee"
                        }
                      ]
                    },
                    {
                      "name": "StakeWithdrawal",
                      "fields": [
                        {
                          "defined": "Fee"
                        }
                      ]
                    },
                    {
                      "name": "SolDeposit",
                      "fields": [
                        {
                          "defined": "Fee"
                        }
                      ]
                    },
                    {
                      "name": "StakeDeposit",
                      "fields": [
                        {
                          "defined": "Fee"
                        }
                      ]
                    },
                    {
                      "name": "SolWithdrawal",
                      "fields": [
                        {
                          "defined": "Fee"
                        }
                      ]
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
            ],
            "metadata": {
              "address": "Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d"
            }
          }