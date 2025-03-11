
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use crate::{invoke_signed::invoke_signed, ProgramResult};

/// Approves a delegate.
///
/// ### Accounts:
///   0. `[WRITE]` The token account.
///   1. `[]` The delegate.
///   2. `[SIGNER]` The source account owner.
pub struct Approve<'a> {
    /// Source Account.
    pub source: &'a NoStdAccountInfo,
    /// Delegate Account
    pub delegate: &'a NoStdAccountInfo,
    /// Source Owner Account
    pub authority: &'a NoStdAccountInfo,
    /// Amount
    pub amount: u64,
}

impl Approve<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // Account metadata
        let account_metas: [AccountMetaC; 3] = [
            self.source.to_meta_c(),
            self.delegate.to_meta_c(),
            self.authority.to_meta_c_signer(),
        ];

        // Instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        let mut instruction_data = [0; 9];

        // Set discriminator as u8 at offset [0]
        instruction_data[0] = 4;
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
            &[self.source, self.delegate, self.authority],
            signers,
        )
    }
}