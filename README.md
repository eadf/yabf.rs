[![Crates.io](https://meritbadge.herokuapp.com/yabf)](https://crates.io/crates/yabf)
[![Documentation](https://docs.rs/yabf/badge.svg)](https://docs.rs/yabf)
[![Workflow](https://github.com/eadf/yabf.rs/workflows/Rust/badge.svg)](https://github.com/eadf/yabf.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/yabf.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/yabf.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/yabf/0.1.0/status.svg)](https://deps.rs/crate/yabf/0.1.0)

# Yabf
Just what the world needed - yet another bit field struct.

This is a small and simple implementation. It only has the basic functionality of a bit field:
 * Set arbitary bit (if you set the millionth bit the list will use at least 125KB of heap space) 
 * Get bit value 
 * An iterator over the set bit indices.

The default implementation uses [smallvec](https://crates.io/crates/smallvec) as a container, so the struct will be
stack allocated if it contains less than 129 bits.

## Rust cargo
Use with std::vec::Vec. Vec is slightly faster for larger bit fields
```cargo
yabf = {version="^0.1.0",default-features=false}
```
Use with smallvec::SmallVec. SmallVec can be stack allocated for really small bit fields
```cargo
yabf = {version="^0.1.0"}
```

## License

Licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)

at your option.
