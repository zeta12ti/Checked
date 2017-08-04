use std::fmt;
use std::ops::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
pub struct Checked<T>(pub Option<T>);

impl<T> Checked<T> {
    pub fn new(x: T) -> Checked<T> {
        Checked(Some(x))
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

// I'd like to
// impl<T, U> From<U> where T: From<U> for Checked<T>
// in the obvious way, but that "conflicts" with the default impl From<T> for T.
// This would subsume both the above Froms.

macro_rules! sh_impl {
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

        impl ShlAssign<$f> for Checked<$t> {
            fn shl_assign(&mut self, other: $f) {
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

        impl ShrAssign<$f> for Checked<$t> {
            fn shr_assign(&mut self, other: $f) {
                *self = *self >> other;
            }
        }
    };
}

macro_rules! sh_impl_all {
    ($($t:ident)*) => ($(
        // When checked_shX is added for other shift sizes, uncomment some of these.
        // sh_impl! { $t, u8 }
        // sh_impl! { $t, u16 }
        sh_impl! { $t, u32 }
        //sh_impl! { $t, u64 }
        //sh_impl! { $t, usize }

        //sh_impl! { $t, i8 }
        //sh_impl! { $t, i16 }
        //sh_impl! { $t, i32 }
        //sh_impl! { $t, i64 }
        //sh_impl! { $t, isize }
    )*)
}

sh_impl_all! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

// implements the unary operator "op &T"
// based on "op T" where T is expected to be `Copy`able
macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
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
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
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


macro_rules! checked_impl {
    ($($t:ty)*) => {
        $(
            impl Add for Checked<$t> {
                type Output = Checked<$t>;

                fn add(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(x.checked_add(y)),
                        _ => Checked(None),
                    }
                }
            }

            impl Add<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn add(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(x.checked_add(other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl Add, add for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl Add, add for Checked<$t>, $t }

            impl AddAssign for Checked<$t> {
                fn add_assign(&mut self, other: Checked<$t>) {
                    *self = *self + other;
                }
            }

            impl AddAssign<$t> for Checked<$t> {
                fn add_assign(&mut self, other: $t) {
                    *self = *self + other;
                }
            }

            impl Sub for Checked<$t> {
                type Output = Checked<$t>;

                fn sub(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(x.checked_sub(y)),
                        _ => Checked(None),
                    }
                }
            }

            impl Sub<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn sub(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(x.checked_sub(other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl Sub, sub for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl Sub, sub for Checked<$t>, $t }

            impl SubAssign for Checked<$t> {
                fn sub_assign(&mut self, other: Checked<$t>) {
                    *self = *self - other;
                }
            }

            impl SubAssign<$t> for Checked<$t> {
                fn sub_assign(&mut self, other: $t) {
                    *self = *self - other;
                }
            }

            impl Mul for Checked<$t> {
                type Output = Checked<$t>;

                fn mul(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(x.checked_mul(y)),
                        _ => Checked(None),
                    }
                }
            }

            impl Mul<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn mul(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(x.checked_mul(other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl Mul, mul for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl Mul, mul for Checked<$t>, $t }

            impl MulAssign for Checked<$t> {
                fn mul_assign(&mut self, other: Checked<$t>) {
                    *self = *self * other;
                }
            }

            impl MulAssign<$t> for Checked<$t> {
                fn mul_assign(&mut self, other: $t) {
                    *self = *self * other;
                }
            }

            impl Div for Checked<$t> {
                type Output = Checked<$t>;

                fn div(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(x.checked_div(y)),
                        _ => Checked(None),
                    }
                }
            }

            impl Div<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn div(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(x.checked_div(other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl Div, div for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl Div, div for Checked<$t>, $t }

            impl DivAssign for Checked<$t> {
                fn div_assign(&mut self, other: Checked<$t>) {
                    *self = *self / other;
                }
            }

            impl DivAssign<$t> for Checked<$t> {
                fn div_assign(&mut self, other: $t) {
                    *self = *self / other;
                }
            }

            impl Rem for Checked<$t> {
                type Output = Checked<$t>;

                fn rem(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(x.checked_rem(y)),
                        _ => Checked(None),
                    }
                }
            }

            impl Rem<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn rem(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(x.checked_rem(other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl Rem, rem for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl Rem, rem for Checked<$t>, $t }

            impl RemAssign for Checked<$t> {
                fn rem_assign(&mut self, other: Checked<$t>) {
                    *self = *self % other;
                }
            }

            impl RemAssign<$t> for Checked<$t> {
                fn rem_assign(&mut self, other: $t) {
                    *self = *self % other;
                }
            }

            impl Not for Checked<$t> {
                type Output = Checked<$t>;

                fn not(self) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(Some(!x)),
                        None => Checked(None)
                    }
                }
            }

            forward_ref_unop! { impl Not, not for Checked<$t> }

            impl BitXor for Checked<$t> {
                type Output = Checked<$t>;

                fn bitxor(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(Some(x ^ y)),
                        _ => Checked(None),
                    }
                }
            }

            impl BitXor<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn bitxor(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(Some(x ^ other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl BitXor, bitxor for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl BitXor, bitxor for Checked<$t>, $t }

            impl BitXorAssign for Checked<$t> {
                fn bitxor_assign(&mut self, other: Checked<$t>) {
                    *self = *self ^ other;
                }
            }

            impl BitXorAssign<$t> for Checked<$t> {
                fn bitxor_assign(&mut self, other: $t) {
                    *self = *self ^ other;
                }
            }

            impl BitOr for Checked<$t> {
                type Output = Checked<$t>;

                fn bitor(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(Some(x | y)),
                        _ => Checked(None),
                    }
                }
            }

            impl BitOr<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn bitor(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(Some(x | other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl BitOr, bitor for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl BitOr, bitor for Checked<$t>, $t }

            impl BitOrAssign for Checked<$t> {
                fn bitor_assign(&mut self, other: Checked<$t>) {
                    *self = *self | other;
                }
            }

            impl BitOrAssign<$t> for Checked<$t> {
                fn bitor_assign(&mut self, other: $t) {
                    *self = *self | other;
                }
            }

            impl BitAnd for Checked<$t> {
                type Output = Checked<$t>;

                fn bitand(self, other: Checked<$t>) -> Checked<$t> {
                    match (self.0, other.0) {
                        (Some(x), Some(y)) => Checked(Some(x & y)),
                        _ => Checked(None),
                    }
                }
            }

            impl BitAnd<$t> for Checked<$t> {
                type Output = Checked<$t>;

                fn bitand(self, other: $t) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(Some(x & other)),
                        _ => Checked(None),
                    }
                }
            }

            forward_ref_binop! { impl BitAnd, bitand for Checked<$t>, Checked<$t> }
            forward_ref_binop! { impl BitAnd, bitand for Checked<$t>, $t }

            impl BitAndAssign for Checked<$t> {
                fn bitand_assign(&mut self, other: Checked<$t>) {
                    *self = *self & other;
                }
            }

            impl BitAndAssign<$t> for Checked<$t> {
                fn bitand_assign(&mut self, other: $t) {
                    *self = *self & other;
                }
            }

            impl Neg for Checked<$t> {
                type Output = Checked<$t>;

                fn neg(self) -> Checked<$t> {
                    match self.0 {
                        Some(x) => Checked(x.checked_neg()),
                        None => Checked(None)
                    }
                }
            }

            forward_ref_unop! { impl Neg, neg for Checked<$t> }
        )*
    };
}

checked_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
