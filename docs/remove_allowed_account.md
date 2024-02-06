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
      .signAndSend(SUDO_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Registrar removed from allowed accounts');
          unsub();
          resolve();
        if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section == "system")) {
          console.error('Failed to removed registrar from allowed accounts');
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

main()
```
