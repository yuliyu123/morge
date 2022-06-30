# 标题
morge -- 一个批量solidity合约执行工具

# 描述
一个批量solidity合约执行工具，rust实现。支持以太坊、celo和其他evm兼容链。项目链接：https://github.com/yuliyu123/morge。

# 依赖条件
运行和测试该项目需要以下依赖：

`solc` (>=0.8.10)

`anvil`

`geth`

# 安装
`cargo install morge`

## 从源代码安装
`cargo install --git https://github.com/yuliyu123/morge`

# 用法
初始化：`morge init`。

设置rpc-url和私钥：
`morge set --rpc-url $RPC_URL --private-key $PRI_KEY`

添加要部署的合约，需要指定合约文件路径、合约名和构造函数：
`morge add -c examples/contract.sol:SimpleStorage --args "value"`

删除合约，指定合约文件路径和合约名：
`morge remove -c examples/contract.sol:SimpleStorage`

列出当前配置：
`morge list`

开始批量部署：
`morge deploy`

清空配置信息：`morge clean`

指定公链名称和交易哈希值验证交易信息：

`morge verify -c rinkeby -t 0xc6e08d3b5b1077f4662907fa547fab34bac033a0501655aca0b903057c118da8`

# 当前支持链
ethernum、goerli、celo、kovan、rinkeby、ropsten、polygon、polygon-mumbai、fantom、fantom-testnet、bsc、bsc-testnet

欢迎提交任何关于这个项目的issue、feature和PR，参与到这个项目的贡献和使用中。