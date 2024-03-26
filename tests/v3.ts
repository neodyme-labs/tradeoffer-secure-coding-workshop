import * as anchor from "@coral-xyz/anchor";
import { BN,web3, Program } from "@coral-xyz/anchor";
import { V3, IDL } from "../target/types/v3";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, mintTo, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
const PROGRAM_ID = new PublicKey("EaBpabfkGswhPnC14mnwz3XxHHyJuqZjPZAMU9mR7KR3");
describe("v3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  //const program = anchor.workspace.v3 as Program<V3>;
  const program = new Program<V3>(IDL, PROGRAM_ID, anchor.getProvider());

  before("setup", async() => {

  });

  it("Is initialized!", async () => {
    // Add your test here.
      //
      const [globalAcc, bump] = await PublicKey.findProgramAddress(
          [
              anchor.utils.bytes.utf8.encode("global"),
          ],
          program.programId
      );

    const admin = web3.Keypair.generate();
    const treasury = Keypair.generate();
    const taker = Keypair.generate();
    const provider = anchor.getProvider();
    const payer: Keypair = provider.wallet.payer;

    console.log(`payer: ${payer.publicKey.toString()}`);


    const tx = await program.methods.initialize({
        admin: payer.publicKey,
        treasury: treasury.publicKey,
        fee: new BN(5000),
    }).accounts({
        admin: payer.publicKey,
        global: globalAcc,
        systemProgram: SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);

    const glob = await program.account.global.fetch(globalAcc);
    console.log(glob);
    console.log(`admin ${glob.admin.toString()}`);
    console.log(`treasury ${glob.treasury.toString()}`);
    console.log(`fee ${glob.fee.toNumber().toString()}`);
    console.log(`creating Mint`);

        const mint_a = await createMint(
            provider.connection,
            payer,
            payer.publicKey,
            payer.publicKey,
            6
        );
    console.log(`creating Mint`);
        const mint_b = await createMint(
            provider.connection,
            payer,
            payer.publicKey,
            payer.publicKey,
            6
        );
    console.log(`creating account`);
        const offererTokenAccountA = await createAssociatedTokenAccount(
            provider.connection,
            payer,
            mint_a,
            payer.publicKey
        );
    console.log(`creating account`);
        const offererTokenAccountB = await createAssociatedTokenAccount(
            provider.connection,
            payer,
            mint_b,
            payer.publicKey
        );
    console.log(`creating account`);
        const takerTokenAccountA = await createAssociatedTokenAccount(
            provider.connection,
            payer,
            mint_a,
            taker.publicKey
        );
    console.log(`creating account`);
        const takerTokenAccountB = await createAssociatedTokenAccount(
            provider.connection,
            payer,
            mint_b,
            taker.publicKey
        );


    
      const [offer, _] = PublicKey.findProgramAddressSync(
          [
              anchor.utils.bytes.utf8.encode("tradeoffer"),
              payer.publicKey.toBuffer(),
          ],
          program.programId
      );
    console.log(`creating account`);
        const escrow = getAssociatedTokenAddressSync(
            mint_a,
            offer,
            true,
        );

    console.log(`minting to account`);
        mintTo(
            provider.connection,
            payer,
            mint_a,
            offererTokenAccountA,
            payer.publicKey,
            10000 * 10**6
        );

    console.log(`minting to account`);
        mintTo(
            provider.connection,
            payer,
            mint_b,
            takerTokenAccountB,
            taker.publicKey,
            10000 * 10**6
        );

        console.log("minted 10k tokens to offerer and taker");
        console.log("creating offer tx");

      const offer_tx = await program.methods.createOffer({
          offerAmount: new BN(1234),
          requestAmount: new BN(3333),
      }).accounts({
          owner: payer.publicKey,
          global: globalAcc,
          offer: offer,
          ownerOfferToken: offererTokenAccountA,
          offerEscrow: escrow,
          offerMint: mint_a,
          requestMint: mint_b,
          treasury: treasury.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
      }).rpc();
      console.log(`tx ${offer_tx}`);
  });
});

