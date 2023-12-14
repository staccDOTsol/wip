import {
    Account,
    Connection,
    Keypair,
    PublicKey,
} from "@solana/web3.js";
import {NodeWallet} from "@project-serum/common"; //TODO remove this

import {use} from "chai";
import ChaiAsPromised from "chai-as-promised";
import {
    Fanout,
    FanoutClient,
    FanoutMint,
    MembershipModel,
} from "../packages/sdk/src";
import fs from 'fs'
async function main(){
    const connection = new Connection("https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718", "confirmed");
    let authorityWallet: Keypair = Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(
                fs.readFileSync(
                    "/Users/jd/jaregm.json",
                    "utf-8"
                )
            )
        ));
    let fanoutSdk: FanoutClient = new FanoutClient(
            connection,
            new NodeWallet(new Account(authorityWallet.secretKey))
        );
        //await airdrop(connection, authorityWallet.publicKey);

            const tipNative = new PublicKey("49juGJy9UMzZaZx1xK9bdMBwoypZX8PQSH2b5yeEjhK4")
            /*
            const {fanout, nativeAccount} = await fanoutSdk.initializeFanout({
                totalShares: 3_000_001,
                name: `real-stacc-megayield`,
                membershipModel: MembershipModel.Wallet,
            });
            const {fanout: tokenFanout, nativeAccount: tokenNative} = await fanoutSdk.initializeFanout({
                totalShares: 0,
                name: `stacc-bonk-megayield`,
                membershipModel: MembershipModel.Token,
                mint: new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263")
            });

            const {membershipAccount: stacc} = await fanoutSdk.addMemberWallet({
                fanout: new PublicKey("2bxwkKqwzkvwUqj3xYs4Rpmo1ncPcA1TedAPzTXN1yHu"),
                fanoutNativeAccount: tipNative,
                membershipKey: fanout,
                shares: 1_000_000,
            });
            // add 97Rdp3xP99DgtDZHA2gMca9gqShbBVtWvqeaNhDzg8fT with 1_000_000 shares
            const {membershipAccount: katz} = await fanoutSdk.addMemberWallet({
                fanout: new PublicKey("2bxwkKqwzkvwUqj3xYs4Rpmo1ncPcA1TedAPzTXN1yHu"),
                fanoutNativeAccount: tipNative,
                membershipKey: new PublicKey("97Rdp3xP99DgtDZHA2gMca9gqShbBVtWvqeaNhDzg8fT"),
                shares: 1_000_000,
            });

            // add 97Rdp3xP99DgtDZHA2gMca9gqShbBVtWvqeaNhDzg8fT with 1_000_000 shares
            const {membershipAccount: staccBackpack1} = await fanoutSdk.addMemberWallet({
                fanout: new PublicKey("2bxwkKqwzkvwUqj3xYs4Rpmo1ncPcA1TedAPzTXN1yHu"),
                fanoutNativeAccount: tipNative,
                membershipKey: new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6"),
                shares: 1,
            });
            // add membershipAccount: azoth with 1_000_000 shares

*/

const fanout = new PublicKey("8ba9hD1SAnKHZx6S7ZAsZhSMuaJpRzTXMWmPPC3LkXXe")
const nativeAccount = new PublicKey("9vjUMh1UhJLUZA2KsiWHG7MCXD72NHTtTdvFCvnBpPzC")
        /*    const {membershipAccount: staccNft1} = await fanoutSdk.addMemberWallet({
                fanout,
                fanoutNativeAccount: nativeAccount,
                membershipKey: new PublicKey("BXyrHAq72V8C2x7PYsbi3HfGeVDBgJFrr1qdWswTs2mj"),
                shares: 1_000_000,
            });
            const {membershipAccount: staccNft2} = await fanoutSdk.addMemberWallet({
                fanout,
                fanoutNativeAccount: nativeAccount,
                membershipKey: new PublicKey("5X79LBTMmwg5cZ218WsfxaQ7ao4kKmFGEKP6n9MEHovZ"),
                shares: 1_000_000,
            });

            const {membershipAccount: staccBonk} = await fanoutSdk.addMemberWallet({
                fanout,
                fanoutNativeAccount: nativeAccount,
                membershipKey: new PublicKey("35n15A1W3m2zeAHq5vM4sh4DSpUG2Ahf1aws5MRWfMjg"),
                shares: 1_000_000,
            });
            const {membershipAccount: staccBackpack} = await fanoutSdk.addMemberWallet({
                fanout,
                fanoutNativeAccount: nativeAccount,
                membershipKey: new PublicKey("Gf3sbc5Jb62jH7WcTr3WSNGDQLk1w6wcKMZXKK1SC1E6"),
                shares: 1,
            });*/
            console.log('sol holding account for fanout', nativeAccount.toBase58())
            const mints = ["So11111111111111111111111111111111111111112",
                "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1"
            ].map((m) => new PublicKey(m));
            const stacc1 = new PublicKey("5X79LBTMmwg5cZ218WsfxaQ7ao4kKmFGEKP6n9MEHovZ")
            const stacc2 = new PublicKey("BXyrHAq72V8C2x7PYsbi3HfGeVDBgJFrr1qdWswTs2mj")
            for (var mint of mints){
                for (const hydra of [stacc1, stacc2]){
                    try {
                        const {fanoutForMint, tokenAccount} =
                        await fanoutSdk.initializeFanoutForMint({
                            fanout: hydra,
                            mint: mint,
                        });
                    console.log('tokenAccount for mint', mint.toBase58(), tokenAccount.toBase58())
                
                    }
                    catch (e){
                        console.log(e)
                    }
                }
            }
                
                
            


