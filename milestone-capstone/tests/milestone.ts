import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Milestone } from "../target/types/milestone";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { expect } from "chai";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAccount,
  createAssociatedTokenAccount,
  createInitializeMintInstruction,
  getAssociatedTokenAddress,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
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
let vaultAta: PublicKey;
let signerAta: PublicKey;
let usdcMint: Keypair;
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
    // creating usdc mint
    try {
      const lamports = await getMinimumBalanceForRentExemptMint(
        provider.connection
      );

      usdcMint = Keypair.generate();

      const tx1 = new Transaction().add(
        SystemProgram.createAccount({
          fromPubkey: signer.publicKey,
          newAccountPubkey: usdcMint.publicKey,
          space: MINT_SIZE,
          lamports,
          programId: TOKEN_PROGRAM_ID,
        }),
        createInitializeMintInstruction(
          usdcMint.publicKey,
          6,
          signer.publicKey,
          signer.publicKey,
          TOKEN_PROGRAM_ID
        )
      );

      console.log(
        `Tx Complete: https://explorer.solana.com/tx/${tx1}?cluster=localnet`
      );
    } catch (error) {
      console.log(`failed creating usdcMint : ${error}`);
    }

    // create signerATA
    try {
      signerAta = await createAssociatedTokenAccount(
        provider.connection,
        signer.payer,
        usdcMint.publicKey,
        signer.publicKey
      );

      const trx = await mintTo(
        provider.connection,
        signer.payer,
        usdcMint.publicKey,
        signerAta,
        signer.publicKey,
        100000
      );
      console.log(`created signerAta : ${trx}`);
    } catch (error) {
      console.log(`Failed creating signerAta ${error}`);
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

    // derive vault ata address only since the program will be creating that account. Not sending
    // try {
    //   vaultAta = await getAssociatedTokenAddress(
    //     usdcMint.publicKey,
    //     vaultAccount
    //   );
    //   console.log("Succefully got vaultAta address");
    // } catch (error) {
    //   console.log(`failed getting vaultata address ${error}`);
    // }
  });

  it("Is initialized!", async () => {
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
        usdcMint: usdcMint.publicKey,
        tokenProgram,
        associatedTokenProgram,
        systemProgram,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
