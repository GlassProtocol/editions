import { Program } from "@metaplex-foundation/mpl-core";

import { PublicKey } from "@solana/web3.js";
import * as errors from "./generated/errors";
import * as instructions from "./generated/instructions";
import * as accounts from "./generated/accounts";
import BN from "bn.js";

export class EditionsProgram extends Program {
  static readonly PLATFORM_SETTINGS_PREFIX = "platform_settings";
  static readonly MINT_AUTHORITY_PREFIX = "mint_authority";
  static readonly METADATA_AUTHORITY_PREFIX = "metadata_authority";
  static readonly TOKEN_PARAMETERS_PREFIX = "token_parameters";
  static readonly PRIMARY_SALE_RECEIPT_PREFIX = "primary_sale_receipt";

  static readonly PUBKEY = new PublicKey(
    "GeTddTEvfE8My8HNbnSeS1o2tzyn9Z4S194tCUZPbQ8Y"
  );
  static readonly instructions = instructions;
  static readonly errors = errors;
  static readonly accounts = accounts;

  static readonly TOKEN_PROGRAM_ID = new PublicKey(
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
  );
  static readonly SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID = new PublicKey(
    "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
  );
  static readonly METADATA_PREFIX = "metadata";
  static readonly TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  static async findAssociatedTokenAccountAddress(
    mint: PublicKey,
    wallet: PublicKey
  ): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [wallet.toBuffer(), this.TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
      this.SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
    );
  }

  static async findAdminAddress(): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [Buffer.from(this.PLATFORM_SETTINGS_PREFIX)],
      this.PUBKEY
    );
  }

  static async findMintAuthorityAddress(): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [Buffer.from(this.MINT_AUTHORITY_PREFIX)],
      this.PUBKEY
    );
  }

  static async findMetadataAuthorityAddress(): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [Buffer.from(this.METADATA_AUTHORITY_PREFIX)],
      this.PUBKEY
    );
  }

  static async findMetadataAddress(
    mint: PublicKey
  ): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [
        Buffer.from(this.METADATA_PREFIX),
        this.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      this.TOKEN_METADATA_PROGRAM_ID
    );
  }

  static async findTokenParametersAddress(
    mint: PublicKey
  ): Promise<[PublicKey, number]> {
    return await PublicKey.findProgramAddress(
      [Buffer.from(this.TOKEN_PARAMETERS_PREFIX), mint.toBuffer()],
      this.PUBKEY
    );
  }

  static async findPrimarySaleReceiptAddress(
    payer: PublicKey,
    mint: PublicKey,
    primary_sale_count: number
  ): Promise<[PublicKey, number]> {
    const buf = Buffer.alloc(4);
    buf.writeUint32LE(primary_sale_count);

    return await PublicKey.findProgramAddress(
      [
        Buffer.from(this.PRIMARY_SALE_RECEIPT_PREFIX),
        payer.toBuffer(),
        mint.toBuffer(),
        buf,
      ],
      this.PUBKEY
    );
  }
}
