```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import { blake2AsHex } from "@polkadot/util-crypto"

async function main(): Promise<void> {
  // const wsProvider = new WsProvider("ws://localhost:9947")
  const wsProvider = new WsProvider("wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws")
  const api = await ApiPromise.create({ provider: wsProvider })

  const keyring = new Keyring({ type: "sr25519" })
  const OPERATOR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice default",
  })
  const OPERATOR_ADDRESS = OPERATOR_KEYRING.address

  const operator_name = "Operator Alice";
  const operator_legal_location = "Alice's location";

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
  const operatorAliceInfo =
    await api.query.workerNodePallet.workerNodeOperatorInventory(OPERATOR_ADDRESS);
  console.log(operatorAliceInfo.toHuman());

  // Connect worker address
  const workerAddress = "5GHtiduViydfQY5RhLu7gWbCTY8VeZeJ4aRt7czkqenyyz91"
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .connectWorkerNode(workerAddress)
      .signAndSend(OPERATOR_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Worker node connected');
          unsub();
          resolve();
        } else if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section === "system")) {
          console.error('Failed to connect worker node');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
          unsub();
          resolve();
        }
      });
  })

  await api.disconnect()
}

main()
```
