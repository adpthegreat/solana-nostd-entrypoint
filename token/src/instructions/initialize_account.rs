
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::invoke_signed, ProgramResult};


/// Initialize a new Token Account.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   2. `[]` The new account's owner/multisignature.
///   3. `[]` Rent sysvar
pub struct InitializeAccount<'a> {
    /// New Account.
    pub account: &'a NoStdAccountInfo,
    /// Mint Account.
    pub mint: &'a NoStdAccountInfo,
    /// Owner of the new Account.
    pub owner: &'a NoStdAccountInfo,
    /// Rent Sysvar Account
    pub rent_sysvar: &'a NoStdAccountInfo,
}

impl InitializeAccount<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 4] = [
            self.account.to_meta_c(),
            self.mint.to_meta_c(),
            self.owner.to_meta_c(),
            self.rent_sysvar.to_meta_c(),
        ];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        let mut instruction_data = [0; 1];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 1;

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 4,
            data: instruction_data.as_ptr(),
            data_len: 1,
            program_id: &crate::ID,
        };

        invoke_signed(&instruction, &[self.mint, self.rent_sysvar], signers)
    }
}