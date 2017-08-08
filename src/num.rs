use std::fmt;
use std::ops::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Checked<T>(pub Option<T>);

impl<T> Checked<T> {
    /// Creates a new Checked instance from some sort of integer.
    /// This is essentially equivalent to From\<T\>.
    /// # Examples
    /// ```
    /// use checked::Checked;
    ///
    /// let x = Checked::new(1_000_u32);
    /// let y = Checked::new(1_000_000_u32);
    /// assert_eq!(x * x, y);
    /// ```
    pub fn new(x: T) -> Checked<T> {
        Checked(Some(x))
    }
}

// The derived Default only works if T has Default
// Even though this is what it would be anyway
impl<T> Default for Checked<T> {
    fn default() -> Checked<T> {
        Checked(None)
    }
}

impl<T: fmt::Debug> fmt::Debug for Checked<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref x) => x.fmt(f),
            None => "overflow".fmt(f),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Checked<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref x) => x.fmt(f),
            None => "overflow".fmt(f),
        }
    }
}

// I'd like to do
// impl<T, U> From<U> where T: From<U> for Checked<T>
// in the obvious way, but that "conflicts" with the default impl From<T> for T.
// This would subsume both the below Froms since Option has the right From impl.
impl<T> From<T> for Checked<T> {
    fn from(x: T) -> Checked<T> {
        Checked(Some(x))
    }
}

impl<T> From<Option<T>> for Checked<T> {
    fn from(x: Option<T>) -> Checked<T> {
        Checked(x)
    }
}

impl<T> Into<Option<T>> for Checked<T> {
    fn into(self) -> Option<T> {
        self.0
    }
}

impl<T> AsRef<Option<T>> for Checked<T> {
    fn as_ref(&self) -> &Option<T> {
        &self.0
    }
}

// implements the unary operator "op &T"
// based on "op T" where T is expected to be `Copy`able
macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty {}) => {
        impl<'a> $imp for &'a $t {
            type Output = <$t as $imp>::Output;

            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    }
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty {}) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

macro_rules! impl_sh {
    ($t:ident, $f:ident) => {
        impl Shl<Checked<$f>> for Checked<$t> {
            type Output = Checked<$t>;

            fn shl(self, other: Checked<$f>) -> Checked<$t> {
                match (self.0, other.0) {
                    (Some(x), Some(y)) => Checked(x.checked_shl(y)),
                    _ => Checked(None),
                }
            }
        }

        impl Shl<$f> for Checked<$t> {
            type Output = Checked<$t>;

            fn shl(self, other: $f) -> Checked<$t> {
                match self.0 {
                    Some(x) => Checked(x.checked_shl(other)),
                    None => Checked(None),
                }
            }
        }

        forward_ref_binop! { impl Shl, shl for Checked<$t>, Checked<$f> {} }
        forward_ref_binop! { impl Shl, shl for Checked<$t>, $f {} }

        impl ShlAssign<$f> for Checked<$t> {
            fn shl_assign(&mut self, other: $f) {
                *self = *self << other;
            }
        }

        impl ShlAssign<Checked<$f>> for Checked<$t> {
            fn shl_assign(&mut self, other: Checked<$f>) {
                *self = *self << other;
            }
        }

        impl Shr<Checked<$f>> for Checked<$t> {
            type Output = Checked<$t>;

            fn shr(self, other: Checked<$f>) -> Checked<$t> {
                match (self.0, other.0) {
                    (Some(x), Some(y)) => Checked(x.checked_shr(y)),
                    _ => Checked(None),
                }
            }
        }

        impl Shr<$f> for Checked<$t> {
            type Output = Checked<$t>;

            fn shr(self, other: $f) -> Checked<$t> {
                match self.0 {
                    Some(x) => Checked(x.checked_shr(other)),
                    None => Checked(None),
                }
            }
        }

        forward_ref_binop! { impl Shr, shr for Checked<$t>, Checked<$f> {} }
        forward_ref_binop! { impl Shr, shr for Checked<$t>, $f {} }

        impl ShrAssign<$f> for Checked<$t> {
            fn shr_assign(&mut self, other: $f) {
                *self = *self >> other;
            }
        }

        impl ShrAssign<Checked<$f>> for Checked<$t> {
            fn shr_assign(&mut self, other: Checked<$f>) {
                *self = *self >> other;
            }
        }
    };
}

