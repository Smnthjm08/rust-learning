import { Keypair } from '@solana/web3.js';
import { LiteSVM } from 'litesvm';
import { test } from 'bun:test';

test("one transfer", ()=>{
    const svm = new LiteSVM();

    const ContractKey = Keypair.generate();

    svm.addProgramFromFile(ContractKey.publicKey, "../cpi-in-solana/target/sbpf-solana-solana/release/cpi_in_solana.so");

    
})