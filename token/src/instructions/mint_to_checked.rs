
use solana_nostd_entrypoint::{AccountInfoC, AccountMetaC, InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::invoke_signed, ProgramResult};


/// Mints new tokens to an account.
///
/// ### Accounts:
///   0. `[WRITE]` The mint.
///   1. `[WRITE]` The account to mint tokens to.
///   2. `[SIGNER]` The mint's minting authority.
///
pub struct MintToChecked<'a> {
    /// Mint Account.
    pub mint: &'a NoStdAccountInfo,
    /// Token Account.
    pub account: &'a NoStdAccountInfo,
    /// Mint Authority
    pub mint_authority: &'a NoStdAccountInfo,
    /// Amount
    pub amount: u64,
    /// Decimals
    pub decimals: u8,
}

impl MintToChecked<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.mint.to_meta_c(),
            self.account.to_meta_c(),
            self.mint_authority.to_meta_c_signer(),
        ];

        // Instruction data layout:
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        // -  [9]: decimals (1 byte, u8)
        let mut instruction_data = [0; 10];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 14;
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
            &[self.mint, self.account, self.mint_authority],
            signers,
        )
    }
}