import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as keys from "./utils/keys"
import { User, Users } from "./config/users";
import { Accounts } from "./config/accounts";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;
  //const user = (program.provider as anchor.AnchorProvider).wallet

  const accts = new Accounts();
  const users = new Users();
  
  it("Is initialized!", async () => {
    const tx = await initializeProgram(
      users.admin,
      accts.yoiuTokenMint.publicKey
    );
});
