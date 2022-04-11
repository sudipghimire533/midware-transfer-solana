use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PalletError {
    #[error("No fund is available for now to withdraw")]
    NoFundAvailable = 0,

    #[error("Deposit of funds from this user failed..")]
    CantDeposit,

    #[error("Withdraw of funds from this user failed...")]
    CantWithdraw,

    #[error("This instruction cannot be understood")]
    InvalidInstruction,

    #[error("Cannot update internal storage")]
    CantUpdate,

    #[error("This is not valid vault address")]
    NotValidVault,

    #[error("This is not valid bank address")]
    NotValidBank,

    #[error("This withdraw request is not actually signed by owner")]
    IllegalWithdrawer,
}

impl Into<ProgramError> for PalletError {
    fn into(self) -> ProgramError {
        ProgramError::Custom(self as u32)
    }
}
