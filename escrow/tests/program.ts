import * as anchor from "@project-serum/anchor";
import { EscrowTier } from "../target/types/escrow_tier";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace
  .EscrowTier as anchor.Program<EscrowTier>;

export const getProgram = () => {
  return program;
};
