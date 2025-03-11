use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;

pub mod instructions;
pub mod invoke_signed;

pub const ID: Pubkey = solana_program::pubkey!(
    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
);

pub struct ElgamalPubkey(pub [u8; 32]);

pub type ProgramResult = Result<(), ProgramError>;
