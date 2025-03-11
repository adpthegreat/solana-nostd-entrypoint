
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::entrypoint::ProgramResult;

use crate::invoke_signed::invoke_signed;

/// Creates an associated token account for the given wallet address and
/// token mint, if it doesn't already exist.  Returns an error if the
/// account exists, but with a different owner.
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Funding account (must be a system account)
///   1. `[WRITE]` Associated token account address to be created
///   2. `[]` Wallet address for the new associated token account
///   3. `[]` The token mint for the new associated token account
///   4. `[]` System program
///   5. `[]` SPL Token program
pub struct CreateIdempotent<'a> {
    /// Funding account (must be a system account)
    pub funding_account: &'a NoStdAccountInfo,
    /// Associated token account address to be created
    pub account: &'a NoStdAccountInfo,
    /// Wallet address for the new associated token account
    pub wallet: &'a NoStdAccountInfo,
    /// The token mint for the new associated token account
    pub mint: &'a NoStdAccountInfo,
    /// System program
    pub system_program: &'a NoStdAccountInfo,
    /// SPL Token program
    pub token_program: &'a NoStdAccountInfo,
}

impl CreateIdempotent<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMetaC; 6] = [
            self.funding_account.to_meta_c_signer(),
            self.account.to_meta_c(),
            self.wallet.to_meta_c(),
            self.mint.to_meta_c(),
            self.system_program.to_meta_c(),
            self.token_program.to_meta_c(),
        ];

        // Instruction data:
        // - [0]: Instruction discriminator (1 byte, u8) (1 for CreateIdempotent)

        let instruction_data = [1u8];

        let instruction = InstructionC {
            accounts: account_metas.as_ptr(),
            accounts_len: 6,
            data: instruction_data.as_ptr(),
            data_len: 1,
            program_id: &crate::ID,
        };

        invoke_signed(
            &instruction,
            &[
                self.funding_account,
                self.account,
                self.wallet,
                self.mint,
                self.system_program,
                self.token_program,
            ],
            signers,
        )
    }
}