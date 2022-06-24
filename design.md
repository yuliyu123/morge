forge create --rpc-url <rpc_url> --private-key <xxx> src/MyToken.sol:MyToken --constructor-args "ForgeUSD" "FUSD" 18 1000000000000000000000 

forge create --rpc-url <rpc_url> --private-key <xxx> src/MyToken.sol:MyToken

cast call 0x7A58C79141Cc1DC833b024E7f539B417be5e6962 "totalSupply()(uint256)" --rpc-url https://rinkeby.infura.io/v3/c8c81708601f4c6ca0ad9b0c7bb1911f

<!-- https://book.getfoundry.sh/cast/index.html#how-to-use-cast
https://book.getfoundry.sh/forge/deploying.html -->


1. morge init: include config.json.
2. morge set --rpc-url <rpc_url>, --private-key <pri_key>: specify public blockchain rpc url and private key, generate .morge dir, include config.json.
3. morge add <x.sol> --args arg1 arg2 arg3: add deploy file to config.json and generate the corresponding json file, such as 1.json, 2.json. If no constructor has no args, don't add --args subcommand. 
4. morge deploy: deploy the sms to the corresponding mainnet, default generate deploy.log
5. morge verify <addr>: verify if addr is deployed or not.
6. morge clean: clean all artifacts file, deployed files and *.json.
7. morge list: list the added files
8. morge remove: remove added contract
<!-- morge update net=<new net>: update to new blockchain. -->


parse commandline -> add to compile tasks -> compile -> deploy -> return results.

<!-- passed -->
cargo run -- init
cargo run -- set --rpc-url $RPC_URL --private-key $PRI_KEY
cargo run -- add -c examples/contract.sol:SimpleStorage --args "value 111"
cargo run -- list
cargo run -- remove -c examples/contract.sol:SimpleStorage
cargo run -- deploy
cargo run -- clean

<!-- todo -->
cargo run -- verify -a 0xxxx
