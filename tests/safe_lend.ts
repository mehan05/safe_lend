import { program } from "./../node_modules/@solana/codecs-data-structures/node_modules/commander/typings/index.d";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Account,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAccount,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { expect } from "chai";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { SafeLend } from "../target/types/safe_lend";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

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
  mint_acc: anchor.web3.PublicKey,
  offcurve = false,
  owner: anchor.web3.Keypair
): Promise<Account> => {
  try {
    const ata = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mint_acc,
      owner.publicKey,
      offcurve
    );

    return ata;
  } catch (error) {
    console.log("Problem in ata", error);
  }
};

const mintTo = async (
  payer: anchor.web3.Keypair,
  mint_acc: anchor.web3.PublicKey,
  to_acc: anchor.web3.PublicKey,
  amount: number
) => {
  await mintToChecked(
    provider.connection,
    payer,
    mint_acc,
    to_acc,
    payer.publicKey,
    amount * anchor.web3.LAMPORTS_PER_SOL,
    6
  );

  console.log("minted");
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
  let treasure_vault: any;
  let lender_ata: Account;
  let lender_vault: any;
  let borrower_ata: Account;
  let borrower_vault: any;

  const program = anchor.workspace.safeLend as Program<SafeLend>;

  before("setting accounts", async () => {
    lender = anchor.web3.Keypair.generate();
    borrower = anchor.web3.Keypair.generate();
    admin = anchor.web3.Keypair.generate();

    await airDrop(lender.publicKey, 10);
    await airDrop(borrower.publicKey, 10);
    await airDrop(admin.publicKey, 10);

    mint_sol = await mint_account(borrower);
    mint_usdt = await mint_account(lender);

    global_state = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global_state"), admin.publicKey.toBuffer()],
      program.programId
    )[0];

    user_state = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("lender"),
        lender.publicKey.toBuffer(),
        SEED.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    loan_state = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("loan"),
        user_state.toBuffer(),
        SEED.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    treasure_vault = await getAssociatedTokenAddress(
      mint_usdt,
      global_state,
      true
    );

    lender_ata = await ata_accounts(lender, mint_usdt, false, lender);

    lender_vault = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        lender,
        mint_usdt,
        user_state,
        true
      )
    ).address;

    borrower_ata = await ata_accounts(borrower, mint_sol, false, borrower);

    borrower_vault = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        borrower,
        mint_sol,
        user_state,
        true
      )
    ).address;
  });

  it("Initialize Lending Pool", async () => {
    // Add your test here.

    // console.log("global_state", global_state);
    // console.log("user_state", user_state);
    // console.log("loan_state", loan_state);
    // console.log("treasure_vault", treasure_vault);
    // console.log("lender_ata", lender_ata.address);
    // console.log("lender_vault", lender_vault);
    // console.log("borrower_vault", borrower_vault);
    // console.log("borrower_ata", borrower_ata);

    await program.methods
      .initializeLend()
      .accountsStrict({
        admin: admin.publicKey,
        globalState: global_state,
        mintSol: mint_sol,
        mintUsdt: mint_usdt,
        treasureVault: treasure_vault,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    let global_state_data = await program.account.globalState.fetch(
      global_state
    );

    expect(global_state_data.bumps).to.not.equal(null);
  });

  it("Register User", async () => {
    console.log("lender", lender.publicKey);
    await program.methods
      .registerUser(SEED)
      .accountsStrict({
        lender: lender.publicKey,
        userState: user_state,
        mintUsdt: mint_usdt,
        lenderAta: lender_ata.address,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([lender])
      .rpc();

    let user_state_data = await program.account.userState.fetch(user_state);

    expect(user_state_data.wallet.toBase58()).to.equal(
      lender.publicKey.toBase58()
    );
    expect(user_state_data.activeLoans.toString()).to.equal("0");
    expect(user_state_data.completedLoans.toString()).to.equal("0");
    expect(user_state_data.reputationScore.toString()).to.equal("0");
    expect(user_state_data.seed.toString()).to.equal("1");
  });

  it("Initialize Lend", async () => {
    await mintTo(lender, mint_usdt, lender_ata.address, 10);
    await mintTo(borrower, mint_sol, borrower_ata.address, 10);

    let initial_lender_ata_info = await getAccount(
      provider.connection,
      lender_ata.address
    );

    let initial_lender_vault_info = await getAccount(
      provider.connection,
      lender_vault
    );

    await program.methods
      .listLend(SEED, new anchor.BN(2), new anchor.BN(3600))
      .accountsStrict({
        lender: lender.publicKey,
        loanState: loan_state,
        userState: user_state,
        mintUsdt: mint_usdt,
        lendVault: lender_vault,
        lenderAta: lender_ata.address,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .signers([lender])
      .rpc();

    let loan_state_data = await program.account.loanState.fetch(loan_state);

    let lender_ata_info = await getAccount(
      provider.connection,
      lender_ata.address
    );

    let lender_vault_info = await getAccount(provider.connection,lender_vault);

    expect(BigInt(lender_ata_info.amount)).to.equal(
      initial_lender_ata_info.amount - BigInt(2)
    );

    expect(BigInt(lender_vault_info.amount)).to.equal(initial_lender_vault_info.amount+BigInt(2));

    expect(loan_state_data.collateralAmount.toString()).to.equal("5");
    expect(loan_state_data.duration.toString()).to.equal("3600");
  });
});
