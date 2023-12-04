```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

  // Before adding a solution to a group
  // Register solution group and solution: refer to the associated docs

	// Then Add solution to the solution group

	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.addSolutionToGroup(groupNamespace, solutionNamespace)
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

  // Get solutions of given group
  let allSolutions = await api.query.workerNodePallet.solutions.entries();
  let solutionsOfGroup = (await Promise.all(allSolutions.map(async ([namespace, s]) => {
    // v0.5.0
    // const group = await api.query.workerNodePallet.groupOfSolution(blake2AsHex(solutionNamespace));
    const group = await api.query.workerNodePallet.groupOfSolution(namespace.toHuman()?.toString());
    return { solution: s, group };
  }))).filter((s) => s.group.toHuman() == groupNamespace);
  console.log("solutions of group", solutionsOfGroup.map((s) => s.solution.toHuman()));

	await api.disconnect();
}

main();

```
