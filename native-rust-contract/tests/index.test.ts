import { expect, test } from "bun:test";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { COUNTER_SIZE, schema } from "./types";
import * as borsh from "borsh";

const CONNECTION_URL = "http://127.0.0.1:8899";

const PROGRAM_ID = new PublicKey(
  "7PEhspPuvqgqw3VQ7uKtJG7rwo53Rt6wUgtszHvmTvX9",
);

let adminAccount = Keypair.generate();
let dataAccount = Keypair.generate();

test("Account is initialized", async () => {
  const connection = new Connection(CONNECTION_URL);

  const txn = await connection.requestAirdrop(
    adminAccount.publicKey,
    1 * LAMPORTS_PER_SOL,
  );
  await connection.confirmTransaction(txn);

  const data = await connection.getAccountInfo(adminAccount.publicKey);

  const lamports =
    await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

  const ix = SystemProgram.createAccount({
    fromPubkey: adminAccount.publicKey,
    lamports,
    space: COUNTER_SIZE,
    programId: PROGRAM_ID,
    newAccountPubkey: dataAccount.publicKey,
  });

  const createAccountTxn = new Transaction();
  createAccountTxn.add(ix);

  const signature = await connection.sendTransaction(createAccountTxn, [
    adminAccount,
    dataAccount,
  ]);

  await connection.confirmTransaction(signature);
  console.log(
    "Account created with signature: ",
    dataAccount.publicKey.toBase58(),
  );

  const dataAccountInfo = await connection.getAccountInfo(
    dataAccount.publicKey,
  );
  const counter = borsh.deserialize(schema, dataAccountInfo?.data);
  console.log("Count: ", counter?.count);

  expect(counter?.count).toBe(0);
});
