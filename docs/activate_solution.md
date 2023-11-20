```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

// Solution has status, which is used by registrar to enable voting round in this solution. Status
// has three variants:
// * Paused = 0. Solution not yet ready for voting
// * Active = 1. New voting can be started
// * Ended = 2. No new voting can be started
// New solution has pending status. To change status `set_activation_status` extrinsic should be used.

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947");
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });
  const REGISTRAR_ADDRESS = REGISTRAR_KEYRING.address;

  const solutionNamespace = "solution namespace";
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .setActivationStatus(
        solutionNamespace,
        1
      )
      .signAndSend(REGISTRAR_KEYRING, ({ status }) => {
        if (status.isFinalized) {
          unsub();
          resolve();
        }
      });
  });

  await api.disconnect()
}

main();
```
