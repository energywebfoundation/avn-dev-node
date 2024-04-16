```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

const ONE_AVT = BigInt("1000000000000000000")

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947");
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });

  const solution_group_namespace = "solution group namespace";

  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .allowOperator(
        solution_group_namespace,
		operator_address
      )
      .signAndSend(OPERATOR_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Operator allowed to subscribe');
          unsub();
          resolve();
        } else if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section === "system")) {
          console.error('Failed to add operator to group allowlist');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
          unsub();
          resolve();
        }
      });
  });

  await api.disconnect()
}

main();
```
