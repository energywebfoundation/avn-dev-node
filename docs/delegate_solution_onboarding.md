```ts
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';

const main = async () => {
  const provider = new WsProvider('wss://your-substrate-node-endpoint');
  const api = await ApiPromise.create({ provider });

  const keyring = new Keyring({ type: 'sr25519' });

  const stashAccount = keyring.addFromUri('//Alice'); // Replace with the actual stash account
  const proxyAccount = keyring.addFromUri('//Bob'); // Replace with the actual proxy account

  // 1 - adding a new delegate
  await delegateSolitionAddition(stashAccount, proxyAccount, api);

  /* 
    Before adding a solution to a group
    Register solution group and solution: 
    refer to the associated docs
  */
  // 2 - Create a transaction to call a function using the proxy
  const solutionAdditionTx = api.tx.workerNodePallet.addSolutionToGroup(groupNamespace, solutionNamespace);

  // 2 - Use the delegatee to add solution to group 
  // Create a proxy transaction to call the function on behalf of the stash account
  const proxyCallTx = api.tx.proxy.proxy(stashAccount.address, 'SolutionRegistrar', solutionAdditionTx);

  // Sign and send the proxy call transaction using the proxy account
  await proxyCallTx.signAndSend(proxyAccount, (result) => {
    const {status, isError} = result;
    
    console.log(`Proxy Call - Current status is ${result.status}`);

    if (status.isFinalized) {
      console.log(`Proxy Call - Transaction finalized at blockHash ${result.status.asFinalized}`);
    } else if (isError) {
      console.error(`Proxy Call - Transaction error: ${result.toHuman()}`);
    }
  }).catch((error) => {
    console.error(`Proxy Call - Failed to send transaction: ${error}`);
  });
};


const delegateSolitionAddition = async (delegator, delegee, api) => {
  // Define the custom proxy type
  // Use createType to ensure the custom proxy type is correctly instantiated
  const solutionAdditionProxyType = api.createType('ProxyType', 'SolutionRegistrar');

  // Define the delay in number of blocks (0 if no delay is needed)
  const delay = 0;

  // Create the transaction to add the proxy
  const tx = api.tx.proxy.addProxy(delegator.address, solutionAdditionProxyType, delay);

  // Sign and send the transaction using the stash account
  const unsub = await tx.signAndSend(stashAccount, (result) => {
    console.log(`Current status is ${result.status}`);

    if (result.status.isInBlock) {
      console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
    } else if (result.status.isFinalized) {
      console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
      unsub();
    }
  });
}

main().catch((error) => {
  console.error(error);
  process.exit(-1);
});
```