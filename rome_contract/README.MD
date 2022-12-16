# This contract is a decentralized encyclopedia. It is live on Juno testnet and functioning! 

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
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"create_post":{"post_title":"Post 2","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["no", "profile","name"]}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/5A3ADF0D0DA56B774324EA4DB90FC2A4CBC5F0E1D7BD9617CC194F5D5AEE1E09

*Register Profile without validation requirement*
junod tx wasm execute juno12szwyek4ngpd96zqykdj8ghuzzff8u5wc2e70chzkay8etzpqzsq7vrkft '{"register_profile_name":{"profile_name":"satoshi"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/48CCD0D6B3E8A87C2E6A1B6D92028E4FC6921A00F6CE380141EDA0BB4504461A

# test migrate for profile query

*Store*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

*Code ID*
3373

*Migrate*
junod tx wasm migrate juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 3373 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://testnet.juno.explorers.guru/transaction/851CC734AF6AAE4B4EB3A0890D14ABA2F3B94F45B63C8B5EE23DC54C3617015F

*Query Profile Name*
junod query wasm contract-state smart juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 '{"profile_name":{"address":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}}'

# Test for profile branch Dec 8


*Migrate*
Code ID: 3249

junod tx wasm migrate juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 3249 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://testnet.juno.explorers.guru/transaction/2CCD4247C543C5C841F02890137140CE483A2E75E10E1665956B19E16DD3207F

*Test Edit & Fee Split:*
junod tx wasm execute juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 '{"edit_post":{"post_id":1,"external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This edit should pass!","tags":["pass","double funds","edited post"]}}' --amount 2000000ujunox --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/9566A0FFF41B285ABDCF9A0FC1A5ABFE2814FFDA2A0927BFF9F7CE80BB059F5C

*Test Delete:*
junod tx wasm execute juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/3BCE254F2D6F1FABD85AD636802720F865548D382C1FF495F175261AA2C0578F

*Withdraw*
junod tx wasm execute juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 '{"withdraw":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/3B97F83B33F4DAC084C90D95549678A41B347647054C54EEB132B2771AC3DB4C

*Query Article Count*
junod query wasm contract-state smart juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 '{"article_count":{}}'

*Query One Post*
junod query wasm contract-state smart juno1w9jw3j2rdm5f9t9mvqsg88z7nkd7j95ezxctkeundp66dxvwp5xq3pqa56 '{"post":{"post_id": 2}}'


# Live Contract
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