# AvN dev node template


⚠️ IMPORTANT NOTE ⚠️ <br />
This template provides a simple example of how to build a parachain node using Avn pallets and MUST NOT BE USED IN PRODUCTION. It is configured with limited functionality and only contains a subset of avn pallets.
---
---
<br />

# AvN Dev Node - an AvN Parachain Template based on Substrate Cumulus
<p align="center">
    <a href="https://github.com/Aventus-Network-Services/avn-dev-node/actions/workflows/ci.yml">
        <img src="https://github.com/Aventus-Network-Services/avn-dev-node/actions/workflows/ci.yml/badge.svg?branch=main" alt=".github/workflows/ci.yml"></a>
</p>

This project template is a modified version of the original
[Substrate parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template). It has been customized to include the minimum necessary dependencies for integrating some of the AvN pallets into the runtime.

For more details on the implementation of the AvN Dev Node runtime, please refer to the [runtime/README.md](runtime/README.md) file.

## Parachains introduction and tutorials
👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and
parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).

🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).

## Building the project
*Based on [Polkadot Build Guide](https://github.com/paritytech/polkadot#building)*

First [Install Rust](https://www.rust-lang.org/tools/install):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you already have Rust installed, make sure you're using the latest version by running:

```bash
rustup update
```

Once done, finish installing the support software:

```bash
# Additional rust targets
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-18
# Additional OS dependencies
sudo apt install build-essential git clang libclang-dev pkg-config libssl-dev
```

### Building the client

Build the client by cloning this repository and running the following commands from the root
directory of the repo:

```bash
cargo build --release
```

### Building the docker image

Prerequisites:
 - [Install Docker Engine](https://docs.docker.com/engine/install/) 24.0.2 or newer
 - [Post-installation steps for Linux | Docker Documentation](https://docs.docker.com/engine/install/linux-postinstall/)

```sh
# Builds the docker image with the build artefacts under target/release
docker build . --tag avn-dev-node:latest
```

## Running the node
Once the build process is complete, you can use the binary to run a node and perform other subcommands.

When running a parachain node, you have the option to configure various settings using the command-line interface (CLI). To view a full list of available options, execute the following command:
```
target/release/avn-dev-node -h
```

The simplest way to run a node is as follows:
```bash
target/release/avn-dev-node --dev -- --chain rococo-local
```
After starting the node, you will notice that no blocks are generated. This is because a parachain needs to be onboarded to a relay chain that it is configured with. Substrate provides useful tutorials on how to set up an environment with a relay chain, which is highly recommended to complete. You can find these tutorials [here](https://docs.substrate.io/tutorials/build-a-parachain/).

To expedite the process of local development for this project, we recommend following the [local setup guide](parachain-launch/README.md) to orchestrate a full local network.


## Updating the pallet-ewx-worker-solution Dependency

If you're using a cargo dependency from GitHub and its reference has been updated, Cargo won't automatically detect the change if there's a cached reference on your local system. To ensure your project uses the updated reference, especially when working with branches during development or relying on the latest main branch, you can use the following command:

```bash
cargo update -p pallet-ewx-worker-solution
```

This command forces your project to update its references to the pallet, ensuring you're using the latest version.
