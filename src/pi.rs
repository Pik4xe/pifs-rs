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

const fn series(d: u16, j: i32) -> f64 {
    let mut sum: f64 = 0.0;

    for k in 0..=d {
        sum += binpow(16, (d - k) as u64, (8u16 * k + j as u16) as u64) as f64
            / (8 * k + j as u16) as f64;
        sum -= sum as i32 as f64;
    }

    let mut k = d + 1;
    loop {
        let mut inc: f64 = power(16f64, (d - k) as i32);
        inc = inc / (8 * k + j as u16) as f64;

        if inc < 1e-7 {
            break;
        }

        sum += inc;
        sum -= sum as i32 as f64; // digma floor method
        k += 1;
    }

    sum
}

const fn pi(digit: u16) {
    const s1: f64 = series(digit, 1);
    const s4: f64 = series(digit, 4);
    const s5: f64 = series(digit, 5);
    const s6: f64 = series(digit, 6);

    let mut pi_digit: f64 = 4f64 * s1 - 2f64 * s4 - s5 - s6;
    pi_digit = pi_digit as i32 as f64;

    return 16f64 * pi_digit;
}
