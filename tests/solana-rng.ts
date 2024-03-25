import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaRng } from "../target/types/solana_rng";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { SYSVAR_SLOT_HASHES_PUBKEY } from "@solana/web3.js";
import { BN } from "bn.js";

const key = require("/Users/raidr/.config/solana/id.json");
let secretKey = Uint8Array.from(key);
let signer = Keypair.fromSecretKey(secretKey);

describe("solana-rng", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaRng as Program<SolanaRng>;

  const userState = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state"), signer.publicKey.toBytes()],
    program.programId
  )[0];

  const state = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), signer.publicKey.toBytes()],
    program.programId
  )[0];

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), state.toBytes()],
    program.programId
  )[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeUserState().accounts({
      user: signer.publicKey,
      userState: userState,
      state,
      vault,
      systemProgram: SystemProgram.programId,
    })
    .signers([signer])
    .rpc()
    .then(confirmTx);
    // console.log("Your transaction signature", tx);
  });

  it("Buy Ticket!", async () => {
    // Add your test here.
    const tx = await program.methods.buyTicket(new BN(.05 * LAMPORTS_PER_SOL)).accounts({
      user: signer.publicKey,
      state,
      vault,
      slotHashes: SYSVAR_SLOT_HASHES_PUBKEY,
      userState: userState,
      systemProgram: SystemProgram.programId,
    })
    .signers([signer])
    .rpc()
    .then(confirmTx);
    // console.log("Your transaction signature", tx);
    console.log(`Success! Checkout your *Buy Ticket* here:
    https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });

  it("Settle Bet!", async () => {
    // Add your test here.
    const tx = await program.methods.settleBet().accounts({
      user: signer.publicKey,
      state,
      vault,
      slotHashes: SYSVAR_SLOT_HASHES_PUBKEY,
      userState: userState,
      systemProgram: SystemProgram.programId,
    })
    .signers([signer])
    .rpc()
    .then(confirmTx);
    // console.log("Your transaction signature", tx);
    console.log(`Success! Checkout your *Settle Bet* here:
    https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  });

  // it("Generate Randomness!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.generateRandomNumber().accounts({
  //     slotHashes: SYSVAR_SLOT_HASHES_PUBKEY,
  //     userState: userState,
  //     systemProgram: SystemProgram.programId,
  //   })
  //   .signers([signer])
  //   .rpc()
  //   .then(confirmTx);
  //   // console.log("Your transaction signature", tx);
  // });

//   it("FETCHING STATS!", async () => {
//     const userData = await program.account.userState.fetch(userState)

// console.log("User: ", userData.userKey.toBase58())
// console.log("State Account: ", userState.toString())
// console.log("R N G: ", userData.lastNumberGenerated)
//   });

  const confirmTx = async (signature: string) => {
    const latestBlockhash = await anchor
      .getProvider()
      .connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      "confirmed"
    );
    // console.log("Tx Signature:", signature);
    // console.log(`Success! Checkout your *TRANSACTION* here:
    // https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    return signature;
  };

});
