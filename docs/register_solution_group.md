The [solutionGroupRegistration](https://github.com/energywebfoundation/ewx-worker-solution-pallet/blob/74eb607617170eb7731334866f271cfeb83896f4/src/lib.rs#L1084) extrinsincs registers a solution group.

Performing this actions creates a new group under which EWX solutions can be onboraded


```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

const ONE_AVT = BigInt("1000000000000000000")

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947"); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });

  const namespace = "solution group namespace";
  const solution_group_info = {
    name: "solution group name",
    description: "solution group description",
    publisherInfo: "solution group publisher info",
  }
  const solution_group_operators_config = {
	// operation_start_block < start_block < operation_end_block
    start_block: 100,
    max_operator_workers: 10,
    allowed_operators: 5,
    // amounts are in units which are 10**(-18) part of 1 AVT. min < max
    staking_amounts: { min: BigInt(1) * ONE_AVT, max: BigInt(3) * ONE_AVT },
	has_operators_allowlist: false
  }
  const solution_group_reward_config = {
    subscription_reward_per_block: ONE_AVT/BigInt(100),
    voting_reward_per_block: 0,
    top_performance_bonus: 0,
  }
  const operation_start_block = 10
  const operation_end_block = 1000
  const withdrawal_delay = 5
  const has_cid_allowance = false;
  ```
Note: You can refer to the [allow_new_cid](allow_new_cid.md) documentation to learn more about the `CID allowance` usage.
```ts

    /**
     * Registers a solution group
     *
     * @param {string} namespace - The namespace of the solution group.
     * @param {object} solution_group_info - The information about the solution group.
     * @param {object} solution_group_operators_config - The configuration for solution group operators.
     * @param {object} solution_group_reward_config - The reward configuration for the solution group.
     * @param {number} operation_start_block - The starting block number for the operation.
     * @param {number} operation_end_block - The ending block number for the operation.
     * @param {number} withdrawal_delay - The number of blocks to wait before withdrawal request is executed
     * @param {boolean} has_cid_allowance - Indicates if the solution group has CID allowance.
     *
     * @returns {Promise<void>} A promise that resolves when the registration is complete.
     *
     *  @dev:
     * - Registering of solution group reserves part of the free balance.
     * - The amount of the reserved funds can be queried as `registrarDeposit()`
     */
    await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .solutionGroupRegistration(
        namespace,
        solution_group_info,
        solution_group_operators_config,
        solution_group_reward_config,
        operation_start_block,
        operation_end_block,
        withdrawal_delay,
        has_cid_allowance,
      )
      .signAndSend(REGISTRAR_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Solution group registered');
          unsub();
          resolve();
        if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section == "system")) {
          console.error('Failed to register solution group');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
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
}

main();
```
