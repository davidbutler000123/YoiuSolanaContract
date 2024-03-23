import * as anchor from "@project-serum/anchor";
import { Escrow } from "../target/types/escrow";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace
  .Escrow as anchor.Program<Escrow>;

export const getProgram = () => {
  return program;
};
