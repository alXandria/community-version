# This contract is a decentralized encyclopedia. It is live on testnet and functioning! However, we will not deploy on Mainnet Desmos. Avoid that team like the plague.

# Full Contract Test With Updated Contract (Oct 10)

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
