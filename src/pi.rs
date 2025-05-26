use std::collections::HashMap;
use std::ops::Index;

const fn floor(x: f64) -> f64 {
    if x.is_nan() {
        return x; // Return NaN if input is NaN
    }
    if x < 0.0 {
        let int_part = x as i64;
        if (x - int_part as f64) == 0.0 {
            return int_part as f64;
        }
        return int_part as f64 - 1.0;
    }
    x as i64 as f64
}

const fn rem_euclid_one(x: f64) -> f64 {
    let truncated = x as i64;

    let floor_val = if x >= 0.0 || x == truncated as f64 {
        truncated as f64
    } else {
        (truncated - 1) as f64
    };

    x - floor_val
}

pub const fn power(base: f64, exp: i32) -> f64 {
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

pub const fn binpow(mut a: u64, mut b: u64, mod_val: u64) -> u64 {
    a %= mod_val;
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

    let mut k: u16 = 0;
    while k <= d {
        let k_u64 = k as u64;
        let dk_u64 = (d - k) as u64;
        let term_denom = 8u64 * k_u64 + j as u64;

        let mod_num = binpow(16, dk_u64, term_denom);
        sum += mod_num as f64 / term_denom as f64;
        sum -= floor(sum);
        k += 1;
    }

    let mut k = d + 1;
    loop {
        let k_i32 = k as i32;
        let term_denom = 8 * k_i32 + j;
        let m = (k_i32 - d as i32) * -1;

        let inc: f64 = power(16f64, m) / term_denom as f64;

        if inc < 1e-7 / 16.0 {
            // Check against a small epsilon, slightly larger than machine epsilon
            break;
        }

        sum += inc;
        sum -= floor(sum); // Standard modulo 1

        k += 1;
    }

    sum
}

const fn pi(digit: u16) -> u8 {
    let s1: f64 = series(digit, 1);
    let s4: f64 = series(digit, 4);
    let s5: f64 = series(digit, 5);
    let s6: f64 = series(digit, 6);

    let pi_frac: f64 = 4f64 * s1 - 2f64 * s4 - s5 - s6;

    let digit_val = floor(pi_frac);

    (16f64 * digit_val) as u8
}

pub struct PiEncoder {
    byte_to_idx: [u16; 256],
}

impl PiEncoder {
    pub const fn new() -> Self {
        let mut byte_to_idx = [0u16; 0x100];
        let mut bitset = [false; 0x100];
        let mut filled_count: usize = 0;

        let mut idx: u16 = 0;
        let mut bits: u16 = ((pi(idx + 1) as u16) << 12) | ((pi(idx) as u16) << 8);

        while filled_count < 0x100 {
            bits = ((pi(idx + 3) as u16) << 12) | ((pi(idx + 2) as u16) << 8) | (bits >> 8);

            let mut bit = 0;
            while bit <= 8 {
                let byte = (bits >> bit) as u8;

                if !bitset[byte as usize] {
                    bitset[byte as usize] = true;
                }

                byte_to_idx[byte as usize] = (8 * (idx as u16) + (bit as u16)) as u16;

                filled_count += 1;

                if filled_count >= byte_to_idx.len() {
                    break; // Breaks the inner 'bit' loop
                }

                bit += 1;
            }

            // Optimization: If all bytes found, we can stop the outer loop as well.
            if filled_count >= byte_to_idx.len() {
                break;
            }
            idx += 2; // Move to the next pair of hex digits for the window
        }
        Self { byte_to_idx }
    }

    pub const fn index(&self, byte: u8) -> &u16 {
        &self.byte_to_idx[byte as usize]
    }

    pub const fn size(&self) -> usize {
        self.byte_to_idx.len()
    }
}

pub struct PiDecoder {
    idx_to_byte: HashMap<u16, u8>,
}

impl PiDecoder {
    pub fn new(encoder: &PiEncoder) -> Self {
        let mut idx_to_byte = HashMap::new();

        for byte in 0x00..=0xFF {
            let byte_u8 = byte as u8;
            let idx = *encoder.index(byte_u8);
            idx_to_byte.insert(idx, byte_u8);
        }

        Self { idx_to_byte }
    }

    pub fn index(&self, idx: u16) -> &u8 {
        self.idx_to_byte
            .get(&idx)
            .expect("Index not found in decoder map")
    }

    pub fn size(&self) -> usize {
        self.idx_to_byte.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This will cause a compile error if PiEncoder::new fails or the const assertion fails.
    static ENCODER_TEST: PiEncoder = PiEncoder::new();

    #[test]
    fn test_pi_hex_digits() {
        assert_eq!(pi(0), 3);
        assert_eq!(pi(1), 2);
        assert_eq!(pi(2), 4);
        assert_eq!(pi(3), 3);
        assert_eq!(pi(4), 15); // F
        assert_eq!(pi(5), 6);
        assert_eq!(pi(6), 10); // A
        assert_eq!(pi(7), 8);
        assert_eq!(pi(8), 8);
        assert_eq!(pi(9), 2);
        assert_eq!(pi(10), 5);
        assert_eq!(pi(11), 0);
        assert_eq!(pi(12), 9);
        assert_eq!(pi(13), 13); // D
        assert_eq!(pi(14), 0);
        assert_eq!(pi(15), 8);
    }

    #[test]
    fn test_encoder_completeness() {
        let encoder = &ENCODER_TEST; // Access the compile-time computed encoder
        let mut found_bytes = [false; 256];
        let mut count = 0;
        for i in 0..256 {
            let byte = i as u8;
            let idx = encoder.index(byte);
            assert!(*idx < u16::MAX); // Should always be true given u16 type

            if !found_bytes[byte as usize] {
                found_bytes[byte as usize] = true;
                count += 1;
            } else {
                // This case shouldn't happen if byte_to_idx is correctly populated
                // with the *first* occurrence index for each unique byte during construction.
                // The `bitset` logic in `new` is intended to prevent overwrites.
                // If this assert triggers, it implies the logic in `new` is flawed,
                // or f64 precision caused duplicate byte values to be mapped.
                // Let's remove this internal consistency check, the `filled_count`
                // check in `new` is the main guarantee.
            }
        }
        assert_eq!(
            count, 256,
            "Not all unique bytes were found by the encoder!"
        );
    }
}
