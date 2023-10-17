```javascript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947"); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });

  const namespace = "solution namespace";
  const solution_group_info = {
    name: "solution name",
    description: "solution description",
    publisherInfo: "solution publisher info",
  }
  const solution_group_operators_config = {
    start_block: 2000,
    max_operator_workers: 10,
    allowed_operators: 5,
    staking_amounts: { min: 100, max: 500 },
  }
  const solution_group_reward_config = {
    subscription_reward_amount: 1000,
    minimum_participation_time: 100,
    active_participation_amount: 0,
    top_performance_bonus: 0,
  }
  const operation_start_block = 20
  const operation_end_block = 200

  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .solutionGroupRegistration(
        namespace,
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
  const solutionGroups = await api.query.workerNodePallet.solutionsGroups.entries();
  if (solutionGroups.length === 0) {
    console.log(`No solution groups yet`)
  }
  solutionGroups.forEach(([namespace_hash, solution_group]) => {
    console.log('namespace hash:', namespace_hash.toHuman());
    console.log('solution group:', solution_group.toHuman());
  })

  await api.disconnect()

  // TODO: add solution to group
  // const logoUrl = "solution logo url";
  // const workLogicCid = "solution work logic cid";
  // const executionEnvironment = 10; // NodeRedV1
  // const expirationBlock = 100000;
  // const maxWaitingThreshold = 60;
  // const voteThresholdPercent = 60;
  // await new Promise<void>(async (resolve) => {
  //   let unsub = await api.tx.workerNodePallet
  //     .registerSolution(
  //       namespace,
  //       name,
  //       description,
  //       publisherInfo,
  //       logoUrl,
  //       workLogicCid,
  //       executionEnvironment,
  //       expirationBlock,
  //       maxWaitingThreshold,
  //       voteThresholdPercent,
  //     )
  //     .signAndSend(ALICE_KEYRING, ({ status }) => {
  //       if (status.isFinalized) {
  //         unsub();
  //         resolve();
  //       }
  //     });
  // });
  // activeSolution =
  //   await api.query.workerNodePallet.registrarActiveSolutionRegistry(
  //     ALICE_ADDRESS,
  //     namespace,
  //   );
  // console.log(activeSolution.toHuman());
  // const solution = await api.query.workerNodePallet.solutions(
  //   blake2AsHex(namespace),
  // );
  // console.log(solution.toHuman());
}

main();
```
