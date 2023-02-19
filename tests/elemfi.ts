import { assert } from "chai";
import { setProvider, AnchorProvider } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { ConnectedWallet, ElemFiSDK, Realm } from "@elemfi/sdk";
import { signTransaction } from "./useWallet";

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

    signTransaction(tx);
    const signature = await provider.connection.sendTransaction(tx);
    await wallet.confirmTransaction(signature);

    const loadedRealm = await sdk.loadRealm(realm.address);
    assert.deepEqual(loadedRealm.authority, wallet.address);

    const realms = await sdk.loadRealms();
    assert.equal(realms.length, 1);

    const myRealms = await sdk.loadRealmsByAuthority(wallet.address);
    assert.equal(myRealms.length, 1);
  });
});
