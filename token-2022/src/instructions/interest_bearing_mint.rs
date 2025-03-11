
use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{pubkey::Pubkey};

 use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_BYTE}, ProgramResult};

 pub struct InitializeInterestBearingMint<'a> {
    mint: &'a NoStdAccountInfo,
   // The interest bearing mint authority
   rate_authority: Option<&'a Pubkey>,
   // The interest rate 
   rate: i16
 }


 impl<'a> InitializeInterestBearingMint<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
            // Instruction data layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1]: initialize_interest_bearing_mint_flag (1 byte, u8)
         // -  [2..6]: rate_authority Coption tag (4 bytes, u8)
         // -  [6..38]: rate_authority (32 bytes, Pubkey)
         // -  [38..40]: rate (2 bytes, i16)
         let account_metas = [
             self.mint.to_meta_c(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 66];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[33]);

         // Set enum position flag for initialize rate instruction
         write_bytes(&mut instruction_data[1..2], &[0]);

         // set the authority if needed
         if let Some(authority) = self.rate_authority {
            write_bytes(&mut instruction_data[2..6], &[1,0,0,0]);
            write_bytes(&mut instruction_data[2..6], authority.as_ref());
         } else {
            write_bytes(&mut instruction_data[2..34], &[0]);
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

 pub struct UpdateRate<'a> {
   // The mint we want to update 
   mint: &'a NoStdAccountInfo,
   // The authority that can set the rate 
   rate_authority: &'a NoStdAccountInfo,
   // The interest rate 
   rate: i16
 }

 impl<'a> UpdateRate<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
             // Instruction data layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1]: update_interest_bearing_mint_rate_flag (1 byte, u8)
         // -  [2..4]: rate (2 bytes, i16)
         let account_metas = [
             self.mint.to_meta_c(),
             self.rate_authority.to_meta_c_signer(),
         ];

         // Instruction data Layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         let mut instruction_data = [UNINIT_BYTE; 2];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data[0..1], &[33]);
        // Set enum position flag for update rate instruction
         write_bytes(&mut instruction_data[1..2], &[1]);    
         // set the interest rate 
         write_bytes(&mut instruction_data[2..4], &self.rate.to_le_bytes());
   
         let instruction = InstructionC {
             program_id: &crate::ID, //as *const Pubkey,
             accounts: account_metas.as_ptr(),
             data: unsafe { core::slice::from_raw_parts(instruction_data.as_ptr() as _, 3).as_ptr() },
            accounts_len: 2,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint, self.rate_authority], signer_seeds)?;

         Ok(())
     }
 }

