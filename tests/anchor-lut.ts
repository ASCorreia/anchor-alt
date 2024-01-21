import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorLut } from "../target/types/anchor_lut";
import { AddressLookupTableProgram, Keypair, PublicKey, sendAndConfirmTransaction } from "@solana/web3.js";
import { Node } from "typescript";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("anchor-lut", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const providerWallet = provider.wallet as NodeWallet;

  const program = anchor.workspace.AnchorLut as Program<AnchorLut>;

  let lookuptable: PublicKey;

  let newAddress = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("dummy_account")], program.programId)[0];

  it("initialize State Account!", async () => {
    const tx = await program.methods.initializeState().accounts({
      signer: provider.wallet.publicKey,
      dummyAccount: newAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    console.log("\n\nState Account Created - TxID: ", tx);
  })

  it("Initialize Address Lookup Table!", async () => {
    // Add your test here.
    let recentSlot = await provider.connection.getSlot();

    const [_ix, lookupTable] = AddressLookupTableProgram.createLookupTable({
      authority: provider.publicKey,
      payer: provider.publicKey,
      recentSlot
    });

    lookuptable = lookupTable;

    const tx = await program.methods.initialize(new anchor.BN(recentSlot)).accounts({
      authority: provider.wallet.publicKey,
      lookupTable: lookuptable,
      addressLookupTableProgram: AddressLookupTableProgram.programId,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("\n\nAddress Lookup Table Created - TxID: ", tx);
  });

  it("Extend Address Lookup Table!", async () => {
    const tx = await program.methods.extendLut().accounts({
      authority: provider.wallet.publicKey,
      lookupTable: lookuptable,
      newAddress1: newAddress,
      addressLookupTableProgram: AddressLookupTableProgram.programId,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("\n\nAddress Lookup Table Extended with Address ", newAddress.toBase58(), " - TxID: ", tx);
  });

  it("Use lookup table to find address!", async () => {
    const tx = await program.methods.useLut().accounts({
      authority: provider.wallet.publicKey,
      lookupTable: lookuptable,
    })
    .rpc({
      skipPreflight: true,
    });

    console.log("\n\nLookup table find method finished - TxID: ", tx);
  });

  it("Increment and Check account counter!", async() => {
    const lookupTableAccount = (await provider.connection.getAddressLookupTable(lookuptable)).value;

    const ix1 = await program.methods.printCounter().accounts({
      signer: provider.wallet.publicKey,
      dummyAccount: newAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();

    const ix2 = await program.methods.incrementCounter().accounts({
      dummyAccount: newAddress,
    })
    .instruction();

    const ix3 = await program.methods.printCounter().accounts({
      signer: provider.wallet.publicKey,
      dummyAccount: newAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .instruction();

    let blockhash = await provider.connection.getLatestBlockhash().then((res) => res.blockhash);;

    const messageV0 = new anchor.web3.TransactionMessage({
      payerKey: provider.wallet.publicKey,
      recentBlockhash: blockhash,
      instructions: [ix1, ix2, ix3], // note this is an array of instructions
    }).compileToV0Message([lookupTableAccount]);

    // create a v0 transaction from the v0 message
    const transactionV0 = new anchor.web3.VersionedTransaction(messageV0);

    // sign the v0 transaction using the file system wallet we created named `payer`
    transactionV0.sign([providerWallet.payer]);

    // send and confirm the transaction
    // (NOTE: There is NOT an array of Signers here;)
    const txid = await sendAndConfirmTransaction(provider.connection, transactionV0);
    console.log("\n\nTransaction confirmed - TxID: ", txid);
  })

  it("Deactivate Address Lookup Table!", async () => {
    const tx = await program.methods.deactivateLut().accounts({
      authority: provider.wallet.publicKey,
      lookupTable: lookuptable,
      addressLookupTableProgram: AddressLookupTableProgram.programId,
    })
    .rpc({
      skipPreflight: true
    });
    console.log("\n\nAddress Lookup Table Deactivated - TxID: ", tx);
  });

  xit("Close Lookup Table!", async () => {
    const tx = await program.methods.closeLut().accounts({
      authority: provider.wallet.publicKey,
      recipient: provider.wallet.publicKey,
      lookupTable: lookuptable,
      addressLookupTableProgram: AddressLookupTableProgram.programId,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("\n\nAddress Lookup Table Closed - TxID: ", tx);
  });

  it("Close State Account!", async () => {
    const tx = await program.methods.closeState().accounts({
      receiver: provider.wallet.publicKey,
      dummyAccount: newAddress,
    })
    .rpc({
      skipPreflight: true,
    });
    console.log("\n\nState Account Closed - TxID: ", tx);
  });
});
