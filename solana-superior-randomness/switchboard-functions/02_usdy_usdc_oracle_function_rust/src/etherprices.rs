// Note: EtherPrices API requires a non-US IP address
use crate::solana_sdk::system_program;
use crate::*;
use crate::anchor_spl::token::spl_token;
use switchboard_solana::get_ixn_discriminator;
use usdy_usd_oracle::{OracleDataBorsh, TradingSymbol, OracleDataWithTradingSymbol, RefreshOraclesParams};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Default, Clone, Debug)]
pub struct Ticker {
    pub symbol: String, // BTCUSDT
    pub mean: I256,  // 0.00000000
    pub median: I256, // 0.00000000
    pub std: I256, // 0.00000000
}

#[derive(Clone, Debug)]
pub struct IndexData {
    pub symbol: String,
    pub data: Ticker,
}
impl TryInto<OracleDataBorsh> for IndexData {
    
    type Error = SbError;

    fn try_into(self) -> Result<OracleDataBorsh, Self::Error> {
        let oracle_timestamp: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| {
                SbError::CustomMessage("Invalid oracle_timestamp".to_string())
            })?
            .as_secs()
            .try_into()
            .map_err(|_| {
                SbError::CustomMessage("Invalid oracle_timestamp".to_string())
            })?;

            switchboard_solana::Result::Ok(OracleDataBorsh {
                oracle_timestamp,
                mean: self.data.mean.as_u64(),
                median: self.data.median.as_u64(),
                std: self.data.std.as_u64(),

            })
    }
}

pub struct EtherPrices {
    pub jitosol_sol: IndexData,
    pub bsol_sol: IndexData,
    pub wsol_borrow: IndexData,

}

impl EtherPrices {

