use criterion::{criterion_group, criterion_main, Criterion};
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

// BigUint bench: time: [13.838 ms 13.850 ms 13.866 ms]
// This is an unfair comparison, BigUint do shrink resizing when needed.
#[cfg(test)]
fn bench_biguint(c: &mut Criterion) {
    let mut bf = num_bigint::BigUint::default();
    c.bench_function("BigUint", |b| {
        b.iter(|| {
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
        })
    });
}

#[cfg(feature = "impl_smallvec")]
criterion_group!(benches1, bench_vec, bench_smallvec, bench_biguint);
#[cfg(not(feature = "impl_smallvec"))]
criterion_group!(benches1, bench_vec, bench_biguint);
criterion_main!(benches1);
