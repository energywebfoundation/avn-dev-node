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
# set default cargo location
source $HOME/.cargo/env
```

If you already have Rust installed, make sure you're using the latest version by running:

```bash
rustup update
```

Once done, finish installing the support software and configure your default toolchain:

```bash
# Install nightly toolchain and use it
rustup toolchain install nightly-2022-10-18
rustup default nightly-2022-10-18
rustup target add --toolchain nightly-2022-10-18 wasm32-unknown-unknown

# Additional OS dependencies
sudo apt install build-essential
sudo apt install --assume-yes git clang curl libssl-dev protobuf-compiler
```

Verify the configuration of your development environment by running the following command:
```bash
rustup show
rustup +nightly show
```
The command displays output similar to the following:
```bash
# rustup show

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.62.1 (e092d0b6b 2022-07-16)

# rustup +nightly show

active toolchain
----------------

nightly-x86_64-unknown-linux-gnu (overridden by +toolchain on the command line)
rustc 1.65.0-nightly (34a6cae28 2022-08-09)
```
See [here](https://docs.substrate.io/install/linux/) for a more detailed guide on installing Rust and the required dependecies.

Build the client by cloning this repository and running the following commands from the root
directory of the repo:

```bash
git checkout <latest tagged release>
cargo build --release
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
docker build . --tag aventus/avn-dev-node:latest
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

## Using Worker pallet in polkadot.js UI

After you launched relay chain and parachain as it is described in parachain-launch/README.md you can interact with pallet on `https://polkadot.js.org/`. By default polkadot.js is connected to Polkadot mainnet, so you need to switch to parachain. For this click on the chain drop-down list in up right conner and select `DEVELOPMENT` sublist. If this list is missing `ws:127.0.0.1:9947` node, then you need to add it in `custom endpoint` field.
Extrinsics can be send from `Developer/extrinsic` menu. Extrinsics of Worker pallet are available in `workerNodePallet`. Before sending extrinsic choose one of the development accounts.

### Signing up solution registrar

- choose `signupSolutionRegistrar` extrinsic
- specify extrinsic parameters:
  - `friendly name`: `registrar name`,
  - `legalLocation`: `registrar location`
- click `Submit Transaction` and in next pop-up click `Sign and Submit`
- go to Network-Explorer and make sure that there is `workerNodePallet.NewSolutionRegistrarSignup` in the list of recent events

### Registering a solution

- select same account which was used to signup solution registrar
- choose `registerSolution` extrinsic
- specify extrinsic parameters:
  - `namespace`: `solution namespace`
  - `name`: `solution name`
  - `description`: `solution description`
  - `publisherInfo`: `solution publisher info`
  - `logoUrl`: `solution logo url` // it is optional
  - `workLogic`: `work logic`
  - `executionEnvironment`: `10`
  - `expirationBlock`: `10000`
  - `maxWaitingThreshhold`: `60`
  - `voteThresholdPercent`: `60`
- click `Submit Transaction` and in next pop-up click `Sign and Submit`
- go to Network-Explorer and make sure that there is
`workerNodePallet.SolutionCreated` in the list of recent events

## Using Worker pallet from JS/TS

### Installation

The polkadot.js library can be installed using the instructions here: https://polkadot.js.org/docs/api/start/install/ .

Use of Typescript may require additional libraries such as `@polkadot/typegen`. See https://polkadot.js.org/docs/api/start/typescript.user.

### State Queries

Pallet state queries can be done via the `.query` API, as shown [here](https://polkadot.js.org/docs/api/start/api.query)

## Example scenarios

### Solution Registrar Flow

Use the follow script as an example to register solution.
`ALICE` is one of the predefined development accounts. This script first
signups ALICE as solution registrar, which allows next to register solution.

```javascript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947");
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const ALICE_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice default",
  });
  const ALICE_ADDRESS = ALICE_KEYRING.address; // "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  const registrar_name = "Alice";
  const registrar_legal_location = "Alice place";
  await new Promise<void>(async (resolve) => {
    let unsubscribe= await api.tx.workerNodePallet
      .signupSolutionRegistrar(registrar_name, registrar_legal_location)
      .signAndSend(ALICE_KEYRING, ({ status }) => {
        if (status.isFinalized) {
          unsubscribe();
          resolve();
        }
      });
  });
  const aliceRegistrarInfo =
    await api.query.workerNodePallet.registrarInventory(ALICE_ADDRESS);
  console.log(aliceRegistrarInfo.toHuman());

  const namespace = "solution namespace";
  const name = "solution name";
  const description = "solution description";
  const publisherInfo = "solution publisher info";
  const logoUrl = "solution logo url";
  const workLogicCid = "solution work logic cid";
  const executionEnvironment = 10; // NodeRedV1
  const expirationBlock = 100000;
  const maxWaitingThreshold = 60;
  const voteThresholdPercent = 60;

  await new Promise<void>(async (resolve) => {
    let unsubscribe = await api.tx.workerNodePallet
      .registerSolution(
        namespace,
        name,
        description,
        publisherInfo,
        logoUrl,
        workLogicCid,
        executionEnvironment,
        expirationBlock,
        maxWaitingThreshold,
        voteThresholdPercent,
      )
      .signAndSend(ALICE_KEYRING, ({ status }) => {
        if (status.isFinalized) {
          unsubscribe();
          resolve();
        }
      });
  });

  const solution = await api.query.workerNodePallet.solutions(
    blake2AsHex(namespace),
  );
  console.log(solution.toHuman());
}

main();
```
Other examples are in docs/
