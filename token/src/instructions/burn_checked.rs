
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::invoke_signed, ProgramResult};


/// Burns tokens by removing them from an account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub struct BurnChecked<'a> {
    /// Source of the Burn Account
    pub account: &'a NoStdAccountInfo,
    /// Mint Account
    pub mint: &'a NoStdAccountInfo,
    /// Owner of the Token Account
    pub authority: &'a NoStdAccountInfo,
    /// Amount
    pub amount: u64,
    /// Decimals
    pub decimals: u8,
}

impl BurnChecked<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // Account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.account.to_meta_c(),
            self.mint.to_meta_c(),
            self.authority.to_meta_c_signer(),
        ];

        // Instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        // -  [9]: decimals (1 byte, u8)
        let mut instruction_data = [0; 10];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 15;
        // Set amount as u64 at offset [1..9]
        instruction_data[1..9].copy_from_slice(&self.amount.to_le_bytes());
        // Set decimals as u8 at offset [9]
        instruction_data[9] = self.decimals;

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 3,
            data: instruction_data.as_ptr(),
            data_len: 10,
            program_id: &crate::ID,
        };

        invoke_signed(
            &instruction,
            &[self.account, self.mint, self.authority],
            signers,
        )
    }
}