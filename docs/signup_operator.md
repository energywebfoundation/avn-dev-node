```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main() {
  const wsProvider = new WsProvider("ws://localhost:9947"); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const OPERATOR_KEYRING = keyring.addFromUri("//Bob", {
    name: "Operator Bob",
  });
  const OPERATOR_ADDRESS = OPERATOR_KEYRING.address; // "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  const operator_name = "Operator Bob";
  const operator_legal_location = "Operator legal location";

  // On subscribing operator deposits part of his free balance. The deposited amount can be queried as `workerNodeOperatorDeposit()`
  // The deposited amount becomes frozen part of the balance
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .signupWorkerNodeOperator(operator_name, operator_legal_location)
      .signAndSend(OPERATOR_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Operator signed up');
          unsub();
          resolve();
        } else if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section === "system")) {
          console.error('Failed to signup operator');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
          unsub();
          resolve();
        }
      });
  });
  const operatorBobInfo =
    await api.query.workerNodePallet.workerNodeOperatorInventory(OPERATOR_ADDRESS);
  console.log(operatorBobInfo.toHuman());

  await api.disconnect()
}

main()
```
