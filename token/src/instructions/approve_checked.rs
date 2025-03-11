
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};


use crate::{invoke_signed::invoke_signed, ProgramResult};

/// Approves a delegate.
///
/// ### Accounts:
///   0. `[WRITE]` The source account.
///   1. `[]` The token mint.
///   2. `[]` The delegate.
///   3. `[SIGNER]` The source account owner.
pub struct ApproveChecked<'a> {
    /// Source Account.
    pub source: &'a NoStdAccountInfo,
    /// Mint Account.
    pub mint: &'a NoStdAccountInfo,
    /// Delegate Account.
    pub delegate: &'a NoStdAccountInfo,
    /// Source Owner Account.
    pub authority: &'a NoStdAccountInfo,
    /// Amount.
    pub amount: u64,
    /// Decimals.
    pub decimals: u8,
}

impl ApproveChecked<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // Account metadata
        let account_metas: [AccountMetaC; 4] = [
            self.source.to_meta_c(),
            self.mint.to_meta_c(),
            self.delegate.to_meta_c(),
            self.authority.to_meta_c_signer(),
        ];

        // Instruction data
        // -  [0]  : instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        // -  [9]   : decimals (1 byte, u8)
        let mut instruction_data = [0; 10];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 13;
        // Set amount as u64 at offset [1..9]
        instruction_data[1..9].copy_from_slice(&self.amount.to_le_bytes());
        // Set decimals as u8 at offset [9]
        instruction_data[9] = self.decimals;

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 4,
            data: instruction_data.as_ptr(),
            data_len: 10,
            program_id: &crate::ID,
        };

        invoke_signed(
            &instruction,
            &[self.source, self.mint, self.delegate, self.authority],
            signers,
        )
    }
}