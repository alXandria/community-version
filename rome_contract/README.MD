# This contract is a decentralized encyclopedia. It is live on testnet and functioning! However, we will not deploy on Mainnet Desmos. Avoid that team like the plague.

# Full Contract Test With Updated Contract (Oct 10)
*Store With Instantiation Limitation:*
desmos tx wasm store alxandria.wasm --from alxandria_test --fees 5000udaric --gas 2000000 --instantiate-only-address desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg -y

https://morpheus.desmos.network/transactions/6B55BDE30D3A81D6F3245A33F7B9E1385CB608F4695FFA0FD7BDAEFE9B0DCB76

*Code ID:*
39

*Instantiation*
desmos tx wasm instantiate 39 '{"admin":"desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg"}' --label alxandria-test --admin desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg --from alxandria_test --fees 5000udaric --gas 2000000 -y

https://morpheus.desmos.network/transactions/65A17289C1BB1D59875E685C1E15E69E1F32D878C70A1AD5E9F78BDD92CE1A06

*Contract Address:*
desmos10cgpnhhw959mrhc6dzmt3z5mzt3xekkufljjcdhjs39jytu4jz8s8gprh9

*Create Post:*
desmos tx wasm execute desmos10cgpnhhw959mrhc6dzmt3z5mzt3xekkufljjcdhjs39jytu4jz8s8gprh9 '{"create_post":{"post_title":"Post 1","external_id": "https://www.github.com","text": "This post should pass!","tags": ["pass", "funds", "attempt"]}}' --amount 100000000udaric --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/F2FDD347B6603269B1D4CA5071837E264F31C63D47DC1B312DE2643502230B29

*Test Edit & Fee Split:*
desmos tx wasm execute desmos10cgpnhhw959mrhc6dzmt3z5mzt3xekkufljjcdhjs39jytu4jz8s8gprh9 '{"edit_post":{"post_id":1,"external_id": "https://www.cloudflare.com","text": "This post should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 200000000udaric --from test_bankmsg --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/67A07A3C77B7FD2DFF4A717CB0FF70A759863D1A4A90760FBAF80C5E0446B722

*Test Delete:*
desmos tx wasm execute desmos10cgpnhhw959mrhc6dzmt3z5mzt3xekkufljjcdhjs39jytu4jz8s8gprh9 '{"delete_post":{"post_id":1}}' --amount 1000000000udaric --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/61CB6D6A2B0DC397B14BED81E655978A6E691544E62565E5998828B44C3247FB

*Query All Posts*
desmos query wasm contract-state smart desmos10cgpnhhw959mrhc6dzmt3z5mzt3xekkufljjcdhjs39jytu4jz8s8gprh9 '{"all_posts":{}}'

*Query One Post*
desmos query wasm contract-state smart desmos10cgpnhhw959mrhc6dzmt3z5mzt3xekkufljjcdhjs39jytu4jz8s8gprh9 '{"post":{"post_id": 1}}'

*Withdraw*
desmos tx wasm execute desmos10cgpnhhw959mrhc6dzmt3z5mzt3xekkufljjcdhjs39jytu4jz8s8gprh9 '{"withdraw":{}}' --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/BB9214455E26473EDEF574AAF6E345BF713791296E6D249BDB6C792A3CE5FBEA

# Outdated
*Store With Instantiation Limitation:*

desmos tx wasm store alxandria.wasm --from alxandria_test --fees 5000udaric --gas 2000000 --instantiate-only-address desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg -y

https://morpheus.desmos.network/transactions/B231FBB712E3A27A2EED74203E11371D16C723DC0DC36C6A8886CE3F39178201

*Code ID:*
38

*Test Instantiate With Non-Admin Account (should fail):*

desmos tx wasm instantiate 38 '{"admin":"desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg"}' --label alxandria-test --admin desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg --from test_bankmsg --fees 5000udaric --gas 2000000 -y

*Fails (:*

https://morpheus.desmos.network/transactions/7AB62262C18E43BBFB0F998E1203ECADE2D068640FBA2CD63FB0D968E06C69BA

*Proper Instantiation:*

desmos tx wasm instantiate 38 '{"admin":"desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg"}' --label alxandria-test --admin desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg --from alxandria_test --fees 5000udaric --gas 2000000 -y

https://morpheus.desmos.network/transactions/48F9E578294A882023A725709D40E5D5362B909BBE5E70E5D1CEC61C578C4D36

*Contract Address:*
desmos1e0e8lq7ulc0zwf7y9h2ueljhd750netd9hy5qngta8xj4zeylhnsmj6wcu

*Create Post:*
desmos tx wasm execute desmos1e0e8lq7ulc0zwf7y9h2ueljhd750netd9hy5qngta8xj4zeylhnsmj6wcu '{"create_post":{"post_title":"Post 1","external_id": "https://www.github.com","text": "This post should pass!","tags": ["pass", "funds", "attempt"]}}' --amount 100000000udaric --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/F8A57F3D53A43511F7D4753C01F69A8000C3BA14D863C59652980EB90445F6D1

*Test Post ID goes to 2:*
desmos tx wasm execute desmos1e0e8lq7ulc0zwf7y9h2ueljhd750netd9hy5qngta8xj4zeylhnsmj6wcu '{"create_post":{"post_title":"Post 2","external_id": "https://www.google.com","text": "This post should pass!","tags": ["pass", "funds", "second attempt"]}}' --amount 100000000udaric --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/979FB3752C8DD9F077539F72C641D14F7EBBC4D95023AA1685B5A40A95A7691B

*Test Edit & Fee Split:*
desmos tx wasm execute desmos1e0e8lq7ulc0zwf7y9h2ueljhd750netd9hy5qngta8xj4zeylhnsmj6wcu '{"edit_post":{"post_id":2,"external_id": "https://www.cloudflare.com","text": "This post should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 200000000udaric --from test_bankmsg --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/F5AA3AE6E8377D9C323AF22A6BD823410B33CCC3DA7DEBDFC55FC7837EE6A15E

*Test Delete:*
desmos tx wasm execute desmos1e0e8lq7ulc0zwf7y9h2ueljhd750netd9hy5qngta8xj4zeylhnsmj6wcu '{"delete_post":{"post_id":1}}' --amount 1000000000udaric --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/19B5A22AB0CDBC30714282F8EB72CBF4DC4AC43E255B895CA7165C605429337F

*Query All Posts*
desmos query wasm contract-state smart desmos1e0e8lq7ulc0zwf7y9h2ueljhd750netd9hy5qngta8xj4zeylhnsmj6wcu '{"all_posts":{}}'

*Query One Post*
desmos query wasm contract-state smart desmos1e0e8lq7ulc0zwf7y9h2ueljhd750netd9hy5qngta8xj4zeylhnsmj6wcu '{"post":{"post_id": 1}}'
