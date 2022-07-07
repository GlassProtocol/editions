/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category UpdateTokenParameters
 * @category generated
 */
export type UpdateTokenParametersInstructionArgs = {
  maxSupply: beet.COption<number>
  primarySaleTokenLimit: beet.COption<number>
  primarySalePrice: beet.COption<beet.bignum>
}
/**
 * @category Instructions
 * @category UpdateTokenParameters
 * @category generated
 */
export const updateTokenParametersStruct = new beet.FixableBeetArgsStruct<
  UpdateTokenParametersInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['maxSupply', beet.coption(beet.u32)],
    ['primarySaleTokenLimit', beet.coption(beet.u32)],
    ['primarySalePrice', beet.coption(beet.u64)],
  ],
  'UpdateTokenParametersInstructionArgs'
)
/**
 * Accounts required by the _updateTokenParameters_ instruction
 *
 * @property [_writable_, **signer**] payer
 * @property [_writable_, **signer**] platformSigner
 * @property [_writable_] tokenParameters
 * @property [] platformSettings
 * @category Instructions
 * @category UpdateTokenParameters
 * @category generated
 */
export type UpdateTokenParametersInstructionAccounts = {
  payer: web3.PublicKey
  platformSigner: web3.PublicKey
  tokenParameters: web3.PublicKey
  platformSettings: web3.PublicKey
  systemProgram?: web3.PublicKey
}

export const updateTokenParametersInstructionDiscriminator = [
  17, 112, 139, 165, 164, 193, 12, 228,
]

/**
 * Creates a _UpdateTokenParameters_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category UpdateTokenParameters
 * @category generated
 */
export function createUpdateTokenParametersInstruction(
  accounts: UpdateTokenParametersInstructionAccounts,
  args: UpdateTokenParametersInstructionArgs,
  programId = new web3.PublicKey('GeTddTEvfE8My8HNbnSeS1o2tzyn9Z4S194tCUZPbQ8Y')
) {
  const [data] = updateTokenParametersStruct.serialize({
    instructionDiscriminator: updateTokenParametersInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.platformSigner,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.tokenParameters,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.platformSettings,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
