

 use solana_nostd_entrypoint::{InstructionC, NoStdAccountInfo};
use solana_program::pubkey::Pubkey;

use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_BYTE}, ProgramResult};
 // Instructions

 pub struct InitializeTransferHook<'a> {
     /// The account to update.
     pub mint: &'a NoStdAccountInfo,
     /// The account authority.
     pub authority: Option<&'a Pubkey>,
    // the program id of the transfer hook program to invoke 
     pub transfer_hook_program_id: Option<&'a Pubkey>,
 }

 impl<'a> InitializeTransferHook<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // account metadata
         let account_metas = [
             self.mint.to_meta_c()
         ];
         // Instruction data Layout
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1]: initialize_transfer_hook_flag (1 byte, u8)
         //-   [2..6]: transfer_hook_authority_Coption_tag (4 bytes, u8)
         //-   [6..38]: transfer_hook_authority (32 bytes, Pubkey)
         //-   [38..42]: transfer_hook_program_id_Coption_tag (4 bytes, u8)
         //-   [42..74]: transfer_hook_program_id (4 bytes, u8)
         let mut instruction_data = [UNINIT_BYTE; 73];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[36]);

         // initialize_transfer_hook_flag (1 byte, u8)
         write_bytes(&mut instruction_data[1..2], &[0]);
        
        // set the transfer_hook_program_id
        if let Some(program_id) = self.transfer_hook_program_id {
             // transfer_hook_authority_Coption (4 bytes, u8)
            write_bytes(&mut instruction_data[2..6], &[1,0,0,0]);
             // transfer_hook_authority
            write_bytes(&mut instruction_data[6..38], program_id.as_ref());
        } else {
             // transfer_hook_authority_Coption (4 bytes, u8)
            write_bytes(&mut instruction_data[2..6], &[0,0,0,0]);
             // transfer_hook_authority
            write_bytes(&mut instruction_data[6..38], &[0; 32]);
        }

         // set the authority if needed
         if let Some(authority) = self.authority {
            // transfer_hook program_id Coption tag 
            write_bytes(&mut instruction_data[38..42], &[1,0,0,0]);
            // transfer_hook program_id 
            write_bytes(&mut instruction_data[42..74], authority.as_ref());
         } else {
            write_bytes(&mut instruction_data[38..42], &[0,0,0,0]);
            write_bytes(&mut instruction_data[42..74], &[0; 32]); 
         };
         
         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 73).as_ptr() },
            accounts_len: account_metas.len() as u64,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint], signer_seeds)
     }
 }

 pub struct UpdateTransferHook<'a> {
     // The account to update.
     pub mint: &'a NoStdAccountInfo,
     // The account authority.
     pub authority: &'a NoStdAccountInfo,
     //  Program id for the transfer_hook_program_id
     pub transfer_hook_program_id:Option<&'a Pubkey>,
 }

 impl<'a> UpdateTransferHook<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // account metadata
         let account_metas = [
             self.mint.to_meta_c(),
             self.authority.to_meta_c(),
         ];

         // Instruction data Layout
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1]: initialize_transfer_hook_flag (1 byte, u8)
         //-   [2..6]: transfer_hook_authority_Coption_tag (4 bytes, u8)
         //-   [6..38]: transfer_hook_authority (32 bytes, Pubkey)
         //-   [38..42]: transfer_hook_program_id_Coption_tag (4 bytes, u8)
         //-   [42..74]: transfer_hook_program_id (4 bytes, u8)
         let mut instruction_data = [UNINIT_BYTE; 38];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[36]);
         // initialize_transfer_hook_flag
         write_bytes(&mut instruction_data[1..2], &[1]);

        if let Some(program_id) = self.transfer_hook_program_id {
             // transfer_hook_authority_Coption (4 bytes, u8)
            write_bytes(&mut instruction_data[2..6], &[1,0,0,0]);
             // transfer_hook_authority
            write_bytes(&mut instruction_data[6..38], program_id.as_ref());
        } else {
             // transfer_hook_authority_Coption (4 bytes, u8)
            write_bytes(&mut instruction_data[2..6], &[0,0,0,0]);
             // transfer_hook_authority
            write_bytes(&mut instruction_data[6..38], &[0; 32]);
        }

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
            data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 38).as_ptr() },
            accounts_len: account_metas.len() as u64,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint, self.authority], signer_seeds)
     }
 }