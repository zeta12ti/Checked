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

mod num;
pub use num::Checked;

#[cfg(test)]
mod tests;
