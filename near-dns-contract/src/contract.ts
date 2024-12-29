// Find all our documentation at https://docs.near.org
import { NearBindgen, near, call, view, UnorderedMap } from 'near-sdk-js';

type DomainRecord = {
  owner: string;
  A: string; // Ipv4 target
  AAAA: string // Ipv6 target
};


@NearBindgen({})
class HelloNear {

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
    this.records.set(domain, {
      owner,
      A,
      AAAA,
    })
  }
  
  @view({})
  get_domain({domain}: {domain: string}): DomainRecord {
    return this.records.get(domain)
  }
}