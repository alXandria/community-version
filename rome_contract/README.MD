# This contract is a decentralized encyclopedia. It is live on Juno testnet and functioning! 

# New Contract
juno1thdtalue0s0zkc2k7edph475uud0y8dpyjmvdad2vdqpelk65jdsu2m4vv


# Test November 30

junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/DD7EABE96BCDB1B5B01A1DE8267DE25483889356C49C60D296E2933F9CF78A6B

*Code ID* [junod query tx]
2901

*Instantiation*
junod tx wasm instantiate 2901 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/25C8AA6D23BBF0B56A1BB19C142910EB13AB78EE368375EEAF0BB7B2F9EA1E9B

*Contract Address*
juno1278hqgjtkcvfghucweyr8gr836efweyy33fcslzy45kgrnyazfcqwxfwxf

*Create Post:*
junod tx wasm execute juno1thdtalue0s0zkc2k7edph475uud0y8dpyjmvdad2vdqpelk65jdsu2m4vv '{"create_post":{"post_title":"Post 2","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass", "funds","attempt"]}}' --amount 1000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/658C317EF0CD953AA7A76D4E70FB43889387AC05760E666064CD12C96B2A8EEC

*Test Edit & Fee Split:*
junod tx wasm execute juno1278hqgjtkcvfghucweyr8gr836efweyy33fcslzy45kgrnyazfcqwxfwxf '{"edit_post":{"post_id":1,"external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This edit should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 2000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/8ADE226BD849E72D83562CCC9C4D25440BA0341AEE95E1C84E571E9EA659E151

*Query One Post*
junod query wasm contract-state smart juno1278hqgjtkcvfghucweyr8gr836efweyy33fcslzy45kgrnyazfcqwxfwxf '{"post":{"post_id": 1}}'

*Test Delete:*
junod tx wasm execute juno1278hqgjtkcvfghucweyr8gr836efweyy33fcslzy45kgrnyazfcqwxfwxf '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/B120822E1451295AD9B256C10E7B5F0089D1B7B180C264EA126CEE91F37F827F

*Query All Posts*
junod query wasm contract-state smart juno1278hqgjtkcvfghucweyr8gr836efweyy33fcslzy45kgrnyazfcqwxfwxf '{"all_posts":{}}'

*Withdraw*
junod tx wasm execute juno1278hqgjtkcvfghucweyr8gr836efweyy33fcslzy45kgrnyazfcqwxfwxf '{"withdraw":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/77A3E34C24CFE7D61436258A4D932011F7F705F5495B77E1492E83569949642B

*Migrate*
Code ID: 2919

junod tx wasm migrate juno1278hqgjtkcvfghucweyr8gr836efweyy33fcslzy45kgrnyazfcqwxfwxf 2919 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/0A9EE3BAA2179A65E3853426F120D5089CF4C8251C8E53C13504F342F09327D7

*Query Article Count*
junod query wasm contract-state smart juno1thdtalue0s0zkc2k7edph475uud0y8dpyjmvdad2vdqpelk65jdsu2m4vv '{"article_count":{}}'