macro_rules! impl_sh_reverse {
    ($t:ident, $f:ident) => {
        impl Shl<Checked<$t>> for $f {
            type Output = Checked<$f>;

            fn shl(self, other: Checked<$t>) -> Checked<$f> {
                match other.0 {
                    Some(x) => Checked(self.checked_shl(x)),
                    None => Checked(None),
                }
            }
        }

        forward_ref_binop! { impl Shl, shl for $f, Checked<$t> {} }

        impl Shr<Checked<$t>> for $f {
            type Output = Checked<$f>;

            fn shr(self, other: Checked<$t>) -> Checked<$f> {
                match other.0 {
                    Some(x) => Checked(self.checked_shr(x)),
                    None => Checked(None),
                }
            }
        }

        forward_ref_binop! { impl Shr, shr for $f, Checked<$t> {} }
    };
}

macro_rules! impl_sh_all {
    ($($t:ident)*) => ($(
        // When checked_shX is added for other shift sizes, uncomment some of these.
        // impl_sh! { $t, u8 }
        // impl_sh! { $t, u16 }
        impl_sh! { $t, u32 }
        //impl_sh! { $t, u64 }
        //impl_sh! { $t, usize }

        //impl_sh! { $t, i8 }
        //impl_sh! { $t, i16 }
        //impl_sh! { $t, i32 }
        //impl_sh! { $t, i64 }
        //impl_sh! { $t, isize }

        // impl_sh_reverse! { u8, $t }
        // impl_sh_reverse! { u16, $t }
        impl_sh_reverse! { u32, $t }
        //impl_sh_reverse! { u64, $t }
        //impl_sh_reverse! { usize, $t }

        //impl_sh_reverse! { i8, $t }
        //impl_sh_reverse! { i16, $t }
        //impl_sh_reverse! { i32, $t }
        //impl_sh_reverse! { i64, $t }
        //impl_sh_reverse! { isize, $t }
    )*)
}

impl_sh_all! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

// implements unary operators for checked types
macro_rules! impl_unop {
    (impl $imp:ident, $method:ident, $checked_method:ident for $t:ty {}) => {
        impl $imp for Checked<$t> {
            type Output = Checked<$t>;

            fn $method(self) -> Checked<$t> {
                match self.0 {
                    Some(x) => Checked(x.$checked_method()),
                    None => Checked(None)
                }
            }
        }

        forward_ref_unop! { impl $imp, $method for Checked<$t> {} }
    }
}

// implements unary operators for checked types (with no checked method)
macro_rules! impl_unop_unchecked {
    (impl $imp:ident, $method:ident for $t:ty {$op:tt}) => {
        impl $imp for Checked<$t> {
            type Output = Checked<$t>;

            fn $method(self) -> Checked<$t> {
                match self.0 {
                    Some(x) => Checked(Some($op x)),
                    None => Checked(None)
                }
            }
        }

        forward_ref_unop! { impl $imp, $method for Checked<$t> {} }
    }
}

