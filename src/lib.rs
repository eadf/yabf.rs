//! # Yabf
//! Just what the world needed - yet another bit field struct.
//!
//! This is a small and simple implementation. It only has the basic functionality of a bit field:
//!  * Set arbitary bit (if you set the millionth bit the list will use at least 125KB of heap space)
//!  * Get bit value
//!  * An iterator over the set bit indices. O(size of container)
//!  * The container never shrinks.
//!
//! The bits are stored in plain (non-sparse) arrays/vectors.
//!
//! ```{rust}
//! use yabf::Yabf;
//! let mut a = Yabf::default();
//! let mut b = Yabf::with_capacity(12345);
//! a.set_bit(45,true);
//! b.set_bit(12345,true);
//! assert!(!a.bit(12345));
//! assert!(a.bit(45));
//! assert!(b.bit(12345));
//! ```
//!
//! ```{rust}
//!# #[cfg(feature = "smallvec")] {
//! use yabf::SmallYabf;
//! let mut a = SmallYabf::default();
//! let mut b = SmallYabf::with_capacity(12345);
//! a.set_bit(45,true);
//! b.set_bit(12345,true);
//! assert!(!a.bit(12345));
//! assert!(a.bit(45));
//! assert!(b.bit(12345));
//!# }
//! ```

#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_imports)]
#![allow(unused_imports)]

use core::fmt;
use std::ops;

#[derive(Clone)]
/// Yet another bit field implementation.
/// This is a simple, small and hopefully efficient bit field implementation.
///
/// It is intended for cases where a program iterates over list or other usize indexed containers
/// and simple bit based bookkeeping is required.
///
pub struct Yabf {
    internals: Vec<u32>,
}

impl Yabf {
    /// Construct an empty bit field with enough capacity pre-allocated to store at least `n`
    /// bits.
    ///
    /// ```
    /// # use yabf::Yabf;
    ///
    /// let bf = Yabf::with_capacity(100);
    ///
    /// assert!(bf.is_empty());
    /// assert!(bf.capacity() >= 100);
    /// ```
    pub fn with_capacity(bits: usize) -> Self {
        Self {
            internals: Vec::<u32>::with_capacity((bits / 32) + 1),
        }
    }

    /// Returns the value of the 'n':th bit in the bit field.
    ///
    /// ```
    /// # use yabf::Yabf;
    ///
    /// let mut bf = Yabf::default();
    ///
    /// assert!(bf.is_empty());
    /// assert!(!bf.bit(10));
    /// bf.set_bit(10,true);
    /// assert!(bf.bit(10));
    /// ```
    pub fn bit(&self, n: usize) -> bool {
        let word = n / 32;
        if word < self.internals.len() {
            if let Some(value) = self.internals.get(word) {
                return value & (1u32 << (n % 32)) != 0;
            }
        }
        false
    }

    /// Sets the 'n':th bit in the bit field. If the bit field capacity is not large enough
    /// more space will be allocated.
    ///
    /// ```
    /// # use yabf::Yabf;
    ///
    /// let mut bf = Yabf::default();
    ///
    /// assert!(bf.is_empty());
    /// assert!(!bf.bit(10));
    /// bf.set_bit(10,true);
    /// assert!(bf.bit(10));
    /// ```
    pub fn set_bit(&mut self, n: usize, state: bool) {
        let word = n / 32;

        if word >= self.internals.capacity() {
            self.internals
                .reserve_exact(1 + word - self.internals.capacity());
        }

        if self.internals.is_empty() {
            // prime the array so that the math works
            self.internals.push(0);
        }
        let bit_mask = 1_u32 << (n % 32);

        if word >= self.internals.len() {
            let old_word = self.internals.len() - 1;
            for _i in old_word..word - 1 {
                self.internals.push(0);
            }
            if state {
                self.internals.push(bit_mask);
            } else {
                self.internals.push(0);
            }
        } else if state {
            self.internals[word] |= bit_mask;
        } else {
            self.internals[word] &= !bit_mask;
        }
    }

    /// Returns `true` if all bits are set to `false`
    #[inline]
    pub fn is_empty(&self) -> bool {
        for e in self.internals.iter() {
            if *e != 0 {
                return false;
            }
        }
        true
    }

    /// The number of bits the bit field can hold without reallocating
    #[inline]
    pub fn capacity(&self) -> usize {
        self.internals.capacity() * 32
    }

    /// The len() of the internal vector
    #[inline]
    pub fn internal_len(&self) -> usize {
        self.internals.len()
    }

