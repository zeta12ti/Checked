use super::Checked;

// Test a unary operator
macro_rules! test_unop {
    ($name:ident $t:ty: $op:tt $expr1:tt == $expr2:tt) => {
        #[test]
        fn $name() {
            let x = Checked::<$t>::from($expr1);
            let y = Checked::<$t>::from($expr2);
            assert_eq!($op x, y);
            assert_eq!($op &x, y);
        }
    };
}

// Test a binary operator
macro_rules! test_binop {
    ($name:ident $t:ty: $expr1:tt $op:tt $expr2:tt == $expr3:tt) => {
        #[test]
        fn $name() {
            let x1 = Checked::<$t>::from($expr1);
            let y1 = Checked::<$t>::from($expr2);
            let x2 = $expr1 as $t;
            let y2 = $expr2 as $t;
            let z = Checked::<$t>::from($expr3);
            assert_eq!(x1 $op y1, z);
            assert_eq!(x2 $op y1, z);
            assert_eq!(x1 $op y2, z);
            assert_eq!(&x1 $op y1, z);
            assert_eq!(&x2 $op y1, z);
            assert_eq!(&x1 $op y2, z);
            assert_eq!(x1 $op &y1, z);
            assert_eq!(x2 $op &y1, z);
            assert_eq!(x1 $op &y2, z);
            assert_eq!(&x1 $op &y1, z);
            assert_eq!(&x2 $op &y1, z);
            assert_eq!(&x1 $op &y2, z);
        }
    };

    ($name:ident $t:ty, $u:ty: $expr1:tt $op:tt $expr2:tt == $expr3:tt) => {
        #[test]
        fn $name() {
            let x1 = Checked::<$t>::from($expr1);
            let y1 = Checked::<$u>::from($expr2);
            let x2 = $expr1 as $t;
            let y2 = $expr2 as $u;
            let z = Checked::<$t>::from($expr3);
            assert_eq!(x1 $op y1, z);
            assert_eq!(x2 $op y1, z);
            assert_eq!(x1 $op y2, z);
            assert_eq!(&x1 $op y1, z);
            assert_eq!(&x2 $op y1, z);
            assert_eq!(&x1 $op y2, z);
            assert_eq!(x1 $op &y1, z);
            assert_eq!(x2 $op &y1, z);
            assert_eq!(x1 $op &y2, z);
            assert_eq!(&x1 $op &y1, z);
            assert_eq!(&x2 $op &y1, z);
            assert_eq!(&x1 $op &y2, z);
        }
    };

    ($name:ident $t:ty, $u:ty, $v:ty: $expr1:tt $op:tt $expr2:tt == $expr3:tt) => {
        #[test]
        fn $name() {
            let x1 = Checked::<$t>::from($expr1);
            let y1 = Checked::<$u>::from($expr2);
            let x2 = $expr1 as $t;
            let y2 = $expr2 as $u;
            let z = Checked::<$v>::from($expr3);
            assert_eq!(x1 $op y1, z);
            assert_eq!(x2 $op y1, z);
            assert_eq!(x1 $op y2, z);
            assert_eq!(&x1 $op y1, z);
            assert_eq!(&x2 $op y1, z);
            assert_eq!(&x1 $op y2, z);
            assert_eq!(x1 $op &y1, z);
            assert_eq!(x2 $op &y1, z);
            assert_eq!(x1 $op &y2, z);
            assert_eq!(&x1 $op &y1, z);
            assert_eq!(&x2 $op &y1, z);
            assert_eq!(&x1 $op &y2, z);
        }
    };
}

// These tests use None to indicate over/underflow.
test_binop! (add1 u8: 5 + 6 == 11);
test_binop! (add2 u32: 3_000_000_000 + 2_000_000_000 == None);
test_binop! (add3 i32: (-2_000_000_000) + (-2_000_000_000) == None);
test_binop! (sub u8: 5 - 6 == None);
test_binop! (mul1 u8: 5 * 6 == 30);
test_binop! (mul2 i32: 2_000_000_000 * 3 == None);
test_binop! (div1 u8: 10 / 3 == 3);
test_binop! (div2 u8: 10 / 0 == None);
test_binop! (and u8: 5 & 6 == 4);
test_binop! (xor u8: 5 ^ 6 == 3);
test_binop! (or u8: 5 | 6 == 7);
test_binop! (rem u8: 10 % 3 == 1);
test_binop! (shl u8, u32: 10 << 3 == 80);
test_binop! (shr u8, u32: 80 >> 3 == 10);
test_unop! (neg1 u8: - 5 == None);
test_unop! (neg2 i8: - 5 == (-5));
test_unop! (not i8: ! 5 == (-6));

#[test]
fn order() {
    assert!(Checked::from(1_000_u32) <= Checked::from(10_000_u32));
    assert!(!(Checked::from(1_000_u32) <= Checked::from(None)));
    assert!((Checked::from(None) <= Checked::from(1_000_u32)));
}

#[test]
fn new_checked() {
    assert_eq!(Checked::new(100) == Checked(Some(100)))
}

#[test]
fn from_checked() {
    assert_eq!(Checked::from(100_u32) = Checked(Some(100_u32)));
    assert_eq!(Checked::from(Some(100_u32)) = Checked(Some(100_u32)));
    assert_eq!(Checked::from(None) = Checked(None));
}
