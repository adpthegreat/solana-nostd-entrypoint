 use solana_nostd_entrypoint::{InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_BYTE}, ProgramResult};

// use crate::{state::AccountState,};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccountState {
    /// Account is not yet initialized
    Uninitialized,

    /// Account is initialized; the account owner and/or delegate may perform
    /// permitted operations on this account
    Initialized,

    /// Account has been frozen by the mint freeze authority. Neither the
    /// account owner nor the delegate are able to perform operations on
    /// this account.
    Frozen,
}

 pub struct DefaultAccountState {
     pub state: AccountState,
 }

 pub struct InitializeDefaultAccountState<'a> {
     /// The mint to initialize
     pub mint: &'a NoStdAccountInfo,
     /// Default account state
     pub state: AccountState,
 }

 impl<'a> InitializeDefaultAccountState<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {

         let account_metas = [
             self.mint.to_meta_c(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 3];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data, &[28]);

         //initialize the default account state 
         write_bytes(&mut instruction_data[1..2], &[0]);

         //set the default account state 
         write_bytes(&mut instruction_data[2..3], &[self.state as u8]);

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 3).as_ptr() },
            accounts_len: 1,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint], signer_seeds)?;

         Ok(())
     }
 }

 pub struct UpdateDefaultAccountState<'a> {
     /// The mint to update
     pub mint: &'a NoStdAccountInfo,
     /// The mint's freeze authority
     pub mint_freeze_authority: &'a NoStdAccountInfo,
     /// The new state
     pub new_state: u8,
 }
 
 impl<'a> UpdateDefaultAccountState<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {

         let account_metas = [
             self.mint.to_meta_c(),
             self.mint_freeze_authority.to_meta_c_signer(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 3];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data, &[28]);
        // Update the default account state
         write_bytes(&mut instruction_data[1..2], &[1]);
        // Set the new state 
         write_bytes(&mut instruction_data[2..3], &[self.new_state as u8]);

         let instruction = InstructionC {
             program_id: &crate::ID, //as *const Pubkey,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 3).as_ptr() },
            accounts_len: account_metas.len() as u64,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint, self.mint_freeze_authority], signer_seeds)?;

         Ok(())
     }
 }


