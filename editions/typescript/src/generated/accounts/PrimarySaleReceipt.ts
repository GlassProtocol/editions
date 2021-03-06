/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'

/**
 * Arguments used to create {@link PrimarySaleReceipt}
 * @category Accounts
 * @category generated
 */
export type PrimarySaleReceiptArgs = {
  buyer: web3.PublicKey
  mint: web3.PublicKey
  primarySalePrice: beet.bignum
  saleQuantity: beet.bignum
  createdAt: beet.bignum
}

export const primarySaleReceiptDiscriminator = [
  41, 192, 32, 184, 198, 120, 3, 45,
]
/**
 * Holds the data for the {@link PrimarySaleReceipt} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class PrimarySaleReceipt implements PrimarySaleReceiptArgs {
  private constructor(
    readonly buyer: web3.PublicKey,
    readonly mint: web3.PublicKey,
    readonly primarySalePrice: beet.bignum,
    readonly saleQuantity: beet.bignum,
    readonly createdAt: beet.bignum
  ) {}

  /**
   * Creates a {@link PrimarySaleReceipt} instance from the provided args.
   */
  static fromArgs(args: PrimarySaleReceiptArgs) {
    return new PrimarySaleReceipt(
      args.buyer,
      args.mint,
      args.primarySalePrice,
      args.saleQuantity,
      args.createdAt
    )
  }

  /**
   * Deserializes the {@link PrimarySaleReceipt} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [PrimarySaleReceipt, number] {
    return PrimarySaleReceipt.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link PrimarySaleReceipt} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey
  ): Promise<PrimarySaleReceipt> {
    const accountInfo = await connection.getAccountInfo(address)
    if (accountInfo == null) {
      throw new Error(`Unable to find PrimarySaleReceipt account at ${address}`)
    }
    return PrimarySaleReceipt.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'GeTddTEvfE8My8HNbnSeS1o2tzyn9Z4S194tCUZPbQ8Y'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, primarySaleReceiptBeet)
  }

  /**
   * Deserializes the {@link PrimarySaleReceipt} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [PrimarySaleReceipt, number] {
    return primarySaleReceiptBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link PrimarySaleReceipt} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return primarySaleReceiptBeet.serialize({
      accountDiscriminator: primarySaleReceiptDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link PrimarySaleReceipt}
   */
  static get byteSize() {
    return primarySaleReceiptBeet.byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link PrimarySaleReceipt} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      PrimarySaleReceipt.byteSize,
      commitment
    )
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link PrimarySaleReceipt} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === PrimarySaleReceipt.byteSize
  }

  /**
   * Returns a readable version of {@link PrimarySaleReceipt} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      buyer: this.buyer.toBase58(),
      mint: this.mint.toBase58(),
      primarySalePrice: (() => {
        const x = <{ toNumber: () => number }>this.primarySalePrice
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      saleQuantity: (() => {
        const x = <{ toNumber: () => number }>this.saleQuantity
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      createdAt: (() => {
        const x = <{ toNumber: () => number }>this.createdAt
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const primarySaleReceiptBeet = new beet.BeetStruct<
  PrimarySaleReceipt,
  PrimarySaleReceiptArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['buyer', beetSolana.publicKey],
    ['mint', beetSolana.publicKey],
    ['primarySalePrice', beet.u64],
    ['saleQuantity', beet.u64],
    ['createdAt', beet.i64],
  ],
  PrimarySaleReceipt.fromArgs,
  'PrimarySaleReceipt'
)
