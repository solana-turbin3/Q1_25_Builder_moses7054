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
  getAccount,
  getAssociatedTokenAddress,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

let program: Program<Milestone>;
let provider: anchor.AnchorProvider;
let signerAdmin: anchor.Wallet;
let signerCompany: anchor.web3.Keypair;
let signerNgo: anchor.web3.Keypair;
let company: PublicKey;
let systemProgram: PublicKey;

// create admin
let admin: PublicKey;
let adminAta: PublicKey;
let mintAuthority: anchor.web3.Keypair;

// create ngo
let ngo: PublicKey;

// create project
let projectAccount: PublicKey;
let vaultAccount: PublicKey;
let signerAta: PublicKey;
let usdcMint: PublicKey;
let tokenProgram: PublicKey = TOKEN_PROGRAM_ID;
let associatedTokenProgram: PublicKey = ASSOCIATED_TOKEN_PROGRAM_ID;

// apply project
let tempTransactionAccount: PublicKey;

// process project
let projectCompletionDetails: PublicKey;

//process payment close accounts
let vaultAta: PublicKey;
let ngoAta: PublicKey;

describe("milestone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  program = anchor.workspace.Milestone as Program<Milestone>;

  before(async () => {
    // Get the provider from anchor
    provider = anchor.getProvider() as anchor.AnchorProvider;
    // Create a new wallet from the provider's payer
    signerAdmin = provider.wallet as anchor.Wallet;
    systemProgram = SystemProgram.programId;

    // admin account
    try {
      [admin] = PublicKey.findProgramAddressSync(
        [Buffer.from("admin")],
        program.programId
      );

      mintAuthority = Keypair.generate();
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
    } catch (error) {
      console.log(`admin test . Error: error`);
    }

    // intialize company
    try {
      signerCompany = Keypair.generate();

      let tx1 = await provider.connection.requestAirdrop(
        signerCompany.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      console.log(
        `Tx Complete: https://explorer.solana.com/tx/${tx1}?cluster=localnet`
      );

      const [companyPda, _bump] = PublicKey.findProgramAddressSync(
        [Buffer.from("company"), signerCompany.publicKey.toBytes()],
        program.programId
      );
      company = companyPda;
    } catch (error) {
      console.error("Error requesting airdrop:", error);
    }

    //intialize ngo
    try {
      signerNgo = Keypair.generate();
      let tx1 = await provider.connection.requestAirdrop(
        signerNgo.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      console.log(
        `Tx Complete: https://explorer.solana.com/tx/${tx1}?cluster=localnet`
      );

      [ngo] = PublicKey.findProgramAddressSync(
        [Buffer.from("ngo"), signerNgo.publicKey.toBytes()], // Use "company" and signer's public key as seeds
        program.programId
      );
    } catch (error) {}

    // create project account
    try {
      // derive project_Account pda
      [projectAccount] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("project"),
          company.toBytes(),
          Buffer.from("Test Project"),
        ],
        program.programId
      );

      // derive vault pda
      [vaultAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), projectAccount.toBytes()],
        program.programId
      );
      // Create and fund signer's ATA
      signerAta = await createAssociatedTokenAccount(
        provider.connection,
        mintAuthority,
        usdcMint,
        signerCompany.publicKey
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

    // apply project
    try {
      [tempTransactionAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("temp_tx"), projectAccount.toBytes(), ngo.toBytes()],
        program.programId
      );
    } catch (error) {
      console.log(`error while applying project ${error}`);
    }

    // process project
    try {
      [projectCompletionDetails] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("project_completion_details"),
          projectAccount.toBytes(),
          ngo.toBytes(),
        ],
        program.programId
      );
    } catch (error) {
      console.log(`error process project ${error}`);
    }

    //process payment close accounts
    try {
      vaultAta = await getAssociatedTokenAddress(usdcMint, vaultAccount);
      console.log("Vaule ATA created:", signerAta.toString());

      ngoAta = await getAssociatedTokenAddress(usdcMint, ngo);
    } catch (error) {
      console.log(`failed getting vaultAta address ${error}`);
    }
  });

  it("intialize admin", async () => {
    const tx = await program.methods
      .initAdmin(10, 10)
      .accountsPartial({
        signer: signerAdmin.publicKey,
        admin,
        usdcMint,
        tokenProgram,
        associatedTokenProgram,
        systemProgram,
      })
      .rpc();

    console.log("Transaction signaure", tx);
    const adminAccount = await program.account.admin.fetch(admin);
    console.log(adminAccount);
    expect(adminAccount.maxProjects).to.equal(10);
    expect(adminAccount.feeBasisPoints).to.equal(10);
  });

  it("intialize company", async () => {
    // tx to create company account
    const tx = await program.methods
      .initCompany("Example Company", "123456789")
      .accountsPartial({
        signer: signerCompany.publicKey,
        company,
        systemProgram,
      })
      .signers([signerCompany])
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
    expect(companyAccount.totalProjects).to.equal(0);
  });

  it("intialize ngo account", async () => {
    const tx = await program.methods
      .initNgo("Example Ngo")
      .accountsPartial({
        signer: signerNgo.publicKey,
        ngo,
        systemProgram,
      })
      .signers([signerNgo])
      .rpc();
    console.log("Your transaction signature", tx);
    const ngoAccount = await program.account.ngoAccount.fetch(ngo);
    console.log(ngoAccount);
    expect(Buffer.from(ngoAccount.name).toString("utf8").trim()).to.equal(
      "Example Ngo"
    );
  });

  it("create project account", async () => {
    const PROJECT_NAME: string = "Test Project";
    const REQUIREMENTS_HASH = Array.from(new Uint8Array(32).fill(1));
    const MAX_SUBMISSIONS: number = 5;
    const amount = new anchor.BN(10);

    // Log the accounts we're using
    console.log("Accounts being used:");
    console.log("Signer:", signerCompany.publicKey.toString());
    console.log("Company:", company.toString());
    console.log("Project Account:", projectAccount.toString());
    console.log("Vault Account:", vaultAccount.toString());
    console.log("Signer ATA:", signerAta.toString());
    console.log("USDC Mint:", usdcMint.toString());
    console.log("Admin:", admin.toString());

    try {
      const txSignature = await program.methods
        .createProject(PROJECT_NAME, REQUIREMENTS_HASH, MAX_SUBMISSIONS, amount)
        .accountsPartial({
          signer: signerCompany.publicKey,
          company,
          projectAccount,
          vaultAccount,
          signerAta,
          usdcMint,
          admin,
          tokenProgram,
          associatedTokenProgram,
          systemProgram,
        })
        .signers([signerCompany])
        .rpc();

      console.log("Transaction signature:", txSignature);
    } catch (error) {
      console.log("Detailed error:", error);
      // If error has logs property, print them
      if (error.logs) {
        console.log("Program logs:", error.logs);
      }
      throw error;
    }
  });

  it("apply project", async () => {
    const PROJECT_NAME: string = "Test Project";
    const SUBMITTED_REQUIREMENTS_HASH = Array.from(new Uint8Array(32).fill(1));

    try {
      const tx = await program.methods
        .initiateProject(PROJECT_NAME, SUBMITTED_REQUIREMENTS_HASH)
        .accountsPartial({
          signer: signerNgo.publicKey,
          ngo,
          projectAccount,
          tempTransactionAccount,
          systemProgram,
        })
        .signers([signerNgo])
        .rpc();

      console.log("Your transaction signature", tx);
      const tempTransactionAccountData =
        await program.account.tempTransactionAccount.fetch(
          tempTransactionAccount
        );
      console.log(tempTransactionAccountData);
      expect(
        Buffer.from(tempTransactionAccountData.submittedRequirementsHash)
      ).to.deep.equal(Buffer.from(SUBMITTED_REQUIREMENTS_HASH));
    } catch (error) {
      console.log(`apply project failed ${error}`);
    }
  });

  it("process project", async () => {
    const STATUS: string = "Accepted";
    const MERKLE_ROOT: Uint8Array | null = new Uint8Array(32).fill(1);

    try {
      const tx = await program.methods
        .processProjectFunding(STATUS, MERKLE_ROOT)
        .accountsPartial({
          signer: signerCompany.publicKey,
          company,
          projectAccount,
          ngo,
          projectCompletionDetails,
          tempTransactionAccount,
          systemProgram,
        })
        .rpc();

      console.log("Your transaction signature", tx);
      const projectCompletionDetailsData =
        await program.account.projectCompletionDetails.fetch(
          projectCompletionDetails
        );

      console.log(projectCompletionDetails);
      expect(
        Buffer.from(projectCompletionDetailsData.merkelRoot)
      ).to.deep.equal(Buffer.from(MERKLE_ROOT));

      expect(projectCompletionDetailsData.ngoPubkey).to.equal(ngo);

      expect(projectCompletionDetailsData.projectPubkey).to.equal(
        projectAccount
      );
    } catch (error) {
      console.log(`apply project failed ${error}`);
    }
  });

  it("process payment and close accounts", async () => {
    const PROJECT_NAME: string = "Test Project";

    try {
      const tx = await program.methods
        .processProjectPayment(PROJECT_NAME)
        .accountsPartial({
          signer: signerCompany.publicKey,
          projectAccount,
          vaultAccount,
          usdcMint,
          vaultAta,
          ngo,
          ngosignerPubkey: signerNgo.publicKey,
          projectCompletionDetails,
          tempTransactionAccount,
          admin,
          adminAta,
          tokenProgram,
          associatedTokenProgram,
          systemProgram,
        })
        .rpc();

      console.log("Your transaction signature", tx);
      // const projectCompletionDetailsData =
      //   await program.account.projectCompletionDetails.fetch(
      //     projectCompletionDetails
      //   );

      // console.log(projectCompletionDetails);
      // expect(
      //   Buffer.from(projectCompletionDetailsData.merkelRoot)
      // ).to.deep.equal(Buffer.from(MERKLE_ROOT));

      // expect(projectCompletionDetailsData.ngoPubkey).to.equal(ngo);

      // expect(projectCompletionDetailsData.projectPubkey).to.equal(
      //   projectAccount
      // );
    } catch (error) {
      console.log(`apply project failed ${error}`);
    }
  });
});
