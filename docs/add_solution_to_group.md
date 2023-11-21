```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

  // Before adding a solution to a group
  // Register solution group and solution: refer to the associated docs

	// Then Add solution to the solution group

	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.addSolutionToGroup(group_namespace, solutionNamespace)
			.signAndSend(REGISTRAR_KEYRING, ({ status }) => {
				if (status.isFinalized) {
					unsub();
					resolve();
				}
			});
	});

	const solutionNamespaceHash = blake2AsHex(solutionNamespace);

	const groupOfSolution = await api.query.workerNodePallet.groupOfSolution(
		solutionNamespaceHash
	);

	console.log(`Group of the Solution ${groupOfSolution}`);

	await api.disconnect();
}

main();

```