import { Connection, Keypair, LAMPORTS_PER_SOL, Message, VersionedTransaction } from "@solana/web3.js";
import { url } from "./urls";

test('forwardTransaction', async () => {
  const connection = new Connection(url, 'confirmed');
  const payer = Keypair.generate();

  await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
  const recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
  const versionedTx = new VersionedTransaction(
    new Message({
      header: {
        numRequiredSignatures: 1,
        numReadonlySignedAccounts: 0,
        numReadonlyUnsignedAccounts: 0,
      },
      recentBlockhash,
      instructions: [],
      accountKeys: [payer.publicKey.toBase58()],
    }),
  );

  versionedTx.sign([payer]);
  await connection.sendTransaction(versionedTx);
});