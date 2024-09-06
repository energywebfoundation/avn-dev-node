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
const operator = "5C8GgQFP3X87mgN7kuFvqnptBo14DvyKiEgE3CuHhtWqEspP";
const solutionGroupNamespace = "2k17ytp2ei";

getScheduledUnsubscriptionOf(operator, solutionGroupNamespace)
  .catch(console.error)
  .finally(() => process.exit());
```