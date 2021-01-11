# Fun stuff for Trevor

```bash
cd contract
./build.sh
```

## Try it out on testnet

Replace `vec.mike.testnet` and `mike.testnet` accordingly below:

```bash
near create-account vec.mike.testnet --masterAccount mike.testnet

near deploy vec.mike.testnet --wasmFile res/init_issue_trevor.wasm --initFunction new --initArgs '{}'

near view vec.mike.testnet who ''
```

## Troubleshooting

Delete and recreate the account with:

    near delete vec.mike.testnet mike.testnet && near create-account vec.mike.testnet --masterAccount mike.testnet