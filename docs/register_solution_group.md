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

  // Registering of solution group reserves part of the free balance. The amount of the reserved funds can be queried as `registrarDeposit()`
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .solutionGroupRegistration(
        namespace,
        solution_group_info,
        solution_group_operators_config,
        solution_group_reward_config,
        operation_start_block,
        operation_end_block,
        withdrawal_delay
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
