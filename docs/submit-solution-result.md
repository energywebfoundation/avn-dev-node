```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import { blake2AsHex } from "@polkadot/util-crypto"
import { stringToU8a } from '@polkadot/util';

// Submitting only possible in the next period after subscribing
async function main(): Promise<void> {
  const wsProvider = new WsProvider("wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws")
  const api = await ApiPromise.create({ provider: wsProvider })

  const WORKER_KEYRING = keyring.addFromUri("//Charlie", {
    name: "Worker node Charlie",
  });

  const solutionNamespace = "ZEV";
  const votingRoundId = '123'
  const resultHash = blake2AsHex('result')
  const signature = workerKeyring.sign(resultHash)
  const utx = await api.tx.workerNodePallet.submitSolutionResult(votingRoundId, resultHash, signature, workerKeyring.publicKey)

  await new Promise<void>(async (resolve) => {
    // note the use of send instead of signAndSend
    // https://polkadot.js.org/docs/api/cookbook/tx/#how-do-i-send-an-unsigned-extrinsic
    let unsub = await utx.send(({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Solution result submitted');
          unsub();
          resolve();
        } else if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section === "system")) {
          console.error('Failed to submit solution result');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
          unsub();
          resolve();
        }
      })
  })

  await api.disconnect()
}

main()

```
