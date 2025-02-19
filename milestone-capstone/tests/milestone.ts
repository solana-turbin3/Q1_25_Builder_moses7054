import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Milestone } from "../target/types/milestone";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { expect } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createMint,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

let program: Program<Milestone>;
let provider: anchor.AnchorProvider;
let signer: anchor.Wallet;
let company: PublicKey;
let systemProgram: PublicKey;

// create project

let projectAccount: PublicKey;
let vaultAccount: PublicKey;
let signerAta: PublicKey;
let usdcMint: PublicKey;
let tokenProgram: PublicKey = TOKEN_PROGRAM_ID;
let associatedTokenProgram: PublicKey = ASSOCIATED_TOKEN_PROGRAM_ID;

describe("milestone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  program = anchor.workspace.Milestone as Program<Milestone>;

  before(async () => {
    // Get the provider from anchor
    provider = anchor.getProvider() as anchor.AnchorProvider;
    // Create a new wallet from the provider's payer
    signer = provider.wallet as anchor.Wallet;
    systemProgram = SystemProgram.programId;

    try {
      let tx1 = await provider.connection.requestAirdrop(
        signer.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      console.log(
        `Tx Complete: https://explorer.solana.com/tx/${tx1}?cluster=localnet`
      );
    } catch (error) {
      console.error("Error requesting airdrop:", error);
    }

    // Derive PDA for company account
    const [companyPda, _bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("company"), signer.publicKey.toBytes()], // Use "company" and signer's public key as seeds
      program.programId
    );
    company = companyPda;

    //  second instruction
    // creating usdc mint, signerAta,
    try {
      const mintAuthority = Keypair.generate();
      // First airdrop some SOL to the mint authority
      const airdropSig = await provider.connection.requestAirdrop(
        mintAuthority.publicKey,
        LAMPORTS_PER_SOL
      );
      // Wait for airdrop confirmation
      const latestBlockHash = await provider.connection.getLatestBlockhash();
      await provider.connection.confirmTransaction({
        signature: airdropSig,
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      });

      // Verify the balance
      const balance = await provider.connection.getBalance(
        mintAuthority.publicKey
      );
      console.log("Mint authority balance:", balance / LAMPORTS_PER_SOL, "SOL");

      usdcMint = await createMint(
        provider.connection,
        mintAuthority,
        mintAuthority.publicKey,
        null,
        6
      );
      console.log("USDC mint created:", usdcMint.toString());

      // Create and fund signer's ATA
      signerAta = await createAssociatedTokenAccount(
        provider.connection,
        mintAuthority,
        usdcMint,
        signer.publicKey
      );
      console.log("Signer ATA created:", signerAta.toString());

      // Mint some tokens to signer's ATA
      await mintTo(
        provider.connection,
        mintAuthority,
        usdcMint,
        signerAta,
        mintAuthority,
        100000
      );
      console.log("Tokens minted to signer's ATA");
    } catch (error) {
      // console.log(`failed creating usdcMint : ${error}`);
      console.error("Error setting up token accounts:", error);
      throw error;
    }

    // derive project_Account pda
    [projectAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("project"), company.toBytes(), Buffer.from("Test Project")],
      program.programId
    );

    // derive vault pda
    [vaultAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), projectAccount.toBytes()],
      program.programId
    );
  });

  it("intialize admin", async () => {
    
  })



  it("intialize company!", async () => {
    // tx to create company account
    const tx = await program.methods
      .initCompany("Example Company", "123456789", 10)
      .accountsPartial({
        signer: signer.publicKey,
        company,
        systemProgram,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const companyAccount = await program.account.companyAccount.fetch(company);
    console.log(companyAccount);
    expect(Buffer.from(companyAccount.name).toString("utf8").trim()).to.equal(
      "Example Company"
    );
    expect(
      Buffer.from(companyAccount.businessRegNum).toString("utf8").trim()
    ).to.equal("123456789");
    expect(companyAccount.maxProjects).to.equal(10);
    expect(companyAccount.totalProjects).to.equal(0);
  });

  // intialize project account
  it("intialize project account", async () => {
    // Arguments
    const PROJECT_NAME: string = "Test Project";
    const REQUIREMENTS_HASH = Array.from(new Uint8Array(32).fill(1));
    const MAX_SUBMISSIONS: number = 5;
    const DEPOSIT_AMOUNT: anchor.BN = new anchor.BN(1000);

    // creating usdc mint

    const tx = await program.methods
      .createProject(
        PROJECT_NAME,
        REQUIREMENTS_HASH,
        MAX_SUBMISSIONS,
        DEPOSIT_AMOUNT
      )
      .accountsPartial({
        signer: signer.publicKey,
        company,
        projectAccount,
        vaultAccount,
        signerAta,
        usdcMint: usdcMint,
        tokenProgram,
        associatedTokenProgram,
        systemProgram,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
