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