    /// Reserve capacity for `additional_bits` more bits to be inserted.
    ///
    /// May reserve more space to avoid frequent re-allocations.
    ///
    /// Panics if the capacity computation overflows `usize`.
    #[inline]
    pub fn reserve(&mut self, additional_bits: usize) {
        let additional = additional_bits / 32;
        let additional = if additional < 1 { 1 } else { additional };
        self.internals.reserve(additional);
    }

    /// Remove all elements from the vector.
    /// This method simply delegates clear() to the underlying vector
    /// std::vec::Vec or smallvec::SmallVec. So what actually happen depends on the
    /// feature set.
    #[inline]
    pub fn clear(&mut self) {
        self.internals.clear();
    }
}

/// Iterator over the bits set to true in the bit field container.
/// Will iterate over the bits from lowest to to highest.
/// This is a relatively expensive O(size of container) operation.
#[derive(Clone)]
pub struct YabfIterator<'s> {
    yabf: &'s Yabf,
    last_word: usize,
    // when this field is usize::MAX it means that the value was not
    // actually the 'last' value yet, but rather that the bit 0 should be tested.
    last_bit: usize,
}

impl<'s> YabfIterator<'s> {
    pub(crate) fn new(yabf: &'s Yabf) -> Self {
        Self {
            yabf,
            last_word: 0,
            last_bit: usize::MAX,
        }
    }
}

impl<'a> IntoIterator for &'a Yabf {
    type Item = usize;
    type IntoIter = YabfIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        YabfIterator::new(self)
    }
}

impl<'s> Iterator for YabfIterator<'s> {
    type Item = usize;

    /// Maybe not the most efficient iterator possible, it iterates over each bit and tests
    /// if it is set and return the corresponding bit number.
    /// It skips to next word if the word bits (32 bits) are all zero, or all upper or lower
    /// 16 bits are zero
    fn next(&mut self) -> Option<usize> {
        let mut next_word = self.last_word;

        let mut next_bit = if self.last_bit == usize::MAX {
            0
        } else {
            self.last_bit + 1
        };

        loop {
            if next_bit > 31 {
                next_bit = 0;
                next_word += 1;
                if next_word >= self.yabf.internals.len() {
                    return None;
                }
            }
            let sample = self.yabf.internals[next_word];
            // Skip if all bits are zero
            if sample == 0 {
                next_word += 1;
                if next_word >= self.yabf.internals.len() {
                    return None;
                }
                next_bit = 0;
                continue;
            }
            // Skip if the lower 16 bits are all zero
            if next_bit < 16 && sample & 0xFFFF == 0 {
                next_bit = 16;
            }
            // Skip if the high 16 bits are all zero
            if next_bit >= 16 && sample & 0xFFFF0000 == 0 {
                next_word += 1;
                if next_word >= self.yabf.internals.len() {
                    return None;
                }
                next_bit = 0;
                continue;
            }

            //println!("Sample:{:?} word:{}, bit:{}", sample, next_word, next_bit);
            while next_bit < 32 {
                if sample & (1u32 << next_bit) != 0 {
                    self.last_bit = next_bit;
                    self.last_word = next_word;
                    return Some(next_word * 32 + (next_bit as usize));
                }
                next_bit += 1;
            }
        }
    }
}

impl fmt::Debug for Yabf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.internals.is_empty() {
            write!(f, "Yabf:0x0")
        } else {
            write!(f, "Yabf:0x")?;
            for i in self.internals.iter().rev() {
                write!(f, "{:08X}_", *i)?;
            }
            Ok(())
        }
    }
}

impl Default for Yabf {
    #[inline]
    fn default() -> Self {
        Self {
            internals: Vec::<u32>::default(),
        }
    }
}

/// bit or assign operation.
/// This is a relatively expensive O(size of container) operation.
/// ```
/// # use yabf::Yabf;
///
/// let mut a = Yabf::default();
/// let mut b = Yabf::default();
/// a.set_bit(45,true);
/// b.set_bit(12345,true);
/// assert!(!a.bit(12345));
/// assert!(a.bit(45));
/// a |= &b;
/// assert!(a.bit(12345));
/// assert!(a.bit(45));
/// ```
impl ops::BitOrAssign<&Yabf> for Yabf {
    fn bitor_assign(&mut self, other: &Yabf) {
        if self.internals.len() < other.internals.len() {
            for v in other
                .internals
                .iter()
                .enumerate()
                .take(self.internals.len())
            {
                self.internals[v.0] |= v.1;
            }
            if other.internals.len() > self.internals.capacity() {
                self.internals
                    .reserve_exact(other.internals.len() - self.internals.capacity());
            }
            for v in other.internals.iter().skip(self.internals.len()) {
                self.internals.push(*v);
            }
        } else {
            for v in other.internals.iter().enumerate() {
                self.internals[v.0] |= v.1;
            }
        }
    }
}

