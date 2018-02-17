//! Checked arithmetic.
//!
//! The `Checked` struct allows you to perform checked arithmetic without an overabundance of
//! `.checked_*` calls.
//! # Examples
//! ```
//! use checked::Checked;
//!
//! let x = Checked::from(1_000_u32);
//! let y = Checked::from(1_000_000_u32);
//!
//! assert_eq!(x * 1_000, y);
//! assert_eq!(1_000_000 * y, Checked::from(None));
//! assert_eq!(x << 3, Checked::<u32>::from(8_000));
//! ```
//!
//! All the basic arithmetic operations are implemented.
//!
//! ```
//! use checked::Checked;
//!
//! assert_eq!(Checked::<u8>::from(5) + Checked::<u8>::from(6), Checked::<u8>::from(11));
//! assert_eq!(Checked::<u8>::from(5) - Checked::<u8>::from(6), Checked::<u8>::from(None));
//! assert_eq!(Checked::<u8>::from(5) * Checked::<u8>::from(6), Checked::<u8>::from(30));
//! assert_eq!(Checked::<u8>::from(5) / Checked::<u8>::from(6), Checked::<u8>::from(0));
//! assert_eq!(- Checked::<u8>::from(6), Checked::<u8>::from(None));
//! assert_eq!(Checked::<u8>::from(5) % Checked::<u8>::from(6), Checked::<u8>::from(5));
//! ```
//!
//! Including bitwise operations.
//!
//! ```
//! use checked::Checked;
//!
//! assert_eq!(Checked::<u8>::from(5) & Checked::<u8>::from(6), Checked::<u8>::from(4));
//! assert_eq!(Checked::<u8>::from(5) | Checked::<u8>::from(6), Checked::<u8>::from(7));
//! assert_eq!(Checked::<u8>::from(5) ^ Checked::<u8>::from(6), Checked::<u8>::from(3));
//! assert_eq!(! Checked::<u8>::from(6), Checked::<u8>::from(249));
//! assert_eq!(Checked::<u8>::from(5) << Checked::<u32>::from(5), Checked::<u8>::from(160));
//! assert_eq!(Checked::<u8>::from(160) >> Checked::<u32>::from(5), Checked::<u8>::from(5));
//! ```
//!
//! Any single operand can be an ordinary integer and it'll get converted.
//!
//! ```
//! use checked::Checked;
//!
//! assert_eq!(5 + Checked::<u8>::from(6), Checked::<u8>::from(11));
//! assert_eq!(Checked::<u8>::from(5) + 6, Checked::<u8>::from(11));
//! assert_eq!(5 - Checked::<u8>::from(6), Checked::<u8>::from(None));
//! assert_eq!(Checked::<u8>::from(5) - 6, Checked::<u8>::from(None));
//! ```
//!
//! `PartialOrd` is implemented, and the overflow state is not comparable to anything else.
//!
//! ```
//! use checked::Checked;
//!
//! assert!(Checked::from(1_000_u32) <= Checked::from(10_000_u32));
//! assert!(!(Checked::from(1_000_u32) <= Checked::from(None)));
//! assert!(!(Checked::from(None) <= Checked::from(1_000_u32)));
//! ```
extern crate num_traits;

mod num;
pub use num::Checked;

#[cfg(test)]
mod tests;