// implements binary operators for checked types
macro_rules! impl_binop {
    (impl $imp:ident, $method:ident, $checked_method:ident for $t:ty {}) => {
        impl $imp for Checked<$t> {
            type Output = Checked<$t>;

            fn $method(self, other: Checked<$t>) -> Checked<$t> {
                match (self.0, other.0) {
                    (Some(x), Some(y)) => Checked(x.$checked_method(y)),
                    _ => Checked(None),
                }
            }
        }

        impl $imp<$t> for Checked<$t> {
            type Output = Checked<$t>;

            fn $method(self, other: $t) -> Checked<$t> {
                match self.0 {
                    Some(x) => Checked(x.$checked_method(other)),
                    _ => Checked(None),
                }
            }
        }

        impl $imp<Checked<$t>> for $t {
            type Output = Checked<$t>;

            fn $method(self, other: Checked<$t>) -> Checked<$t> {
                match other.0 {
                    Some(x) => Checked(self.$checked_method(x)),
                    None => Checked(None),
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for Checked<$t>, Checked<$t> {} }
        forward_ref_binop! { impl $imp, $method for Checked<$t>, $t {} }
        forward_ref_binop! { impl $imp, $method for $t, Checked<$t> {} }
    }
}

// implements binary operators for checked types (no checked method)
macro_rules! impl_binop_unchecked {
    (impl $imp:ident, $method:ident for $t:ty {$op:tt}) => {
        impl $imp for Checked<$t> {
            type Output = Checked<$t>;

            fn $method(self, other: Checked<$t>) -> Checked<$t> {
                match (self.0, other.0) {
                    (Some(x), Some(y)) => Checked(Some(x $op y)),
                    _ => Checked(None),
                }
            }
        }

        impl $imp<$t> for Checked<$t> {
            type Output = Checked<$t>;

            fn $method(self, other: $t) -> Checked<$t> {
                match self.0 {
                    Some(x) => Checked(Some(x $op other)),
                    _ => Checked(None),
                }
            }
        }

        impl $imp<Checked<$t>> for $t {
            type Output = Checked<$t>;

            fn $method(self, other: Checked<$t>) -> Checked<$t> {
                match other.0 {
                    Some(x) => Checked(Some(self $op x)),
                    None => Checked(None),
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for Checked<$t>, Checked<$t> {} }
        forward_ref_binop! { impl $imp, $method for Checked<$t>, $t {} }
        forward_ref_binop! { impl $imp, $method for $t, Checked<$t> {} }
    }
}

// implements assignment operators for checked types
macro_rules! impl_binop_assign {
    (impl $imp:ident, $method:ident for $t:ty {$op:tt}) => {
        impl $imp for Checked<$t> {
            fn $method(&mut self, other: Checked<$t>) {
                *self = *self $op other;
            }
        }

        impl $imp<$t> for Checked<$t> {
            fn $method(&mut self, other: $t) {
                *self = *self $op other;
            }
        }
    };
}

macro_rules! checked_impl {
    ($($t:ty)*) => {
        $(
            impl_binop! { impl Add, add, checked_add for $t {} }
            impl_binop_assign! { impl AddAssign, add_assign for $t {+} }
            impl_binop! { impl Sub, sub, checked_sub for $t {} }
            impl_binop_assign! { impl SubAssign, sub_assign for $t {-} }
            impl_binop! { impl Mul, mul, checked_mul for $t {} }
            impl_binop_assign! { impl MulAssign, mul_assign for $t {*} }
            impl_binop! { impl Div, div, checked_div for $t {} }
            impl_binop_assign! { impl DivAssign, div_assign for $t {/} }
            impl_binop! { impl Rem, rem, checked_rem for $t {} }
            impl_binop_assign! { impl RemAssign, rem_assign for $t {%} }
            impl_unop_unchecked! { impl Not, not for $t {!} }
            impl_binop_unchecked! { impl BitXor, bitxor for $t {^} }
            impl_binop_assign! { impl BitXorAssign, bitxor_assign for $t {^} }
            impl_binop_unchecked! { impl BitOr, bitor for $t {|} }
            impl_binop_assign! { impl BitOrAssign, bitor_assign for $t {|} }
            impl_binop_unchecked! { impl BitAnd, bitand for $t {&} }
            impl_binop_assign! { impl BitAndAssign, bitand_assign for $t {&} }
            impl_unop! { impl Neg, neg, checked_neg for $t {} }
        )*
    };
}

checked_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