#[derive(Clone)]
/// Yet another bit field implementation.
/// This is a simple, small and hopefully efficient bit field implementation. It uses SmallVec
/// as an internal container. The first 128 bits will be stored on the stack.
///
/// It is intended for cases where a program iterates over list or other usize indexed containers
/// and simple bit based bookkeeping is required.
///
#[cfg(feature = "smallvec")]
pub struct SmallYabf {
    internals: smallvec::SmallVec<[u32; 4]>,
}

#[cfg(feature = "smallvec")]
impl SmallYabf {
    /// Construct an empty bit field with enough capacity pre-allocated to store at least `n`
    /// bits.
    ///
    /// Will create a heap allocation only if `n` is larger than the inline capacity of the
    /// internal SmallVec.
    ///
    /// ```
    /// # use yabf::SmallYabf;
    ///
    /// let bf = SmallYabf::with_capacity(100);
    ///
    /// assert!(bf.is_empty());
    /// assert!(bf.capacity() >= 100);
    /// ```
    pub fn with_capacity(bits: usize) -> Self {
        Self {
            internals: smallvec::SmallVec::<[u32; 4]>::with_capacity((bits / 32) + 1),
        }
    }

    /// Returns the value of the 'n':th bit in the bit field.
    ///
    /// ```
    /// # use yabf::SmallYabf;
    ///
    /// let mut bf = SmallYabf::default();
    ///
    /// assert!(bf.is_empty());
    /// assert!(!bf.bit(10));
    /// bf.set_bit(10,true);
    /// assert!(bf.bit(10));
    /// ```
    pub fn bit(&self, n: usize) -> bool {
        let word = n / 32;
        if word < self.internals.len() {
            if let Some(value) = self.internals.get(word) {
                return value & (1u32 << (n % 32)) != 0;
            }
        }
        false
    }

    /// Sets the 'n':th bit in the bit field. If the bit field capacity is not large enough
    /// more space will be allocated.
    ///
    /// ```
    /// # use yabf::SmallYabf;
    ///
    /// let mut bf = SmallYabf::default();
    ///
    /// assert!(bf.is_empty());
    /// assert!(!bf.bit(10));
    /// bf.set_bit(10,true);
    /// assert!(bf.bit(10));
    /// ```
    pub fn set_bit(&mut self, n: usize, state: bool) {
        let word = n / 32;

        if self.internals.is_empty() {
            // prime the array so that the math works
            self.internals.push(0);
        }
        let bit_mask = 1_u32 << (n % 32);

        if word >= self.internals.len() {
            self.internals.reserve(1 + word - self.internals.len());
            let old_word = self.internals.len() - 1;
            for _i in old_word..word - 1 {
                self.internals.push(0);
            }
            if state {
                self.internals.push(bit_mask);
            } else {
                self.internals.push(0);
            }
        } else if state {
            self.internals[word] |= bit_mask;
        } else {
            self.internals[word] &= !bit_mask;
        }
    }

    /// Returns `true` if all bits are set to `false`
    #[inline]
    pub fn is_empty(&self) -> bool {
        for e in self.internals.iter() {
            if *e != 0 {
                return false;
            }
        }
        true
    }

    /// The number of bits the bit field can hold without reallocating
    #[inline]
    pub fn capacity(&self) -> usize {
        self.internals.capacity() * 32
    }

    /// The len() of the internal vector
    #[inline]
    pub fn internal_len(&self) -> usize {
        self.internals.len()
    }

    /// Reserve capacity for `additional_bits` more bits to be inserted.
    ///
    /// May reserve more space to avoid frequent re-allocations.
    ///
    /// Panics if the capacity computation overflows `usize`.
    #[inline]
    pub fn reserve(&mut self, additional_bits: usize) {
        let additional = additional_bits / 32;
        let additional = if additional < 1 { 1 } else { additional };
        self.internals.reserve(additional);
    }

    /// Remove all elements from the vector.
    /// This method simply delegates clear() to the underlying vector
    /// std::vec::Vec or smallvec::SmallVec. So what actually happen depends on the
    /// feature set.
    #[inline]
    pub fn clear(&mut self) {
        self.internals.clear();
    }
}

