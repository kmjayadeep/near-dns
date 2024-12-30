import nearAPI from "near-api-js";

const DUCKDNS_TOKEN = process.env["DUCKDNS_TOKEN"];

const { connect, keyStores } = nearAPI;


const connectionConfig = {
  networkId: "testnet",
  keyStore: new keyStores.InMemoryKeyStore(),
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://testnet.mynearwallet.com/",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://testnet.nearblocks.io",
};

const near = await connect(connectionConfig);

const contractId = "near-dns-test2.testnet";
const methodName = "get_all_domains";

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


async function processRecords(records) {
    for(let i=0;i<records.length; i++){
        const record = records[0]
        const domain = record[0]
        const { A, AAAA } = record[1]
        await updateRecord(domain, A, AAAA)
    }
}

async function updateRecord(domain, A, AAAA) {
    const url = `https://www.duckdns.org/update?domains=${domain}&token=${DUCKDNS_TOKEN}&ip=${A}&ipv6=${AAAA}&verbose=true`;

    (async () => {
        try {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
            const data = await response.text();
            console.log("Response data:", data);
        } catch (error) {
            console.error("Error:", error.message);
        }
    })();
}