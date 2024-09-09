# getScheduledUnsubscriptionOf

The `getScheduledUnsubscriptionOf` funciton retreives info on the scheduled unsubscription for given operator and solutionGroupNamespace.

```ts
import { blake2AsHex } from "@polkadot/util-crypto";
import { TypeRegistry, Text } from "@polkadot/types";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { u8aConcat, u8aToHex, stringToU8a } from "@polkadot/util";

/**
 * getScheduledUnsubscribtionOf : retreive info on the scheduled unsubscription for given operator and solutionGroupNamespace
 * @param operator 
 * @param solutionGroupNamespace 
 */

const getScheduledUnsubscriptionOf = async (operator, solutionGroupNamespace) => {
  const wsProvider = new WsProvider(
    "wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws"
  );
  const api = await ApiPromise.create({ provider: wsProvider });

  // retrives the schedule name
  const scheduleName = calculateScheduleName(operator, solutionGroupNamespace);

  // const scheduledUnsubscribe = await api.query.scheduler.lookup(scheduleName);
  const scheduledUnsubscribe = await api.query.scheduler.agenda(scheduleName);

    if (scheduledUnsubscribe.isSome) {
        const operatorUnsubscription = scheduledUnsubscribe.unwrap();
        console.log('Scheduled Task Found:', operatorUnsubscription.toHuman());
    // TODO : return  unsubscription info
    // - scheduled unsubscription blockNumber

    } else {
        console.log('No scheduled task found with the given schedule name.');
    }

    await api.disconnect();
};
```

It relies on the calculation of the `scheduleName`, based on the operator and the group name.
The logic of this calulation is implemented into the following `calculateScheduleName` function:

```ts
const calculateScheduleName = (operator, solutionGroupNamespace) => {

	const registry = new TypeRegistry();
	const operatorEncoded = registry.createType("AccountId", operator);
	const namespaceEncoded = registry.createType(
		"Vec<u8>",
		u8aToHex(stringToU8a(solutionGroupNamespace))
	);

	const unsubscribeEncoded = new Text(registry, "Unsubscribe").toU8a();

	const firstEncode = u8aConcat(
		operatorEncoded.toU8a(true),
		namespaceEncoded.toU8a(false)
	);
	const finalEncoded = u8aConcat(unsubscribeEncoded, firstEncode);

	const scheduleName = blake2AsHex(finalEncoded);

	console.log(`Scheduled unsubscribe name: ${scheduleName}`);
	return scheduleName;
};
```

Below is an illustration of how the `getScheduledUnsubscriptionOf` can be used :

```ts
const main = async () => {
  const operator = "5HidH4dPA3uTbhXMrL63s34bV31MksJHKiUrBQMDRee8d2z3";
  const solutionGroupNamespace = "ulwea0aiw7";
  const scheduledUnsubscriptionInfo = await getScheduledUnsubscriptionOf(operator, solutionGroupNamespace);
  console.log("\nUnsubcription infos", scheduledUnsubscriptionInfo);
};

main()
	.catch(console.error)
	.finally(() => process.exit());
```

Script output:

```shell
Unsubcription infos {
  operator: '5HidH4dPA3uTbhXMrL63s34bV31MksJHKiUrBQMDRee8d2z3',
  solutionGroupNamespace: 'ulwea0aiw7',
  scheduleName: '0x8e45a00ccc7b7ca6a82131cf4c5d66b0f4bbb95287fa278f5b8176ab7d58c644',
  scheduledAtBlock: 27344
}
```