import anchor, { BN, Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as spl from "@solana/spl-token";

import { type TokenMill } from "./types/token_mill";
import TokenMillIdl from "./idl/token_mill.json";

const connection = new anchor.web3.Connection(
  process.env.RPC_URL ?? "",
  "confirmed"
);

const wallet = anchor.Wallet.local();

// Token Mill program is deployed on mainnet and devnet at JoeaRXgtME3jAoz5WuFXGEndfv4NPH9nBxsLq44hk9J
// Devnet version is already upgraded with the permissioned market creation feature
const program = new Program<TokenMill>(TokenMillIdl as any, {
  connection,
});

// The config account stores some default market parameters
// It is already created and just need to be provided to the program
// Mainnet : EVEVHBNUQ1gVG3LEMuJJov563CghdPdBB5nXNDHpHEA1
// Devnet : 8924mCgUTs7DE9UzNmN6bNFAdKmviqqe22Cx2C1abbPf
const config = new PublicKey(process.env.TOKEN_MILL_CONFIG ?? "");

// Only wSOL is currently supported as quote token
// So11111111111111111111111111111111111111112
const quoteTokenMint = new PublicKey(process.env.QUOTE_TOKEN ?? "");

// The quote token badge is used to identify supported quote tokens
// It is created by the program admins
const quoteTokenBadge = PublicKey.findProgramAddressSync(
  [
    Buffer.from("quote_token_badge"),
    config.toBuffer(),
    quoteTokenMint.toBuffer(),
  ],
  program.programId
)[0];

// "Base token" is the token that is being created alongside the market
const baseTokenKeypair = Keypair.generate();
const baseTokenMint = baseTokenKeypair.publicKey;

// The market account is a PDA derived from the base token mint
const market = PublicKey.findProgramAddressSync(
  [Buffer.from("market"), baseTokenMint.toBuffer()],
  program.programId
)[0];

// Creating the market requires providing its base token ATA
// As the whole token supply is minted and sent to the market account
const marketBaseTokenAta = spl.getAssociatedTokenAddressSync(
  baseTokenMint,
  market,
  true,
  spl.TOKEN_2022_PROGRAM_ID
);

{
  // The create market instruction takes the following parameters:
  // - name: The market name
  // - symbol: The market symbol
  // - uri: The market URI
  // - supply: The total token supply. 1 Billion supply is 1_000_000_000e6
  // - creatorFeeShare: The creator fee share in basis points (100% = 10_000)
  // - stakingFeeShare: The staking fee share in basis points (100% = 10_000)
  // creatorFeeShare + stakingFeeShare must sum up to 8_000 (80%), as the remaining 20% is the protocol fee
  // The transaction signer, also called "creator", will have admin rights over the market, allowing him to change the fee repartition, and claim the creator fees
  // Creator rights can be transferred
  const transaction = await program.methods
    .createMarket("Test Market", "TM", "", new BN(1_000_000_000e6), 8_000, 0)
    .accountsPartial({
      config,
      market,
      baseTokenMint,
      marketBaseTokenAta,
      quoteTokenBadge,
      quoteTokenMint,
      creator: wallet.publicKey,
    })
    .signers([wallet.payer, baseTokenKeypair])
    .transaction();

  const transactionSignature = await connection.sendTransaction(transaction, [
    wallet.payer,
    baseTokenKeypair,
  ]);

  const result = await connection.confirmTransaction(transactionSignature);

  if (result.value.err) {
    console.log("Market creation failed:", result.value.err);
    process.exit(1);
  }
}

console.log("Market created:", market.toBase58());

// Locking the market will only allow the specified authority to perform swaps
// This will usually be a PDA from the program built on top of Token Mill
// Authority is unique, immutable and can only be set right after market creation, when no tokens have eben bought yet
const swapAuthority = Keypair.generate().publicKey;
{
  const transaction = await program.methods
    .lockMarket(swapAuthority)
    .accountsPartial({
      market,
      creator: wallet.publicKey,
    })
    .signers([wallet.payer])
    .transaction();

  const transactionSignature = await connection.sendTransaction(transaction, [
    wallet.payer,
  ]);

  const result = await connection.confirmTransaction(transactionSignature);

  if (result.value.err) {
    console.log("Market lock failed:", result.value.err);
    process.exit(1);
  }
}

console.log("Market locked");

// Configuring the curve used by the market is done in a separate instruction
// Refer to the documentation for more information on the curve parameters
// Here dummy prices are used
// Market is now ready to be used
{
  const bidPrices: BN[] = [];
  const askPrices: BN[] = [];

  for (let i = 0; i < 11; i++) {
    bidPrices.push(new BN(i * 9e5));
    askPrices.push(new BN(i * 1e6));
  }

  const transaction = await program.methods
    .setMarketPrices(bidPrices, askPrices)
    .accountsPartial({
      market,
      creator: wallet.publicKey,
    })
    .signers([wallet.payer])
    .transaction();

  const transactionSignature = await connection.sendTransaction(transaction, [
    wallet.payer,
  ]);

  const result = await connection.confirmTransaction(transactionSignature);

  if (result.value.err) {
    console.log("Set prices failed:", result.value.err);
    process.exit(1);
  }
}

console.log("Prices set");
