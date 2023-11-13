```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

const ONE_AVT = BigInt("1000000000000000000");
const RPC_URL = "ws://localhost:9947" //wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws";

async function main(): Promise<void> {
	const wsProvider = new WsProvider(RPC_URL); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
	const api = await ApiPromise.create({ provider: wsProvider });

	const keyring = new Keyring({ type: "sr25519" });
	const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
		name: "Alice registrar keyring",
	});

	const group_namespace = "solution group namespace";
	const solution_group_info = {
		name: "solution group name",
		description: "solution group description",
		publisherInfo: "solution group publisher info",
	};
	const solution_group_operators_config = {
		start_block: 2000,
		max_operator_workers: 10,
		allowed_operators: 5,
		// amounts are in units which are 10**(-18) part of 1 AVT
		staking_amounts: { min: BigInt(1) * ONE_AVT, max: BigInt(3) * ONE_AVT },
	};
	const solution_group_reward_config = {
		subscription_reward_amount: ONE_AVT / BigInt(100),
		minimum_participation_time: 100,
		active_participation_amount: 0,
		top_performance_bonus: 0,
	};
	const operation_start_block = 20;
	const operation_end_block = 200;

	// Registering of solution group reserves part of the free balance. The amount of the reserved funds can be queried as `registrarDeposit()`
	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.solutionGroupRegistration(
				group_namespace,
				solution_group_info,
				solution_group_operators_config,
				solution_group_reward_config,
				operation_start_block,
				operation_end_block
			)
			.signAndSend(REGISTRAR_KEYRING, ({ status }) => {
				if (status.isFinalized) {
					unsub();
					resolve();
				}
			});
	});
	const solutionGroups =
		await api.query.workerNodePallet.solutionsGroups.entries();
	if (solutionGroups.length === 0) {
		console.log(`No solution groups yet`);
	}
	solutionGroups.forEach(([namespace_hash, solution_group]) => {
		console.log("namespace hash:", namespace_hash.toHuman());
		console.log("solution group:", solution_group.toHuman());
	});

	// Register Solution
	const solutionNamespace = "solution 1 namespace";
	const solutionName = "solution name";
	const solutionDescription = "solution description";
	const publisherInfo = "publisher info";
	const logoUrl = "solution logo url";
	const workLogicCid = "solution work logic cid";
	const executionEnvironment = 10; // NodeRedV1
	const expirationBlock = 100000;
	const maxWaitingThreshold = 60;
	const voteThresholdPercent = 60;
	await new Promise<void>(async (resolve) => {
		let unsub = await api.tx.workerNodePallet
			.registerSolution(
				solutionNamespace,
				solutionName,
				solutionDescription,
				publisherInfo,
				logoUrl,
				workLogicCid,
				executionEnvironment,
				expirationBlock,
				maxWaitingThreshold,
				voteThresholdPercent
			)
			.signAndSend(REGISTRAR_KEYRING, ({ status }) => {
				if (status.isFinalized) {
					unsub();
					resolve();
				}
			});
	});

	// Add solution to the solution group
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