# Checked
Implements a wrapper over the primitive Rust types that better indicates overflow during arithmetic.

The struct Checked is actually a wrapper over an Option, so if x is a Checked object, then x.0 can be used like an Option.

## Example

```
extern crate checked
use checked::Checked;

    let x = Checked::from(1_000_000_000_u32) * 3 + 2_000_000_000;
    match x.0 {
        Some(y) => println!("Didn't overflow: x is {}.", y),
        None => println!("The arithmetic overflowed."),
    }
```

Note that Add\<T\> is implemented for Checked\<T\> for all the primitive integer types T (u8, i16, etc.) so really, only the left-most integer needs to be a Checked object. However, It's impossible to implement Add\<Checked\<T\>\> for T due to the orphan rule: neither Add nor the integer primitives are defined in this crate. This means that if you add/subtract/etc. a Checked int and a normal int, you have to put the Checked int on the left.

This struct is based on std::num::Wrapping, except using checked arithmetic instead of wrapped arithmetic.

## Contributing
I may try to add more features in time: make an Issue or Pull request to get the ball rolling.

## Upcoming features
More detailed docs are coming.
