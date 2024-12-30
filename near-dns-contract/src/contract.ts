// Find all our documentation at https://docs.near.org
import { NearBindgen, near, call, view, UnorderedMap } from 'near-sdk-js';

type DomainRecord = {
  owner: string;
  A: string; // Ipv4 target
  AAAA: string // Ipv6 target
};


@NearBindgen({})
class NearDNS {

  private records = new UnorderedMap<DomainRecord>("v1");

  static schema = {
    records: {
      class: UnorderedMap,
      value: {
        owner: 'string',
        A: 'string',
        AAAA: 'string'
      }
    }
  };

  @call({})
  register_domain({ domain, A, AAAA }: { domain: string, A: string, AAAA: string }): void {
    near.log(`Saving domain ${domain}, ${A}, ${AAAA}`);
    const owner = near.signerAccountId();

    const existing = this.records.get(domain)
    if(existing && existing.owner != owner) {
      throw new Error("only owner can update the domain")
    }

    this.records.set(domain, {
      owner,
      A,
      AAAA,
    })
  }

  @call({})
  delete_domain({ domain }: { domain: string }): void {
    near.log(`Deleting domain ${domain}`);
    const owner = near.signerAccountId();

    const record = this.records.get(domain);
    if(record.owner != owner){ 
      throw new Error("only owner can delete the domain")
    }

    this.records.remove(domain)
  }
  
  
  @view({})
  get_domain({domain}: {domain: string}): DomainRecord {
    return this.records.get(domain)
  }

  @view({})
  get_all_domains(): [string, DomainRecord][] {
    return this.records.toArray()
  }
}