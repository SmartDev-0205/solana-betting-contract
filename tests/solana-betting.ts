import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getMint, getAccount, Mint, Account, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { HamsterBet } from "../target/types/hamster_bet";
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';

import {
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  createMint,
  transfer,
  mintTo
} from "@solana/spl-token"
import { bytes } from "@project-serum/anchor/dist/cjs/utils";

const raceSeed = Buffer.from("RACE_TAG");
const valutSeed = Buffer.from("TOKEN_VAULT_TAG");
const RACEID = 245385;
const systemProgram = anchor.web3.SystemProgram.programId;
const tokenProgram = TOKEN_PROGRAM_ID;
const rent = anchor.web3.SYSVAR_RENT_PUBKEY;
const clock = anchor.web3.SYSVAR_CLOCK_PUBKEY;


const pepeTokenMint = new anchor.web3.PublicKey("Gjqcz9iibYYUXf2dEpWD8yCz3aaFvbNxwTPzRutk3H9e");

export const pda = (
  seeds: (Buffer | Uint8Array)[],
  programId: anchor.web3.PublicKey
): anchor.web3.PublicKey => {
  const [pdaKey] = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,
    programId
  );
  return pdaKey;
}

describe("Add Hamster", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  let connection = provider.connection;
  const program = anchor.workspace.HamsterBet as Program<HamsterBet>;
  it("Is initialized!", async () => {
    console.log("provider.wallet.publicKey",provider.wallet.publicKey);
    const raceId = new anchor.BN(RACEID);
    const racePDA = await pda([raceSeed,raceId.toArrayLike(Buffer, 'le', 8)], program.programId);
    const token_valutPDA = await pda([valutSeed, racePDA.toBuffer(), pepeTokenMint.toBuffer()], program.programId);
    const txid = await program.methods.createRace({raceId}).accounts({
      authority: provider.wallet.publicKey,
      race: racePDA,
      tokenMint: pepeTokenMint,
      tokenVault: token_valutPDA,
      tokenProgram,
      systemProgram,
      rent,
      clock
    }).rpc({skipPreflight: true});
    console.log(txid);
  });
});
// describe("Create Race", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   let connection = provider.connection;
//   const program = anchor.workspace.HamsterBet as Program<HamsterBet>;
//   it("Is initialized!", async () => {
//     console.log("provider.wallet.publicKey",provider.wallet.publicKey);
//     const raceId = new anchor.BN(RACEID);
//     const racePDA = await pda([raceSeed,raceId.toArrayLike(Buffer, 'le', 8)], program.programId);
//     const token_valutPDA = await pda([valutSeed, racePDA.toBuffer(), pepeTokenMint.toBuffer()], program.programId);
//     const txid = await program.methods.createRace({raceId}).accounts({
//       authority: provider.wallet.publicKey,
//       race: racePDA,
//       tokenMint: pepeTokenMint,
//       tokenVault: token_valutPDA,
//       tokenProgram,
//       systemProgram,
//       rent,
//       clock
//     }).rpc({skipPreflight: true});
//     console.log(txid);
//   });
// });
  