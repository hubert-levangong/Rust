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

// nlen: intended length of the modulus n
// exp: public verification exponent
// seed: string of  2 * security_length (associated to nlen) bits from RBG
fn prime_generator(nlen: usize, exp : u32, seed: Vec<u8>) -> BigUint {
    let mut prime_candidate : BigUint = Zero::zero();

    return prime_candidate;
}


fn is_prime(candidate : BigUint) -> bool {
    return false;
}


// Returning twice the security strength bits of random data
fn get_seed(nlen: usize) -> Vec<u8> {
    let vect = Vec::with_capacity(2 * nlen);

    return vect;
}