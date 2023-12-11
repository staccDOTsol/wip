//! Math for preserving precision of token amounts which are limited
//! by the SPL Token program to be at most u64::MAX.
//!
//! Decimals are internally scaled by a WAD (10^18) to preserve
//! precision up to 18 decimal places. Decimals are sized to support
//! both serialization and precise math for the full range of
//! unsigned 64-bit integers. The underlying representation is a
//! u192 rather than u256 to reduce compute cost while losing
//! support for arithmetic operations at the high end of u64 range.

#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unreadable_literal)]
// allow no docs
#![allow(missing_docs)]
use core::ops::Mul;
use crate::{
    error::LendingError,
    math::{common::*, Rate},
};
use solana_program::program_error::ProgramError;
use std::{convert::TryFrom, fmt};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)]
pub struct U192(pub [u64; 3]);
impl TryFrom<u128> for U192 {
    type Error = LendingError;
    fn try_from(val: u128) -> Result<Self, Self::Error> {
        Ok(Self([val as u64, (val >> 64) as u64, 0]))
    }
}
impl Mul for U192 {
    type Output = Self;
  
    fn mul(self, rhs: Self) -> Self {
        let mut ret = [0u64; 3];
        let mut carry = [0u64; 3];
        for i in 0..3 {
            for j in 0..=i {
                let (prod, c1) = self.0[j].overflowing_mul(rhs.0[i - j]);
                let (sum, c2) = ret[i].overflowing_add(prod);
                let (sum, c3) = sum.overflowing_add(carry[j]);
                carry[j] = if c1 || c2 || c3 { 1 } else { 0 };
                ret[i] = sum;
            }
        }
        Self(ret)
    }
}
impl U192 {
    pub fn as_u64(&self) -> u64 {
        self.0[0]
    }
    pub fn exp10(exp: usize) -> Self {
        let mut ret = Self::from(1);
        for _ in 0..exp {
            ret = ret.checked_mul(Self::from(10)).unwrap();
        }
        ret
    }
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        let mut ret = [0u64; 3];
        let mut borrow = [0u64; 3];
        for i in 0..3 {
            let (diff, b1) = self.0[i].overflowing_sub(rhs.0[i]);
            let (diff, b2) = diff.overflowing_sub(borrow[i]);
            borrow[i] = if b1 || b2 { 1 } else { 0 };
            ret[i] = diff;
        }
        if borrow[2] == 0 {
            Some(Self(ret))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        let mut copy = *self;
        for _ in 0..3 {
            let rem = copy.checked_rem(U192::from(10)).unwrap();
            copy = copy.checked_div(U192::from(10)).unwrap();
            ret.push_str(&rem.to_string());
        }
        ret.chars().rev().collect()
    }
    pub const fn zero() -> Self {
        Self([0; 3])
    }
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let mut ret = [0u64; 3];
        let mut carry = 0u64;
        for i in 0..3 {
            let (sum, c1) = self.0[i].overflowing_add(rhs.0[i]);
            let (sum, c2) = sum.overflowing_add(carry);
            carry = if c1 || c2 { 1 } else { 0 };
            ret[i] = sum;
        }
        if carry == 0 {
            Some(Self(ret))
        } else {
            None
        }
    }
    // checked_sub, checked_mul, checked_div, checked_rem, checked_shl, checked_shr

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let mut ret = [0u64; 3];
        let mut borrow = 0u64;
        for i in 0..3 {
            let (diff, b1) = self.0[i].overflowing_sub(rhs.0[i]);
            let (diff, b2) = diff.overflowing_sub(borrow);
            borrow = if b1 || b2 { 1 } else { 0 };
            ret[i] = diff;
        }
        if borrow == 0 {
            Some(Self(ret))
        } else {
            None
        }
    }

    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let mut ret = [0u64; 3];
        let mut carry = [0u64; 3];
        for i in 0..3 {
            for j in 0..=i {
                let (prod, c1) = self.0[j].overflowing_mul(rhs.0[i - j]);
                let (sum, c2) = ret[i].overflowing_add(prod);
                let (sum, c3) = sum.overflowing_add(carry[j]);
                carry[j] = if c1 || c2 || c3 { 1 } else { 0 };
                ret[i] = sum;
            }
        }
        if carry[2] == 0 {
            Some(Self(ret))
        } else {
            None
        }
    }

    pub fn checked_div(mut self, rhs: Self) -> Option<Self> {
        let mut ret = [0u64; 3];
        let mut rem = [0u64; 3];
        let mut rhs = rhs;
        for i in (0..3).rev() {
            let mut quot = 0;
            while self.0[i] >= rhs.0[2] {
                quot += 1;
                let (diff, b) = self.0[i].overflowing_sub(rhs.0[2]);
                self.0[i] = diff;
                if b {
                    break;
                }
            }
            ret[i] = quot;
            let mut carry = 0;
            for j in (0..=2).rev() {
                let (prod, c1) = rhs.0[j].overflowing_mul(quot);
                let (sum, c2) = rem[j].overflowing_add(carry);
                let (diff, c3) = sum.overflowing_sub(prod);
                rem[j] = diff;
                carry = if c1 || c2 || c3 { 1 } else { 0 };
            }
            rhs = rhs.checked_shr(1)?;
        }
        Some(Self(ret))
    }
    pub fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs >= 192 {
            return None;
        }
        let mut ret = [0u64; 3];
        let mut rem = rhs;
        for i in 0..3 {
            let shift = if rem >= 64 {
                rem -= 64;
                0
            } else {
                64 - rem
            };
            ret[i] = self.0[i].checked_shr(rem)?;
            if i < 2 && shift > 0 {
                ret[i] |= self.0[i + 1].checked_shl(shift)?;
            }
        }
        Some(Self(ret))
    }



    pub const fn from(val: u128) -> Self {
        
        Self([val as u64, (val >> 64) as u64, 0])
    }

    pub const fn from_u64(val: u64) -> Self {
        Self([val, 0, 0])
    }

    pub const fn from_u256(val: [u64; 4]) -> Self {
        Self([val[0], val[1], val[2]])
    }

    pub const fn from_u256_truncate(val: [u64; 4]) -> Self {
        Self([val[0], val[1], val[2]])
    }

    pub fn from_u256_round(val: [u64; 4]) -> Self {
        let mut ret = Self([val[0], val[1], val[2]]);
        if val[3] > 0x8000_0000_0000_0000 {
            ret = ret.checked_add(Self::from(1)).unwrap();
        }
        ret
    }

    pub const fn from_u512(val: [u64; 8]) -> Self {
        Self([val[0], val[1], val[2]])
    }

    pub const fn from_u512_truncate(val: [u64; 8]) -> Self {
        Self([val[0], val[1], val[2]])
    }

    pub fn from_u512_round(val: [u64; 8]) -> Self {
        let mut ret = Self([val[0], val[1], val[2]]);
        if val[3] > 0x8000_0000_0000_0000 {
            ret = ret.checked_add(Self::from(1)).unwrap();
        }
        ret
    }

    pub const fn from_u1024(val: [u64; 16]) -> Self {
        Self([val[0], val[1], val[2]])
    }

    pub const fn from_u1024_truncate(val: [u64; 16]) -> Self {
        Self([val[0], val[1], val[2]])
    }

    pub fn from_u1024_round(val: [u64; 16]) -> Self {
        let mut ret = Self([val[0], val[1], val[2]]);
        if val[3] > 0x8000_0000_0000_0000 {
            ret = ret.checked_add(Self::from(1)).unwrap();
        }
        ret
    }

    pub const fn to_u128(&self) -> u128 {
        (self.0[0] as u128) | ((self.0[1] as u128) << 64)
    }
}


