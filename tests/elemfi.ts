import { assert } from "chai";
import { setProvider, AnchorProvider } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { ConnectedWallet, ElemFiSDK, Realm, Strategy, TokenAmountUtil, Vault } from "@elemfi/sdk";
import { signTransaction } from "./useWallet";

describe("elemfi", () => {
  const provider = AnchorProvider.env();
  setProvider(provider);

  const wallet = new ConnectedWallet(provider.connection, provider.publicKey);
  const sdk = new ElemFiSDK(provider);

  let realm_1: Realm;
  let vault_1: Vault;
  let underlyingToken_1: PublicKey;

  before(async () => {
    const payerKP = Keypair.generate();
    await wallet.confirmTransaction(await provider.connection.requestAirdrop(payerKP.publicKey, LAMPORTS_PER_SOL));
    underlyingToken_1 = await createMint(provider.connection, payerKP, wallet.address, null, 9);
  });

  it("should create a realm", async () => {
    const { tx, realm } = await Realm.create(sdk.program, wallet, {
      // delegator: Keypair.generate().publicKey,
      // approver: Keypair.generate().publicKey,
    });

    signTransaction(tx);
    await wallet.confirmTransaction(await provider.connection.sendTransaction(tx));

    realm_1 = await sdk.loadRealm(realm.address);
    assert.deepEqual(realm_1.authority, wallet.address);

    const realms = await sdk.loadRealms();
    assert.equal(realms.length, 1);

    const myRealms = await sdk.loadRealmsByAuthority(wallet.address);
    assert.equal(myRealms.length, 1);
  });

  it("should create a vault", async () => {
    const { tx, vault } = await Vault.create(realm_1, wallet, {
      collateralSupply: "100",
      collateralMaxSupply: "10000000",
      collateralMinAmount: "1",
      collateralMaxAmount: "100",
      underlyingToken: underlyingToken_1,
      underlyingLiquidity: "110",
      escrowCollection: null,
    });

    signTransaction(tx);
    await wallet.confirmTransaction(await provider.connection.sendTransaction(tx));

    vault_1 = await sdk.loadVault(realm_1, vault.address);
    assert.equal(vault_1.collateralSupply, "100.000000000");
    assert.equal(vault_1.collateralMaxSupply, "10000000.000000000");
    assert.equal(vault_1.collateralMinAmount, "1.000000000");
    assert.equal(vault_1.collateralMaxAmount, "100.000000000");
    assert.equal(vault_1.underlyingLiquidity, "110.000000000");

    const vaults = await sdk.loadVaultsByRealm(realm_1);
    assert.equal(vaults.length, 1);
  });

  it("should create a strategy", async () => {
    const { tx } = await Strategy.create(vault_1, wallet, {
      strategyAuthority: Keypair.generate().publicKey,
      utilizedAmount: "110",
      utilizationMaxAmount: "1000000",
    });

    signTransaction(tx);
    await wallet.confirmTransaction(await provider.connection.sendTransaction(tx));
  });
});
