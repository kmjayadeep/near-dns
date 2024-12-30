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
  const secondAccount = await root.createSubAccount('second')
  const contract = await root.createSubAccount('test-account');

  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );

  // Save state for test runs, it is unique for each test
  t.context.accounts = { root, contract, secondAccount };
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

test('can update domains', async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, 'register_domain', { domain: 'abc', A: 'A', AAAA: 'AAAA' });
  const record = await contract.view('get_domain', {domain: 'abc'});
  t.deepEqual(record, {
    owner: 'test.near',
    A: 'A',
    AAAA: 'AAAA'
  });
  ``
  await root.call(contract, 'register_domain', { domain: 'abc', A: 'A2', AAAA: 'AAAA2' });
  const record2 = await contract.view('get_domain', {domain: 'abc'});
  t.deepEqual(record2, {
    owner: 'test.near',
    A: 'A2',
    AAAA: 'AAAA2'
  });
});

test('only owner can update domains', async (t) => {
  const { root, contract, secondAccount } = t.context.accounts;
  await root.call(contract, 'register_domain', { domain: 'abc', A: 'A', AAAA: 'AAAA' });
  const record = await contract.view('get_domain', {domain: 'abc'});
  t.deepEqual(record, {
    owner: 'test.near',
    A: 'A',
    AAAA: 'AAAA'
  });

  const promise = secondAccount.call(contract, 'register_domain', { domain: 'abc', A: 'A2', AAAA: 'AAAA2' });
  await t.throwsAsync(promise);
});

test('owner can delete domains', async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, 'register_domain', { domain: 'abc', A: 'A', AAAA: 'AAAA' });
  const record = await contract.view('get_domain', {domain: 'abc'});
  t.deepEqual(record, {
    owner: 'test.near',
    A: 'A',
    AAAA: 'AAAA'
  });
  await root.call(contract, 'delete_domain', { domain: 'abc'});
  const record2 = await contract.view('get_domain', {domain: 'example'});
  t.falsy(record2);
});

test('only owner can delete domains', async (t) => {
  const { root, contract, secondAccount } = t.context.accounts;
  await root.call(contract, 'register_domain', { domain: 'abc', A: 'A', AAAA: 'AAAA' });
  const record = await contract.view('get_domain', {domain: 'abc'});
  t.deepEqual(record, {
    owner: 'test.near',
    A: 'A',
    AAAA: 'AAAA'
  });
  const promise = secondAccount.call(contract, 'delete_domain', { domain: 'abc'});
  await t.throwsAsync(promise);
});

test('return all domains', async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, 'register_domain', { domain: 'abc', A: 'A', AAAA: 'AAAA' });
  const records = await contract.view('get_all_domains');
  t.is(records.length, 1)
  t.is(records[0].length, 2)
  t.is(records[0][0], "abc")
  t.deepEqual(records[0][1], {
    owner: 'test.near',
    A: 'A',
    AAAA: 'AAAA'
  });
});