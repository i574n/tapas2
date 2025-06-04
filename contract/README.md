# tapas2_contract

## testnet

```ps1
near login

near deploy i574n.testnet "dist/tapas2.wasm" --networkId testnet
near call i574n.testnet new --networkId testnet --accountId i574n.testnet

near call i574n.testnet act --networkId testnet --accountId i574n.testnet --gas 110000000000000 "{\`"key\`": \`"\`", \`"proof\`": \`"\`", \`"data\`": 1}"
```

```txt
testnet
mynearwallet.com
i574n.testnet

height message cook awesome coast glide crush flush pave brick size delay
```

## mainnet

```ps1
near login

near deploy luckier.near "dist/tapas2.wasm" --networkId=mainnet --accountId=i574n.near
near call luckier.near new --networkId mainnet --accountId=i574n.near

near call luckier.near act --accountId=i574n.near --networkId=mainnet --gas 110000000000000 "{\`"key\`": \`"\`", \`"proof\`": \`"\`", \`"data\`": 1}"
```
