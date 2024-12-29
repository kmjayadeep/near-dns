import anyTest from 'ava';
import { Worker } from 'near-workspaces';
import { setDefaultResultOrder } from 'dns'; setDefaultResultOrder('ipv4first'); // temp fix for node >v17

/**
 *  @typedef {import('near-workspaces').NearAccount} NearAccount
 *  @type {import('ava').TestFn<{worker: Worker, accounts: Record<string, NearAccount>}>}
 */
const test = anyTest;

test.beforeEach(async t => {
  // Create sandbox
  const worker = t.context.worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount('test-account');

  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );

  // Save state for test runs, it is unique for each test
  t.context.accounts = { root, contract };
});

test.afterEach.always(async (t) => {
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('returns empty when no record exist for the record', async (t) => {
  const { contract } = t.context.accounts;
  const record = await contract.view('get_domain', {domain: 'example'});
  t.falsy(record);
});

test('can register domains', async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, 'register_domain', { domain: 'abc', A: 'A', AAAA: 'AAAA' });
  const record = await contract.view('get_domain', {domain: 'abc'});
  t.deepEqual(record, {
    owner: 'test.near',
    A: 'A',
    AAAA: 'AAAA'
  });
});