
# title
morge -- A batch of solidity contracts deployment tool

# Description
A batch of solidity contracts deployment tool developed by rust, support Ethereum and Celo and other evm-compatible chains. Project url: https://github.com/yuliyu123/morge.

# Requirements
Running this project
and tests require the following installed:

`solc` (>=0.8.10). We also recommend using solc-select for more flexibility.

`anvil`

`geth`

# Installation
Run the following command directly to install from crates.io.

`cargo install morge`

## Installing from Source

`cargo install --git https://github.com/yuliyu123/morge`

# Usage
Initialize morge to create config file under .morge directory: `morge init`

Set rpc url and private key: 

`morge set --rpc-url $RPC_URL --private-key $PRI_KEY`

Add any numbers of contracts that you want to deploy:

`morge add -c examples/contract.sol:SimpleStorage --args "value"`

Remove any contract that you want to delete:

`morge remove -c examples/contract.sol:SimpleStorage`

List configuration:

`morge list`

Start deploy:

`morge deploy`

Clear configuration:

`morge clean`

Verify transaction execution status by specify chainnet and transaction hash:

`morge verify -c rinkeby -t 0xc6e08d3b5b1077f4662907fa547fab34bac033a0501655aca0b903057c118da8`

## Build & Test
build: `cargo build`

test: `cargo test -- --test-threads=1`

support chainnet till now:

`ethernum、goerli、celo、kovan、rinkeby、ropsten、polygon、polygon-mumbai、fantom、fantom-testnet、bsc、bsc-testnet`

All kinds of issues, features and PR is welcome to contribute, thanks.
