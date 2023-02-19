import fs from "fs";
import { setProvider, AnchorProvider } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { ConnectedWallet, ElemFiSDK, Realm } from "@elemfi/sdk";

const WALLET_KEYPAIR = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(process.env.ANCHOR_WALLET as string, { encoding: "utf8" })))
);

describe("elemfi", () => {
  const provider = AnchorProvider.env();
  const wallet = new ConnectedWallet(provider.connection, provider.publicKey);
  const sdk = new ElemFiSDK(provider);
  setProvider(provider);

  it("should create a realm", async () => {
    const { tx, realm } = await Realm.create(sdk.program, wallet, {
      realmKP: Keypair.generate(),
      delegator: Keypair.generate().publicKey,
      approver: Keypair.generate().publicKey,
      escrowCollection: null,
    });

    tx.sign([WALLET_KEYPAIR]);
    const signature = await provider.connection.sendTransaction(tx);
    await wallet.confirmTransaction(signature);
  });
});
