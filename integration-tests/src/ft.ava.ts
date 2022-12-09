import { Worker, NearAccount, NEAR } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';
import { BN } from 'bn.js';
import { read, readFileSync } from 'fs';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

const TGAS = new BN("1000000000000")

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();
  const root = worker.rootAccount;

  // Deploy TDA contract
  await root.deploy('./aux/TLA.wasm')

  // Create test accounts
  const ft = await root.createSubAccount('ft');
  const contract = await root.createSubAccount('contract');
  const creator = await root.createSubAccount('creator');
  const alice = await root.createSubAccount('alice');

  // Deploy contracts
  await contract.deploy(process.argv[2]);
  await ft.deploy('./aux/FT.wasm');

  // Initialize the contracts
  await contract.call(contract, "new", { top_level_account: root.accountId })
  await ft.call(ft, 'new_default_meta', { owner_id: creator.accountId, name: "token", symbol: "tt", total_supply: "1" + "0".repeat(24) })

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, creator, alice, ft };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('drop on an existing account', async (t) => {
  const { contract, creator, alice, ft } = t.context.accounts;

  // Check the account balances
  const contractBalance = await contract.availableBalance()
  const aliceBalance = await alice.availableBalance()

  // Create a public key and add as a drop key 
  const pkDrop = await alice.getKey();

  await creator.call(
    contract, "create_ft_drop",
    {
      public_key: pkDrop?.getPublicKey()?.toString(),
      ft_contract: ft.accountId
    },
    { attachedDeposit: "46040000000000000000000" }
  )

  await creator.call(
    ft, 'storage_deposit',
    {
      account_id: contract.accountId,
    }, { attachedDeposit: "12500000000000000000000" }
  )
  await creator.call(
    ft, 'ft_transfer_call',
    {
      receiver_id: contract.accountId,
      amount: '1',
      msg: pkDrop?.getPublicKey()?.toString()
    }, { gas: "300000000000000",  attachedDeposit: "1" }
  )

  // Use the key to call "claim_for" to claim NEAR for account_id 
  await contract.setKey(pkDrop!)
  const claim = await contract.call(contract, "claim_for", { account_id: alice.accountId }, { gas: TGAS.muln(150), signWithKey: pkDrop! })
  t.true(claim)

  // Alice should now have fungible tokens
  const aliceTokens = await ft.view('ft_balance_of', { account_id: alice.accountId })
  t.deepEqual(aliceTokens, "1")

  // Try to use the key again
  const callAgain = contract.call(contract, "claim_for", { account_id: alice.accountId }, { gas: TGAS.muln(150), signWithKey: pkDrop! })
  await t.throwsAsync(callAgain)

  // Ideally, there should be no surplus in the contract
  const contractNewBalance = await contract.availableBalance()
  t.true(contractNewBalance >= contractBalance)

  console.log("EA - Contract balance surplus:", contractNewBalance.sub(contractBalance).toHuman())
});

// test('drop on an existing account', async (t) => {
//   const { contract, creator, alice, ft } = t.context.accounts;

//   // Check the account balances
//   const contractBalance = await contract.availableBalance()
//   const aliceBalance = await alice.availableBalance()

//   // Create a public key and add as a drop key 
//   const pkDrop = await alice.getKey();

//   await creator.call(
//     contract, "create_ft_drop",
//     {
//       public_key: pkDrop?.getPublicKey()?.toString(),
//       ft_contract: ft.accountId
//     },
//     { attachedDeposit: "46040000000000000000000" }
//   )

//   await creator.call(
//     ft, 'storage_deposit',
//     {
//       account_id: contract.accountId,
//     }, { attachedDeposit: "12500000000000000000000" }
//   )


//   // Use the key to call "claim_for" to claim NEAR for account_id 
//   await contract.setKey(pkDrop!)
//   const claim = await contract.call(contract, "claim_for", { account_id: alice.accountId }, { gas: TGAS.muln(150), signWithKey: pkDrop! })
//   t.true(claim)

//   // Alice should now have fungible tokens
//   const aliceTokens = await ft.view('ft_balance_of', { account_id: alice.accountId })
//   t.deepEqual(aliceTokens, "1")

//   // Try to use the key again
//   const callAgain = contract.call(contract, "claim_for", { account_id: alice.accountId }, { gas: TGAS.muln(150), signWithKey: pkDrop! })
//   await t.throwsAsync(callAgain)

//   // Ideally, there should be no surplus in the contract
//   const contractNewBalance = await contract.availableBalance()
//   t.true(contractNewBalance >= contractBalance)

//   console.log("EA - Contract balance surplus:", contractNewBalance.sub(contractBalance).toHuman())
// });