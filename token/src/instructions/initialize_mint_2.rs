
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::invoke_signed, ProgramResult};


/// Thaw a Frozen account using the Mint's freeze_authority
///
/// ### Accounts:
///   0. `[WRITE]` The account to thaw.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub struct ThawAccount<'a> {
    /// Token Account to thaw.
    pub account: &'a NoStdAccountInfo,
    /// Mint Account.
    pub mint: &'a NoStdAccountInfo,
    /// Mint Freeze Authority Account
    pub freeze_authority: &'a NoStdAccountInfo,
}

impl ThawAccount<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.account.to_meta_c(),
            self.mint.to_meta_c(),
            self.freeze_authority.to_meta_c_signer(),
        ];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        let mut instruction_data = [0; 1];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 11;

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: 1,
            program_id: &crate::ID,
        };

        invoke_signed(
            &instruction,
            &[self.account, self.mint, self.freeze_authority],
            signers,
        )
    }
}