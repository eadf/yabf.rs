use yabf::Yabf;
use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(test)]
// Smallvec: bench:  [13.926 ms 13.929 ms 13.933 ms]
// std::Vec: bench:  [12.546 ms 12.554 ms 12.562 ms]
fn bench_1(c: &mut Criterion) {
    #[cfg(feature="impl_smallvec")]
    println!("running bench with Smallvec");
    #[cfg(not(feature="impl_smallvec"))]
    println!("running bench with std::vec::Vec");

    let mut bf = Yabf::default();
    c.bench_function("bench1", |b| b.iter({||
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
    }));
}

// BigUint bench: time: [14.123 ms 14.320 ms 14.605 ms]
#[cfg(test)]
fn bench_2(c: &mut Criterion) {
    let mut bf = num_bigint::BigUint::default();
    c.bench_function("bench_2", |b| b.iter({||
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
    }));
}

criterion_group!(benches1, bench_1, bench_2);
criterion_main!(benches1);