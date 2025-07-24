import { BaseContractState, StateMigrations } from '../../../../chaindexing-ts/chaindexing-core/src';

export class Nft extends BaseContractState {
  constructor(
    public tokenId: number,
    public ownerAddress: string
  ) {
    super();
  }

  tableName(): string {
    return 'nfts';
  }
}

export class NftMigrations implements StateMigrations {
  migrations(): string[] {
    return [
      `CREATE TABLE IF NOT EXISTS nfts (
        token_id INTEGER NOT NULL,
        owner_address TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT NOW(),
        updated_at TIMESTAMP DEFAULT NOW()
      )`,
      `CREATE UNIQUE INDEX IF NOT EXISTS nfts_token_id_index ON nfts(token_id)`,
    ];
  }
} 