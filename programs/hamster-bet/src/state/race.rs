use {
    anchor_lang::prelude::*,
    crate::{constant::*, error::ContractError, state::*, utils::*},
};

#[repr(C)]
#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Copy)]

pub struct Hamster {
    pub id: u8,
    pub pool: u64,
    pub is_winner: bool,
    pub boost: u64, // Add boost field
}

#[account]
#[derive(Default)]
pub struct Race {
    pub bump: u8,
    pub race_id: u64,
    pub authority: Pubkey, // admin's wallet
    pub token_mint: Pubkey,
    pub hamsters: Vec<Hamster>,
    pub status: u8,
    pub race_pool: u64,
    pub house_pool: u64,
    pub boost_pool: u64,
    pub created_at: u64,
    pub reserved: [u128; 5],
}

impl Race {
    pub fn is_bet_available(&self) -> Result<bool> {
        Ok(self.status == 1)
    }

    pub fn get_hamster_index(&self, hamster_id: u8) -> Result<usize> {
        match self.hamsters.iter().position(|t| t.id == hamster_id) {
            Some(index) => Ok(index),
            None => Err(error!(ContractError::HamsterNotExist)),
        }
    }

    pub fn handle_bet(&mut self, hamster_id: u8, amount: u64) -> Result<()> {
        let index = self.get_hamster_index(hamster_id)?;
        let hamster = &mut self.hamsters[index];
        hamster.pool = hamster.pool.safe_add(amount).unwrap();

        // Update race_pool, house_pool, and boost_pool
        self.race_pool = self.race_pool.safe_add(amount * 925).unwrap();
        self.house_pool = self.house_pool.safe_add(amount * 50).unwrap();
        self.boost_pool = self.boost_pool.safe_add(amount * 25).unwrap();

        Ok(())
    }

    pub fn get_wining_hamster_id(&mut self) -> Result<u8> {
        let index = self.hamsters.iter().position(|t| t.is_winner == true);
        if index.is_some() {
            Ok(self.hamsters[index.unwrap()].id)
        } else {
            Err(error!(ContractError::HamsterNotExist))
        }
    }

    pub fn reveal_wining_hamster(&mut self, hamster_id: u8) -> Result<()> {
        let index = self.get_hamster_index(hamster_id);
        self.hamsters[index.unwrap()].is_winner = true;

        Ok(())
    }

    // Function to get the pool amount associated with a specific hamster ID
    pub fn get_hamster_pool(&self, hamster_id: u8) -> Option<u64> {
        // Find the index of the hamster with the given ID
        if let Some(hamster) = self
            .hamsters
            .iter()
            .find(|hamster| hamster.id == hamster_id)
        {
            Some(hamster.pool)
        } else {
            None
        }
    }

    // Function to get the boost amount associated with a specific hamster ID
    pub fn get_hamster_boost(&self, hamster_id: u8) -> Option<u64> {
        // Find the index of the hamster with the given ID
        if let Some(hamster) = self
            .hamsters
            .iter()
            .find(|hamster| hamster.id == hamster_id)
        {
            Some(hamster.boost)
        } else {
            None
        }
    }
}
