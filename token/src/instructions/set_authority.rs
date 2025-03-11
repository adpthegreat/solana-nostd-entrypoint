
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::pubkey::Pubkey;

use crate::{invoke_signed::invoke_signed, ProgramResult};



#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AuthorityType {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}

/// Sets a new authority of a mint or account.
///
/// ### Accounts:
///   0. `[WRITE]` The mint or account to change the authority of.
///   1. `[SIGNER]` The current authority of the mint or account.
pub struct SetAuthority<'a> {
    /// Account (Mint or Token)
    pub account: &'a NoStdAccountInfo,

    /// Authority of the Account.
    pub authority: &'a NoStdAccountInfo,

    /// The type of authority to update.
    pub authority_type: AuthorityType,

    /// The new authority
    pub new_authority: Option<&'a Pubkey>,
}

impl SetAuthority<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 2] =
            [self.account.to_meta_c(), self.authority.to_meta_c_signer()];

        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1]: authority_type (1 byte, u8)
        // -  [2]: new_authority presence flag (1 byte, AuthorityType)
        // -  [3..35] new_authority (optional, 32 bytes, Pubkey)
        let mut instruction_data = [0; 35];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 6;
        // Set authority_type as u8 at offset [1]
        instruction_data[1] = self.authority_type as u8;
        // Set new_authority as [u8; 32] at offset [2..35]
        if let Some(new_authority) = self.new_authority {
            instruction_data[2] = 1;
            instruction_data[2..35].copy_from_slice(new_authority.as_ref());
        } else {
            instruction_data[2] = 0;
        }

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 2,
            data: instruction_data.as_ptr(),
            data_len: 35,
            program_id: &crate::ID,
        };

        invoke_signed(&instruction, &[self.account, self.authority], signers)
    }
}