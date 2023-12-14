import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair, clusterApiUrl, Connection, SystemProgram, TransactionInstruction, Transaction } from '@solana/web3.js'
import { Account, TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
require('dotenv').config();
const fs = require('fs');


const idl = JSON.parse(fs.readFileSync("./src/idl.json", "utf8"));
const programId = new anchor.web3.PublicKey('SVBzw5fZRY9iNRwy5JczFYni2X9aDqur6HhAP1CXX7T');
let connection = new Connection(process.env.ANCHOR_PROVIDER_URL as string)
const user = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(process.env.ANCHOR_WALLET as string))))
const provider = new anchor.AnchorProvider(connection, new NodeWallet(user), {})
const program = new anchor.Program(idl, programId, provider);
const token = new PublicKey('DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263'); 
const CONTRACT_SEED = 'contract';
const GAME_USER_SEED = 'gameuser';
const VERSION = 1;
const versionSeed = new anchor.BN(VERSION).toBuffer('le', 1);

const getGameEnd = async (gameIndex: number) => {
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
    for (var i = 0; i < 0; i++) {
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
    const tx = new Transaction().add(ix).add(memoInstruction);
    tx.recentBlockhash = latestBlockhash.blockhash;
    tx.feePayer = user.publicKey;
    const sig = await program.provider.sendAndConfirm(tx);
    await connection.confirmTransaction({
        signature: sig,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
    });

    console.log(sig);
    return sig 
}

const findPdaAddressByStringSeeds = (seeds:string[], version: Buffer) => {
    const seedBuffers = seeds.map((seedString) => {
        return Buffer.from(anchor.utils.bytes.utf8.encode(seedString));
    });
    seedBuffers.push(version);
    const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(seedBuffers, new anchor.web3.PublicKey(programId.toString()));
    const pdaAddress = new anchor.web3.PublicKey(pda);
    return pdaAddress;
}

const findGameUserPdaAddress = (stringSeed, gameIndex: number, user?: PublicKey) => {
    const gameUserSeed = Buffer.from(anchor.utils.bytes.utf8.encode(stringSeed));
    const gameIndexSeed = new anchor.BN(gameIndex).toBuffer('le', 4);
    const userSeed = user != undefined ? user.toBuffer() : undefined
    if (userSeed != undefined){
        const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [gameUserSeed, gameIndexSeed, userSeed, versionSeed], 
            new anchor.web3.PublicKey(programId.toString()));
        console.log(`Bump: `, bump);
        console.log(`PDA: `, pda.toString());
        return pda;
    }
    else {
        const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [gameUserSeed, gameIndexSeed, versionSeed], 
            new anchor.web3.PublicKey(programId.toString()));
        console.log(`Bump: `, bump);
        console.log(`PDA: `, pda.toString());
        return pda;
    }
}

export default {getGameEnd, buy};