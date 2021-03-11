#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_imports)]
#![allow(unused_imports)]

use core::fmt;

#[derive(Clone)]
/// Yet another bit field implementation.
/// This is a simple, small and hopefully efficient bit field implementation.
///
/// It is intended for cases where a program iterates over list or other usize indexed containers
/// and simple bit based bookkeeping is required.
///
pub struct Yabf {
    #[cfg(not(feature = "impl_smallvec"))]
    internals: Vec<u32>,
    #[cfg(feature = "impl_smallvec")]
    internals: smallvec::SmallVec<[u32; 4]>,
}

impl Yabf {
    /// Construct an empty bit field with enough capacity pre-allocated to store at least `n`
    /// bits.
    ///
    /// Will create a heap allocation only if `n` is larger than the inline capacity of the
    /// internal SmallVec.
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
            #[cfg(not(feature = "impl_smallvec"))]
            internals: Vec::<u32>::with_capacity((bits / 32) + 1),
            #[cfg(feature = "impl_smallvec")]
            internals: smallvec::SmallVec::<[u32; 4]>::with_capacity((bits / 32) + 1),
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
    ///
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
    ///
    /// ```
    pub fn set_bit(&mut self, n: usize, state: bool) {
        let word = n / 32;

        /* THIS DOES NOT WORK!!!
        #[cfg(all(feature="impl_smallvec",feature="extend_one"))]
        if word >= self.internals.len() {
            if self.internals.capacity() < word {
                self.internals.extend_reserve(word-self.internals.capacity());
            }
        }*/
        #[cfg(not(feature = "impl_smallvec"))]
        if word >= self.internals.len() {
            if self.internals.capacity() < word {
                self.internals
                    .reserve_exact(word - self.internals.capacity());
            }
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
            self.internals.push(bit_mask);
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
#[derive(Clone)]
pub struct YabfIterator<'s> {
    yabf: &'s Yabf,
    last_word: usize,
    // when this field is u16::MAX it means that the value was not
    // actually the 'last' value yet, but rather that the bit 0 should be tested.
    last_bit: u16,
}

impl<'s> YabfIterator<'s> {
    pub(crate) fn new(yabf: &'s Yabf) -> Self {
        Self {
            yabf,
            last_word: 0,
            last_bit: u16::MAX,
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

        let mut next_bit = if self.last_bit == u16::MAX {
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
            #[cfg(not(feature = "impl_smallvec"))]
            internals: Vec::<u32>::default(),
            #[cfg(feature = "impl_smallvec")]
            internals: smallvec::SmallVec::<[u32; 4]>::default(),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    #[cfg(feature = "impl_smallvec")]
    fn test_capacity_0() {
        let mut bf = crate::Yabf::default();

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
    fn test_capacity_1() {
        let bf = crate::Yabf::with_capacity(100);
        assert!(bf.is_empty());
        assert!(bf.capacity() >= 100);
    }

    #[test]
    fn test_capacity_2() {
        let mut bf = crate::Yabf::default();
        assert!(bf.is_empty());
        #[cfg(feature = "impl_smallvec")]
        assert_eq!(bf.capacity(), 4 * 32);
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
}
