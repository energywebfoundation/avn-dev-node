```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main() {
  const wsProvider = new WsProvider("ws://localhost:9947"); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const OPERATOR_KEYRING = keyring.addFromUri("//Bob", {
    name: "Operator Bob",
  });
  const OPERATOR_ADDRESS = OPERATOR_KEYRING.address;

	...
  // The operator should be registered before being allowed to add a new worker
  // To register a new worker node operator: refer to the associated documentation (/docs/signup_operator.md)
	// The previous step gives the OPERATOR_KEYRING value, required to connect the new worker node

  const WORKER_KEYRING = keyring.addFromUri("//Charlie", {
    name: "Worker node Charlie",
  });

	const WORKER_NODE_ADDRESS = WORKER_KEYRING.address;

	// The OPERATOR adds connects the worker node group
	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.connectWorkerNode(WORKER_NODE_ADDRESS)
			.signAndSend(OPERATOR_KEYRING, ({ events }) => {
				if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
					console.log('Worker node connected');
					unsub();
					resolve();
				} else if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section === "system")) {
					console.error('Failed to connect worker node');
					events.forEach(({ phase, event: { data, method, section } }) => {
						console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
					});
					unsub();
					resolve();
				}
			});
	});

	const operatorAddress = await api.query.workerNodePallet.workerNodeToOperator(WORKER_NODE_ADDRESS);

	if (operatorAddress.toString() == OPERATOR_KEYRING.address) {
		console.log(`Worker ${WORKER_NODE_ADDRESS} correctly connected to Operator ${operatorAddress}`)
	} else {
		throw(`Something went wrong`)
	}

	await api.disconnect();
}

main();

```
