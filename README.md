[![Crates.io](https://meritbadge.herokuapp.com/yabf)](https://crates.io/crates/yabf)
[![Documentation](https://docs.rs/yabf/badge.svg)](https://docs.rs/yabf)
[![Workflow](https://github.com/eadf/yabf.rs/workflows/Rust/badge.svg)](https://github.com/eadf/yabf.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/yabf.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/yabf.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/yabf/0.1.1/status.svg)](https://deps.rs/crate/yabf/0.1.1)

# Yabf
Just what the world needed - yet another bit field struct.

This is a small and simple implementation. It only has the basic functionality of a bit field:
 * Set arbitary bit (if you set the millionth bit the list will use at least 125KB of heap space) 
 * Get bit value 
 * An iterator over the set bit indices. O(size of container)
 * The container never shrinks.

yabf::Yabf is a `std::vec::Vec` based bit field
```rust
let mut a = Yabf::default();
let mut b = Yabf::with_capacity(12345);
a.set_bit(45,true);
b.set_bit(12345,true);
assert!(!a.bit(12345));
assert!(a.bit(45));
assert!(b.bit(12345));
```

yabf::SmallYabf is a `smallvec::SmallVec` based bit field. The struct will be entirely
stack allocated if it contains less than 129 bits.
```rust
let mut a = SmallYabf::default();
let mut b = SmallYabf::with_capacity(12345);
a.set_bit(45,true);
b.set_bit(12345,true);
assert!(!a.bit(12345));
assert!(a.bit(45));
assert!(b.bit(12345));
```

## Rust cargo
yabf::SmallYabf is enabled by default, disable like this:
```toml
yabf = {version="^0.1.1",default-features=false}
```
yabf::SmallYabf is enabled by default
```toml
yabf = {version="^0.1.1"}
```


## License

Licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)

at your option.
