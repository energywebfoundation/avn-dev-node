```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

const ONE_AVT = BigInt("1000000000000000000")

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947"); wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const OPERATOR_KEYRING = keyring.addFromUri("//Bob", {
    name: "Operator Bob",
  });

  const solution_group_namespace = "solution group namespace";
  // This amount will be reserved from free balance. The amount is in units. One unit = 1 AVT/(10**18)
  const stake = BigInt(2) * ONE_AVT // should be in solution_group_operators_config.staking_amounts range

  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .subscribeToSolutionGroup(
        solution_group_namespace,
        stake,
      )
      .signAndSend(OPERATOR_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Operator subscribed to solution group');
          unsub();
          resolve();
        } else if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section === "system")) {
          console.error('Failed to subscribe to solution group');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
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
