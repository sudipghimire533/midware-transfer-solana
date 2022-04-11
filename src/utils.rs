use crate::error::PalletError;
use crate::state::Storage;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub const BANK_SEEDS: &'static [&[u8]] = &[b"derive-this"];

pub fn update_internal(
    vault: &AccountInfo,
    receiver: &AccountInfo,
    modifier: impl FnOnce(u64) -> u64,
) -> ProgramResult {
    let mut storage = Storage::upack(&mut &vault.data.borrow()[..]).unwrap_or_default();

    let new_balance = modifier(*storage.bank.entry(*receiver.key).or_insert(0));
    storage.bank.insert(*receiver.key, new_balance);

    storage
        .pack(&mut &mut vault.data.borrow_mut()[..])
        .map_err(|e| {
            msg!("Cannot write update storage. Reason: {:?}", e);
            ProgramError::Custom(PalletError::CantUpdate as u32)
        })?;

    Ok(())
}

pub fn is_vault(program_id: &Pubkey, vault_account: &AccountInfo) -> bool {
    vault_account.owner == program_id
}

pub fn ensure_bank(
    program_id: &Pubkey,
    bank: &AccountInfo,
) -> Option<(Pubkey, u8)> {

    let (pda_key, bump_seed) = Pubkey::find_program_address(BANK_SEEDS, program_id);

    msg!("[derive] Pda key: {:?} and seed: {}", pda_key, bump_seed);

    if *bank.key != pda_key {
        return None;
    }

    Some((pda_key, bump_seed))
}
