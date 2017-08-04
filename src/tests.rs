use super::Checked;

macro_rules! test_unop {
    ($name:ident $t:ty: $op:tt $expr1:tt == $expr2:tt) => {
        #[test]
        fn $name() {
            let x = Checked::<$t>::from($expr1);
            let y = Checked::<$t>::from($expr2);
            assert_eq!($op x, y);
        }
    };
}

macro_rules! test_binop {
    ($name:ident $t:ty: $expr1:tt $op:tt $expr2:tt == $expr3:tt) => {
        #[test]
        fn $name() {
            let x = Checked::<$t>::from($expr1);
            let y = Checked::<$t>::from($expr2);
            let z = Checked::<$t>::from($expr3);
            let w = $expr2 as $t;
            assert_eq!(x $op y, z);
            assert_eq!(x $op w, z);
        }
    };
}

test_binop! (add1 u8: 5 + 6 == 11);
test_binop! (add2 u32: 3_000_000_000 + 2_000_000_000 == None);
test_binop! (add3 i32: (-2_000_000_000) + (-2_000_000_000) == None);
test_binop! (sub u8: 5 - 6 == None);
test_binop! (mul u8: 5 * 6 == 30);
test_binop! (mul2 i32: 2_000_000_000 * 3 == None);
test_binop! (div1 u8: 10 / 3 == 3);
test_binop! (div2 u8: 10 / 0 == None);
test_binop! (and u8: 5 & 6 == 4);
test_binop! (xor u8: 5 ^ 6 == 3);
test_binop! (or u8: 5 | 6 == 7);
test_binop! (rem u8: 10 % 3 == 1);
test_binop! (shl u32: 10 << 3 == 80);
test_binop! (shr u32: 80 >> 3 == 10);
test_unop! (neg1 u8: - 5 == None);
test_unop! (neg2 i8: - 5 == (-5));
test_unop! (not i8: ! 5 == (-6));