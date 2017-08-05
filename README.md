# Checked
Implements a wrapper over the primitive Rust types that better indicates overflow during arithmetic.

The struct Checked is actually a wrapper over an Option, so if x is a Checked object, then x.0 can be used like an Option.

Note that Add\<T\> is implemented for Checked\<T\> for all the primitive integer types T (u8, i16, etc.) so really, only the left-most integer needs to be a Checked object.
Once the arithmetic hits a Checked\<T\> on the right OR left, all the remaining results are Checked too.
Just make sure there's a Checked somewhere before the first potential overflow.

This struct is based on std::num::Wrapping, except using checked arithmetic instead of wrapped arithmetic.

## Example

```
extern crate checked
use checked::Checked;

fn main() {
    let x = Checked::from(1_000_000_000_u32) * 3 + 2_000_000_000;
    match x.0 {
        Some(y) => println!("Didn't overflow: x is {}.", y),
        None => println!("The arithmetic overflowed."),
    }
}
```

## Documentation
Documentation may be found [here](https://docs.rs/checked/0.3.0/checked/).

## Contributing
I may try to add more features in time: make an Issue or Pull request to get the ball rolling.
