use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use num_bigint::BigInt;
use std::cmp::Ordering::*;

#[derive(Clone, Debug)]
pub struct Decimal {
    int: BigInt,
    scale: isize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if let Some((a, b)) = input.split_once(".") {
            let int = BigInt::from_str((a.to_string() + b).as_str()).ok()?;
            Some(Self {
                int,
                scale: b.len() as isize,
            })
        } else {
            let int = BigInt::from_str(input).ok()?;
            Some(Self { int, scale: 0 })
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let scale_diff = self.scale - other.scale;
        match scale_diff.cmp(&0) {
            Equal => self.int == other.int,
            Greater => {
                let new_other_int = other.int.clone() * BigInt::from(10).pow(scale_diff as u32);
                self.int == new_other_int
            }
            Less => {
                let new_self_int = self.int.clone() * BigInt::from(10).pow(scale_diff.abs() as u32);
                new_self_int == other.int
            }
        }
    }
}

impl Eq for Decimal {}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let scale_diff = self.scale - other.scale;
        match scale_diff.cmp(&0) {
            Equal => self.int.partial_cmp(&other.int),
            Greater => {
                let new_other_int = other.int.clone() * BigInt::from(10).pow(scale_diff as u32);
                self.int.partial_cmp(&new_other_int)
            }
            Less => {
                let new_self_int = self.int.clone() * BigInt::from(10).pow(scale_diff.abs() as u32);
                new_self_int.partial_cmp(&other.int)
            }
        }
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let scale_diff = self.scale - rhs.scale;
        match scale_diff.cmp(&0) {
            Equal => Self {
                int: self.int + rhs.int,
                scale: self.scale,
            },
            Greater => {
                let new_rhs_int = rhs.int.clone() * BigInt::from(10).pow(scale_diff as u32);
                Self {
                    int: self.int + new_rhs_int,
                    scale: self.scale,
                }
            }
            Less => {
                let new_self_int = self.int.clone() * BigInt::from(10).pow(scale_diff.abs() as u32);
                Self {
                    int: new_self_int + rhs.int,
                    scale: rhs.scale,
                }
            }
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let scale_diff = self.scale - rhs.scale;
        match scale_diff.cmp(&0) {
            Equal => Self {
                int: self.int - rhs.int,
                scale: self.scale,
            },
            Greater => {
                let new_rhs_int = rhs.int.clone() * BigInt::from(10).pow(scale_diff as u32);
                Self {
                    int: self.int - new_rhs_int,
                    scale: self.scale,
                }
            }
            Less => {
                let new_self_int = self.int.clone() * BigInt::from(10).pow(scale_diff.abs() as u32);
                Self {
                    int: new_self_int - rhs.int,
                    scale: rhs.scale,
                }
            }
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            int: self.int * rhs.int,
            scale: self.scale + rhs.scale,
        }
    }
}
