import anchor, { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

import { type TokenMill } from "./types/token_mill";
import TokenMillIdl from "./idl/token_mill.json";

const connection = new anchor.web3.Connection(
  process.env.RPC_URL ?? "",
  "confirmed"
);

const wallet = anchor.Wallet.local();

const program = new Program<TokenMill>(TokenMillIdl as any, {
  connection,
});

// Fetch required accounts
const market = new PublicKey(process.env.MARKET ?? "");

// The swap authority has to be signer of the transaction
{
  const transaction = await program.methods
    .freeMarket()
    .accountsPartial({
      market,
      swapAuthority: wallet.publicKey,
    })
    .signers([wallet.payer])
    .transaction();

  const transactionSignature = await connection.sendTransaction(transaction, [
    wallet.payer,
  ]);

  const result = await connection.confirmTransaction(transactionSignature);

  if (result.value.err) {
    console.log("Market freeing failed:", result.value.err);
    process.exit(1);
  }
}

console.log("Market freed");
