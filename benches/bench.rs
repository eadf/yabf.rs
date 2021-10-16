use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yabf::Yabf;

#[cfg(test)]
// std::Vec: bench:  [12.335 ms 12.337 ms 12.339 ms]
fn bench_vec(c: &mut Criterion) {
    println!("running bench with std::vec::Vec");

    let mut bf = Yabf::default();
    c.bench_function("Yabf", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                for i in (0..2090_usize).rev() {
                    bf.set_bit(i, true);
                }
                for i in 0..2090_usize {
                    assert!(bf.bit(i));
                }
                for i in (0..2090_usize).rev() {
                    bf.set_bit(i, false);
                }
                for i in 0..2090_usize {
                    assert!(!bf.bit(i));
                }
            }
        })
    });
}

#[cfg(feature = "impl_smallvec")]
#[cfg(test)]
// Smallvec: bench:  [12.430 ms 12.432 ms 12.433 ms]
fn bench_smallvec(c: &mut Criterion) {
    println!("running bench with Smallvec");

    let mut bf = Yabf::default();
    c.bench_function("SmallYabf", |b| {
        b.iter(|| {
            black_box(for _ in 0..1000 {
                for i in (0..2090_usize).rev() {
                    bf.set_bit(i, true);
                }
                for i in 0..2090_usize {
                    assert!(bf.bit(i));
                }
                for i in (0..2090_usize).rev() {
                    bf.set_bit(i, false);
                }
                for i in 0..2090_usize {
                    assert!(!bf.bit(i));
                }
            })
        })
    });
}

// BigUint bench: time: [13.838 ms 13.850 ms 13.866 ms]
// This is an unfair comparison, BigUint do shrink resizing when needed.
#[cfg(test)]
fn bench_biguint(c: &mut Criterion) {
    let mut bf = num_bigint::BigUint::default();
    c.bench_function("BigUint", |b| {
        black_box(b.iter(|| {
            for _ in 0..1000 {
                for i in (0..2090_u64).rev() {
                    bf.set_bit(i, true);
                }
                for i in 0..2090_u64 {
                    assert!(bf.bit(i));
                }
                for i in (0..2090_u64).rev() {
                    bf.set_bit(i, false);
                }
                for i in 0..2090_u64 {
                    assert!(!bf.bit(i));
                }
            }
        }))
    });
}

// Vob bench: time: [13.838 ms 13.850 ms 13.866 ms]
#[cfg(test)]
fn bench_vob(c: &mut Criterion) {
    type VobU32 = vob::Vob<u32>;

    trait GrowingVob {
        /// Will create a new Vob and fill it with `default`
        fn fill(initial_size: usize, default: bool) -> VobU32;
        /// Grow to fit new size, set ´bit´ to ´state´ value
        fn set_grow(&mut self, bit: usize, state: bool) -> bool;
        /// get() with default value `false`
        fn get_f(&self, bit: usize) -> bool;
    }

    impl GrowingVob for VobU32 {
        #[inline]
        fn fill(initial_size: usize, default: bool) -> Self {
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

    let mut vob = VobU32::new_with_storage_type(0);
    c.bench_function("Vob", |b| {
        b.iter(|| {
            black_box(for _ in 0..1000 {
                for i in (0..2090).rev() {
                    vob.set_grow(i, true);
                }
                for i in 0..2090 {
                    assert!(vob.get_f(i));
                }
                for i in (0..2090).rev() {
                    vob.set_grow(i, false);
                }
                for i in 0..2090 {
                    assert!(!vob.get_f(i));
                }
            })
        })
    });
}

#[cfg(feature = "impl_smallvec")]
criterion_group!(
    benches1,
    bench_vec,
    bench_smallvec,
    bench_biguint,
    bench_vob
);
#[cfg(not(feature = "impl_smallvec"))]
criterion_group!(benches1, bench_vec, bench_biguint, bench_vob);
criterion_main!(benches1);
