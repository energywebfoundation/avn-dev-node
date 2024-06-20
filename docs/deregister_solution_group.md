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

	// Prerequisites:
	// * No solutions in group
	// * All rewards are calculated
	// * Group was not deregistered already
  await new Promise<void>(async (resolve) => {
	let deregisteredGroups = await api.query.workerNodePallet.deregisteredGroupsWithRewards.entries();
	if deregisteredGroupsWithRewards.some(([group, _])=> namespace === namespace) {
		throw new Error('Group was deregistered already')
	};
    let unsub = await api.tx.workerNodePallet
      .solutionGroupDeregistration(
        namespace,
      )
      .signAndSend(REGISTRAR_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Solution group deregistered');
          unsub();
          resolve();
        if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section == "system")) {
          console.error('Failed to deregister solution group');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
          unsub();
          resolve();
        }
      });
  });
  const solutionGroups = await api.query.workerNodePallet.solutionsGroups.entries();
  let isRewardsPaid = solutionGroups.find(([namespace, _]);
  if (isRewardsPaid) {
  	console.log(`All group data was removed`)
  } else {
  	console.warn(`Some group rewards are not paid yet. Group data will be deleted after last
	subscriber claims his rewards`
	);
  }

  await api.disconnect()
}

main();
```
