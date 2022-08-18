import { IndexedTx, SigningStargateClient, StargateClient } from "@cosmjs/stargate"
import { Tx } from "cosmjs-types/cosmos/tx/v1beta1/tx"
import { MsgSend } from "cosmjs-types/cosmos/bank/v1beta1/tx"
import { readFile } from "fs/promises"
import { DirectSecp256k1HdWallet, OfflineDirectSigner } from "@cosmjs/proto-signing"

//testnet rpc
const rpc = "rpc.sentry-01.theta-testnet.polypore.xyz:26657"

//top-level function that returns an OfflineDirectSigner
const getAliceSignerFromMnemonic = async (): Promise<OfflineDirectSigner> => {
    return DirectSecp256k1HdWallet.fromMnemonic((await readFile("./testnet.alice.mnemonic.key")).toString(), {
        prefix: "cosmos",
    })
}

const runAll = async(): Promise<void> => {
    const client = await StargateClient.connect(rpc)
    const faucetTx: IndexedTx = (await client.getTx(
        "A67B4D2DC219DA01068A7118BA8C55AE727C04A8C0ECA1D9913A3ABF879CAD08",
    ))!
    const decodedTx: Tx = Tx.decode(faucetTx.tx)
    const sendMessage: MsgSend = MsgSend.decode(decodedTx.body!.messages[0].value)
    const faucet: string = sendMessage.fromAddress
    const validator: string = "cosmosvaloper178h4s6at5v9cd8m9n7ew3hg7k9eh0s6wptxpcn"
    const rawLog = JSON.parse(faucetTx.rawLog)
    const aliceSigner: OfflineDirectSigner = await getAliceSignerFromMnemonic()
    const alice = (await aliceSigner.getAccounts())[0].address
    const signingClient = await SigningStargateClient.connectWithSigner(rpc, aliceSigner)

    //send tokens and store the result
    // const result = await signingClient.signAndBroadcast(
    //     // the signerAddress
    //     alice,
    //     // the message(s)
    //     [
    //         {
    //             typeUrl: "/cosmos.bank.v1beta1.MsgSend",
    //             value: {
    //                 fromAddress: alice,
    //                 toAddress: faucet,
    //                 amount: [
    //                     { denom: "uatom", amount: "100000" },
    //                 ],
    //             },
    //         },
    //     ],
    //     // the fee
    //     {
    //         amount: [{ denom: "uatom", amount: "500" }],
    //         gas: "200000",
    //     },
    // )

    //send tokens and delegate simultaneously
    const result = await signingClient.signAndBroadcast(
        alice,
        [
            {
                typeUrl: "/cosmos.bank.v1beta1.MsgSend",
                value: {
                    fromAddress: alice,
                    toAddress: faucet,
                    amount: [
                        { denom: "uatom", amount: "100000" },
                    ],
                },
            },
            {
                typeUrl: "/cosmos.staking.v1beta1.MsgDelegate",
                value: {
                    delegatorAddress: alice,
                    validatorAddress: validator,
                    amount: { denom: "uatom", amount: "1000", },
                },
              },
        ],
        {
            amount: [{ denom: "uatom", amount: "500" }],
            gas: "200000",
        }
    )
    
    console.log("Alice balance after:", await client.getAllBalances(alice))
    console.log("Faucet balance after:", await client.getAllBalances(faucet))
}

runAll()