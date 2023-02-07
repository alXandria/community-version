# This contract is a decentralized encyclopedia. It is live on Juno! https://alxandria.org

# Mainnet upgrade

*Store*
junod tx wasm store alxandria.wasm --from mainnet_admin_wallet --fees 5000ujuno --gas 2000000 --instantiate-only-address juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945 -y

https://www.mintscan.io/juno/txs/A0FEAFD86867F5A2490127B92DA9AE1A7055AD3AA5400BF032268FD310C03689

*Code ID*
1622

*Instantiation*
junod tx wasm instantiate 1622 '{"admin":"juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945"}' --label rome_contract --admin juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945 --from mainnet_admin_wallet --fees 5000ujuno --gas 2000000 -y

https://www.mintscan.io/juno/txs/A965E82A6D35AEF8B8BF98661AEA961EBD8481F738DB3FF207C9AA73888F2345

*Contract Address*
juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q

*Create Post With Profile Name*
junod tx wasm execute juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q '{"create_post":{"post_title":"Bitcoin Whitepaper","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmRfYVREfwTajdbarkxwmCcn78TM16rN7W22RZNUMiaCED","text":"Satoshis original whitepaper for his invention, Bitcoin","tags":["Bitcoin", "Crypto", "Satoshi"]}}' --from mainnet_admin_wallet --fees 500ujuno --gas 200000 -y

*Query All Posts*
junod query wasm contract-state smart juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q '{"all_posts":{}}'

*Query One Post*
junod query wasm contract-state smart juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q '{"post":{"post_id": 1}}'

*Query Article Count*
junod query wasm contract-state smart juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q '{"article_count":{}}'

*Query Profile Name*
junod query wasm contract-state smart juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q '{"profile_name":{"address":"juno1rmxl4fps24pe8s9uv8an3nqpng3ggyf8j8f20z"}}'

*Withdraw*
junod tx wasm execute juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q '{"withdraw_juno":{}}' --from mainnet_admin_wallet --fees 500ujuno --gas 200000 -y

https://www.mintscan.io/juno/txs/7F2310F01DABABC24B4850135CA5BF154F6EDA4DFA0247D74249C26F5FA7FC96

*Migrate Store*
junod tx wasm store alxandria.wasm --from mainnet_admin_wallet --fees 5000ujuno --gas 2000000 --instantiate-only-address juno1xh3mylsdmpvn0cp8mpz6uja34nev9w7ur8f945 -y

https://www.mintscan.io/juno/txs/E7ABFB92CA45300F4C2F680B67E8C5871C07E2479E0C17CF490D5FA8EF1054E1

*Code ID*
1628

*Migrate*
junod tx wasm migrate juno1d86vnmmnhc30m24mxeea0d99q3xkkh0h04mezjp38k9l79ucqdnskynj6q 1628 '{"migrate_msg":{}}' --from mainnet_admin_wallet --fees 5000ujuno --gas 2000000 -y

https://www.mintscan.io/juno/txs/9F7EEA286493342C640CCF1985D04CEFB3A26502545B856BE06F9DB2F994B29C

# Full Testnet Test December 27, 2022

*Store*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

*Code ID*
3792

*Instantiation*
junod tx wasm instantiate 3792 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

*Contract Address*
juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd

*Register Profile and check formatting (lowercase + remove space)*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"register_profile_name":{"profile_name":"Vitali k"}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/AC42DAC713BE36FFD4813F7382B49761996E4241B220A4C5DF8393AF8C4365D7

*Create Post With Profile Name*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"create_post":{"post_title":"Post 1","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass", "funds","attempt"]}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/F2CDCFBD7F1F9A41F61E68F23E1B8F40AF9B34B075401E57C74667F542CB858E

*Query All Posts*
junod query wasm contract-state smart juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"all_posts":{}}'

*Create Post Without Profile Name*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"create_post":{"post_title":"Post 2","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["no",  "profile", "name"]}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/1A2AD650E045064EAD4E6EA11B6CE51BC8A194C43CCEE7166193B983DC2FDCEE

*Register Profile without validation requirement*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"register_profile_name":{"profile_name":"satoshi"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/DE24D219AB77A76E682D4F923F9CD9EDE18B9041462356E0AEA718B809A39230

*Test Edit*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"edit_post":{"post_id":1,"external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This edit should pass!","tags":["pass","double funds","edited post"]}}' --amount 200000ujunox --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/77688802D5D0B34E208D6C169BE0AF9F6637C7C3CAD96F12AB88062D2575776C

*Test Delete:*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/25E47ACC39B3A9466314C6AF163FAC9D3AA60CA2AD0E836C39D50282AE4742BB

*Like Post*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"like_post":{"post_id":2}}' --amount 10000ujunox --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/0FFFC08D1A5149B78F7C7A55D99CB21BC73742BFF631F96CFC6C75D4BBA1B35B

*Withdraw*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"withdraw_juno":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/A6846E6B172F9AD164FE7FD9BA859EA9A01D86F766D83F67D16B9CC09A22F2BB

*Withdraw from non-admin fail*
junod tx wasm execute juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"withdraw_juno":{}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/0953DA4285C7BF85D07E6C1F49FAFFD9CDA6DE11551578263946FCA9395A264A

*Query Article Count*
junod query wasm contract-state smart juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"article_count":{}}'

*Query One Post*
junod query wasm contract-state smart juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"post":{"post_id": 2}}'

*Query Profile Name*
junod query wasm contract-state smart juno18v27s236gawlfq00x6qmauhgerqvwm3neyfv643rwkmfcn329j9s9exhzd '{"profile_name":{"address":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}}'