/// Large decimal values, precise to 18 digits
#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)]
pub struct Decimal(pub U192);

impl Decimal {
    pub fn serialize(&self) -> [u8; 24] {
        let mut ret = [0u8; 24];
        let bytes = self.0.to_string().into_bytes();
        ret[..bytes.len()].copy_from_slice(&bytes);
        ret
    }
    pub fn serialize_writer<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.serialize())
    }
    /// One
    pub fn one() -> Self {
        Self(Self::wad())
    }

    /// Zero
    pub fn zero() -> Self {
        Self(U192::zero())
    }

    /// WAD (10^18)
    pub fn wad() -> U192 {
        U192::exp10(SCALE)
    }

    /// Half WAD (10^18)
    pub fn half_wad() -> U192 {
        U192::exp10(SCALE - 1)
    }

    /// Create scaled decimal from percent value
    pub fn from_percent(percent: u8) -> Self {
        Self::from(percent as u64).try_div(100).unwrap()
    }

    /// Create scaled decimal from deca bps value
    pub fn from_deca_bps(deca_bps: u8) -> Self {
        Self::from(deca_bps as u64).try_div(1000).unwrap()
    }

    /// Create scaled decimal from bps value
    pub fn from_bps(bps: u64) -> Self {
        Self::from(bps).try_div(10_000).unwrap()
    }

    /// Return raw scaled value if it fits within u128
    #[allow(clippy::wrong_self_convention)]
    pub fn to_scaled_val(&self) -> Result<u128, ProgramError> {
        Ok(u128::try_from(self.0.to_u128()).map_err(|_| LendingError::MathOverflow)?)
    }

    /// Create decimal from scaled value
    pub fn from_scaled_val(scaled_val: u128) -> Self {
        Self(U192::try_from(scaled_val).unwrap())
    }

    /// Round scaled decimal to u64
    pub fn try_round_u64(&self) -> Result<u64, ProgramError> {
        let rounded_val = Self::half_wad()
            .checked_add(self.0)
            .ok_or(LendingError::MathOverflow)?
            .checked_div(Self::wad())
            .ok_or(LendingError::MathOverflow)?;
        Ok(u64::try_from(rounded_val.to_u128()).map_err(|_| LendingError::MathOverflow)?)
    }

    /// Ceiling scaled decimal to u64
    pub fn try_ceil_u64(&self) -> Result<u64, ProgramError> {
        let ceil_val = Self::wad()
            .checked_sub(U192::from_u64(1u64))
            .ok_or(LendingError::MathOverflow)?
            .checked_add(self.0)
            .ok_or(LendingError::MathOverflow)?
            .checked_div(Self::wad())
            .ok_or(LendingError::MathOverflow)?;
        Ok(u64::try_from(ceil_val.to_u128()).map_err(|_| LendingError::MathOverflow)?)
    }

    /// Floor scaled decimal to u64
    pub fn try_floor_u64(&self) -> Result<u64, ProgramError> {
        let ceil_val = self
            .0
            .checked_div(Self::wad())
            .ok_or(LendingError::MathOverflow)?;
        Ok(u64::try_from(ceil_val.to_u128()).map_err(|_| LendingError::MathOverflow)?)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut scaled_val = self.0.to_string();
        if scaled_val.len() <= SCALE {
            scaled_val.insert_str(0, &vec!["0"; SCALE - scaled_val.len()].join(""));
            scaled_val.insert_str(0, "0.");
        } else {
            scaled_val.insert(scaled_val.len() - SCALE, '.');
        }
        f.write_str(&scaled_val)
    }
}

