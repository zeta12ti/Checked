use super::Checked;

#[test]
fn readme_example() {
    let x = Checked::from(1_000_000_000_u32) * 3 + 2_000_000_000;
    match *x {
        Some(y) => println!("Didn't overflow: x is {}.", y),
        None => println!("The arithmetic overflowed."),
    }
}

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
    assert!(!(Checked::from(None) <= Checked::from(1_000_u32)));
}

#[test]
fn new_checked() {
    assert_eq!(Checked::new(100), Checked(Some(100)))
}

#[test]
fn from_checked() {
    assert_eq!(Checked::from(100_u32), Checked(Some(100_u32)));
    assert_eq!(Checked::from(Some(100_u32)), Checked(Some(100_u32)));
    assert_eq!(Checked::<u64>::from(None), Checked::<u64>(None));
}

#[test]
fn consuming_option_methods() {
    let x = Checked::from(1_000_i64);
    assert_eq!(x.expect("AHH!!!"), 1_000);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.unwrap(), 1_000);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.unwrap_or(10), 1_000);

    let x = Checked::<i64>::from(None);
    assert_eq!(x.unwrap_or(10), 10);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.unwrap_or_else(|| 10), 1_000);

    let x = Checked::<i64>::from(None);
    assert_eq!(x.unwrap_or_else(|| 10), 10);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.map(|x| x + 15), Some(1_015));

    let x = Checked::<i64>::from(None);
    assert_eq!(x.map(|x| x + 15), None);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.map_or(10, |x| x + 15), 1_015);

    let x = Checked::<i64>::from(None);
    assert_eq!(x.map_or(10, |x| x + 15), 10);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.map_or_else(|| 10, |x| x + 15), 1_015);

    let x = Checked::<i64>::from(None);
    assert_eq!(x.map_or_else(|| 10, |x| x + 15), 10);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.ok_or(14), Ok(1_000));

    let x = Checked::<i64>::from(None);
    assert_eq!(x.ok_or(14), Err(14));

    let x = Checked::from(1_000_i64);
    assert_eq!(x.ok_or_else(|| 14), Ok(1_000));

    let x = Checked::<i64>::from(None);
    assert_eq!(x.ok_or_else(|| 14), Err(14));

    let x = Checked::from(1_000_i64);
    assert_eq!(x.and(Some(19)), Some(19));

    let x = Checked::<i64>::from(None);
    assert_eq!(x.and(Some(19)), None);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.and_then(|x| Some(x + 15)), Some(1_015));

    let x = Checked::<i64>::from(None);
    assert_eq!(x.and_then(|x| Some(x + 15)), None);

    let x = Checked::from(1_000_i64);
    assert_eq!(x.or(Some(19)), Some(1_000));

    let x = Checked::<i64>::from(None);
    assert_eq!(x.or(Some(19)), Some(19));

    let x = Checked::from(1_000_i64);
    assert_eq!(x.or_else(|| Some(19)), Some(1_000));

    let x = Checked::<i64>::from(None);
    assert_eq!(x.or_else(|| Some(19)), Some(19));
}

#[test]
fn ref_option_methods() {
    let x = Checked::from(1_000_i64);
    let y = Checked::<i64>::from(None);

    assert!(x.is_some());
    assert!(!y.is_some());

    assert!(!x.is_none());
    assert!(y.is_none());

    Option::as_ref(&x);
    Option::as_ref(&y);

    println!("Numbers in x: ");
    for t in x.iter() {
        println!("{:?}", t);
    }


    println!("Numbers in y: ");
    for t in y.iter() {
        println!("{:?}", t);
    }
}

#[test]
fn mut_ref_option_methods() {
    let mut x = Checked::from(1_000_i64);
    let mut y = Checked::<i64>::from(None);

    Option::as_mut(&mut x);
    Option::as_mut(&mut y);

    println!("Numbers in x: ");
    for t in x.iter_mut() {
        println!("{:?}", t);
    }


    println!("Numbers in y: ");
    for t in y.iter_mut() {
        println!("{:?}", t);
    }

    let z = x.take();

    assert_eq!(z, Some(1_000_i64));
    assert_eq!(x, Checked::<i64>::from(None));

    let w = y.take();
    assert_eq!(w, None);
    assert_eq!(y, Checked::<i64>::from(None));
}
