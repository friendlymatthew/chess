use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, BitXorAssign, Shl, Shr},
};

/// A `BitBoard` is just an abstraction over a u64
///
/// This way it's easy to debug by printing the actual board,
/// rather than a random number
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Bitboard(pub(crate) u64);

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Shl<u64> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u64> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LAST_BIT: u64 = 63;

        writeln!(f)?;

        for rank in (0..8).rev() {
            for file in 0..8 {
                let mask = 1u64 << (LAST_BIT - rank * 8 - file);
                let ch = if self.0 & mask != 0 { '1' } else { '0' };
                write!(f, "{ch} ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
