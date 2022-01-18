# NEAR Rust Riddles

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/near-examples/rust-status-message)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

This smart contract allows users to post riddles with bounties and hints. It also allows users to pay to get hints and pay to try and solve those riddles. If they solve a riddle, they get rewarded the initial bounty and the NEAR amount that was spent on each hint retrieved for that riddle as well as the NEAR amount that was spent on each unsuccessful attempt to solve the riddle.

**Note**: Windows users: please visit the [Windows-specific README file](README-Windows.md).

## Prerequisites

Ensure `near-cli` is installed by running:

```bash
near --version
```

If needed, install `near-cli`:

```bash
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```bash
rustc --version
```

If needed, install `Rust`:

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Quick Start

To run this project locally:

1. Prerequisites: Make sure you have Node.js ≥ 16 installed (<https://nodejs.org>)

## Building this contract

To make the build process compatible with multiple operating systems, the build process exists as a script in `package.json`.
There are a number of special flags used to compile the smart contract into the wasm file.
Run this command to build and place the wasm file in the `res` directory:

```bash
npm run build
```

**Note**: Instead of `npm`, users of [yarn](https://yarnpkg.com) may run:

```bash
yarn build
```

### Important

If you encounter an error similar to:

> note: the `wasm32-unknown-unknown` target may not be installed

Then run:

```bash
rustup target add wasm32-unknown-unknown
```

## Using this contract

### Quickest deploy

Build and deploy this smart contract to a development account. This development account will be created automatically and is not intended to be permanent. Please see the "Standard deploy" section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile res/near_rust_riddles.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

> Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing the key to the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an environment variable containing this development account id and use that when copy/pasting commands.
Run this command to the environment variable:

```bash
source neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:

```bash
echo $CONTRACT_NAME
```

The next command will call the contract's `create_riddle` method:

```bash
near call $CONTRACT_NAME create_riddle '{"title": "riddle1", "text": "riddle1", "hint": "riddle1", "answer": "riddle1"}' --accountId $CONTRACT_NAME --attachedDeposit 100
```

To retrieve the riddle from the contract, call `get_riddle` with the following:

```bash
near view $CONTRACT_NAME get_riddle '{"title": "riddle1"}'
```

### Standard deploy

In this option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:

```bash
near login
```

Deploy the contract:

```bash
near deploy --wasmFile res/near_rust_riddles.wasm --accountId YOUR_ACCOUNT_NAME
```

Create a riddle:

```bash
near call YOUR_ACCOUNT_NAME create_riddle '{"title": "riddle1", "text": "riddle1", "hint": "riddle1", "answer": "riddle1"}' --accountId YOUR_ACCOUNT_NAME --attachedDeposit 100
```

Get the riddle:

```bash
near view YOUR_ACCOUNT_NAME get_riddle '{"title": "riddle1"}'
```

Note that these riddles are stored in a `HashMap` based on their title. See `src/lib.rs` for the code. We can try the same steps with another riddle to verify.

We can create another riddle:

```bash
near call YOUR_ACCOUNT_NAME create_riddle '{"title": "riddle2", "text": "riddle2", "hint": "riddle2", "answer": "riddle2"}' --accountId YOUR_ACCOUNT_NAME --attachedDeposit 100
```

```bash
near view YOUR_ACCOUNT_NAME get_riddles '{"from_index": 0, "limit": 2, "solved": false}'
```

Returns both the riddles.

We can ask for a hint to a riddle:

```bash
near view YOUR_ACCOUNT_NAME get_riddle_hint '{"title": "riddle1"}' --attachedDeposit 0.1
```

We can try to solve the riddle:

```bash
near call YOUR_ACCOUNT_NAME solve_riddle '{"title": "riddle1", "answer": "riddle1"}' --accountId YOUR_ACCOUNT_NAME --attachedDeposit 0.1
```

Now when we call `get_riddles` we should only see the `riddle2` riddle.

```bash
near view YOUR_ACCOUNT_NAME get_riddles '{"from_index": 0, "limit": 2, "solved": false}'
```

## Testing

To run unit tests on the contract, run the following command:

```bash
cargo test --package near-rust-riddles -- --nocapture
```

To run integration tests on the contract, run the following command:

```bash
./test.sh
```
