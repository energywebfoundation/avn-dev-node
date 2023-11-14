```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

  // Before adding a solution to a group
  // Register solution group and solution: refer to the associated docs

	// Then Add solution to the solution group
	const groupNamespaceHash = blake2AsHex(group_namespace);
	const solutionNamespaceHash = blake2AsHex(solutionNamespace);

	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.addSolutionToGroup(groupNamespaceHash, solutionNamespaceHash)
			.signAndSend(REGISTRAR_KEYRING, ({ status }) => {
				if (status.isFinalized) {
					unsub();
					resolve();
				}
			});
	});

	const groupOfSolution = await api.query.workerNodePallet.groupOfSolution(
		solutionNamespaceHash
	);

	console.log(`Group of the Solution ${groupOfSolution}`);

	await api.disconnect();
}

main();

```