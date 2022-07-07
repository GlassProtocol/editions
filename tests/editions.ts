import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Transaction, PublicKey, Keypair, TransactionBlockhashCtor } from '@solana/web3.js';
import { EditionsProgram } from "../editions/typescript/src/EditionsProgram";
import { CreatePlatformSettingsInstructionAccounts, CreatePlatformSettingsInstructionArgs, PlatformSettings, PlatformSettingsArgs } from "../editions/typescript/src/generated";
import fs = require('fs');

describe("editions", () => {
  // Configure the client to use the local cluster.

  let provider = anchor.AnchorProvider.env() 


  anchor.setProvider(anchor.AnchorProvider.env());
  let platformSettings: PublicKey;

  const platform_treasury_data = JSON.parse(
    fs.readFileSync('target/deploy/platform-treasury-keypair.json', { encoding: 'utf8' }),
  );
  const platform_treasury = Keypair.fromSecretKey(new Uint8Array(platform_treasury_data));
  console.log('platform_treasury: ', platform_treasury.publicKey.toString());


  const platform_signer_data = JSON.parse(
    fs.readFileSync('target/deploy/platform-signer-keypair.json', { encoding: 'utf8' }),
  );
  const platform_signer = Keypair.fromSecretKey(new Uint8Array(platform_signer_data));
  console.log('platform_signer: ', platform_signer.publicKey.toString());


  const program = EditionsProgram



  const payer = new anchor.Wallet(Keypair.generate());

  it("Is initialized!", async () => {
    // Add your test here.
    [platformSettings] = await program.findPlatformSettingsAddress();

    const args: PlatformSettingsArgs = {
      platformTreasury: platform_treasury.publicKey,
      platformSigner: platform_signer.publicKey,
    };

    const data = PlatformSettings.fromArgs(args);

    data.serialize()

    let instruction = { data } as CreatePlatformSettingsInstructionArgs


    const ix = program.instructions.createCreatePlatformSettingsInstruction(
      { payer: platform_signer.publicKey, platformSettings } as CreatePlatformSettingsInstructionAccounts,
      instruction
    );




    // const accounts: CreatePlatformSettingsInstructionAccounts = {
    //   payer: platform_signer.publicKey,
    //   platformSettings: platformSettings,
    // }

    // console.log(PlatformSettings)



    // const platformSettingsData = PlatformSettings.fromArgs({
    //   platformSigner: platform_signer.publicKey,
    //   platformTreasury: platform_treasury.publicKey,
    // })


    // const args: CreatePlatformSettingsInstructionArgs = {
    //   data: platformSettingsData,
    // }

    // console.log(args)

    // console.log(platformSettingsData.serialize())

    // const [data] = createPlatformSettingsStruct.serialize({
    //   instructionDiscriminator: createPlatformSettingsInstructionDiscriminator,
    //   ...args,
    // })


    // let txInstruction = program.instructions.createCreatePlatformSettingsInstruction(accounts, args)


    const transaction = new Transaction({
      feePayer: platform_signer.publicKey,
    } as TransactionBlockhashCtor);


    transaction.add(ix);

    // let tx = await provider.sendAndConfirm(transaction, [platform_signer]);



  });
});
