```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main() {
  const wsProvider = new WsProvider("ws://localhost:9947");
  const api = await ApiPromise.create({ provider: wsProvider });

// Permission to sign up as registrar can be withdrawn. It is only possible before account signed up
// as registrar.
  const SUDO_KEYRING = new Keyring({ type: "sr25519" }).addFromUri("//Ferdie", {
    name: "Sudo keyring",
  });

  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.sudo.sudo(
      api.tx.workerNodePallet
        .removeAllowedAccount(REGISTRAR_ADDRESS)

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

  await api.disconnect()
}

main()
```
