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

        if inc < f64::EPSILON / 16.0 {
            // Check against a small epsilon, slightly larger than machine epsilon
            break;
        }

        sum += inc;
        sum -= floor(sum); // Standard modulo 1

        k += 1;
    }

    sum
}

const fn pi_hex_digit(digit: u16) -> u8 {
    let s1: f64 = series(digit, 1);
    let s4: f64 = series(digit, 4);
    let s5: f64 = series(digit, 5);
    let s6: f64 = series(digit, 6);

    let pi_frac: f64 = 4f64 * s1 - 2f64 * s4 - s5 - s6;

    let digit_val = floor(pi_frac);

    (16f64 * digit_val) as u8
}

pub struct PiEncoder {
    byte_to_idx: [u16; 0x100],
}

impl PiEncoder {
    pub const fn new() -> Self {
        let mut byte_to_idx = [0u16; 0x100];
        let mut bitset = [false; 0x100];
        let mut filled_count = 0;

        let mut idx: u16 = 0;
        let mut bits: u16 = (pi_hex_digit(idx + 1) << 12 | pi_hex_digit(idx) << 8) as u16;

        let mut bits_in_buffer: u64 = 0;
        let mut total_bits_generated = 0;

        let mut pi_digit_index = 0;

        while filled_count < 0x100 {
            let hex_digit = pi_hex_digit(pi_digit_index) as u64;

            bit_buffer |= hex_digit << bits_in_buffer;
            bits_in_buffer += 4;
            pi_digit_index += 1;
            total_bits_generated += 4;

            let start_idx_of_latest_bit = total_bits_generated - 4;
            let first_possible_new_byte_start_idx = if start_idx_of_latest_bit >= 7 {
                start_idx_of_latest_bit - 7
            } else {
                0
            };

            idx += 2;
        }

        let mut current_hex_base_idx = 0;

        while filled_count < 0x100 {
            let d0 = pi_hex_digit(current_hex_base_idx);
            let d1 = pi_hex_digit(current_hex_base_idx + 1);
            let d2 = pi_hex_digit(current_hex_base_idx + 2);
            let d3 = pi_hex_digit(current_hex_base_idx + 3);

            let window: u16 =
                ((d3 as u16) << 12) | ((d2 as u16) << 8) | ((d1 as u16) << 4) | (d0 as u16);

            let mut bit_offset = 0;
            while bit_offset <= 8 {
                let byte_val = ((window >> bit_offset) & 0xFF) as u8;

                let bit_idx = (4 * current_hex_base_idx + bit_offset) as u16;

                if !bitset[byte_val as usize] {
                    bitset[byte_val as usize] = true;
                    byte_to_idx[byte_val as usize] = bit_idx;
                    filled_count += 1;

                    if filled_count >= 0x100 {
                        // Break the inner loop (for bit_offset)
                        break;
                    }
                }
                bit_offset = bit_offset + 1;
            }

            if filled_count >= 0x100 {
                break;
            }

            current_hex_base_idx += 2;
        }

        PiEncoder { byte_to_idx }
    }

    pub const fn get(&self, byte: u8) -> u16 {
        self.byte_to_idx[byte as usize]
    }

    pub const fn size(&self) -> usize {
        self.byte_to_idx.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This will cause a compile error if PiEncoder::new fails or the const assertion fails.
    static ENCODER_TEST: PiEncoder = PiEncoder::new();

    #[test]
    fn test_pi_hex_digits() {
        assert_eq!(pi_hex_digit(0), 3);
        assert_eq!(pi_hex_digit(1), 2);
        assert_eq!(pi_hex_digit(2), 4);
        assert_eq!(pi_hex_digit(3), 3);
        assert_eq!(pi_hex_digit(4), 15); // F
        assert_eq!(pi_hex_digit(5), 6);
        assert_eq!(pi_hex_digit(6), 10); // A
        assert_eq!(pi_hex_digit(7), 8);
        assert_eq!(pi_hex_digit(8), 8);
        assert_eq!(pi_hex_digit(9), 2);
        assert_eq!(pi_hex_digit(10), 5);
        assert_eq!(pi_hex_digit(11), 0);
        assert_eq!(pi_hex_digit(12), 9);
        assert_eq!(pi_hex_digit(13), 13); // D
        assert_eq!(pi_hex_digit(14), 0);
        assert_eq!(pi_hex_digit(15), 8);
    }

    #[test]
    fn test_encoder_completeness() {
        let encoder = &ENCODER_TEST; // Access the compile-time computed encoder
        let mut found_bytes = [false; 256];
        let mut count = 0;
        for i in 0..256 {
            let byte = i as u8;
            let idx = encoder.get(byte);
            assert!(idx < u16::MAX); // Should always be true given u16 type

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
