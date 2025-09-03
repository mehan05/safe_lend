import { program } from "./../node_modules/@solana/codecs-data-structures/node_modules/commander/typings/index.d";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Account,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { expect } from "chai";
import { SafeLend } from "../target/types/safe_lend";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

let provider = anchor.AnchorProvider.env();

const mint_account = async (
  payer: anchor.web3.Keypair
): Promise<anchor.web3.PublicKey> => {
  try {
    const mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      6
    );

    return mint;
  } catch (error) {
    console.log("Error in creating mint", error);
  }
};

const airDrop = async (to: anchor.web3.PublicKey, amount: number) => {
  try {
    const tx = await provider.connection.requestAirdrop(
      to,
      anchor.web3.LAMPORTS_PER_SOL * amount
    );
    await provider.connection.confirmTransaction(tx, "confirmed");
  } catch (error) {
    console.log("error in airDrop", error);
  }
};

const ata_accounts = async (
  payer: anchor.web3.Keypair,
  mint_acc: anchor.web3.PublicKey
): Promise<Account> => {
  try {
    const ata = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint_acc,
      payer.publicKey
    );

    return ata;
  } catch (error) {
    console.log("Problem in ata", error);
  }
};

const create_pda = async (
  programId: anchor.web3.PublicKey,
  maker: anchor.web3.Keypair,
  secret_seed: anchor.BN,
  seed_const: string
): Promise<anchor.web3.PublicKey> => {
  try {
    const pda = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        secret_seed.toArrayLike(Buffer, "le", 8),
      ],
      programId
    )[0];

    return pda;
  } catch (error) {
    console.log("Error in creating PDA", error);
  }
};

describe("safe_lend", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let SEED = new anchor.BN(1);

  //users
  let lender: anchor.web3.Keypair;
  let borrower: anchor.web3.Keypair;
  let admin: anchor.web3.Keypair;

  //mint
  let mint_sol: anchor.web3.PublicKey;
  let mint_usdt: anchor.web3.PublicKey;

  //pdas
  let global_state: anchor.web3.PublicKey;
  let loan_state: anchor.web3.PublicKey;
  let user_state: anchor.web3.PublicKey;

  //ata
  let treasure_vault: anchor.web3.PublicKey;
  let lender_ata: anchor.web3.PublicKey;
  let lender_vault: anchor.web3.PublicKey;
  let borrower_ata: anchor.web3.PublicKey;
  let borrower_vault: anchor.web3.PublicKey;

  const program = anchor.workspace.safeLend as Program<SafeLend>;

  it("Initialize Lending Pool", async () => {
    // Add your test here.
    let lender = anchor.web3.Keypair.generate();
    let borrower = anchor.web3.Keypair.generate();
    let admin = anchor.web3.Keypair.generate();

    airDrop(lender.publicKey, 2);
    airDrop(borrower.publicKey, 2);
    airDrop(admin.publicKey, 2);

    let mint_sol = await mint_account(lender);
    let mint_usdt = await mint_account(borrower);

    let global_state =  anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global_state"), admin.publicKey.toBuffer()],
      program.programId
    )[0];

    let user_state =  anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lender"),
        lender.publicKey.toBuffer(),
        SEED.toArrayLike(Buffer, "le", 8)
      ],program.programId
    )[0];

    let loan_state =  anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("loan"),
        user_state.toBuffer(),
        admin.publicKey.toBuffer(),
        SEED.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    let treasure_vault =  getAssociatedTokenAddressSync(
      mint_sol,
      admin.publicKey
    );
    let lender_ata =  getAssociatedTokenAddressSync(
      mint_sol,
      borrower.publicKey
    );
    let lender_vault =  getAssociatedTokenAddressSync(
      mint_sol,
      lender.publicKey
    );
    let borrower_ata =  getAssociatedTokenAddressSync(
      mint_sol,
      borrower.publicKey
    );
    let borrower_vault =  getAssociatedTokenAddressSync(
      mint_sol,
      borrower.publicKey
    );

    console.log("global_state", global_state);
    console.log("user_state", user_state);
    console.log("loan_state", loan_state);
    console.log("treasure_vault", treasure_vault);
    console.log("lender_ata", lender_ata);
    console.log("lender_vault", lender_vault);
    console.log("borrower_ata", borrower_ata);
    console.log("borrower_vault", borrower_vault);

    await program.methods.initialize()
    .accountsStrict({
      admin: admin.publicKey,
      globalState: global_state,
      mintSol: mint_sol,
      mintUsdt: mint_usdt,
      treasureVault: treasure_vault   ,
      systemProgram:anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([admin]).rpc();

    let global_state_data = await program.account.globalState.fetch(
      global_state, 
    );

    expect(global_state_data.bumps).to.not.equal(null);

  });

  // it("Register User", async () => {

  //   console.log("lender", lender.publicKey);
  //     await program.methods.registerUser(SEED).accountsStrict({
  //       lender: lender.publicKey,
  //       userState: user_state[0],
  //       mintUsdt: mint_usdt,
  //       lenderAta: lender_ata,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //       systemProgram: SYSTEM_PROGRAM_ID,
  //       tokenProgram: TOKEN_PROGRAM_ID
  //     }).signers([lender]).rpc();

  //     let user_state_data = await program.account.userState.fetch(user_state[0]);
      
  //     expect(user_state_data.wallet).to.equal(lender.publicKey);
  //     expect(user_state_data.activeLoans).to.equal(0);
  //     expect(user_state_data.completedLoans).to.equal(0);
  //     expect(user_state_data.reputationScore).to.equal(0);
  //     expect(user_state_data.bumps).to.not.equal(null);
  //     expect(user_state_data.seed).to.equal(SEED);    
  // });



});
