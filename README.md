# Checked
Implements a wrapper over the primitive Rust types that better indicates overflow during arithmetic.

The struct `Checked` derefs into an option that either contains the number, or `None` if an overflow occurred.
This means that all of `Option`'s methods can be used.

Note that `Add<T>` is implemented for `Checked<T>` for all the primitive integer types `T` (`u8`, `i16`, etc.) so really, only the left-most integer needs to be a `Checked` object.
Once the arithmetic hits a `Checked<T`> on the right OR left, all the remaining results are `Checked` too.
Just make sure there's a `Checked` somewhere before the first potential overflow.

This struct is based on `std::num::Wrapping`, except using checked arithmetic instead of wrapped arithmetic.
There was an RFC that mentioned this approach, but as far as I know, it was never implemented anywhere.

## Example

```
extern crate checked
use checked::Checked;

fn main() {
    let x = Checked::from(1_000_000_000_u32) * 3 + 2_000_000_000;
    match *x {
        Some(y) => println!("Didn't overflow: x is {}.", y),
        None => println!("The arithmetic overflowed."),
    }
}
```

## Documentation
Documentation may be found [here](https://docs.rs/checked).

## Contributing
I may try to add more features in time: make an Issue or Pull request to get the ball rolling.
