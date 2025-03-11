pub mod amount_to_ui_amount;
pub mod approve;
pub mod approve_checked;
pub mod burn;
pub mod burn_checked;
pub mod close_account;
pub mod freeze_account;
pub mod get_account_data_size;
pub mod initialize_account;
pub mod initialize_account_2;
pub mod initialize_account_3;
pub mod initialize_mint_2;
pub mod initialize_multisig;
pub mod initialize_multisig2;
pub mod initialize_immutable_owner;
pub mod mint_to;
pub mod mint_to_checked;
pub mod revoke;
pub mod set_authority;
pub mod sync_native;
pub mod thaw_account;
pub mod transfer;
pub mod transfer_checked;
pub mod ui_amount_to_amount;
pub mod token_program {
    use solana_program::pubkey::Pubkey;

    pub const ID: Pubkey = solana_program::pubkey!(
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    );
}

pub mod token_2022 {
    use solana_program::pubkey::Pubkey;

    pub const ID: Pubkey = solana_program::pubkey!(
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
    );
}