impl fmt::Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<u64> for Decimal {
    fn from(val: u64) -> Self {
        Self(Self::wad() * U192::from_u64(val))
    }
}

impl From<u128> for Decimal {
    fn from(val: u128) -> Self {
        Self(Self::wad() * U192::from(val))
    }
}

impl From<Rate> for Decimal {
    fn from(val: Rate) -> Self {
        Self(U192::from(val.to_scaled_val()))
    }
}

impl TryAdd for Decimal {
    fn try_add(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_add(rhs.0)
                .ok_or(LendingError::MathOverflow)?,
        ))
    }
}

impl TrySub for Decimal {
    fn try_sub(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_sub(rhs.0)
                .ok_or(LendingError::MathOverflow)?,
        ))
    }
}

impl TryDiv<u64> for Decimal {
    fn try_div(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_div(U192::from_u64(rhs))
                .ok_or(LendingError::MathOverflow)?,
        ))
    }
}

impl TryDiv<Rate> for Decimal {
    fn try_div(self, rhs: Rate) -> Result<Self, ProgramError> {
        self.try_div(Self::from(rhs))
    }
}

impl TryDiv<Decimal> for Decimal {
    fn try_div(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(Self::wad())
                .ok_or(LendingError::MathOverflow)?
                .checked_div(rhs.0)
                .ok_or(LendingError::MathOverflow)?,
        ))
    }
}

