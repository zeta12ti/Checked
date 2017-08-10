//! This module provides implementations of Option's methods for objects that are sufficiently
//! like Options. This modules is a candidate to be put in its own crate.
//!
//! Most of the examples for these methods were adapted from the `std::option` examples.
//!
//! # Example
//!
//! ```
//! use checked::AsRefOption;
//!
//! struct OptionWrapper<T>(Option<T>);
//!
//! impl<T> AsRef<Option<T>> for OptionWrapper<T> {
//!     fn as_ref(&self) -> &Option<T>{
//!         &self.0
//!     }
//! }
//!
//! impl<T> AsRefOption<T> for OptionWrapper<T> {}
//!
//! let x = OptionWrapper(Some(5_i32)); // x is an OptionWrapper<i32>
//!
//! // We can now use some of Option's methods on x directly.
//! assert!(x.is_some());
//! ```

use std::option::{Iter, IterMut};


// These impls don't seem to work as intended.
// impl<T, U> IntoOption<T> for U
// where
//     U: Into<Option<T>>,
// {
// }
//
// impl<T, U> AsRefOption<T> for U
// where
//     U: AsRef<Option<T>>,
// {
// }
//
// impl<T, U> AsMutOption<T> for U
// where
//     U: AsMut<Option<T>>,
// {
// }

/// The `IntoOption` trait implements all the methods of `Option` that consume self.
pub trait IntoOption<T>: Into<Option<T>> {
    /// Unwraps an option-like, yielding the content of a `Some`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a `None` with a custom panic message provided by
    /// `msg`.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use checked::{Checked, IntoOption};
    ///
    /// let x = Checked::from(5_i32);
    /// assert_eq!(x.expect("the world is ending"), 5);
    /// ```
    ///
    /// ```{.should_panic}
    /// use checked::{Checked, IntoOption};
    ///
    /// let x: Checked<&str> = Checked(None);
    /// x.expect("the world is ending"); // panics with `the world is ending`
    /// ```
    fn expect(self, msg: &str) -> T {
        <Self as Into<Option<T>>>::into(self).expect(msg)
    }

    /// Moves the value `v` out of the `Option<T>` if it is `Some(v)`.
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the `None`
    /// case explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals `None`.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use checked::{Checked, IntoOption};
    ///
    /// let x = Checked(Some("air"));
    /// assert_eq!(x.unwrap(), "air");
    /// ```
    ///
    /// ```{.should_panic}
    /// use checked::{Checked, IntoOption};
    ///
    /// let x: Checked<&str> = Checked(None);
    /// assert_eq!(x.unwrap(), "air"); // fails
    /// ```
    fn unwrap(self) -> T {
        <Self as Into<Option<T>>>::into(self).unwrap()
    }

    /// Returns the contained value or a default.
    ///
    /// # Examples
    ///
    /// ```
    /// use checked::{Checked, IntoOption};
    ///
    /// assert_eq!(Checked(Some("car")).unwrap_or("bike"), "car");
    /// assert_eq!(Checked(None).unwrap_or("bike"), "bike");
    /// ```
    fn unwrap_or(self, def: T) -> T {
        <Self as Into<Option<T>>>::into(self).unwrap_or(def)
    }

    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        <Self as Into<Option<T>>>::into(self).unwrap_or_else(f)
    }

    fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        <Self as Into<Option<T>>>::into(self).map(f)
    }

    fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        <Self as Into<Option<T>>>::into(self).map_or(default, f)
    }

    fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        <Self as Into<Option<T>>>::into(self).map_or_else(default, f)
    }

    fn ok_or<E>(self, err: E) -> Result<T, E> {
        <Self as Into<Option<T>>>::into(self).ok_or(err)
    }

    fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        <Self as Into<Option<T>>>::into(self).ok_or_else(err)
    }

    fn and<U>(self, optb: Option<U>) -> Option<U> {
        <Self as Into<Option<T>>>::into(self).and(optb)
    }

    fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        <Self as Into<Option<T>>>::into(self).and_then(f)
    }

    fn or(self, optb: Option<T>) -> Option<T> {
        <Self as Into<Option<T>>>::into(self).or(optb)
    }

    fn or_else<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        <Self as Into<Option<T>>>::into(self).or_else(f)
    }
}

/// The `AsRefOption` trait implements all the methods of `Option` that take an immutable reference
/// to self.
pub trait AsRefOption<T>: AsRef<Option<T>> {
    fn is_some(&self) -> bool {
        <Self as AsRef<Option<T>>>::as_ref(self).is_some()
    }

    fn is_none(&self) -> bool {
        <Self as AsRef<Option<T>>>::as_ref(self).is_none()
    }

    fn as_ref(&self) -> Option<&T> {
        <Self as AsRef<Option<T>>>::as_ref(self).as_ref()
    }

    fn iter(&self) -> Iter<T> {
        <Self as AsRef<Option<T>>>::as_ref(self).iter()
    }
}

/// The `AsMutOption` trait implements all the methods of `Option` that take a mutable reference to
/// self.
pub trait AsMutOption<T>: AsMut<Option<T>> {
    fn as_mut(&mut self) -> Option<&mut T> {
        <Self as AsMut<Option<T>>>::as_mut(self).as_mut()
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        <Self as AsMut<Option<T>>>::as_mut(self).iter_mut()
    }

    #[cfg(feature = "option_entry")]
    fn get_or_insert(&mut self, v: T) -> &mut T {
        <Self as AsMut<Option<T>>>::as_mut(self).get_or_insert(v)
    }

    #[cfg(feature = "option_entry")]
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        <Self as AsMut<Option<T>>>::as_mut(self).get_or_insert_with(f)
    }

    fn take(&mut self) -> Option<T> {
        <Self as AsMut<Option<T>>>::as_mut(self).take()
    }
}
