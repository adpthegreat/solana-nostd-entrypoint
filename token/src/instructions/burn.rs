
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::invoke_signed, ProgramResult};


/// Burns tokens by removing them from an account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub struct Burn<'a> {
    /// Source of the Burn Account
    pub account: &'a NoStdAccountInfo,
    /// Mint Account
    pub mint: &'a NoStdAccountInfo,
    /// Owner of the Token Account
    pub authority: &'a NoStdAccountInfo,
    /// Amount
    pub amount: u64,
}

impl Burn<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.account.to_meta_c(),
            self.mint.to_meta_c(),
            self.authority.to_meta_c_signer(),
        ];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        let mut instruction_data = [0; 9];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 8;
        // Set amount as u64 at offset [1..9]
        instruction_data[1..9].copy_from_slice(&self.amount.to_le_bytes());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: 9,
            program_id: &crate::ID,
        };

        invoke_signed(
            &instruction,
            &[self.account, self.mint, self.authority],
            signers,
        )
    }
}

//in the TokenInstruction, the enum is not actually encoded in the data , it is used in the pack matching arm and pushes the discriminator, arguemnts , etc to the buffer as data

