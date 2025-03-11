use solana_nostd_entrypoint::{
     InstructionC,
    NoStdAccountInfo,
};

use crate::{invoke_signed::invoke_signed, ProgramResult};

pub struct InitializeImmutableOwner<'a> {
    pub token_program: &'a NoStdAccountInfo,
    pub account: &'a NoStdAccountInfo,
}

impl InitializeImmutableOwner<'_> {
    pub fn invoke_signed(
        &self,
        signer_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let account_metas = [self.account.to_meta_c()];

        let instruction_data = &[17];

        let instruction = InstructionC {
            program_id: self.token_program.key(),
            accounts: account_metas.as_ptr(),
            accounts_len: account_metas.len() as u64,
            data: instruction_data.as_ptr(),
            data_len: instruction_data.len() as u64,
        };

        invoke_signed(&instruction, &[self.account], signer_seeds)
    }
}
