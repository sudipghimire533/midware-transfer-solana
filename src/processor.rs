#![allow(unused_imports)]

use crate::{
    error::{self, PalletError},
    instruction::{self, Instruction},
    state::{self, Storage},
    utils,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{self, next_account_info, AccountInfo},
    entrypoint::{self, ProgramResult},
    msg,
    native_token::LAMPORTS_PER_SOL,
    program,
    program_error::{self, ProgramError},
    pubkey::{self, Pubkey},
    system_instruction,
};

const UNIT_SOL: u64 = 1_000_000_000;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(instruction_data)?;

    msg!("[Program instruction] {:?}", instruction);
    match instruction {
        Instruction::InitVault => init_vault(program_id, accounts),

        Instruction::InitBank => init_bank(program_id, accounts),

        Instruction::Deposit { amount } => deposit_fund(program_id, accounts, amount),

        Instruction::Withdraw { amount } => withdraw_fund(program_id, accounts, amount),
    }
}

fn deposit_fund(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let system_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let bank_account = next_account_info(accounts_iter)?;
    let sender = next_account_info(accounts_iter)?;
    let receiver = next_account_info(accounts_iter)?;

    if !utils::is_vault(program_id, vault_account) {
        return Err(PalletError::NotValidVault.into());
    }
    if utils::ensure_bank(program_id, bank_account).is_none() {
        return Err(PalletError::NotValidBank.into());
    }

    program::invoke(
        &system_instruction::transfer(&sender.key, &bank_account.key, amount),
        &[sender.clone(), bank_account.clone(), system_account.clone()],
    )?;

    utils::update_internal(vault_account, receiver, |old| old + amount)
}

fn withdraw_fund(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let system_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let bank_account = next_account_info(accounts_iter)?;
    let withdrawer = next_account_info(accounts_iter)?;

    withdrawer
        .signer_key()
        .ok_or(ProgramError::Custom(PalletError::IllegalWithdrawer as u32))?;

    if !utils::is_vault(program_id, vault_account) {
        return Err(PalletError::NotValidVault.into());
    }

    let (_pda, bump) = utils::ensure_bank(program_id, bank_account)
        .ok_or(ProgramError::Custom(PalletError::NotValidBank as u32))?;

    let transfer_instruction =
        system_instruction::transfer(bank_account.key, withdrawer.key, amount);
    program::invoke_signed(
        &transfer_instruction,
        &[
            bank_account.clone(),
            withdrawer.clone(),
            system_account.clone(),
        ],
        &[&[utils::BANK_SEEDS[0], &[bump]]],
    )?;

    utils::update_internal(vault_account, withdrawer, |old| old - amount)
}

fn init_bank(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let system_account = next_account_info(accounts_iter)?;
    let bank_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;

    let (_pda, bump) = utils::ensure_bank(program_id, bank_account)
        .ok_or(ProgramError::Custom(PalletError::NotValidBank as u32))?;

    let create_account_instruction = system_instruction::create_account(
        payer_account.key,
        bank_account.key,
        LAMPORTS_PER_SOL,
        0,
        system_account.key,
    );

    program::invoke_signed(
        &create_account_instruction,
        &[
            bank_account.clone(),
            payer_account.clone(),
            system_account.clone(),
        ],
        &[&[utils::BANK_SEEDS[0], &[bump]]],
    )?;

    Ok(())
}

fn init_vault(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let system_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;

    let create_account_instruction = system_instruction::create_account(
        payer_account.key,
        vault_account.key,
        UNIT_SOL,
        1000,
        program_id,
    );
    let assign_account_instruction = system_instruction::assign(vault_account.key, program_id);

    program::invoke(
        &create_account_instruction,
        &[
            vault_account.clone(),
            payer_account.clone(),
            system_account.clone(),
        ],
    )?;

    program::invoke(
        &assign_account_instruction,
        &[
            vault_account.clone(),
            payer_account.clone(),
            system_account.clone(),
        ],
    )?;

    Ok(())
}
