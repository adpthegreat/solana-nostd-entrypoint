
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::pubkey::Pubkey;

use crate::{invoke_signed::invoke_signed, ProgramResult};



/// Initialize a new Token Account.
///
/// ### Accounts:
///   0. `[WRITE]`  The account to initialize.
///   1. `[]` The mint this account will be associated with.
///   3. `[]` Rent sysvar
pub struct InitializeAccount2<'a> {
    /// New Account.
    pub account: &'a NoStdAccountInfo,
    /// Mint Account.
    pub mint: &'a NoStdAccountInfo,
    /// Rent Sysvar Account
    pub rent_sysvar: &'a NoStdAccountInfo,
    /// Owner of the new Account.
    pub owner: &'a Pubkey,
}

impl InitializeAccount2<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.account.to_meta_c(),
            self.mint.to_meta_c(),
            self.rent_sysvar.to_meta_c(),
        ];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..33]: owner (32 bytes, Pubkey)
        let mut instruction_data = [0; 33];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 16;
        // Set owner as [u8; 32] at offset [1..33]
        instruction_data[1..33].copy_from_slice(self.owner.as_ref());

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: 33,
            program_id: &crate::ID,
        };

        invoke_signed(
            &instruction,
            &[self.account, self.mint, self.rent_sysvar],
            signers,
        )
    }
}