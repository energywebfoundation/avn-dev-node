```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947"); wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const OPERATOR_KEYRING = keyring.addFromUri("//Bob", {
    name: "Operator Bob",
  });

  const solution_group_namespace = "solution namespace";
  // This amount will be reserved from free balance. The amount is in units. One unit = 1 AVT/(10^18)
  const stake = 2 * (10^18) // should be in solution_group_operators_config.staking_amounts range


  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .subscribeToSolutionGroup(
        solution_group_namespace,
        stake,
      )
      .signAndSend(OPERATOR_KEYRING, ({ status }) => {
        if (status.isFinalized) {
          unsub();
          resolve();
        }
      });
  });
  const subscriptions = await api.query.workerNodePallet.solutionGroupSubscriptionRegistry.entries();
  if (subscriptions.length === 0) {
    console.log(`No subscriptions yet`)
  }
  subscriptions.forEach(([key, subscription]) => {
    console.log('subscription keys', key.args.map((k) => k.toHuman()))
    console.log('subscription', subscription.toHuman());
  })

  await api.disconnect()
}

main();
```
