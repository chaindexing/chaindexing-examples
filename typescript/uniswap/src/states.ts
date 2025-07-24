import { BaseContractState, BaseMultiChainState, StateMigrations } from '../../../../chaindexing-ts/chaindexing-core/src';

export class Pool extends BaseContractState {
  constructor(
    public token0Address: string,
    public token1Address: string,
    public fee: number,
    public tickSpacing: number,
    public poolContractAddress: string
  ) {
    super();
  }

  tableName(): string {
    return 'uniswap_pools';
  }
}

export class PoolMigrations implements StateMigrations {
  migrations(): string[] {
    return [
      `CREATE TABLE IF NOT EXISTS uniswap_pools (
        token0_address VARCHAR NOT NULL,
        token1_address VARCHAR NOT NULL,
        pool_contract_address VARCHAR NOT NULL,
        fee INTEGER NOT NULL,
        tick_spacing INTEGER NOT NULL,
        created_at TIMESTAMP DEFAULT NOW()
      )`,
      `CREATE INDEX IF NOT EXISTS uniswap_pools_pool_contract_address_index ON uniswap_pools(pool_contract_address)`,
      `CREATE INDEX IF NOT EXISTS uniswap_pools_token_addresses_index ON uniswap_pools(token0_address, token1_address)`,
    ];
  }
}

/**
 * Volumes swapped on Uniswap per token address
 * This is a MultiChainState as it aggregates volume across all chains
 */
export class TokenSwapVolume extends BaseMultiChainState {
  constructor(
    public tokenAddress: string,
    public amountEther: string,
    public lastUpdatedAt: number
  ) {
    super();
  }

  tableName(): string {
    return 'uniswap_token_swap_volumes';
  }
}

export class TokenSwapVolumeMigrations implements StateMigrations {
  migrations(): string[] {
    return [
      `CREATE TABLE IF NOT EXISTS uniswap_token_swap_volumes (
        token_address VARCHAR NOT NULL PRIMARY KEY,
        amount_ether VARCHAR NOT NULL,
        last_updated_at BIGINT NOT NULL,
        created_at TIMESTAMP DEFAULT NOW(),
        updated_at TIMESTAMP DEFAULT NOW()
      )`,
      `CREATE INDEX IF NOT EXISTS uniswap_token_swap_volumes_token_address_index ON uniswap_token_swap_volumes(token_address)`,
      `CREATE INDEX IF NOT EXISTS uniswap_token_swap_volumes_last_updated_index ON uniswap_token_swap_volumes(last_updated_at)`,
    ];
  }
} 