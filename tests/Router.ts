import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Router } from "../target/types/router";
import { BN } from "bn.js";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { createAssociatedTokenAccountInstruction, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { airdrop } from "./utils";

describe("router", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Router as Program<Router>;
  const connection = provider.connection;

  const mint = new PublicKey("G7RdYEyuuqWa5Fwn3iVs99LyNcwVUE5hbaQEL8cUpump");
  const global = new PublicKey("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
  const bondingCurve = new PublicKey("9KVkzgk6A3dNPxGvSJkBmg6VaciErEKsSSizpYRQJEzb");
  const feeRecipient = new PublicKey("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");
  const eventAuthority = new PublicKey("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
  const pfProgram = new PublicKey("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
  const payer = Keypair.generate();


  before(async () => {
    await airdrop(connection, payer.publicKey, 1 * LAMPORTS_PER_SOL);
  });

  it("Buy PF!", async () => {

    const buyAmount = 1_000;

    const authority = payer.publicKey;
    const userAta = getAssociatedTokenAddressSync(mint, authority);
    const bondingCurveAta = getAssociatedTokenAddressSync(bondingCurve, authority, true);

    const createUserAtaIx = createAssociatedTokenAccountInstruction(authority, userAta, authority, mint);

    // const tx = await program.methods.pfBuy(
    //   new BN(buyAmount),
    // )
    //   .accounts({
    //     mint,
    //     authority,
    //     global,
    //     bondingCurve,
    //     userAta,
    //     bondingCurveAta,
    //     feeRecipient,
    //     eventAuthority,
    //     pfProgram,
    //   })
    //   .preInstructions([createUserAtaIx])
    //   .signers([payer])
    //   .rpc();
    // console.log("Your transaction signature", tx);
  });
});
