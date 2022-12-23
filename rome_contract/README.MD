# This contract is a decentralized encyclopedia. It is live on Juno testnet and functioning! 

# Test Withdraw

*Store*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

*Code ID*
3773

*Instantiation*
junod tx wasm instantiate 3773 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

*Contract Address*
juno167h49ng8gftcc5njsvfz9a2x78lc7wps7f34ysurfpk6kdkj5w2q0j39z4

*Create Post Without Profile Name*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"create_post":{"post_title":"Post 2","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["no",  "profile", "name"]}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

# December 16 Full Test

*Store*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

*Code ID*
3485

*Instantiation*
junod tx wasm instantiate 3485 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

https://testnet.juno.explorers.guru/transaction/E52B73F184F81C0600B699116EA1D45C39F96B646DBF65DF28D9473F5B6DA72A

*Contract Address*
juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft

*Register Profile and check formatting (lowercase + remove space)*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"register_profile_name":{"profile_name":"Vitali k"}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/7091D1A51EB10928EA2D6094A0C45A7CE110DA48D89E1A611895AF8A9D210DFD

*Create Post With Profile Name*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"create_post":{"post_title":"Post 1","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass", "funds","attempt"]}}' --amount 1000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/E3E3EA760C62C5D91AFD9DDBA5493079DA17E209CCC43253791F86E1A67E9A1A

*Query All Posts*
junod query wasm contract-state smart juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"all_posts":{}}'

*Create Post Without Profile Name*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"create_post":{"post_title":"Post 2","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["no",  "profile", "name"]}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/5A3ADF0D0DA56B774324EA4DB90FC2A4CBC5F0E1D7BD9617CC194F5D5AEE1E09

*Register Profile without validation requirement*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"register_profile_name":{"profile_name":"satoshi"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/48CCD0D6B3E8A87C2E6A1B6D92028E4FC6921A00F6CE380141EDA0BB4504461A

*Code ID*
3486

*Migrate*
junod tx wasm migrate juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft 3486 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://testnet.juno.explorers.guru/transaction/960A1C5C0315AF4F7954E4A9A0CCE28FE85F68A1D30333CB8D44A3F743B110C6

*Test Edit*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"edit_post":{"post_id":1,"external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This edit should pass!","tags":["pass","double funds","edited post"]}}' --amount 200000ujunox --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/B8FA2CF9D7DF1A74D23AEB0A24A4C14759D74EDCAAE8A8E485110346FA510C7E

*Test Delete:*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/A1A9571550DD27C82086B465F5E4D3BB033CEBAE12CB170F7424EBCF3ABA8AFE

*Withdraw*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"withdraw":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/E1E94B0E2669F6B046485513089E72227EB695DD316B0B4A9576C7846CA5FE53

*Like Post*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"like_post":{"post_id":2}}' --amount 10000ujunox --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/CEDBB69072B783A36128A7204E42A3D6704E7AC1595BE2D1FB29888F846CA081

*Query Article Count*
junod query wasm contract-state smart juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"article_count":{}}'

*Query One Post*
junod query wasm contract-state smart juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"post":{"post_id": 2}}'

*Query Profile Name*
junod query wasm contract-state smart juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"profile_name":{"address":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}}'


# Live Contract
juno1thdtalue0s0zkc2k7edph475uud0y8dpyjmvdad2vdqpelk65jdsu2m4vv
