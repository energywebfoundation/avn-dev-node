```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947");
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });

  const solutionNamespace = "ZEV";

  // No prerequisites for removing from group yet. Later on will need to change solution status to PAUSED or ENDED
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .removeSolutionFromGroup(
        solutionNamespace
      )
      .signAndSend(REGISTRAR_KEYRING, ({ status }) => {
        if (status.isFinalized) {
          unsub();
          resolve();
        }
      });
  });

  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .deleteSolution(
        solutionNamespace
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
```