    // Fetch data from the EtherPrices API
    pub async fn fetch(mean:  ethers::types::U256, median:  ethers::types::U256, std:  ethers::types::U256,
         mean2:  ethers::types::U256, median2:  ethers::types::U256, std2:  ethers::types::U256,
         mean3:  ethers::types::U256, median3:  ethers::types::U256, std3:  ethers::types::U256) -> std::result::Result<EtherPrices, SbError> {
        let symbols = ["BSOL_sol", "JITOSOL_sol", "WSOL_borrow"];
        let mean: I256 = mean.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid mean".to_string())
        }).unwrap();
        let median: I256 = median.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid median".to_string())
        }).unwrap();
        let std: I256 = std.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid std".to_string())
        }).unwrap();
        let mean2: I256 = mean2.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid mean".to_string())
        }).unwrap();
        let median2: I256 = median2.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid median".to_string())
        }).unwrap();
        let std2: I256 = std2.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid std".to_string())
        }).unwrap();
        let mean3: I256 = mean3.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid mean".to_string())
        }).unwrap();
        let median3: I256 = median3.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid median".to_string())
        }).unwrap();
        let std3: I256 = std3.try_into().map_err(|_| {
            SbError::CustomMessage("Invalid std".to_string())
        }).unwrap();
        Ok(EtherPrices {
            bsol_sol: {
                let symbol = symbols[0];
                
                IndexData {
                    symbol: symbol.to_string(),
                    data: Ticker {
                        symbol: symbol.to_string(),
                        mean,
                        median,
                        std,
                    
                    }
                }
            },
            jitosol_sol: {
                let symbol = symbols[1];
                
                IndexData {
                    symbol: symbol.to_string(),
                    data: Ticker {
                        symbol: symbol.to_string(),
                        mean: mean2,
                        median: median2,
                        std: std2,
                    
                    }
                }
            },
            wsol_borrow: {
                let symbol = symbols[2];
                
                IndexData {
                    symbol: symbol.to_string(),
                    data: Ticker {
                        symbol: symbol.to_string(),
                        mean: mean3,
                        median: median3,
                        std: std3,
                    
                    }
                }
            },
        })
    }
    pub fn findGameUserPdaAddress(gameUserSeed: &[u8], gameIndex: u64, user: Option<Pubkey>) -> Pubkey {
        let CONTRACT_SEED = b"contract";
        let GAME_USER_SEED = b"gameuser";
        /*
        const VERSION = 1;
        const versionSeed = new anchor.BN(VERSION).toBuffer('le', 1);
         */
        let VERSION: i32 = 1;
        let versionSeed = VERSION.to_le_bytes();
        let raffle = b"raffle";
        let gameIndex: i32 = 99;
        let gameIndexSeed = gameIndex.to_le_bytes();
        if (user.is_some()){
            let userSeed = user.unwrap().to_bytes();
            
            let (pda, bump) = Pubkey::find_program_address(
                &[gameUserSeed, &gameIndexSeed, &userSeed, &versionSeed], 
                &Pubkey::from_str("SVBzw5fZRY9iNRwy5JczFYni2X9aDqur6HhAP1CXX7T").unwrap()
            );
            println!("Bump: {:?}", bump);
            println!("PDA: {:?}", pda.to_string());
            return pda;
        }
        else {
            let (pda, bump) = Pubkey::find_program_address(
                &[gameUserSeed, &gameIndexSeed, &versionSeed], 
                &Pubkey::from_str("SVBzw5fZRY9iNRwy5JczFYni2X9aDqur6HhAP1CXX7T").unwrap()
            );
                println!("Bump: {:?}", bump);
                println!("PDA: {:?}", pda.to_string());
            return pda;
        }
    }

    pub fn to_ixns(&self, runner: &FunctionRunner) -> Vec<Instruction> {
        let program_id =Pubkey::from_str("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d").unwrap();
        let rows: Vec<OracleDataWithTradingSymbol> = vec![
            OracleDataWithTradingSymbol {
                symbol: TradingSymbol::Bsol_sol,
                data: self.bsol_sol.clone().try_into().map_err(|_| {
                    SbError::CustomMessage("Invalid oracle data".to_string())
                }).unwrap(),
            },
            OracleDataWithTradingSymbol {
                symbol: TradingSymbol::Jitosol_sol,
                data: self.jitosol_sol.clone().try_into().map_err(|_| {
                    SbError::CustomMessage("Invalid oracle data".to_string())
                }).unwrap(),
            }
            // OracleDataWithTradingSymbol {
            // symbol: TradingSymbol::Sol,
            // data: self.sol_usdt.clone().into(),
            // },
            // OracleDataWithTradingSymbol {
            // symbol: TradingSymbol::Doge,
            // data: self.doge_usdt.clone().into(),
            // },
        ];

        let params = RefreshOraclesParams { rows };

        let (program_state_pubkey, _state_bump) =
            Pubkey::find_program_address(&[b"USDY_USDC_ORACLE_V2"], &Pubkey::from_str("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d").unwrap());

        let (oracle_pubkey, _oracle_bump) =
            Pubkey::find_program_address(&[b"ORACLE_USDY_SEED_V2"], &Pubkey::from_str("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d").unwrap());
/*const getGameEnd = async (gameIndex: number) => {
    const contractPdaAddress = findPdaAddressByStringSeeds([CONTRACT_SEED], versionSeed);
    const data:any = await program.account.contract.fetch(contractPdaAddress);
    return Number(data.games[gameIndex].blocktimeEnd.toString());
}

const buy = async (user: Keypair, gameIndex: number, qty: number) => {
    const contractPdaAddress = findPdaAddressByStringSeeds([CONTRACT_SEED], versionSeed);
    const gameUserPdaAddress = findGameUserPdaAddress(GAME_USER_SEED, gameIndex, user.publicKey);
    const rafflePdaAddress = findGameUserPdaAddress('raffle', gameIndex);
    const contractTokenAccount:Account = await getOrCreateAssociatedTokenAccount(
        connection, 
        user, 
        token, 
        contractPdaAddress,
        true
    );

    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    const atas = await connection.getTokenAccountsByOwner(user.publicKey, { mint: token });
    const ata = atas.value[0].pubkey;
    for (var i = 0; i < gameIndex; i++) {
        try {
            
    const gameUserPdaAddress = findGameUserPdaAddress(GAME_USER_SEED, i, user.publicKey);
    const contractTokenAccount:Account = await getOrCreateAssociatedTokenAccount(
        connection, 
        user, 
        token, 
        contractPdaAddress,
        true
    );
    const balance = await (await connection.getTokenAccountBalance(ata)).value.uiAmount;
    if (balance > 0){
        const tx = await program.methods
        .claim(i,  VERSION)
        .accounts(
            {
                authority: user.publicKey,
                contract: contractPdaAddress,
                gameUser: gameUserPdaAddress,
                contractTokenAccount: contractTokenAccount.address,

                claimTokenAccount: ata,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,

            })
        .rpc();
        console.log(tx)
        }
    }
        catch (err){
console.log(err)
        }
    }
    const ix = await program.methods
        .buy(gameIndex, qty, VERSION)
        .accounts(
            {
                signer: user.publicKey,
                contract: contractPdaAddress,
                contractTokenAccount: contractTokenAccount.address,
                gameUser: gameUserPdaAddress,
                buyerTokenAccount: ata,
                tokenProgram: TOKEN_PROGRAM_ID,
                raffle: rafflePdaAddress,
                instructionSysvarAccount: new PublicKey('Sysvar1nstructions1111111111111111111111111'),
                systemProgram: SystemProgram.programId,

            })
        .remainingAccounts([
            { pubkey: new PublicKey('68Cj4MgS3KgRMwfKPbrPVekBNijNNg27Pu8F3bCRG2rX'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('F8FqZuUKfoy58aHLW6bfeEhfW9sTtJyqFTqnxVmGZ6dU'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('76JQzVkqHsWWXA3z4WvzzwnxVD4M1tFmFfp4NhnfcrUH'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('86C3VW44St7Nrgd3vAkwJaQuFZWYWmKCr97sJHrHfEm5'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('DveZWxw2nBDSNdqPmUmZMaxniqobWkTZdBBjvQaE2Bjx'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('EefQxy3SUAHWN7bURnMZzXXyp3BNaD73QmaMn7Do1sAc'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('FrPSjSDWsRth6euNiaGAkzv6cYHgQysbWS9xMgkQcHXk'), isWritable: true, isSigner: false }
        ])
        .signers([user])    
        .instruction();
        const memo = Buffer.from(user.publicKey.toBase58()+'-0-1-1');
        const memoInstruction = new TransactionInstruction({
            keys: [
                { pubkey: user.publicKey, isSigner: true, isWritable: true },
            ],
            data: memo,
            programId: new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'),
        });
 */
let CONTRACT_SEED = b"contract";
let GAME_USER_SEED = b"gameuser";
/*
const VERSION = 1;
const versionSeed = new anchor.BN(VERSION).toBuffer('le', 1);
 */
let VERSION: i32 = 1;
let versionSeed = VERSION.to_le_bytes();
let raffle = b"raffle";
let gameIndex = 100;
let signer = Pubkey::from_str("A1HH4zftzrX9mbvfRMLHtz14inCSKRG3sVa7KZvrnUYK").unwrap();
let gameUserPdaAddress = Self::findGameUserPdaAddress(GAME_USER_SEED, gameIndex, Some(signer));
let rafflePdaAddress = Self::findGameUserPdaAddress(raffle, gameIndex, None);


    let (marginfi_pda, _bump) =
        Pubkey::find_program_address(&[b"jarezi", Pubkey::from_str("JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm").unwrap().as_ref()], &program_id);
    let (marginfi_pda_switchboard, _bump) =
        Pubkey::find_program_address(&[b"jarezi", marginfi_pda.as_ref()], &program_id);
    let winner_winner_chickum_dinner = Pubkey::from_str("JARehRjGUkkEShpjzfuV4ERJS25j8XhamL776FAktNGm").unwrap();
let buyerTokenAccount = Pubkey::from_str("GQPKZ6rFqWyehRH4Xo1BymguaJQByrQDHTkvMsch3q3w").unwrap();
        let ixn = Instruction {
            program_id: Pubkey::from_str("Gyb6RKsLsZa1UCJkCmKYHtEJQF15wF6ZeEqMUSCneh9d").unwrap(),
            accounts: vec![
                AccountMeta {
                    pubkey: program_state_pubkey,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: oracle_pubkey,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: runner.function,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: runner.signer,
                    is_signer: true,
                    is_writable: false,
                },
                /// BONK
                AccountMeta {
                    pubkey: Pubkey::from_str("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263").unwrap(),
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: marginfi_pda,
                    is_signer: false,
                    is_writable: true
                },
                AccountMeta {
                    pubkey: signer, 
                    is_signer: false,
                    is_writable: true
                },
                /// contract EzjgtouVfUGoqbDPgAzd91fG6hVPcvRN4uFKwPpwL36T
                AccountMeta {
                    pubkey: Pubkey::from_str("EzjgtouVfUGoqbDPgAzd91fG6hVPcvRN4uFKwPpwL36T").unwrap(),
                    is_signer: false,
                    is_writable: true
                },///7Rin3Gt2zqi1YciPfQQh5hJ8HYaTKFchZP6HaqXYVpFd
                AccountMeta {
                    pubkey: Pubkey::from_str("7Rin3Gt2zqi1YciPfQQh5hJ8HYaTKFchZP6HaqXYVpFd").unwrap(),
                    is_signer: false,
                    is_writable: true
                },
                /*                gameUser: gameUserPdaAddress,
                buyerTokenAccount: ata,
                tokenProgram: TOKEN_PROGRAM_ID,
                raffle: rafflePdaAddress,
                instructionSysvarAccount: new PublicKey('Sysvar1nstructions1111111111111111111111111'),
                systemProgram: SystemProgram.programId,
 */ 
               AccountMeta {
                pubkey: gameUserPdaAddress, 
                is_signer: false,
                is_writable: true
               },
                AccountMeta {
                 pubkey: buyerTokenAccount,
                    is_signer: false,
                    is_writable: true
                },
                AccountMeta {
                    pubkey: spl_token::id(),
                    is_signer: false,
                    is_writable: false
                },
                AccountMeta {
                    pubkey: rafflePdaAddress,
                    is_signer: false,
                    is_writable: true
                },
                AccountMeta {
                    pubkey: Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap(),
                    is_signer: false,
                    is_writable: false
                },
                AccountMeta {
                    pubkey: system_program::id(),
                    is_signer: false,
                    is_writable: false
                },

/*
            { pubkey: new PublicKey('68Cj4MgS3KgRMwfKPbrPVekBNijNNg27Pu8F3bCRG2rX'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('F8FqZuUKfoy58aHLW6bfeEhfW9sTtJyqFTqnxVmGZ6dU'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('76JQzVkqHsWWXA3z4WvzzwnxVD4M1tFmFfp4NhnfcrUH'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('86C3VW44St7Nrgd3vAkwJaQuFZWYWmKCr97sJHrHfEm5'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('DveZWxw2nBDSNdqPmUmZMaxniqobWkTZdBBjvQaE2Bjx'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('EefQxy3SUAHWN7bURnMZzXXyp3BNaD73QmaMn7Do1sAc'), isWritable: true, isSigner: false },
            { pubkey: new PublicKey('FrPSjSDWsRth6euNiaGAkzv6cYHgQysbWS9xMgkQcHXk'), isWritable: true, isSigner: false }
 */
            AccountMeta {
                pubkey: Pubkey::from_str("68Cj4MgS3KgRMwfKPbrPVekBNijNNg27Pu8F3bCRG2rX").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("F8FqZuUKfoy58aHLW6bfeEhfW9sTtJyqFTqnxVmGZ6dU").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("76JQzVkqHsWWXA3z4WvzzwnxVD4M1tFmFfp4NhnfcrUH").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("9dKYKpinYRdC21CYqAW2mwEpZuPwBN6wkoswsvpHXioA").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("86C3VW44St7Nrgd3vAkwJaQuFZWYWmKCr97sJHrHfEm5").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("DveZWxw2nBDSNdqPmUmZMaxniqobWkTZdBBjvQaE2Bjx").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("EefQxy3SUAHWN7bURnMZzXXyp3BNaD73QmaMn7Do1sAc").unwrap(),
                is_signer: false,
                is_writable: true
            },
            AccountMeta {
                pubkey: Pubkey::from_str("FrPSjSDWsRth6euNiaGAkzv6cYHgQysbWS9xMgkQcHXk").unwrap(),
                is_signer: false,
                is_writable: true
            },
            ///MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr
            AccountMeta {
                pubkey: Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap(),
                is_signer: false,
                is_writable: false
            },
            AccountMeta //SVBzw5fZRY9iNRwy5JczFYni2X9aDqur6HhAP1CXX7T
            {
                pubkey: Pubkey::from_str("SVBzw5fZRY9iNRwy5JczFYni2X9aDqur6HhAP1CXX7T").unwrap(),
                is_signer: false,
                is_writable: true
            },
            ],
            data: [
                get_ixn_discriminator("refresh_oracles").to_vec(),
                params.try_to_vec().unwrap(),
            ]
            .concat(),
        };
        vec![ixn]
    }
}

