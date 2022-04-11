use solana_program::{self, account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

fn start(program_id: &Pubkey, account_info: &[AccountInfo], instructions: &[u8]) -> ProgramResult {
    crate::processor::process_instruction(program_id, account_info, instructions)
}

solana_program::entrypoint!(start);
