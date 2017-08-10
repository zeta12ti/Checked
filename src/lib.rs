//! Checked arithmetic.
//!
//! The Checked struct allows you to perform checked arithmetic without an overabundance of
//! .checked_* calls.
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
//! Ord is implemented, and the overflow state is considered less than everything
//! (may change this in the future)
//!
//! ```
//! use checked::Checked;
//!
//! assert!(Checked::from(1_000_u32) <= Checked::from(10_000_u32));
//! assert!(!(Checked::from(1_000_u32) <= Checked::from(None)));
//! assert!((Checked::from(None) <= Checked::from(1_000_u32)));
//! ```

mod num;
pub use num::Checked;

mod option_like;
pub use option_like::{IntoOption, AsRefOption, AsMutOption};

#[cfg(test)]
mod tests;
