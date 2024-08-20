```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";
import BN from 'bn.js';

const MILLY_UNIT = new BN(10).pow(new BN(15));

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947"); // wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws
  const api = await ApiPromise.create({ provider: wsProvider });
 const keyring = new Keyring({ type: "sr25519" });
 const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
   name: "Alice registrar keyring",
 });

 ...register solution group...

// Rewards can be raised by group registrar. Extrinsic can be called multiple times, but will take
// effect in next period. Each call applies raise to current rewards, not initially configured for
// group. You can raise reward separately for subscription and voting. If rewards are raised by
// multiplying of current value, then multiplier can not be 0.
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet.raiseGroupRewards(
	groupNamespace,
	{
		// subscription reward per block increased two times
		subscription: { Times: new BN(2) },
		// total voting reward per block increased by 50 milly tokens
		voting: { Amount: new BN(50).mul(MILLY_UNIT) },
		// raising bonus reward is not implemented yet as well as paying bonus
		bonus: null,
	}
	)
  .signAndSend(REGISTRAR_KEYRING, ({ events }) => {
	if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
		console.log('Solution group registered');
		unsub();
		resolve();
	}
	if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section == "system")) {
		console.error('Failed to register solution group');
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

main();
```
