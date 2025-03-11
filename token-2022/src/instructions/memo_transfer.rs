

 use solana_nostd_entrypoint::{InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_BYTE}, ProgramResult};
 // State
 pub struct MemoTransfer {
     /// Require transfers into this account to be accompanied by a memo
     pub require_incoming_transfer_memos: bool,
 }
 // Instructions

 pub struct EnableMemoTransfer<'a> {
     /// The account to update.
     pub account: &'a NoStdAccountInfo,
     /// The account owner.
     pub account_owner: &'a NoStdAccountInfo,
 }

 impl<'a> EnableMemoTransfer<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // account metadata
         let account_metas = [
             self.account.to_meta_c(),
             self.account_owner.to_meta_c()
         ];

         // Instruction data Layout
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[30]);

         // Enable incoming transfer memos
         write_bytes(&mut instruction_data[1..2], &[0]);

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 2).as_ptr() },
            accounts_len: account_metas.len() as u64,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.account], signer_seeds)
     }
 }

 pub struct DisableMemoTransfer<'a> {
     /// The account to update.
     pub account: &'a NoStdAccountInfo,
     /// The account owner.
     pub account_owner: &'a NoStdAccountInfo,
 }

 impl<'a> DisableMemoTransfer<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // account metadata
         let account_metas = [
             self.account.to_meta_c(),
             self.account_owner.to_meta_c(),
         ];

         // instruction data
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[30]);
         // Disable incoming transfer memos
         write_bytes(&mut instruction_data[1..2], &[1]);

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
            data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 2).as_ptr() },
            accounts_len: account_metas.len() as u64,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.account, self.account_owner], signer_seeds)
     }
 }

 