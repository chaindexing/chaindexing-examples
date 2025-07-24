import {
  PureHandler,
  SideEffectHandler,
  PureHandlerContext,
  SideEffectHandlerContext,
} from '../../../../chaindexing-ts/chaindexing-core/src';
import { Nft } from './states';

export class TransferHandler implements PureHandler {
  abi(): string {
    return 'event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)';
  }

  async handleEvent(context: PureHandlerContext): Promise<void> {
    const eventParams = context.getEventParams();

    // Extract each parameter as exactly specified in the ABI:
    // "event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)"
    const from = eventParams.getAddressString('from');
    const to = eventParams.getAddressString('to');
    const tokenId = eventParams.getU32('tokenId');

    // For now, we'll always create or update (the TypeScript implementation doesn't have readOne yet)
    if (from === '0x0000000000000000000000000000000000000000') {
      // Mint: create new NFT
      const newNft = new Nft(tokenId, to);
      await newNft.create(context);
      console.log(`Minted new NFT ${tokenId} to ${to}`);
    } else {
      // Transfer: create a new record (in a full implementation we'd update existing)
      const transferredNft = new Nft(tokenId, to);
      await transferredNft.create(context);
      console.log(`Transferred NFT ${tokenId} from ${from} to ${to}`);
    }
  }
}

export class TransferSideEffectHandler implements SideEffectHandler<any> {
  abi(): string {
    return 'event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)';
  }

  async handleEvent(context: SideEffectHandlerContext<any>): Promise<void> {
    const eventParams = context.getEventParams();
    const tokenId = eventParams.getU32('tokenId');
    const to = eventParams.getAddressString('to');

    // Side effect: Send notification
    await this.sendTransferNotification(tokenId, to);
    
    console.log(`Sent transfer notification for NFT ${tokenId} to ${to}`);
  }

  private async sendTransferNotification(tokenId: number, toAddress: string): Promise<void> {
    // Mock notification service - in real implementation this would call
    // external services like email, push notifications, webhooks, etc.
    console.log(`📧 Notification: NFT ${tokenId} has been transferred to ${toAddress}`);
    
    // Simulate async operation
    await new Promise(resolve => setTimeout(resolve, 100));
  }
} 