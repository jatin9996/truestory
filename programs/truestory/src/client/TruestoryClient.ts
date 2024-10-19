import { Connection, PublicKey, TransactionInstruction, Transaction } from '@solana/web3.js';
import { Program, AnchorProvider, web3, utils } from '@coral-xyz/anchor';
import { Truestory } from '../../target/types/truestory';
import { BN } from 'bn.js';

export class TruestoryClient {
    private program: Program<Truestory>;
    private provider: AnchorProvider;

    constructor() {
        this.provider = AnchorProvider.env();
        this.program = new Program(Truestory, this.provider, new PublicKey("3dupjHU543SdKpSkdTyPSbPLAowgTPRT15jG2rJd9fD1"));
    }

    async createLiquidityPool(tsmTokenAccount: PublicKey, solTokenAccount: PublicKey, initialTsm: number, initialSol: number): Promise<string> {
        const tx = await this.program.methods.createLiquidityPool(new web3.BN(initialTsm), new web3.BN(initialSol))
            .accounts({
                tsmTokenAccount,
                solTokenAccount,
                poolProgram: this.program.programId,
                poolAuthority: this.provider.wallet.publicKey,
            })
            .rpc();
        return tx;
    }

    async addLiquidity(tsmTokenAccount: PublicKey, solTokenAccount: PublicKey, tsmAmount: number, solAmount: number): Promise<string> {
        const tx = await this.program.methods.addLiquidity(new web3.BN(tsmAmount), new web3.BN(solAmount))
            .accounts({
                tsmTokenAccount,
                solTokenAccount,
                poolProgram: this.program.programId,
                poolAuthority: this.provider.wallet.publicKey,
            })
            .rpc();
        return tx;
    }

    async removeLiquidity(tsmTokenAccount: PublicKey, solTokenAccount: PublicKey, tsmAmount: number, solAmount: number): Promise<string> {
        const tx = await this.program.methods.removeLiquidity(new web3.BN(tsmAmount), new web3.BN(solAmount))
            .accounts({
                tsmTokenAccount,
                solTokenAccount,
                poolProgram: this.program.programId,
                poolAuthority: this.provider.wallet.publicKey,
            })
            .rpc();
        return tx;
    }

    async swapTokens(fromToken: PublicKey, toToken: PublicKey, amount: number): Promise<string> {
        const tx = await this.program.methods.swapTokens(new web3.BN(amount), fromToken, toToken)
            .accounts({
                poolProgram: this.program.programId,
                poolAuthority: this.provider.wallet.publicKey,
            })
            .rpc();
        return tx;
    }

    async fetchPriceData(): Promise<number> {
        const price = await this.program.methods.fetchPriceData().rpc();
        return price;
    }
}