impl TryMul<u64> for Decimal {
    fn try_mul(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(U192::from_u64(rhs))
                .ok_or(LendingError::MathOverflow)?,
        ))
    }
}

impl TryMul<Rate> for Decimal {
    fn try_mul(self, rhs: Rate) -> Result<Self, ProgramError> {
        self.try_mul(Self::from(rhs))
    }
}

impl TryMul<Decimal> for Decimal {
    fn try_mul(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(rhs.0)
                .ok_or(LendingError::MathOverflow)?
                .checked_div(Self::wad())
                .ok_or(LendingError::MathOverflow)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scaler() {
        assert_eq!(U192::exp10(SCALE), Decimal::wad());
    }

    #[test]
    fn test_u192() {
        let one = U192::from(1);
        assert_eq!(one.0, [1u64, 0, 0]);

        let wad = Decimal::wad();
        assert_eq!(wad.0, [WAD, 0, 0]);

        let hundred = Decimal::from(100u64);
        // 2^64 * 5 + 7766279631452241920 = 1e20
        assert_eq!(hundred.0 .0, [7766279631452241920, 5, 0]);
    }

    #[test]
    fn test_from_percent() {
        let left = Decimal::from_percent(20);
        let right = Decimal::from(20u64).try_div(Decimal::from(100u64)).unwrap();

        assert_eq!(left, right);
    }

    #[test]
    fn test_from_deca_bps() {
        let left = Decimal::from_deca_bps(250);
        assert_eq!(left, Decimal::from_percent(25));
    }

    #[test]
    fn test_from_bps() {
        let left = Decimal::from_bps(190000);
        assert_eq!(left, Decimal::from(19u64));
    }

    #[test]
    fn test_to_scaled_val() {
        assert_eq!(
            Decimal(U192::from(u128::MAX)).to_scaled_val().unwrap(),
            u128::MAX
        );

        assert_eq!(
            Decimal(U192::from(u128::MAX))
                .try_add(Decimal(U192::from(1)))
                .unwrap()
                .to_scaled_val(),
            Err(ProgramError::from(LendingError::MathOverflow))
        );
    }

    #[test]
    fn test_round_floor_ceil_u64() {
        let mut val = Decimal::one();
        assert_eq!(val.try_round_u64().unwrap(), 1);
        assert_eq!(val.try_floor_u64().unwrap(), 1);
        assert_eq!(val.try_ceil_u64().unwrap(), 1);

        val = val
            .try_add(Decimal::from_scaled_val(HALF_WAD as u128 - 1))
            .unwrap();
        assert_eq!(val.try_round_u64().unwrap(), 1);
        assert_eq!(val.try_floor_u64().unwrap(), 1);
        assert_eq!(val.try_ceil_u64().unwrap(), 2);

        val = val.try_add(Decimal::from_scaled_val(1)).unwrap();
        assert_eq!(val.try_round_u64().unwrap(), 2);
        assert_eq!(val.try_floor_u64().unwrap(), 1);
        assert_eq!(val.try_ceil_u64().unwrap(), 2);
    }

    #[test]
    fn test_display() {
        assert_eq!(Decimal::from(1u64).to_string(), "1.000000000000000000");
        assert_eq!(
            Decimal::from_scaled_val(1u128).to_string(),
            "0.000000000000000001"
        );
    }
}
