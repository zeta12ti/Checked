//! Checked arithmetic.
//!
//! The Checked struct allows you to perform checked arithmetic without an overabundance of
//! .checked_* calls.

mod num;
pub use num::Checked;

#[cfg(test)]
mod tests;
