
import * as anchor from "@coral-xyz/anchor";
import { BN,web3, Program } from "@coral-xyz/anchor";
import { V3, IDL } from "../target/types/v3";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, mintTo, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
const PROGRAM_ID = new PublicKey("EaBpabfkGswhPnC14mnwz3XxHHyJuqZjPZAMU9mR7KR3");
// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

//const program = anchor.workspace.v3 as Program<V3>;
const program = new Program<V3>(IDL, PROGRAM_ID, anchor.getProvider());

const offerSubscriptionId = program.addEventListener("OfferEvent", (event) => {
    console.log(`Event: ${event}`);
});

const adminSubscriptionId = program.addEventListener("AdminEvent", (event) => {
    console.log(`Event: ${event}`);
});
