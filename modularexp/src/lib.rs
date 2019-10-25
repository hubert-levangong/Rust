mod modularexp {
    // calculates b^e mod m
    pub fn mod_and_square(b: u32, e :u32, m: u32) -> u32 {
        let mut i = e;
        let mut v = b;
        while i > 1 {
            v = (v * v) % m;
            i /= 2;
        }
        return v;
    }
}

pub fn square_and_multiply(base: u32, power: u32, modulo: u32) -> u32 {
    let mut i = 0;
    let mut val = 1;
    let mut p = power;
    while i < 32 {
        if (p & 0x1) == 1 {
            val = (val * modularexp::mod_and_square(base, u32::pow(2, i), modulo)) % modulo ;
        }
        p = p >> 1;
        i += 1;
    }
    return val;
}
