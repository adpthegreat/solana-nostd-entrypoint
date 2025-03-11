
use std::mem::MaybeUninit;
use std::slice::from_raw_parts;

use solana_nostd_entrypoint::{AccountMetaC, InstructionC, NoStdAccountInfo};

use crate::{invoke_signed::{invoke_signed, write_bytes, UNINIT_ACC_METAS, UNINIT_BYTE}, ProgramResult};

/// Initialize a multisig account with the given signers.
///
/// ### Accounts:
///   0. `[]` The token program.
///   1. `[WRITE]` The multisig account.
///   2. `[]` The signer accounts required for multisig approval.
///   3. `[]` The number of required signers.
pub struct InitializeMultisig2<'a, const ACCOUNTS_LEN: usize> {
    /// Token Program
    pub token_program: &'a NoStdAccountInfo,
    /// Multisig Account
    pub multisig_account: &'a NoStdAccountInfo,
    /// Number of multisig signer accounts
    pub multisig_signer_accounts: &'a[&'a NoStdAccountInfo],
    /// Number of multisigs
    pub m: &'a u8,
}

impl <'a, const ACCOUNTS_LEN: usize>InitializeMultisig2<'a, ACCOUNTS_LEN> {

    #[inline(always)]
    pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> ProgramResult {

         let mut account_metas = [UNINIT_ACC_METAS; ACCOUNTS_LEN];

         account_metas[0].write(self.token_program.to_meta_c());
         account_metas[1].write(self.multisig_account.to_meta_c());

         for (i, account) in self.multisig_signer_accounts.iter().enumerate() {
             account_metas[2 + i].write(account.to_meta_c());
         }
         //ACCOUNTS_LEN
          let acc_metas = unsafe {
             core::slice::from_raw_parts(account_metas.as_ptr() as *const  AccountMetaC, ACCOUNTS_LEN)
         };
        // instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        let mut instruction_data = [UNINIT_BYTE; 1];

        // Set discriminator as u8 at offset [0]
        write_bytes(&mut instruction_data, &[19]);

        let instruction = InstructionC {
            accounts: acc_metas.as_ptr(),
            accounts_len: acc_metas.len() as u64,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1).as_ptr() },
            data_len: instruction_data.len() as u64,
            program_id: &crate::ID,
        };

        const UNINIT_ACC_INFOS: MaybeUninit<&NoStdAccountInfo> = MaybeUninit::<&NoStdAccountInfo>::uninit();

         let mut accounts = [UNINIT_ACC_INFOS; ACCOUNTS_LEN];

        accounts[0].write(self.token_program);
        accounts[1].write(self.multisig_account);

        let acc_infos: [&NoStdAccountInfo; ACCOUNTS_LEN] = unsafe {
             core::slice::from_raw_parts(accounts.as_ptr() as *const &NoStdAccountInfo, ACCOUNTS_LEN)
                 .try_into()
                 .unwrap() // this is safe as we know the length of the array
         };

        invoke_signed(
            &instruction,
            &acc_infos,
            signers,
        )?;
        Ok(())
    }
}
