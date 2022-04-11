use crate::error::PalletError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{msg, program_error::ProgramError};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq)]

pub enum Instruction {
    InitVault,
    InitBank,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

impl Instruction {
    pub fn unpack(instruction_bytes: &[u8]) -> Result<Self, ProgramError> {
        Self::unpack_raw(instruction_bytes).map_err(|e| ProgramError::Custom(e as u32))
    }

    fn unpack_raw(raw: &[u8]) -> Result<Self, PalletError> {
        let (call_index, rest) = raw.split_first().ok_or(PalletError::InvalidInstruction)?;

        let res = match call_index {
            0 => Instruction::InitVault,
            1 => Instruction::InitBank,
            2 => {
                let amount = <u64 as BorshDeserialize>::deserialize(
                    &mut rest.get(..8).ok_or(PalletError::InvalidInstruction)?,
                )
                .map_err(|_| {
                    msg!("Cannot get amount from deposit instruction.");
                    PalletError::InvalidInstruction
                })?;

                Instruction::Deposit { amount }
            }
            3 => {
                let amount = <u64 as BorshDeserialize>::deserialize(
                    &mut rest.get(..8).ok_or(PalletError::InvalidInstruction)?,
                )
                .map_err(|_| {
                    msg!("Cannot get amount from withdraw instruction.");
                    PalletError::InvalidInstruction
                })?;

                Instruction::Withdraw { amount }
            }

            _ => return Err(PalletError::InvalidInstruction),
        };

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unpack_is_ok() {
        assert_eq!(Ok(Instruction::InitVault), Instruction::unpack_raw(&[0]));

        assert_eq!(
            Ok(Instruction::Deposit { amount: 10u64 }),
            Instruction::unpack_raw(&[1, 10, 0, 0, 0, 0, 0, 0, 0])
        );

        assert_eq!(
            Err(PalletError::InvalidInstruction),
            Instruction::unpack_raw(&[u8::MAX])
        );
    }

    #[test]
    fn print_pack_samples() {
        eprintln!("-----------------------");
        let print_bytes = |code| {
            let mut target = vec![];
            <Instruction as BorshSerialize>::serialize(&code, &mut target).unwrap();
            println!("{:?} is {:?}", code, target);
        };

        print_bytes(Instruction::InitVault);
        print_bytes(Instruction::Deposit { amount: 10u64 });
        print_bytes(Instruction::Deposit {
            amount: 1_000_000_000u64,
        });
        print_bytes(Instruction::Withdraw { amount: 100u64 });
        print_bytes(Instruction::Deposit { amount: 0u64 });
        print_bytes(Instruction::Withdraw { amount: 0u64 });
        print_bytes(Instruction::Withdraw {
            amount: 1_000_000_000u64 / 2u64,
        });

        eprintln!("-----------------------");
    }
}
