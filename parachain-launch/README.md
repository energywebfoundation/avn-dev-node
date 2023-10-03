# Local Setup Guide

This guide helps developers to setup a local network that uses the avn-dev-node node for development purposes.

## Prerequisites

This setup uses [parachain-launch](https://github.com/open-web3-stack/parachain-launch) to create the local environment. Follow the project instructions to build and allow global or local usage of the script. The commands in this guide assume that the `parachain-launch` script is part of your system's path. If this is not the case, adjust the commands accordingly to point to the script's path.

## Generation

Follow the project instructions in the [README](../README.md/#building-the-client) to build a release version of the node and a Docker image with the tag aventus/`avn-dev-node:latest`.

Then, run the following command:
```sh
parachain-launch generate --output parachain-launch/output parachain-launch/config.yml -y

Note: If this command fails with message

```
Error parsing spec file: unknown field `session_length_in_blocks`
```
, then try to comment

```
  overrides: # additonal runtime override
    session_length_in_blocks: 10

```
in the `parachain-launch/config.yml`
```
This will generate Docker files in a folder called `output` within your current working directory or in the directory provided to the `--output` option.

**NOTE**: The generation step must be repeated every time a new docker image is generated.

## Start relaychain and parachain

To start the nodes, navigate to the output folder that you generated the scripts in and build the docker container:

```sh
cd ./parachain-launch/output

docker compose up -d --build
```

**NOTE**: If you regenerate the output directory then you will need to rebuild the docker images.

The parachain is automatically onboarded to the relay chain and you can interact with both chains using the following URLs:
- [ws://localhost:9944](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2Flocalhost%3A9944#/parachains) to interact with the relay chain
- [ws://localhost:9947](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2Flocalhost%3A9944#/accounts) to interact with the parachain


## Additional Docker Commands

List all of the containers:

```sh
docker ps -a
docker compose ps
```

Track container logs:

```sh
docker logs -f [container_id/container_name]
docker compose logs -f [service_name]
```

Stop all of the containers:

```sh
docker compose stop
```

Remove all of the containers:

```sh
docker compose rm
```

Remove all of the containers and volumes (This will wipe any existing chain data):

```sh
docker compose down -v
```
