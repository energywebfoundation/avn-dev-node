```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";

async function main(): Promise<void> {
	// set the adequate RPC URL to the parachain.
	// for a connection to dev parachain: const RPC_URL = "wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws";

	const RPC_URL = "ws://localhost:9947";
	const wsProvider = new WsProvider(RPC_URL);
	const api = await ApiPromise.create({ provider: wsProvider });

	const keyring = new Keyring({ type: "sr25519" });
	const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
		name: "Alice registrar keyring",
	});

	const groupNamespace = "someNamespace";
	const cidToAllow = "solution_work_logic_cid";

	/** @dev: some considerations on allowing a new CID:
   - This extrinsic can only be called by the registrar who owns the solution group.
   - The CID should be a valid work logic identifier (not empty).
   - This action allows the work logic to be used within the specified solution group.
   **/

	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.allowNewCid(groupNamespace, cidToAllow)
			.signAndSend(REGISTRAR_KEYRING, ({ events }) => {
				if (
					events.some(
						({ event: { method, section } }) =>
							"ExtrinsicSuccess" === method && section == "system"
					)
				) {
					console.log("CID allowed successfully in solution group");
					unsub();
					resolve();
				}
				if (
					events.some(
						({ event: { method, section } }) =>
							"ExtrinsicFailed" === method && section == "system"
					)
				) {
					console.error("Failed to allow CID in solution group");
					events.forEach(({ phase, event: { data, method, section } }) => {
						console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
					});
					unsub();
					resolve();
				}
			});
	});

	await api.disconnect();
}

main();
```
