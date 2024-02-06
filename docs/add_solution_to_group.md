```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

  // Before adding a solution to a group
  // Register solution group and solution: refer to the associated docs

	// Then Add solution to the solution group

	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.addSolutionToGroup(groupNamespace, solutionNamespace)
			.signAndSend(REGISTRAR_KEYRING, ({ events }) => {
				if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
					console.log('Solution added to group');
					unsub();
					resolve();
				if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section == "system")) {
					console.error('Failed to add solution to group');
					events.forEach(({ phase, event: { data, method, section } }) => {
						console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
					});
					unsub();
					resolve();
				}
			});
		});
	});


	const groupOfSolution = await api.query.workerNodePallet.groupOfSolution(
    // v0.6.1
    // blake2AsHex(solutionNamespace);
		solutionNamespace
	);

	console.log(`Group of the Solution ${groupOfSolution}`);

  // Get solutions of given group
  let allSolutions = await api.query.workerNodePallet.solutions.entries();
  let solutionsOfGroup = (await Promise.all(allSolutions.map(async ([namespace, s]) => {
    // v0.6.1
    // const group = await api.query.workerNodePallet.groupOfSolution(blake2AsHex(solutionNamespace));
    const group = await api.query.workerNodePallet.groupOfSolution(namespace.toHuman()?.toString());
    return { solution: s, group };
  }))).filter((s) => s.group.toHuman() == groupNamespace);
  console.log("solutions of group", solutionsOfGroup.map((s) => s.solution.toHuman()));

	await api.disconnect();
}

main();

```
