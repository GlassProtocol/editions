/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  PlatformSettings,
  platformSettingsBeet,
} from '../accounts/PlatformSettings'

/**
 * @category Instructions
 * @category CreatePlatformSettings
 * @category generated
 */
export type CreatePlatformSettingsInstructionArgs = {
  data: PlatformSettings
}
/**
 * @category Instructions
 * @category CreatePlatformSettings
 * @category generated
 */
export const createPlatformSettingsStruct = new beet.BeetArgsStruct<
  CreatePlatformSettingsInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['data', platformSettingsBeet],
  ],
  'CreatePlatformSettingsInstructionArgs'
)
/**
 * Accounts required by the _createPlatformSettings_ instruction
 *
 * @property [_writable_, **signer**] payer
 * @property [_writable_] platformSettings
 * @category Instructions
 * @category CreatePlatformSettings
 * @category generated
 */
export type CreatePlatformSettingsInstructionAccounts = {
  payer: web3.PublicKey
  platformSettings: web3.PublicKey
  systemProgram?: web3.PublicKey
}

export const createPlatformSettingsInstructionDiscriminator = [
  53, 34, 109, 69, 19, 197, 220, 231,
]

/**
 * Creates a _CreatePlatformSettings_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category CreatePlatformSettings
 * @category generated
 */
export function createCreatePlatformSettingsInstruction(
  accounts: CreatePlatformSettingsInstructionAccounts,
  args: CreatePlatformSettingsInstructionArgs,
  programId = new web3.PublicKey('GeTddTEvfE8My8HNbnSeS1o2tzyn9Z4S194tCUZPbQ8Y')
) {
  const [data] = createPlatformSettingsStruct.serialize({
    instructionDiscriminator: createPlatformSettingsInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.platformSettings,
      isWritable: true,
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