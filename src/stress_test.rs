extern crate rand;
extern crate rand_chacha;

use fnv::FnvHashSet;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
#[cfg(feature = "impl_smallvec")]
use yabf::SmallYabf;
use yabf::Yabf;

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(38);
    let mut q = Yabf::with_capacity(1024);
    #[cfg(feature = "impl_smallvec")]
    let mut q1 = SmallYabf::with_capacity(1024);

    let mut in_q = FnvHashSet::<usize>::default();
    let mut to_add = FnvHashSet::<usize>::default();
    let mut to_del = FnvHashSet::<usize>::default();
    let mut transactions: usize = 0;

    let mut loop_number = 0;
    let max_size = 14;
    let min_size = 7;

    println!("running stress test with std::vec::Vec");
    #[cfg(feature = "impl_smallvec")]
    println!("...and with Smallvec");

    loop {
        loop_number += 1;

        to_add.clear();
        to_del.clear();

        loop {
            let key: usize = rng.gen_range(0..4096);
            // only add keys not in q
            if in_q.contains(&key) {
                continue;
            }
            to_add.insert(key);
            if to_add.len() >= 5 || to_add.len() + in_q.len() > max_size {
                break;
            }
        }
        loop {
            if in_q.len() - to_del.len() == 0 {
                break;
            }
            // only delete keys found in q
            let key = *in_q.iter().nth(rng.gen_range(0..in_q.len())).unwrap();
            to_del.insert(key);
            if to_del.len() >= 5 || in_q.len() - to_del.len() < min_size {
                break;
            }
        }

        println!("Adding {:?}", to_add);
        for key in to_add.iter() {
            q.set_bit(*key, true);
            #[cfg(feature = "impl_smallvec")]
            q1.set_bit(*key, true);
            in_q.insert(*key);
        }

        println!("Deleting {:?}", to_del);
        for key in to_del.iter() {
            q.set_bit(*key, false);
            if q.bit(*key) {
                println!("Error: {} should not be set", key);
                panic!();
            }
            #[cfg(feature = "impl_smallvec")]
            {
                q1.set_bit(*key, false);
                if q1.bit(*key) {
                    println!("Error: {} should not be set", key);
                    panic!();
                }
            }
            in_q.remove(key);
        }
        transactions += to_del.len() + to_add.len();

        println!("Checking {:?}", in_q);
        for key in in_q.iter() {
            if !q.bit(*key) {
                println!("Error: {} was not set", key);
                println!("{:?}", q);
                panic!();
            }
            #[cfg(feature = "impl_smallvec")]
            if !q1.bit(*key) {
                println!("Error: {} was not set", key);
                println!("{:?}", q);
                panic!();
            }
            let another_key = key + 1;
            if !in_q.contains(&another_key) {
                if q.bit(another_key) {
                    println!("Error: {} should not be set", another_key);
                    println!("{:?}", q);
                    panic!();
                }
                #[cfg(feature = "impl_smallvec")]
                if q1.bit(another_key) {
                    println!("Error: {} should not be set", another_key);
                    println!("{:?}", q);
                    panic!();
                }
            }
        }

        assert_eq!(
            in_q.iter()
                .sorted_unstable()
                .map(|x| *x)
                .collect::<Vec<usize>>(),
            q.into_iter().collect::<Vec<usize>>()
        );

        #[cfg(feature = "impl_smallvec")]
        assert_eq!(
            in_q.iter()
                .sorted_unstable()
                .map(|x| *x)
                .collect::<Vec<usize>>(),
            q1.into_iter().collect::<Vec<usize>>()
        );
        print!(
            "**** loop {}, transactions {} ***** vec.len {} vec.capacity {}",
            loop_number,
            transactions,
            q.internal_len(),
            q.capacity()
        );
        #[cfg(feature = "impl_smallvec")]
        print!(
            "  smallvec.len {} smallvec.capacity {}",
            q1.internal_len(),
            q1.capacity()
        );
        println!();
    }
}
