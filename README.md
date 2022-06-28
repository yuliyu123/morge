
# Description
A batch of sol contracts deployment tool implemented by rust, support Ethereum and Celo and other evm-compatible chains.

# Build & Test
build: `cargo build`
test: `cargo test -- --test-threads=1`

# Usage
morge init
morge set --rpc-url $RPC_URL --private-key $PRI_KEY
morge add -c examples/contract.sol:SimpleStorage --args "value 111"
morge list
morge remove -c examples/contract.sol:SimpleStorage
morge deploy
morge clean
morge verify -c rinkeby -t 0xc6e08d3b5b1077f4662907fa547fab34bac033a0501655aca0b903057c118da8

# TODO
Memory optimization
Log optimization
Multi-threads async deploy contracts
Improving tests coverage
Calling deployed contract method
