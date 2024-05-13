use anchor_lang::prelude::*;
use crate::{
    error::ContractError,
    constant::*,
};
use std::str::FromStr;

pub fn is_admin(key: &Pubkey)->bool {
    let mut authorities = vec![];
    for admin in ADMINS {
        authorities.push(Pubkey::from_str(admin).unwrap())
    }
    return authorities.contains(key)
    // return true
}

pub trait SafeCalc<T> {
    fn safe_add(&self, num: T) -> Result<T>;
    fn safe_sub(&self, num: T) -> Result<T>;
    fn safe_mul(&self, num: T) -> Result<T>;
    fn safe_div(&self, num: T) -> Result<T>;
    fn safe_pow(&self, num: u32) -> Result<T>;
}
impl SafeCalc<u32> for u32 {
    fn safe_add(&self, num: u32) -> Result<u32> {
        let result = self.checked_add(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_sub(&self, num: u32) -> Result<u32> {
        let result = self.checked_sub(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_mul(&self, num: u32) -> Result<u32> {
        let result = self.checked_mul(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_div(&self, num: u32) -> Result<u32> {
        let result = self.checked_div(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_pow(&self, num: u32) -> Result<u32> {
        let result = self.checked_pow(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
}
impl SafeCalc<u64> for u64 {
    fn safe_add(&self, num: u64) -> Result<u64> {
        let result = self.checked_add(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_sub(&self, num: u64) -> Result<u64> {
        let result = self.checked_sub(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_mul(&self, num: u64) -> Result<u64> {
        let result = self.checked_mul(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_div(&self, num: u64) -> Result<u64> {
        let result = self.checked_div(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_pow(&self, num: u32) -> Result<u64> {
        let result = self.checked_pow(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
}
impl SafeCalc<u128> for u128 {
    fn safe_add(&self, num: u128) -> Result<u128> {
        let result = self.checked_add(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_sub(&self, num: u128) -> Result<u128> {
        let result = self.checked_sub(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_mul(&self, num: u128) -> Result<u128> {
        let result = self.checked_mul(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_div(&self, num: u128) -> Result<u128> {
        let result = self.checked_div(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_pow(&self, num: u32) -> Result<u128> {
        let result = self.checked_pow(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
}
impl SafeCalc<i64> for i64 {
    fn safe_add(&self, num: i64) -> Result<i64> {
        let result = self.checked_add(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_sub(&self, num: i64) -> Result<i64> {
        let result = self.checked_sub(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_mul(&self, num: i64) -> Result<i64> {
        let result = self.checked_mul(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_div(&self, num: i64) -> Result<i64> {
        let result = self.checked_div(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_pow(&self, num: u32) -> Result<i64> {
        let result = self.checked_pow(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
}
impl SafeCalc<i128> for i128 {
    fn safe_add(&self, num: i128) -> Result<i128> {
        let result = self.checked_add(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_sub(&self, num: i128) -> Result<i128> {
        let result = self.checked_sub(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_mul(&self, num: i128) -> Result<i128> {
        let result = self.checked_mul(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_div(&self, num: i128) -> Result<i128> {
        let result = self.checked_div(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
    fn safe_pow(&self, num: u32) -> Result<i128> {
        let result = self.checked_pow(num);
        if result.is_none() {
            return Err(error!(ContractError::MathOverflow));
        }
        Ok(result.unwrap())
    }
}