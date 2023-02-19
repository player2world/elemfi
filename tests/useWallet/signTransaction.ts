import fs from "fs";
import { Keypair, VersionedTransaction } from "@solana/web3.js";

export function signTransaction(tx: VersionedTransaction) {
  tx.sign([
    Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(fs.readFileSync(process.env.ANCHOR_WALLET as string, { encoding: "utf8" })))
    ),
  ]);
}
