extern crate num_bigint;
extern crate rand;

use num_bigint::{BigUint, ToBigInt, RandBigInt};
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

    let mut rng = rand::thread_rng();
    let a = rng.gen_bigint(1000);
    let low = 1.to_bigint().unwrap();
    let high = 10000.to_bigint().unwrap();
    let b = rng.gen_bigint_range(&low, &high);
    println!("a: {}", a);
    println!("b: {}", b);
    println!("a*b: {}", a * b);
}
