# listPendingUnsubscriptions

This example illustrates how we can leverage `polkadotjs` SDK to retrieve the list of all scheduled unsubscriptions
into the worker node decentralized system.

```js
import { hexToU8a } from "@polkadot/util";
import { ApiPromise, WsProvider } from "@polkadot/api";

const wsProvider = new WsProvider(
	"wss://ewx-dev-parachain-aule-qb9wx41jvm.energyweb.org/ws"
);

async function listPendingUnsubscriptions(): Promise<void> {
	const api = await ApiPromise.create({ provider: wsProvider });

	// Retrieve all scheduled tasks
	const scheduledTasks = await api.query.scheduler.agenda.entries();

	// Filter out pending unsubscriptions
	const pendingScheduled = scheduledTasks.flatMap(([blockNumber, agenda]) => {
		const humanBlockNumber = blockNumber.toHuman();

		// Check each scheduled task in the agenda for each block
		return agenda.map((maybeSchedule) => {
			const scheduled = maybeSchedule.unwrapOr(null);
			if (scheduled) {
				const { call, maybeId } = scheduled;

				// Decode the inline call from hex
				const decodedCall = api.registry.createType(
					"Call",
					hexToU8a(call.asInline.toHex())
				);

				// Extract the module and method from the call
				const callSection = decodedCall.toHuman().section;
				const callMethod = decodedCall.toHuman().method;

				// Check if this is the 'execute_scheduled_unsubscribe' call
				if (
					callSection === "workerNodePallet" &&
					callMethod === "executeScheduledUnsubscribe" &&
					maybeId.isSome
				) {
					return {
						blockNumber: humanBlockNumber,
						operator: decodedCall.args[0].toHuman(),
						namespace: decodedCall.args[1].toHuman(),
						scheduleName: maybeId.unwrap().toHuman(),
					};
				}
			}
			return null;
		});
	});

	console.log("\nPending scheduled unsubscriptions:\n", pendingScheduled);
}

listPendingUnsubscriptions()
	.catch(console.error)
	.finally(() => process.exit());
```

If we have two scheduled unsubscriptions, the execution of this scripts will have an output similar to :

```js
Pending scheduled unsubscriptions:
 [
  {
    blockNumber: [ '12,065' ],
    operator: '5C8GgQFP3X87mgN7kuFvqnptBo14DvyKiEgE3CuHhtWqEspP',
    namespace: '2k17ytp2ei',
    scheduleName: '0x04cf172b592c3a2a4fadf44ea218a3f0d5c99ce4196650157d4a6b9592c45f7b'
  },
  {
    blockNumber: [ '12,067' ],
    operator: '5Dvjf2btMJpNjJWGNWvUiBgkBQqRssyDmz9sA1e2tkae7A7r',
    namespace: '2k17ytp2ei',
    scheduleName: '0x99b5ebb82fe61ff6d9cff9c089be0d2cfbc7a1b2ad460de8e4a63da76f2d0903'
  }
]
```
