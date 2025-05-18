const fn power(base: f64, exp: i32) -> f64 {
    match exp {
        0 => 1.0,
        1 => base,
        2 => base * base,
        3 => base * base * base,
        _ => {
            let mut result = 1.0;
            let mut e = exp.abs();
            let mut b = base;

            while e > 0 {
                if e % 2 == 1 {
                    result *= b;
                }
                b *= b;
                e /= 2;
            }

            if exp < 0 { 1.0 / result } else { result }
        }
    }
}

pub const fn binpow(a: u64, mut b: u64, mod_val: u64) -> u64 {
    let mut a = a % mod_val;
    let mut result: u64 = 1;

    while b > 0 {
        if b & 1 == 1 {
            result = (result * a) % mod_val;
        }
        a = (a * a) % mod_val;
        b >>= 1;
    }

    result
}
