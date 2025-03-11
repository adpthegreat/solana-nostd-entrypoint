
use solana_nostd_entrypoint::{InstructionC, NoStdAccountInfo};
use solana_program::{pubkey::Pubkey};

 use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_BYTE}, ProgramResult};

 pub struct InitializeGroupMemberPointer<'a> {
   // The mint we want to apply the GroupMemberPointer
   mint: &'a NoStdAccountInfo,
   // The group member pointer authority.
   authority: Option<&'a Pubkey>,
   // The new account address that holds the group
   member_address : Option<&'a Pubkey>,
 }


 impl<'a> InitializeGroupMemberPointer<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // Instruction data layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1]: initialize_group_member_pointer_flag (1 byte, u8)
         // -  [2..6]:   group_member_pointer_authority Coption tag (4 bytes, u8)
         // -  [6..38]:  group_member_pointer_authority (32 bytes, Pubkey)
         // -  [38..42]: member_address Coption tag (4 bytes, u8)
         // -  [42..74]: member_address (32 bytes, Pubkey)
         let account_metas = [  
             self.mint.to_meta_c(),
         ];

         let mut instruction_data = [UNINIT_BYTE; 75];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[41]);

         //initialize_group_member_pointer enum flag
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
        if let Some(member_address) = self.member_address {
            write_bytes(&mut instruction_data[38..42],&[1,0,0,0] );
            write_bytes(&mut instruction_data[42..74], member_address.as_ref());
        } else {
            write_bytes(&mut instruction_data[38..42], &[0,0,0,0]);
            write_bytes(&mut instruction_data[42..74], &[0; 32]);
        }

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 73).as_ptr() },
            accounts_len: 1,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint], signer_seeds)?;

         Ok(())
     }
 }

 pub struct UpdateGroupMemberPointer<'a> {
   // The mint we want to update 
   mint: &'a NoStdAccountInfo,
   // The authority of the mint
   authority: &'a NoStdAccountInfo,
   // The new account address that holds the group
   member_address : Option<&'a Pubkey>,
 }

 impl<'a> UpdateGroupMemberPointer<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // Instruction data layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1]: initialize_group_member_pointer_flag (1 byte, u8)
         // -  [2..6]:  member_address Coption tag (4 bytes, u8)
         // -  [6..38]: member_address (32 bytes, Pubkey)

         let account_metas = [
             self.mint.to_meta_c(),
             //because there is no list of signers
             self.authority.to_meta_c_signer(),
         ];

         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[41]);

         write_bytes(&mut instruction_data[1..2], &[1]);

         // set the member_address
        if let Some(member_address) = self.member_address {
             write_bytes(&mut instruction_data[2..6], &[1,0,0,0]);
             write_bytes(&mut instruction_data[6..38], member_address.as_ref());
        } else {
             write_bytes(&mut instruction_data[2..6], &[0,0,0,0]);
             write_bytes(&mut instruction_data[6..38], &[0; 32]);
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


