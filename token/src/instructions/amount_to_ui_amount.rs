use solana_nostd_entrypoint::{
     InstructionC,
    NoStdAccountInfo,
};

use crate::{invoke_signed::invoke_signed, ProgramResult};

pub struct AmountToUiAmount<'a> {
    pub token_program: &'a NoStdAccountInfo,
    pub mint_account: &'a NoStdAccountInfo,
    pub amount: u64
}

impl<'a> AmountToUiAmount<'a> {
    pub fn invoke_signed(
        &self,
        signer_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let account_metas = [self.token_program.to_meta_c(), self.mint_account.to_meta_c()];

        let mut instruction_data  = [0, 8];

        instruction_data[0] = 23;
        //reference - https://docs.rs/spl-token/latest/src/spl_token/instruction.rs.html#1671
        instruction_data[1] = self.amount as u8;
        
        let instruction = InstructionC {
            program_id: self.token_program.key(),
            accounts: account_metas.as_ptr(),
            accounts_len: account_metas.len() as u64,
            data: instruction_data.as_ptr(),
            data_len: 9,
        };

        invoke_signed(&instruction, &[self.token_program, self.mint_account], signer_seeds)
    }
}


// 681
