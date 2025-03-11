
 use core::slice::from_raw_parts;


use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::pubkey::Pubkey;

 use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_BYTE}, ProgramResult};

 
//  use crate::{write_bytes, TOKEN_2022_PROGRAM_ID, UNINIT_BYTE};

 pub struct InitializeGroupPointer<'a> {
   // The mint we want to apply the GroupPointer
   mint: &'a NoStdAccountInfo,
   // The group pointer authority.
   authority: Option<&'a Pubkey>,
    // The new account address that holds the group
   group_address : Option<&'a Pubkey>,
 }


 impl<'a> InitializeGroupPointer<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // Instruction data layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1]: initialize_group_member_pointer_flag (1 byte, u8)
         // -  [2..6]:   group_pointer Coption tag (4 bytes, u8)
         // -  [6..38]:  group_pointer_authority (32 bytes, Pubkey)
         // -  [38..42]: group_address Coption tag (4 bytes, u8)
         // -  [42..74]: group_address (32 bytes, Pubkey)

         let account_metas = [
             self.mint.to_meta_c(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[41]);

         //
         write_bytes(&mut instruction_data[1..2], &[0]);

           // set the authority if needed
         if let Some(authority) = self.authority {
            write_bytes(&mut instruction_data[2..6], &[1,0,0,0]);
            write_bytes(&mut instruction_data[6..38], authority.as_ref());
         } else {
            write_bytes(&mut instruction_data[2..6], &[0,0,0,0]);
            write_bytes(&mut instruction_data[6..38], &[0; 32]);
         }
         
          // set the member_address
        if let Some(member_address) = self.group_address {
            write_bytes(&mut instruction_data[38..42],&[1,0,0,0] );
            write_bytes(&mut instruction_data[42..74], member_address.as_ref());
        } else {
            write_bytes(&mut instruction_data[38..42], &[0,0,0,0]);
            write_bytes(&mut instruction_data[42..74], &[0; 32]);
        }

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 2).as_ptr() },
            accounts_len: 1,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint], signer_seeds)?;

         Ok(())
     }
 }

 pub struct UpdateGroupPointer<'a> {
   // The mint we want to update 
   mint: &'a NoStdAccountInfo,
   // The authority of the mint
   authority: &'a NoStdAccountInfo,
   //
   group_address : Option<&'a Pubkey>,
 }

 impl<'a> UpdateGroupPointer<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {

         let account_metas = [
             self.mint.to_meta_c(),
             //because there is no list of signers
             self.authority.to_meta_c_signer(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[41]);

         write_bytes(&mut instruction_data[1..2], &[1]);

         // set the group_address
        if let Some(group_address) = self.group_address {
             write_bytes(&mut instruction_data[2..33], group_address.as_ref());
        } else {
            write_bytes(&mut instruction_data[2..33], &[0]);
        }
         let instruction = InstructionC {
             program_id: &crate::ID, //as *const Pubkey,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 2).as_ptr() },
            accounts_len: account_metas.len() as u64,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint, self.authority], signer_seeds)?;

         Ok(())
     }
 }


