
# Description
A batch of solidity contracts deployment tool developed by rust, support Ethereum and Celo and other evm-compatible chains.

# Installation




# Usage
Init: Initialize morge to generate config file under .morge directory.

`morge init`

`morge set --rpc-url $RPC_URL --private-key $PRI_KEY`

`morge add -c examples/contract.sol:SimpleStorage --args "value"`

`morge remove -c examples/contract.sol:SimpleStorage`

`morge list`

`morge deploy`

`morge clean`

`morge verify -c rinkeby -t 0xc6e08d3b5b1077f4662907fa547fab34bac033a0501655aca0b903057c118da8`

# Todo
Memory optimization

Log optimization

Multi-threads async deploy contracts

Improving tests coverage

Calling deployed contract method


# Contributing
## Build & Test
build: `cargo build`

test: `cargo test -- --test-threads=1`