#[cfg(feature = "smallvec")]
/// Iterator over the bits set to true in the bit field container.
/// Will iterate over the bits from lowest to to highest.
/// This is a relatively expensive O(size of container) operation.
#[derive(Clone)]
pub struct SmallYabfIterator<'s> {
    yabf: &'s SmallYabf,
    last_word: usize,
    // when this field is usize::MAX it means that the value was not
    // actually the 'last' value yet, but rather that the bit 0 should be tested.
    last_bit: usize,
}

#[cfg(feature = "smallvec")]
impl<'s> SmallYabfIterator<'s> {
    pub(crate) fn new(yabf: &'s SmallYabf) -> Self {
        Self {
            yabf,
            last_word: 0,
            last_bit: usize::MAX,
        }
    }
}

#[cfg(feature = "smallvec")]
impl<'a> IntoIterator for &'a SmallYabf {
    type Item = usize;
    type IntoIter = SmallYabfIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SmallYabfIterator::new(self)
    }
}

#[cfg(feature = "smallvec")]
impl<'s> Iterator for SmallYabfIterator<'s> {
    type Item = usize;

    /// Maybe not the most efficient iterator possible, it iterates over each bit and tests
    /// if it is set and return the corresponding bit number.
    /// It skips to next word if the word bits (32 bits) are all zero, or all upper or lower
    /// 16 bits are zero
    fn next(&mut self) -> Option<usize> {
        let mut next_word = self.last_word;

        let mut next_bit = if self.last_bit == usize::MAX {
            0
        } else {
            self.last_bit + 1
        };

        loop {
            if next_bit > 31 {
                next_bit = 0;
                next_word += 1;
                if next_word >= self.yabf.internals.len() {
                    return None;
                }
            }
            let sample = self.yabf.internals[next_word];
            // Skip if all bits are zero
            if sample == 0 {
                next_word += 1;
                if next_word >= self.yabf.internals.len() {
                    return None;
                }
                next_bit = 0;
                continue;
            }
            // Skip if the lower 16 bits are all zero
            if next_bit < 16 && sample & 0xFFFF == 0 {
                next_bit = 16;
            }
            // Skip if the high 16 bits are all zero
            if next_bit >= 16 && sample & 0xFFFF0000 == 0 {
                next_word += 1;
                if next_word >= self.yabf.internals.len() {
                    return None;
                }
                next_bit = 0;
                continue;
            }

            //println!("Sample:{:?} word:{}, bit:{}", sample, next_word, next_bit);
            while next_bit < 32 {
                if sample & (1u32 << next_bit) != 0 {
                    self.last_bit = next_bit;
                    self.last_word = next_word;
                    return Some(next_word * 32 + (next_bit as usize));
                }
                next_bit += 1;
            }
        }
    }
}

#[cfg(feature = "smallvec")]
impl fmt::Debug for SmallYabf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.internals.is_empty() {
            write!(f, "SmallYabf:0x0")
        } else {
            write!(f, "SmallYabf:0x")?;
            for i in self.internals.iter().rev() {
                write!(f, "{:08X}_", *i)?;
            }
            Ok(())
        }
    }
}

#[cfg(feature = "smallvec")]
impl Default for SmallYabf {
    #[inline]
    fn default() -> Self {
        Self {
            internals: smallvec::SmallVec::<[u32; 4]>::default(),
        }
    }
}

