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

  // Deploy FT
  const ft = await root.createSubAccount('ft');
  await ft.deploy('./aux/FT.wasm');

  // Deploy contract
  const contract = await root.createSubAccount('contract');
  await contract.deploy(process.argv[2]);

  // Initialize the contract
  await contract.call(contract, "new", { top_level_account: root.accountId })

  // Create test accounts
  const creator = await root.createSubAccount('creator');
  const alice = await root.createSubAccount('alice');

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, creator, alice };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});