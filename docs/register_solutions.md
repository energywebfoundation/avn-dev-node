```ts
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { blake2AsHex } from "@polkadot/util-crypto";

async function main(): Promise<void> {
  const wsProvider = new WsProvider("ws://localhost:9947");
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const REGISTRAR_KEYRING = keyring.addFromUri("//Alice", {
    name: "Alice registrar keyring",
  });
  const REGISTRAR_ADDRESS = REGISTRAR_KEYRING.address; // "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  const solutionNamespace1 = "solution 1 namespace";
  const solutionName = "solution name";
  const solutionDescription = "solution description";
  const publisherInfo = "publisher info";
  const logoUrl = "solution logo url";
  const workLogicCid = "solution work logic cid";
  const executionEnvironment = 10; // NodeRedV1
  const expirationBlock = 100000;
  const maxWaitingThreshold = 60;
  const voteThresholdPercent = 60;
  const addition_to_extraneous_groups_allowed = false;
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .registerSolution(
        solutionNamespace1,
        solutionName,
        solutionDescription,
        publisherInfo,
        logoUrl,
        workLogicCid,
        executionEnvironment,
        expirationBlock,
        maxWaitingThreshold,
        voteThresholdPercent,
        addition_to_extraneous_groups_allowed,
      )
      .signAndSend(REGISTRAR_KEYRING, ({ status }) => {
        if (status.isFinalized) {
          unsub();
          resolve();
        }
      });
  });

  let solutionNamespace2 = "solution namespace 2";
  await new Promise<void>(async (resolve) => {
    let unsub = await api.tx.workerNodePallet
      .registerSolution(
        solutionNamespace2,
        solutionName,
        solutionDescription,
        publisherInfo,
        logoUrl,
        workLogicCid,
        executionEnvironment,
        expirationBlock,
        maxWaitingThreshold,
        voteThresholdPercent,
        addition_to_extraneous_groups_allowed,
      )
      .signAndSend(REGISTRAR_KEYRING, ({ events }) => {
        if (events.some(({ event: { method, section } }) => "ExtrinsicSuccess" === method && section == "system")) {
          console.log('Solution registered');
          unsub();
          resolve();
        if (events.some(({ event: { method, section } }) => "ExtrinsicFailed" === method && section == "system")) {
          console.error('Failed to register solution');
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`\t' ${phase}: ${section}.${method}:: ${data}`);
          });
          unsub();
          resolve();
        }
      });
  });

  const solutions = await api.query.workerNodePallet.solutions.multi(
    [blake2AsHex(solutionNamespace1), blake2AsHex(solutionNamespace2)],
  );
  // or
  const solutionKeys = await api.query.workerNodePallet.solutions.keys();
  await api.query.workerNodePallet.solutions.multi(
   solutionKeys.map((k) => (k.toHuman() as any)[0]), (solutions) => {
     solutions.map((s) => console.log(s.toHuman()))
   }
  );
  // or
  await api.query.workerNodePallet.solutions.entries();

  console.log(solutions.map((solution) => solution.toHuman()));

  await api.disconnect()
}

main();
```
