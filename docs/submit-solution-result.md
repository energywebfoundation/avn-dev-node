```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import { blake2AsHex } from "@polkadot/util-crypto"
import { stringToU8a } from '@polkadot/util';

async function main(): Promise<void> {
  // const wsProvider = new WsProvider("ws://localhost:9947")
  const wsProvider = new WsProvider("wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws")
  const api = await ApiPromise.create({ provider: wsProvider })

  const keyring = new Keyring({ type: "sr25519" })
  const alice = keyring.addFromUri("//Alice", {
    name: "Alice default",
  })
  const ALICE_ADDRESS = alice.address // "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  const publicKey = alice.publicKey
  const votingRoundId = '123'
  const resultHash = blake2AsHex('result')
  const signature = alice.sign(resultHash)
  const utx = await api.tx.workerNodePallet.submitSolutionResult(votingRoundId, resultHash, signature, publicKey)

  await new Promise<void>(async (resolve) => {
    // note the use of send instead of signAndSend
    // https://polkadot.js.org/docs/api/cookbook/tx/#how-do-i-send-an-unsigned-extrinsic
    let unsub = await utx.send(({ status }) => {
        if (status.isFinalized) {
          unsub()
          resolve()
        }
      })
  })

  await api.disconnect()
}

main()

```