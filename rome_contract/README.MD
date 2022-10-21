# This contract is a decentralized encyclopedia. It is live on Juno testnet and functioning! 

# Full Test October 21

*Store With Instantiation Limitation:*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/2B392DB9C7268E12E56FED5B9B6CBDCA6A325DBED8E08F3E8225ED375A30641A

*Code ID*
1256

*Instantiation*
junod tx wasm instantiate 1256 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/15F590278E70E72211CA41E5D32692782069B1D20C69F6BA548D0C066D51A3BC

*Contract Address*
juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08

*Create Post:*
junod tx wasm execute juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"create_post":{"post_title":"Post 1","external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This post should pass!","tags": ["pass", "funds", "attempt"]}}' --amount 1000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/7F681F74FBD8E1E3487715980A0305A1527A95A10835D8CF86C93D9486D9A58F

*Upload for migration*
https://explorer.testnet.sourceprotocol.io/juno/tx/E4EA5F6B0EFB4E1377A5ADF24A4D1B7DF6A8BD71AC65B33F1C1B2348B72A5EDA

*Code ID*
1258

*Migrate*
junod tx wasm migrate juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 1258 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/01FC12FBC3E2961B470C7397D7A37E5B4A0493817C79CD72F975B6343C3CBF74

*Test Edit & Fee Split:*
junod tx wasm execute juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"edit_post":{"post_id":1,"external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This edit should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 2000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/C1C589FA6D393C1831FF1FEA629B18549B97A4C9F30D65AEFB274AFCC63841C1

*Test Delete:*
junod tx wasm execute juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/468A1077735D3E778A9F24CAD268FDCD7459B14294BC1EA2FF4F880A19617EEC

*Query All Posts*
junod query wasm contract-state smart juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"all_posts":{}}'

*Query One Post*
junod query wasm contract-state smart juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"post":{"post_id": 1}}'

*Withdraw*
junod tx wasm execute juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"withdraw":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/D627D7FA21D950F1B8120601DD9E59A6ED6D00A0D301D79A1F479054A0020DF3
