import bs58 from "bs58";
import prompt from "prompt-sync";

// Function to convert Base58 private key to wallet (byte array)
const base58ToWallet = () => {
  const input = prompt()("Enter your Base58 private key: ");
  try {
    const wallet = bs58.decode(input);
    console.log("Wallet byte array:", Array.from(wallet));
  } catch (error: any) {
    console.error("Error decoding Base58:", error.message);
  }
};

// Function to convert wallet (byte array) to Base58
const walletToBase58 = () => {
  const input = prompt()("Enter wallet byte array (comma separated): ");
  const walletArray = input.split(",").map(Number);
  try {
    const base58 = bs58.encode(Buffer.from(walletArray));
    console.log("Base58 encoded key:", base58);
  } catch (error: any) {
    console.error("Error encoding wallet:", error.message);
  }
};

base58ToWallet();
walletToBase58();
