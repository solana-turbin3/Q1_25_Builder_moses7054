import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "../../id.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("H6enouvEWdpeFBJ2Lapv846XY6xUwaoUvKsLRXvKTjnk");

// Recipient address
const to = new PublicKey("kYd9wDWWjQz1NXE4WbGx2CasNNZ8Bh6W3SDqC6NfG18");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const fromAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    const toAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );
    // Get the token account of the toWallet address, and if it does not exist, create it
    // Transfer the new token to the "toTokenAccount" we just created
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
