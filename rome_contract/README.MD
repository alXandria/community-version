# This contract is a decentralized encyclopedia. It is live on testnet and functioning! Proof below (Scroll to bottom for New Contract).

# successful store alxandria

successful command: desmos tx wasm store alxandria.wasm --from alxandria_test --fees 5000udaric --gas 2000000 -y

Code ID: 13

https://morpheus.desmos.network/transactions/E6F6C784402A0C1D4A879140DB5AEACEF28E9B09FBA010414980FE3613084B43

# successful instantiation

successful command: desmos tx wasm instantiate 13 '{"admin":"desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg"}' --label alxandria-test --admin desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg --from alxandria_test --fees 5000udaric --gas 2000000 -y

https://morpheus.desmos.network/transactions/460CC8FE0C27BAEA7323816608ACECA680DBD8903D509BA98B04073BCC1388CC

# query contract’s existence

*get code ID from json output from instantiation

desmos query wasm list-contract-by-code 13

contract address: desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s

# execute message

successful command: desmos tx wasm execute desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s '{"create_post": {"post_id": 1,"external_id": "https://www.google.com","text": "This is my first post","tags": ["google", "post", "attempt"]}}' --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/E86E97DC701B341E4BAAF005BEAAD5FB7814C88379780BF647FE21A46923E7B1

# query new post

successful command: desmos query wasm contract-state smart desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s '{"post":{"post_id": 1}}'

output: post":{"post_id": 1}}' data: post: author: desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg creation_date: "1664212492.539425833" deleter: null editor: null external_id: https://www.google.com last_edit_date: null post_id: 1 tags: - google - post - attempt text: This is my first post

# Query All Posts

successful command:
desmos query wasm contract-state smart desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s '{"all_posts":{}}'

# Migrate

successful command:
desmos tx wasm migrate desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s 15 '{"migrate_msg":{}}' --from alxandria_test --fees 5000udaric --gas 2000000 -y

https://morpheus.desmos.network/transactions/D964BADF078AD9669823356ADD1895E96DDCA3B1D7B9BE21E52BC9E93EF1412C

# Fee Gating Actions

* This works properly, but the error message was incorrectly set. Fixed in codebase.

Store:
https://morpheus.desmos.network/transactions/2D7B3E766B753F4DC705CF78F40764861923406E29C9B920891D6628E4E81C6D

Code ID: 
17

Migrate:
https://morpheus.desmos.network/transactions/E7422797FB4154F54EC176669DB18F6A80E0E1F8CE570E2EE7A15DF39E51DF81

Intentional Fail Post (Fee Gate):

Command:
desmos tx wasm execute desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s '{"create_post": {"post_id": 2,"external_id": "https://www.github.com","text": "This post should fail","tags": ["failure", "fees", "attempt"]}}' --from alxandria_test --fees 5000udaric --gas 200000 -y

https://morpheus.desmos.network/transactions/21919959367B871442F892E5BF3C5DD9BBC9A2BAD3CC594B86DA864A587F0C93

# Proper Error Message Test

Store:
https://morpheus.desmos.network/transactions/F2F51964614FF626321A3366D94D268A2A105002251EE2EF13A28076D60ED80B

Code ID:
18

Migrate:
https://morpheus.desmos.network/transactions/97586398FEF9BB030B21FC7C1EAA2F1BEBEC23140211ACB81B49A0BDE6550790

Tx Fail With Proper Error Msg (Fee Gate):
https://morpheus.desmos.network/transactions/6EC1538B7002DA6C419C523247DB379EEB895EA97818A3E8EB0D6E4A9B0CAAB8

# BankMsg Test!

Store:
https://morpheus.desmos.network/transactions/7B55C8B811202AD77130931C31D591F23CB2F2EAB43D4AD8AEF2D51BB8438935

Code ID:
19

Migrate:
https://morpheus.desmos.network/transactions/707011D99D59F928570736B8E1A404616521F9EF3468A1DCFCD59E146E1E995E

Tx With Funds:
desmos tx wasm execute desmos1l6mexg4802n9703vryf37zgca0d777y4y6cymvjpq5y6w74ugy8qguqz8s '{"create_post": {"post_id": 3,"external_id": "https://www.github.com","text": "This post should pass!","tags": ["pass", "funds", "attempt"]}}' --amount 100000000udaric --from alxandria_test --fees 5000udaric --gas 200000 -y

Success!!!!
https://morpheus.desmos.network/transactions/40584667D6C1C6282069A20670AAA2E21028131B62DFC83175D18E61C893403D

All Coins Forwarded to desmos1a7vpcddckf9qwezmva8gpzphmzhhdr5y2pllvr:
https://morpheus.desmos.network/accounts/desmos1a7vpcddckf9qwezmva8gpzphmzhhdr5y2pllvr

# NEW CONTRACT (UPDATED)

Instantiate:
desmos tx wasm instantiate 35 '{"admin":"desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg"}' --label alxandria-test --admin desmos1etw2v4std305a6tsyvawdrgancv00j65yn2hgg --from alxandria_test --fees 5000udaric --gas 2000000 -y

Success:
https://morpheus.desmos.network/transactions/11A55718DD8F3794176CA35FC52F57C85C6A9E710ADB1D677A49CB70AAD9F3CD

Contract Address: desmos1lmt2ed3tpxxghz968wyhdluczv2ljt90nctxy6ctyavm0wxnzahqgyaxqy

# Create Post:

Command:
desmos tx wasm execute desmos1lmt2ed3tpxxghz968wyhdluczv2ljt90nctxy6ctyavm0wxnzahqgyaxqy '{"create_post": {"post_id": 1,"post_title":"Github","external_id": "https://www.github.com","text": "This post should pass!","tags": ["pass", "funds", "attempt"]}}' --amount 100000000udaric --from alxandria_test --fees 5000udaric --gas 200000 -y

Success:
https://morpheus.desmos.network/transactions/3133D0AC4B6E92FF6150D63FBA89470683ABC9C3C7C5314CD6DB8EEC9AE956AE

# Query All Posts:

desmos query wasm contract-state smart desmos1lmt2ed3tpxxghz968wyhdluczv2ljt90nctxy6ctyavm0wxnzahqgyaxqy '{"all_posts":{}}'