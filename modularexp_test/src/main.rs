extern crate modularexp;

fn main() {
    let v :u32 = 3;
    let e :u32 = 200;
    let m :u32 = 50;

    println!("{}^{} mod {} = {}", v, e, m, modularexp::square_and_multiply(v, e, m));
}
