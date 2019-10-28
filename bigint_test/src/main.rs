extern crate num_bigint;

use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

fn main() {
    let n : usize = 5000;
    let mut val0 : BigUint = Zero::zero();
    let mut val1 : BigUint = One::one();

    for _ in 0..n {
        let val2 = val0 + &val1;
        val0 = replace(&mut val1, val2);
    }
    println!(" val0: {}", val0);
}
