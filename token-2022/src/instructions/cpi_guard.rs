
 use core::slice::from_raw_parts;
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::pubkey::Pubkey;

 use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_BYTE}, ProgramResult};

 
//  use crate::{write_bytes, TOKEN_2022_PROGRAM_ID, UNINIT_BYTE};

 pub struct CpiGuard {
     /// Lock privileged token operations from happening via CPI
     pub lock_cpi: bool,
 }

 // Instructions
 pub struct EnableCpiGuard<'a> {
     /// Account to enable the CPI guard
     pub account: &'a NoStdAccountInfo,
     /// The account's owner
     pub account_owner: &'a NoStdAccountInfo,
 }

 impl<'a> EnableCpiGuard<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {

         let account_metas = [
             self.account.to_meta_c(),
             self.account_owner.to_meta_c_signer(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[34]);

         // Enable the CPI guard
         write_bytes(&mut instruction_data[1..2], &[0]);

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 2).as_ptr() },
            accounts_len: 2,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.account, self.account_owner], signer_seeds)?;

         Ok(())
     }
 }

 pub struct DisableCpiGuard<'a> {
     /// Account to disable the CPI guard
     pub account: &'a NoStdAccountInfo,
     /// The account's owner
     pub account_owner: &'a NoStdAccountInfo,
 }

 impl<'a> DisableCpiGuard<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {

         let account_metas = [
             self.account.to_meta_c(),
             self.account_owner.to_meta_c_signer(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[34]);

         // Disable the CPI guard
         write_bytes(&mut instruction_data[1..2], &[1]);

         let instruction = InstructionC {
             program_id: &crate::ID, //as *const Pubkey,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 2).as_ptr() },
            accounts_len: 2,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.account, self.account_owner], signer_seeds)?;

         Ok(())
     }
 }


