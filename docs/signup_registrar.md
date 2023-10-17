```javascript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main() {
  const wsProvider = new WsProvider("ws://localhost:9947"); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });
  const REGISTRAR_ADDRESS = REGISTRAR_KEYRING.address; // "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  // Free operator balance should be no less than registrar signup deposit. Signup deposit amount of the free balance will be frozen
  const registrar_name = "Alice registrar";
  const registrar_legal_location = "Alice registrar place";
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .signupSolutionRegistrar(registrar_name, registrar_legal_location)
      .signAndSend(REGISTRAR_KEYRING, ({ status }) => {
        console.log(status.toHuman());
        if (status.isFinalized) {
          unsub();
          resolve();
        }
      });
  });
  const aliceRegistrarInfo =
    await api.query.workerNodePallet.registrarInventory(REGISTRAR_ADDRESS);
  console.log(aliceRegistrarInfo.toHuman());

  await api.disconnect()
}

main()
```
