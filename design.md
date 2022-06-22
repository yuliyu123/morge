forge create --rpc-url <rpc_url> --private-key <xxx> src/MyToken.sol:MyToken --constructor-args "ForgeUSD" "FUSD" 18 1000000000000000000000 

forge create --rpc-url <rpc_url> --private-key <xxx> src/MyToken.sol:MyToken

cast call 0x7A58C79141Cc1DC833b024E7f539B417be5e6962 "totalSupply()(uint256)" --rpc-url https://rinkeby.infura.io/v3/c8c81708601f4c6ca0ad9b0c7bb1911f

<!-- https://book.getfoundry.sh/cast/index.html#how-to-use-cast
https://book.getfoundry.sh/forge/deploying.html -->


morge init: include config.json.
morge set --rpc-url <rpc_url>, --private-key <pri_key>: specify public blockchain rpc url and private key, generate .morge dir, include config.json.
morge add <x.sol> --args arg1 arg2 arg3: add deploy file to config.json and generate the corresponding json file, such as 1.json, 2.json. If no constructor has no args, don't add --args subcommand. 
morge deploy: deploy the sms to the corresponding mainnet, default generate deploy.log
morge verify <addr>: verify if addr is deployed or not.
morge clean: clean all artifacts file, deployed files and *.json.
morge list: list the added files
<!-- morge update net=<new net>: update to new blockchain. -->


parse commandline -> add to compile tasks -> compile -> deploy -> return results.

<!-- passed -->
init: cargo run -- init
set: cargo run -- set --rpc-url https://rinkeby.infura.io/v3/c8c81708601f4c6ca0ad9b0c7bb1911f --private-key 1b21c77b2d99d0ddf1edccc6575c79fa3c9466f8a735fbd16833f530da52f0bc
list: cargo run -- list
add: cargo run -- add -c src/examples/contract.sol.sol:SimpleStorage --args "value 111"
remove: cargo run -- remove -c x.sol::x
