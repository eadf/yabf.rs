extern crate test;
use crate::Yabf;
use test::Bencher;

#[cfg(test)]
#[bench]
// Yabf is roughly 15% faster than num_bigint::BigUint in this specific test
// Smallvec: bench:  11,386,583 ns/iter (+/- 30,916)
// std::Vec: bench:  10,617,075 ns/iter (+/- 47,050)
fn bench_1(b: &mut Bencher) {
    #[cfg(feature="impl_smallvec")]
    println!("running bench with Smallvec");
    #[cfg(not(feature="impl_smallvec"))]
    println!("running bench with std::Vec");

    let mut bf = Yabf::default();
    b.iter(move || {
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
    });
}

// BigUint bench:  14,251,391 ns/iter (+/- 168,832)
#[cfg(test)]
#[bench]
fn bench_2(b: &mut Bencher) {
    let mut bf = num_bigint::BigUint::default();
    b.iter(move || {
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
    });
}
