[![crates.io](https://img.shields.io/crates/v/yabf.svg)](https://crates.io/crates/yabf)
[![Documentation](https://docs.rs/yabf/badge.svg)](https://docs.rs/yabf)
[![Workflow](https://github.com/eadf/yabf.rs/workflows/Rust/badge.svg)](https://github.com/eadf/yabf.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/yabf.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/yabf.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/yabf/0.3.0/status.svg)](https://deps.rs/crate/yabf/0.3.0)
![license](https://img.shields.io/crates/l/yabf)

# This crate is deprecated
This crate can be replaced with [vob](https://crates.io/crates/vob) if you add this trait to your code:

```rust
// u32 is slightly faster for random access w/o any bit operations
pub(crate) type VobU32 = vob::Vob<u32>;

pub(crate) trait GrowingVob {
    /// Will create a new Vob and fill it with `default`
    fn fill(initial_size: usize, default:bool) -> VobU32;
    /// Grow to fit new size, set ´bit´ to ´state´ value
    fn set_grow(&mut self, bit: usize, state: bool) -> bool;
    /// get() with default value `false`
    fn get_f(&self, bit: usize) -> bool;
}

impl GrowingVob for VobU32 {
    #[inline]
    fn fill(initial_size: usize, default:bool) -> Self {
        let mut v = Self::new_with_storage_type(0);
        v.resize(initial_size, default);
        v
    }

    #[inline]
    fn set_grow(&mut self, bit: usize, state: bool) -> bool {
        if bit >= self.len() {
            self.resize(bit + 64, false);
        }
        self.set(bit, state)
    }

    #[inline]
    fn get_f(&self, bit: usize) -> bool {
        self.get(bit).unwrap_or(false)
    }
}
```
# Yabf
Just what the world needed - yet another bit field struct.

This is a small and simple implementation. It only has the basic functionality of a bit field:
 * Set arbitrary bit (if you set the millionth bit the list will use at least 125KB of heap space) 
 * Get bit value 
 * An iterator over the set bit indices. O(size of container)
 * The container never shrinks.

yabf::Yabf is a `std::vec::Vec` based bit field
```rust
let mut a = Yabf::default();
let mut b = Yabf::with_capacity(12345);
// bits are false by default
assert_eq!(a.bit(45), false);
a.set_bit(12345, true);
assert_eq!(a.bit(12345), true);
b.set_bit(345, true);
assert_eq!(b.bit(345), true);
```

yabf::SmallYabf is a `smallvec::SmallVec` based bit field. The struct will be entirely
stack allocated if it contains less than 129 bits.
```rust
let mut a = SmallYabf::default();
let mut b = SmallYabf::with_capacity(12345);
a.set_bit(45, false);
b.set_bit(12345, true);
assert_eq!(a.bit(45), false);
assert_eq!(b.bit(12345), true);
assert_eq!(a.bit(12345), false);
```

## Rust cargo
yabf::SmallYabf is enabled by default, disable like this:
```toml
yabf = {version="0.3",default-features=false}
```
yabf::SmallYabf is enabled by default
```toml
yabf = {version="0.3"}
```


## License

Licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)

at your option.
