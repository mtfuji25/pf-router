import { Connection, PublicKey } from "@solana/web3.js";

export const airdrop = async (
  connection: Connection,
  publicKey: PublicKey,
  amount: number
) => {
  let fundSig = await connection.requestAirdrop(publicKey, amount);

  return confirmTx(connection, fundSig);
};

export const confirmTx = async (connection: Connection, sig) => {
  const latestBlockHash = await connection.getLatestBlockhash("processed");

  await connection.confirmTransaction(
    {
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: sig,
    },
    "confirmed"
  );

  return await connection.getTransaction(sig, {
    maxSupportedTransactionVersion: 0,
    commitment: "confirmed",
  });
};