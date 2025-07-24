import {
  PureHandler,
  PureHandlerContext,
} from '../../../../chaindexing-ts/chaindexing-core/src';
import { Pool, TokenSwapVolume } from './states';

export class PoolCreatedEventHandler implements PureHandler {
  abi(): string {
    return 'event PoolCreated(address indexed token0, address indexed token1, uint24 indexed fee, int24 tickSpacing, address pool)';
  }

  async handleEvent(context: PureHandlerContext): Promise<void> {
    const eventParams = context.getEventParams();

    // Extract each parameter as exactly specified in the ABI
    const token0Address = eventParams.getAddressString('token0');
    const token1Address = eventParams.getAddressString('token1');
    const poolContractAddress = eventParams.getAddressString('pool');
    const fee = eventParams.getU32('fee');
    const tickSpacing = eventParams.getU32('tickSpacing'); // Note: treating as unsigned for now

    console.log(`Creating pool: ${token0Address}/${token1Address} (fee: ${fee}, pool: ${poolContractAddress})`);

    // Create new pool state
    const pool = new Pool(
      token0Address,
      token1Address,
      fee,
      tickSpacing,
      poolContractAddress
    );

    await pool.create(context);

    console.log(`✅ Created pool ${poolContractAddress} for tokens ${token0Address}/${token1Address}`);

    // TODO: In the full implementation, we would also include the new pool contract for indexing
    // This would be equivalent to: chaindexing::include_contract(&context, "UniswapV3Pool", &pool_contract_address)
    // For now, we'll log that this would happen
    console.log(`📝 Would include contract ${poolContractAddress} for future indexing`);
  }
}

export class SwapEventHandler implements PureHandler {
  abi(): string {
    return 'event Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick)';
  }

  async handleEvent(context: PureHandlerContext): Promise<void> {
    const eventParams = context.getEventParams();
    const event = context.event;

    console.log(`Processing swap in pool: ${event.contractAddress}`);

    // TODO: In the full implementation, we would read the pool data
    // For now, we'll simulate the pool data (this would normally come from the database)
    const simulatedPool = {
      token0Address: '0xA0b86a33E6441e05e60BDA6C00C0b0D07c7e7b5C', // Mock token0
      token1Address: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2', // Mock token1 (WETH)
    };

    // Extract swap amounts (using BigInt for large numbers, then converting to number)
    const amount0 = Number(eventParams.getBigInt('amount0')) / 1e18; // Convert from wei to ether
    const amount1 = Number(eventParams.getBigInt('amount1')) / 1e18; // Convert from wei to ether

    const lastUpdatedAt = event.blockTimestamp;

    console.log(`Swap amounts: ${amount0} (token0), ${amount1} (token1)`);

    // Update volume for both tokens
    await this.createOrUpdateTokenSwapVolume(
      simulatedPool.token0Address,
      Math.abs(amount0),
      lastUpdatedAt,
      context
    );

    await this.createOrUpdateTokenSwapVolume(
      simulatedPool.token1Address,
      Math.abs(amount1),
      lastUpdatedAt,
      context
    );

    console.log(`✅ Updated swap volumes for tokens ${simulatedPool.token0Address} and ${simulatedPool.token1Address}`);
  }

  private async createOrUpdateTokenSwapVolume(
    tokenAddress: string,
    amountEther: number,
    lastUpdatedAt: number,
    context: PureHandlerContext
  ): Promise<void> {
    // TODO: In the full implementation, we would read existing volume from database
    // For now, we'll always create new records (this would normally update existing)
    
    console.log(`Creating/updating volume for token ${tokenAddress}: ${amountEther} ETH`);

    const tokenSwapVolume = new TokenSwapVolume(
      tokenAddress,
      amountEther.toString(),
      lastUpdatedAt
    );

    await tokenSwapVolume.create(context);

    console.log(`📊 Updated volume for ${tokenAddress}: ${amountEther} ETH`);
  }
} 