import nearAPI from "near-api-js";

const ADGUARD_PASSWORD = process.env["ADGUARD_PASSWORD"];

const { connect, keyStores } = nearAPI;


const connectionConfig = {
  networkId: "testnet",
  keyStore: new keyStores.InMemoryKeyStore(),
  nodeUrl: "https://archival-rpc.testnet.near.org",
  walletUrl: "https://testnet.mynearwallet.com/",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://testnet.nearblocks.io",
};

const near = await connect(connectionConfig);

const contractId = "near-dns-test2.testnet";
const methodName = "get_all_domains";

async function run() {
  try {
      const result = await near.connection.provider.query({
          request_type: "call_function",
          account_id: contractId,
          method_name: methodName,
          args_base64: "",
          finality: "final",
      });

      const records = JSON.parse(Buffer.from(result.result).toString());
      processRecords(records)
  } catch (error) {
      console.error("Error calling read-only method:", error);
  }
}

async function processRecords(records) {
  const existing = await listRecords()

  for(let i=0;i<records.length; i++){
    const record = records[i]
    const domain = `${record[0]}.local`
    const { A, AAAA } = record[1]

    if(!existing[domain]) {
      await addRecord(domain, A, AAAA)
    }else{
      if(existing[domain].answer != A ) {
        await updateRecord(existing[domain], domain, A, AAAA)
      }
    }

  }
}

async function listRecords() {
  console.log('listing domains')
  const url = `http://gatekeeper.cosmos.cboxlab.com/control/rewrite/list`;
  const basicAuth = btoa(`admin:${ADGUARD_PASSWORD}`);


  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Basic ${basicAuth}`
    }
  });

  const body = await response.json()
  console.log(`got ${body.length} domains`)
  const records = new Map()

  for (const r of body) {
    records[r.domain] = r
  }

  return records
}

async function addRecord(domain, A, AAAA) {
  console.log(`adding ${domain} with ${A}`)
  const url = `http://gatekeeper.cosmos.cboxlab.com/control/rewrite/add`;
  const basicAuth = btoa(`admin:${ADGUARD_PASSWORD}`);


  const rewriteRule = {
    domain,
    answer: A,
  };

  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Basic ${basicAuth}`
    },
    body: JSON.stringify(rewriteRule)
  });
  if(response.ok) {
    console.log("added successfully")
  }
}

async function updateRecord(record, domain, A, AAAA) {
  console.log(`updating ${record} with ${A}`)
  const url = `http://gatekeeper.cosmos.cboxlab.com/control/rewrite/update`;
  const basicAuth = btoa(`admin:${ADGUARD_PASSWORD}`);

  const payload = {
    target: record,
    update: {
      domain,
      answer: A,
    }
  }

  const response = await fetch(url, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Basic ${basicAuth}`
    },
    body: JSON.stringify(payload)
  });
  if(response.ok) {
    console.log("updated successfully")
  }
}

const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));

while(1) {
  await run()
  await sleep(60000)
}