#[cfg(feature = "smallvec")]
/// bit or assign operation
/// This is a relatively expensive O(size of container) operation.
/// ```
/// # use yabf::SmallYabf;
///
/// let mut a = SmallYabf::default();
/// let mut b = SmallYabf::default();
/// a.set_bit(45,true);
/// b.set_bit(12345,true);
/// assert!(!a.bit(12345));
/// assert!(a.bit(45));
/// a |= &b;
/// assert!(a.bit(12345));
/// assert!(a.bit(45));
/// ```
impl ops::BitOrAssign<&SmallYabf> for SmallYabf {
    fn bitor_assign(&mut self, other: &SmallYabf) {
        if self.internals.len() < other.internals.len() {
            for v in other
                .internals
                .iter()
                .enumerate()
                .take(self.internals.len())
            {
                self.internals[v.0] |= v.1;
            }
            if other.internals.len() > self.internals.capacity() {
                self.internals
                    .reserve_exact(other.internals.len() - self.internals.capacity());
            }
            for v in other.internals.iter().skip(self.internals.len()) {
                self.internals.push(*v);
            }
        } else {
            for v in other.internals.iter().enumerate() {
                self.internals[v.0] |= v.1;
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_capacity_1() {
        let bf = crate::Yabf::with_capacity(100);
        assert!(bf.is_empty());
        assert!(bf.capacity() >= 100);
    }

    #[test]
    fn test_capacity_2() {
        let mut bf = crate::Yabf::default();
        assert!(bf.is_empty());
        bf.set_bit(129, true);
        bf.reserve(10);
        assert!(bf.capacity() >= 5 * 32);
    }

    #[test]
    fn test_iter() {
        let mut bf = crate::Yabf::default();
        bf.set_bit(129, true);
        println!("{:?}", bf.into_iter().collect::<Vec<usize>>());
        assert_eq!(bf.into_iter().next().unwrap(), 129);
        bf.set_bit(29, true);
        bf.set_bit(167, true);
        println!("{:?}", bf.into_iter().collect::<Vec<usize>>());
    }

    #[test]
    fn test_or() {
        let mut a = crate::Yabf::default();
        let mut b = crate::Yabf::default();
        a.set_bit(45, true);
        b.set_bit(44, true);
        b.set_bit(4444, true);
        assert!(a.bit(45));
        assert!(!a.bit(44));
        assert!(!a.bit(4444));
        a |= &b;
        assert!(a.bit(45));
        assert!(a.bit(44));
        assert!(a.bit(4444));

        let mut b = crate::Yabf::default();
        b.set_bit(23, true);
        assert!(!a.bit(23));
        assert!(a.bit(45));
        //println!("{:?}",a);
        a |= &b;
        //println!("{:?}",a);
        assert!(a.bit(23));
        assert!(a.bit(45));
        assert!(a.bit(44));
        assert!(a.bit(4444));
    }
}

#[cfg(feature = "smallvec")]
#[cfg(test)]
mod test_small {

    #[test]
    #[cfg(feature = "smallvec")]
    fn test_capacity_0() {
        let mut bf = crate::SmallYabf::default();

        assert!(bf.is_empty());
        assert_eq!(bf.capacity(), 4 * 32);
        bf.set_bit(30 + 32 * 0, true);
        assert_eq!(bf.bit(30 + 32 * 0), true);
        assert_eq!(bf.capacity(), 4 * 32);
        bf.set_bit(30 + 32 * 1, true);
        assert_eq!(bf.bit(30 + 32 * 1), true);
        assert_eq!(bf.capacity(), 4 * 32);
        bf.set_bit(30 + 32 * 2, true);
        assert_eq!(bf.bit(30 + 32 * 2), true);
        assert_eq!(bf.capacity(), 4 * 32);
        bf.set_bit(30 + 32 * 3, true);
        assert_eq!(bf.bit(30 + 32 * 3), true);
        assert_eq!(bf.capacity(), 4 * 32);
        bf.set_bit(30 + 32 * 4, true);
        assert_eq!(bf.bit(30 + 32 * 4), true);
        assert!(bf.capacity() >= 5 * 32);
    }

    #[test]
    #[cfg(feature = "smallvec")]
    fn test_capacity_1() {
        let bf = crate::SmallYabf::with_capacity(100);
        assert!(bf.is_empty());
        assert!(bf.capacity() >= 100);
    }

    #[test]
    #[cfg(feature = "smallvec")]
    fn test_capacity_2() {
        let mut bf = crate::SmallYabf::default();
        assert!(bf.is_empty());
        assert_eq!(bf.capacity(), 4 * 32);
        bf.set_bit(129, true);
        bf.reserve(10);
        assert!(bf.capacity() >= 5 * 32);
    }

    #[test]
    #[cfg(feature = "smallvec")]
    fn test_iter() {
        let mut bf = crate::SmallYabf::default();
        bf.set_bit(129, true);
        println!("{:?}", bf.into_iter().collect::<Vec<usize>>());
        assert_eq!(bf.into_iter().next().unwrap(), 129);
        bf.set_bit(29, true);
        bf.set_bit(167, true);
        println!("{:?}", bf.into_iter().collect::<Vec<usize>>());
    }

    #[test]
    fn readme_1() {
        use crate::Yabf;
        let mut a = Yabf::default();
        let mut b = Yabf::with_capacity(12345);
        // bits are false by default
        assert_eq!(a.bit(45), false);
        a.set_bit(12345, true);
        assert_eq!(a.bit(12345), true);
        b.set_bit(345, true);
        assert_eq!(b.bit(345), true);
    }

    #[test]
    #[cfg(feature = "smallvec")]
    fn readme_2() {
        use crate::SmallYabf;
        let mut a = SmallYabf::default();
        let mut b = SmallYabf::with_capacity(12345);
        a.set_bit(45, false);
        b.set_bit(12345, true);
        assert_eq!(a.bit(45), false);
        assert_eq!(b.bit(12345), true);
        assert_eq!(a.bit(12345), false);
    }
}
