
 use core::slice::from_raw_parts;


 use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};
use solana_program::{pubkey::Pubkey};

 use crate::{invoke_signed::{invoke_signed, write_bytes}, ProgramResult, ElgamalPubkey};

use core::mem::MaybeUninit;

const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

 // Instructions

 /// Initialize a new mint for a confidential transfer.
 pub struct InitializeMint<'a> {
     pub mint: &'a NoStdAccountInfo, //nostdaccount
     /// Authority to modify the `ConfidentialTransferMint` configuration and to
     /// approve new accounts.
     pub authority: Option<&'a Pubkey>,
     /// Determines if newly configured accounts must be approved by the
     /// `authority` before they may be used by the user.
     pub auto_approve_new_accounts: bool,
     /// New authority to decode any transfer amount in a confidential transfer.
     pub auditor_elgamal_pubkey: Option<&'a ElgamalPubkey>,
 }

 impl<'a> InitializeMint<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self,  signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // Account metadata
         let account_metas: [AccountMetaC; 1] = [self.mint.to_meta_c()];

         // Instruction data layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1..2]:   initialize_confidential_mint flag (1 byte, isize)
         // -  [2..6]:   authority_Coption_tag (4 bytes, u8)
         // -  [6..38]:  authority (32 bytes, Pubkey)
         // -  [38.39]:  auto_approve_new_accounts (32 bytes, Pubkey)
         // -  [39..43]: auditor_elgamal_pubkey_Coption_tag (4 bytes, u8)
         // -  [43..75]: auditor_elgamal_pubkey (32 bytes, Pubkey)


         let mut instruction_data = [UNINIT_BYTE; 75];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data, &[27]);  

         // initialize confidential mint, set enum value of instruction 
         write_bytes(&mut instruction_data[1..2], &[0]);  

         if let Some(authority) = self.authority {
             // Set authority as Pubkey at offset [2..34] i also have to amke it an account  // self.mint.to_meta_c().pubkey
             write_bytes(&mut instruction_data[2..6], &[1, 0, 0, 0]);
             write_bytes(&mut instruction_data[6..38], authority.as_ref());
         } else {
             write_bytes(&mut instruction_data[2..6], &[0, 0, 0, 0]);
             write_bytes(&mut instruction_data[6..38], &[0; 32]);
         }

         write_bytes(
             &mut instruction_data[38..39],
             &[self.auto_approve_new_accounts as u8],
         );

        if let Some(elgamal_pubkey) = self.auditor_elgamal_pubkey {
             // Set authority as Pubkey at offset [2..34] i also have to amke it an account  // self.mint.to_meta_c().pubkey
             write_bytes(&mut instruction_data[2..6], &[1, 0, 0, 0]);
             write_bytes(&mut instruction_data[6..38], elgamal_pubkey); // yeah i need to fix this 
         } else {
             write_bytes(&mut instruction_data[2..6], &[0, 0, 0, 0]);
             write_bytes(&mut instruction_data[6..38], &[0; 32]);
         }

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 75).as_ptr() },
             accounts_len: account_metas.len() as u64,
             data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint], signer_seeds)
     }
 }

 pub struct UpdateMint<'a> {
     /// Mint Account.
     pub mint: &'a NoStdAccountInfo,
     /// `ConfidentialTransfer` transfer mint authority..
     pub mint_authority: &'a Pubkey,
     /// Determines if newly configured accounts must be approved by the
     /// `authority` before they may be used by the user.
     pub auto_approve_new_accounts: bool,
     /// New authority to decode any transfer amount in a confidential transfer.
     pub auditor_elgamal_pubkey: Option<&'a ElgamalPubkey>,
 }

 impl<'a> UpdateMint<'a> {
     #[inline(always)]
     pub fn invoke(&self) -> ProgramResult {
         self.invoke_signed(&[])
     }

     pub fn invoke_signed(&self, signer_seeds: &[&[&[u8]]]) -> ProgramResult {
         // Account metadata
         let account_metas = [self.mint.to_meta_c()];

         // Instruction data layout:
         // -  [0]: instruction discriminator (1 byte, u8)
         // -  [1..33]: mint_authority (32 bytes, Pubkey)
         // This is because it uses `TokenInstruction::SetAuthority` to update the confidential transfer mint authority.
         //reference - https://docs.rs/spl-token-2022/7.0.0/src/spl_token_2022/extension/confidential_transfer/instruction.rs.html#52

         let mut instruction_data = [UNINIT_BYTE; 33];

         // Set discriminator as u8 at offset [0]
         write_bytes(&mut instruction_data, &[27]);
         // Set mint_authority as Pubkey at offset [1..33]
         write_bytes(&mut instruction_data[1..33], self.mint_authority.as_ref()); //check this 

         let instruction = InstructionC {
             program_id: &crate::ID,
             accounts: account_metas.as_ptr(),
             data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 33).as_ptr() },
            accounts_len: account_metas.len() as u64,
            data_len: instruction_data.len() as u64,
         };

         invoke_signed(&instruction, &[self.mint], signer_seeds)
     }
 }

 

//ConfigureAccount
//ApproveAccount
//EmptyAccount
//Deposit
//Withdraw
//Transfer
// ApplyPendingBalance
//EnableConfidentialCredits
//DisableConfidentialCredits
//EnableNonConfidentialCredits
//DisableNonConfidentialCredits
//TransferWithFee
//ConfigureAccountWithRegistry

 //increase the size of the instruction data