/*

        it("Adds Members With Wallet", async () => {
            const init = await fanoutSdk.initializeFanout({
                totalShares: 100,
                name: `Test${Date.now()}`,
                membershipModel: MembershipModel.Wallet,
            });
            const member = new Keypair();
            const {membershipAccount} = await fanoutSdk.addMemberWallet({
                fanout: init.fanout,
                fanoutNativeAccount: init.nativeAccount,
                membershipKey: member.publicKey,
                shares: 10,
            });
            const fanoutAccount = await fanoutSdk.fetch<Fanout>(init.fanout, Fanout);
            const membershipAccountData =
                await fanoutSdk.fetch<FanoutMembershipVoucher>(
                    membershipAccount,
                    FanoutMembershipVoucher
                );
            expect(fanoutAccount.membershipModel).to.equal(MembershipModel.Wallet);
            expect(fanoutAccount.lastSnapshotAmount.toString()).to.equal("0");
            expect(fanoutAccount.totalMembers.toString()).to.equal("1");
            expect(fanoutAccount.totalInflow.toString()).to.equal("0");
            expect(fanoutAccount.totalAvailableShares.toString()).to.equal("90");
            expect(fanoutAccount.totalShares.toString()).to.equal("100");
            expect(fanoutAccount.membershipMint).to.equal(null);
            expect(fanoutAccount.totalStakedShares).to.equal(null);
            expect(membershipAccountData?.shares?.toString()).to.equal("10");
            expect(membershipAccountData?.membershipKey?.toBase58()).to.equal(
                member.publicKey.toBase58()
            );
        });

        it("Distribute a Native Fanout with Wallet Members", async () => {
            let builtFanout = await builtWalletFanout(fanoutSdk, 100, 5);
            expect(
                builtFanout.fanoutAccountData.totalAvailableShares.toString()
            ).to.equal("0");
            expect(builtFanout.fanoutAccountData.totalMembers.toString()).to.equal(
                "5"
            );
            expect(
                builtFanout.fanoutAccountData.lastSnapshotAmount.toString()
            ).to.equal("0");
            const distBot = new Keypair();
            const sent = 10;
            //await airdrop(connection, builtFanout.fanoutAccountData.accountKey, sent);
            //await airdrop(connection, distBot.publicKey, 1);

            let member1 = builtFanout.members[0];
            let member2 = builtFanout.members[1];
            let distMember1 = await fanoutSdk.distributeWalletMemberInstructions({
                distributeForMint: false,
                member: member1.wallet.publicKey,
                fanout: builtFanout.fanout,
                payer: distBot.publicKey,
            });
            let distMember2 = await fanoutSdk.distributeWalletMemberInstructions({
                distributeForMint: false,
                member: member2.wallet.publicKey,
                fanout: builtFanout.fanout,
                payer: distBot.publicKey,
            });
            const holdingAccountReserved =
                await connection.getMinimumBalanceForRentExemption(1);
            const memberDataBefore1 = await connection.getAccountInfo(
                member1.wallet.publicKey
            );
            const memberDataBefore2 = await connection.getAccountInfo(
                member2.wallet.publicKey
            );
            const holdingAccountBefore = await connection.getAccountInfo(
                builtFanout.fanoutAccountData.accountKey
            );
            expect(memberDataBefore2).to.be.null;
            expect(memberDataBefore1).to.be.null;
            const firstSnapshot = sent * LAMPORTS_PER_SOL;
            expect(holdingAccountBefore?.lamports).to.equal(
                firstSnapshot + holdingAccountReserved
            );
            const tx = await fanoutSdk.sendInstructions(
                [...distMember1.instructions, ...distMember2.instructions],
                [distBot],
                distBot.publicKey
            );
            if (!!tx.RpcResponseAndContext.value.err) {
                const txdetails = await connection.getConfirmedTransaction(
                    tx.TransactionSignature
                );
                console.log(txdetails, tx.RpcResponseAndContext.value.err);
            }
            const memberDataAfter1 = await connection.getAccountInfo(
                member1.wallet.publicKey
            );
            const memberDataAfter2 = await connection.getAccountInfo(
                member2.wallet.publicKey
            );
            const holdingAccountAfter = await connection.getAccountInfo(
                builtFanout.fanoutAccountData.accountKey
            );
            const membershipAccount1 = await fanoutSdk.fetch<FanoutMembershipVoucher>(
                member1.voucher,
                FanoutMembershipVoucher
            );

            expect(memberDataAfter1?.lamports).to.equal(firstSnapshot * 0.2);
            expect(memberDataAfter2?.lamports).to.equal(firstSnapshot * 0.2);
            expect(holdingAccountAfter?.lamports).to.equal(
                firstSnapshot - firstSnapshot * 0.4 + holdingAccountReserved
            );
            expect(
                builtFanout.fanoutAccountData.lastSnapshotAmount.toString()
            ).to.equal("0");
            expect(membershipAccount1.totalInflow.toString()).to.equal(
                `${firstSnapshot * 0.2}`
            );
        });

        it("Transfer Shares", async () => {
            let builtFanout = await builtWalletFanout(fanoutSdk, 100, 5);
            const sent = 10;
            //await airdrop(connection, builtFanout.fanoutAccountData.accountKey, sent);
            //await airdrop(connection, fanoutSdk.wallet.publicKey, 1);
            const member0Wallet = builtFanout.members[0].wallet;
            const member1Wallet = builtFanout.members[1].wallet;
            const member0Voucher = builtFanout.members[0].voucher;
            const member1Voucher = builtFanout.members[1].voucher;

            await fanoutSdk.transferShares({
                fromMember: member0Wallet.publicKey,
                toMember: member1Wallet.publicKey,
                fanout: builtFanout.fanout,
                shares: 20
            });

            const membershipAccount0 = await fanoutSdk.fetch<FanoutMembershipVoucher>(
                member0Voucher,
                FanoutMembershipVoucher
            );
            const membershipAccount1 = await fanoutSdk.fetch<FanoutMembershipVoucher>(
                member1Voucher,
                FanoutMembershipVoucher
            );

            expect(membershipAccount0.shares.toString()).to.equal("0");
            expect(membershipAccount1.shares.toString()).to.equal("40");
        });

        it("Remove Member", async () => { //haha nvm
            let builtFanout = await builtWalletFanout(fanoutSdk, 100, 5);
            const sent = 10;
            const rando = new Keypair();
            //await airdrop(connection, builtFanout.fanoutAccountData.accountKey, sent);
            //await airdrop(connection, fanoutSdk.wallet.publicKey, 1);
            const member0Wallet = builtFanout.members[0].wallet;
            const member1Wallet = builtFanout.members[1].wallet;
            const member0Voucher = builtFanout.members[0].voucher;

            await fanoutSdk.transferShares({
                fromMember: member0Wallet.publicKey,
                toMember: member1Wallet.publicKey,
                fanout: builtFanout.fanout,
                shares: 20
            });
            await fanoutSdk.removeMember({
                destination: rando.publicKey,
                fanout: builtFanout.fanout,
                member: member0Wallet.publicKey
            });

            let fanout_after = await fanoutSdk.fetch<Fanout>(builtFanout.fanout, Fanout)
            expect(fanout_after.totalMembers.toString()).to.equal('4');


            expect(fanoutSdk.getAccountInfo(member0Voucher)).to.be.rejectedWith(new Error('Account Not Found'));
        });*/
}
main()