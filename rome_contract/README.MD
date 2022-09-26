This contract is a decentralized encyclopedia. It is live on testnet and functioning! Proof below.

successful store alxandria

successful command:
desmos tx wasm store alxandria.wasm --from alxandria_test --fees 5000udaric --gas 2000000 -y

Code ID: 13

https://morpheus.desmos.network/transactions/E6F6C784402A0C1D4A879140DB5AEACEF28E9B09FBA010414980FE3613084B43

successful instantiation

successful command:
desmos tx wasm instantiate 13 '{"admin":"desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg"}' --label alxandria-test --admin desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg --from alxandria_test --fees 5000udaric --gas 2000000 -y

https://morpheus.desmos.network/transactions/460CC8FE0C27BAEA7323816608ACECA680DBD8903D509BA98B04073BCC1388CC

query contract’s existence

*get code ID from json output from instantiation

desmos query wasm list-contract-by-code 13

contract address: desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s

execute message

successful command:
desmos tx wasm execute desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s '{"create_post": {"post_id": 1,"external_id": "https://www.google.com","text": "This is my first post","tags": ["google", "post", "attempt"]}}' --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/E86E97DC701B341E4BAAF005BEAAD5FB7814C88379780BF647FE21A46923E7B1

query new post

successful command:
desmos query wasm contract-state smart desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s '{"post":{"post_id": 1}}'

output:
post":{"post_id": 1}}'
data:
  post:
    author: desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg
    creation_date: "1664212492.539425833"
    deleter: null
    editor: null
    external_id: https://www.google.com
    last_edit_date: null
    post_id: 1
    tags:
    - google
    - post
    - attempt
    text: This is my first post