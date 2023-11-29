```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main() {
  const wsProvider = new WsProvider("ws://localhost:9947"); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  // Account has to be added to the allowed list before signing up as registrar. Only SUDO can allow
  // registrar
  const SUDO_KEYRING = new Keyring({ type: "sr25519" }).addFromUri("//Ferdie", {
    name: "Sudo keyring",
  });

  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.sudo.sudo(
      api.tx.workerNodePallet
        .allowAccount(REGISTRAR_ADDRESS)

    )
      .signAndSend(SUDO_KEYRING, ({ status }) => {
        console.log(status.toHuman());
        if (status.isFinalized) {
          console.log("account added to allowed")
          unsub();
          resolve();
        }
      });
  });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });
  const REGISTRAR_ADDRESS = REGISTRAR_KEYRING.address; // "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  // Free registrar balance should be no less than registrar signup deposit. Signup deposit amount of the free balance will be frozen
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
