# NEAR Rust Riddles

This smart contract allows users to post riddles with bounties and hints. It also allows users to pay to get hints and pay to try and solve those riddles. If they solve a riddle, they get rewarded the initial bounty and the NEAR amount that was spent on each hint retrieved for that riddle as well as the NEAR amount that was spent on each unsuccessful attempt to solve the riddle.

**Note**: this README is specific to Windows and this example. For development on OS X or Linux, please see [README.md](README.md).

## Prerequisites

Ensure `near-cli` is installed by running:

```shell
near --version
```

If needed, install `near-cli`:

```shell
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```shell
rustc --version
```

If needed, install `Rust`:

```shell
curl https://sh.rustup.rs -sSf | sh
```

Install dependencies

```shell
npm install
```

## Building this contract

To make the build process compatible with multiple operating systems, the build process exists as a script in `package.json`.
There are a number of special flags used to compile the smart contract into the wasm file.
Run this command to build and place the wasm file in the `res` directory:

```shell
npm run build
```

**Note**: Instead of `npm`, users of [yarn](https://yarnpkg.com) may run:

```shell
yarn build
```

### Important

If you encounter an error similar to:

> note: the `wasm32-unknown-unknown` target may not be installed

Then run:

```shell
rustup target add wasm32-unknown-unknown
```

## Using this contract

### Quickest deploy

Build and deploy this smart contract to an development account. This development account will be created automatically and is not intended to be permanent. Please see the "Standard deploy" section for creating a more personalized account to deploy to.

```shell
near dev-deploy --wasmFile res/near_rust_riddles.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

> Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing the key to the account, located at `neardev/dev-account.env`. To make the next few steps easier, we're going to set an environment variable containing this development account id and use that when copy/pasting commands.

If the account name is not immediately visible on the Command Prompt, you may find it by running:

```shell
type neardev\dev-account.env
```

It will display something similar to `CONTRACT_NAME=dev-12345678901234`.
Please set the Windows environment variable by copying that value and running `set` like so:

```shell
set CONTRACT_NAME=dev-12345678901234
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:

```shell
echo %CONTRACT_NAME%
```

The next command will call the contract's `create_riddle` method:

```shell
near call $CONTRACT_NAME create_riddle '{"title": "riddle1", "text": "riddle1", "hint": "riddle1", "answer": "riddle1"}' --accountId $CONTRACT_NAME --attachedDeposit 100
```

To retrieve the riddle from the contract, call `get_riddle` with the following:

```shell
near view $CONTRACT_NAME get_riddle '{"title": "riddle1"}'
```

### Standard deploy

In this option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:
In this option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:

```shell
near login
```

Deploy the contract:

```shell
near deploy --wasmFile res/near_rust_riddles.wasm --accountId YOUR_ACCOUNT_NAME
```

Create a riddle:

```shell
near call YOUR_ACCOUNT_NAME create_riddle '{"title": "riddle1", "text": "riddle1", "hint": "riddle1", "answer": "riddle1"}' --accountId YOUR_ACCOUNT_NAME --attachedDeposit 100
```

Get the riddle:

```shell
near view YOUR_ACCOUNT_NAME get_riddle '{"title": "riddle1"}'
```

Note that these riddles are stored in a `HashMap` based on their title. See `src/lib.rs` for the code. We can try the same steps with another riddle to verify.

We can create another riddle:

```shell
near call YOUR_ACCOUNT_NAME create_riddle '{"title": "riddle2", "text": "riddle2", "hint": "riddle2", "answer": "riddle2"}' --accountId YOUR_ACCOUNT_NAME --attachedDeposit 100
```

```shell
near view YOUR_ACCOUNT_NAME get_riddles '{"from_index": 0, "limit": 2, "solved": false}'
```

Returns both the riddles.

We can ask for a hint to a riddle:

```shell
near view YOUR_ACCOUNT_NAME get_riddle_hint '{"title": "riddle1"}' --attachedDeposit 0.1
```

We can try to solve the riddle:

```shell
near call YOUR_ACCOUNT_NAME solve_riddle '{"title": "riddle1", "answer": "riddle1"}' --accountId YOUR_ACCOUNT_NAME --attachedDeposit 0.1
```

Now when we call `get_riddles` we should only see the `riddle2` riddle.

```shell
near view YOUR_ACCOUNT_NAME get_riddles '{"from_index": 0, "limit": 2, "solved": false}'
```

## Testing

To run unit tests on the contract, run the following command:

```shell
cargo test --package near-rust-riddles -- --nocapture
```

To run integration tests on the contract, run the following command:

```shell
./test.sh
```
