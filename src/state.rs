use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

type BankStore = std::collections::hash_map::HashMap<Pubkey, u64>;

// Primary storage to keep in account
#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct Storage {
    pub bank: BankStore,
}

impl Storage {
    pub fn upack(mut raw_bytes: &[u8]) -> Option<Self> {
        <Storage as BorshDeserialize>::deserialize(&mut raw_bytes).ok()
    }

    pub fn pack<W: std::io::Write>(&self, target: &mut W) -> Result<(), std::io::Error> {
        <Storage as BorshSerialize>::serialize(self, target)
    }
}
