extern crate test;
use crate::Yabf;
use test::Bencher;

#[cfg(test)]
#[bench]
/// Yabf is just barely faster than num_bigint::BigUint in this test
fn bench_1(b: &mut Bencher) {
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
        }
    });
}

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
        }
    });
}
