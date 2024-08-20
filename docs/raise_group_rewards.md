```typescript
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";
import BN from 'bn.js';

const MILLY_UNIT = new BN(10).pow(new BN(15));

async function main(): Promise<void> {
   // set the adequate RPC url to the parachain. 
  // for a connection to dev parachain: const RPC_URL = "wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws";
  
  const RPC_URL = "ws://localhost:9947";
  const wsProvider = new WsProvider(RPC_URL);
  const api = await ApiPromise.create({ provider: wsProvider });
 const keyring = new Keyring({ type: "sr25519" });
 const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
   name: "Alice registrar keyring",
 });

 ...register solution group...

/** @dev: some considerations on rewards raising :
  - Rewards can only be raised by group registrar. 
  - The `raiseGroupRewards` extrinsic can be called multiple times, but will take effect in next period. 
  - Each call applies raise to current rewards, not initially configured for
 group. 
 - You can separately raise subscription rewards and voting rewards.
 - There a two modes of rewards raising:
     -  Amount : a direct addition of a specific amount to the current reward value
     - Times : the current reward value is increased by multiplying it with a specific value
 - If rewards are raised by multiplying of current value (Times mode), then multiplier can not be 0. 
 **/
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
		console.log('Reward raised on group');
		unsub();
		resolve();
	}
	if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section == "system")) {
		console.error('Failed to raise solution group rewards');
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
