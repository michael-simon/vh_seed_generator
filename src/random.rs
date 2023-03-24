#[derive(Debug, Clone)]
pub struct VHRandom {
    seed: u32,
    // The code used to generate the seed, using the internal representation instead of characters
    code: [u8; 10],
}

const CODE_CONVERSION_STR: &str = "BCDFGHJKLMNPQRSTAIUEO VWXYZ.,&♂♀";

impl VHRandom {
    /// Creates a new RNG from a seed directly, also creating a valid code that can generate
    /// that initial seed.
    pub fn from_seed(seed: u32) -> Self {
        VHRandom {
            seed,
            code: [
                (seed >> 0x1c) as u8,
                (seed >> 0x18 & 0xf) as u8,
                (seed >> 0x14 & 0xf) as u8,
                (seed >> 0x10 & 0xf) as u8,
                (seed >> 0x0c & 0xf) as u8,
                (seed >> 0x08 & 0xf) as u8,
                (seed >> 0x04 & 0xf) as u8,
                (seed & 0xf) as u8,
                0,
                0,
            ],
        }
    }

    /// Creates a new RNG from a valid string code. The code must be the full 10 characters,
    /// pad the code with spaces first for "shorter" codes.
    pub fn from_code(code: &str) -> Option<Self> {
        let raw_code: Vec<u8> = code
            .chars()
            .map(|c| {
                CODE_CONVERSION_STR
                    .chars()
                    .position(|x| x == c)
                    .map(|x| x as u8)
            })
            .collect::<Option<_>>()?;

        if raw_code.len() != 10 {
            return None;
        }

        // This here is *almost* the result of packing all of the raw letter bytes into
        // 2 32 bit values from high to low, except there's a bug in how it splits
        // a value that would cross the boundary between the two. The code ends up or-ing
        // the letter's full value on the first 32 bits, and
        let mut c1: u32 = 0;
        let mut c1_shift: u32 = 0;
        let mut c2: u32 = 0;
        let mut c2_shift: u32 = 0;
        let mut code_array: [u8; 10] = [0; 10];

        for (i, &letter) in raw_code.iter().enumerate() {
            code_array[i] = letter;
            if c1_shift < 32 {
                let shift: u32 = if letter & 0x10 == 0 { 4 } else { 5 };
                let letter_mask = (1 << shift) - 1;

                let letter_shifted: u32 = ((letter & letter_mask) as u32)
                    .wrapping_shl((32 - c1_shift).saturating_sub(shift));

                c1 |= letter_shifted;
                c1_shift += shift;

                if c1_shift > 32 {
                    let bits_to_move = c1_shift - 32;
                    let moving_bits_mask = (1 << bits_to_move) - 1;
                    c2 = ((letter & moving_bits_mask) as u32) << (32 - bits_to_move);
                    c2_shift = bits_to_move;
                }
            } else if c2_shift < 32 {
                let shift: u32 = if letter & 0x10 == 0 { 4 } else { 5 };
                let letter_mask = (1 << shift) - 1;
                let letter_shifted: u32 = ((letter & letter_mask) as u32)
                    .wrapping_shl((32 - c2_shift).saturating_sub(shift));

                c2 |= letter_shifted;
                c2_shift += shift;
            }
        }

        Some(VHRandom {
            seed: c1 ^ (c2 >> 32 - c2_shift),
            code: code_array,
        })
    }

    ///Returns the current value of the RNG seed
    pub fn get_seed(&self) -> u32 {
        self.seed
    }

    /// Sets the current value of the RNG seed. Required because of a weird method
    /// of dealing with seeds that fail to place features successfully.
    pub fn set_seed(&mut self, new_seed: u32) {
        self.seed = new_seed
    }

    /// Returns the code, converting to a string able to be input to Virtual Hydlide
    /// Panics if the internal code values are out of range
    pub fn get_code(&self) -> String {
        self.code
            .iter()
            .map(|&v| CODE_CONVERSION_STR.chars().nth(v as usize).unwrap())
            .collect()
    }

    /// Returns a number between 0 and n-1 inclusive, using the method used by Virtual Hydlide
    pub fn rand(&mut self, range: u32) -> u32 {
        self.seed = (!self.seed).wrapping_mul(0x1863d);
        (self.seed >> 0x10) % range
    }

    /// Returns a random byte, using the method used by Virtual Hydlide
    pub fn rand_byte(&mut self) -> u8 {
        self.seed = (!self.seed).wrapping_mul(0x1863d);
        ((self.seed >> 0x10) & 0xff) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::VHRandom;

    #[test]
    fn zero_seed() {
        let rng = VHRandom::from_seed(0);
        assert!(rng.get_seed() == 0);
        assert!(rng.get_code() == "BBBBBBBBBB");
    }

    #[test]
    fn all_bs_code() {
        let rng = VHRandom::from_code("BBBBBBBBBB").unwrap();
        assert!(rng.get_seed() == 0);
    }

    #[test]
    fn all_w_code() {
        let rng = VHRandom::from_code("♀♀♀♀♀♀♀♀♀♀").unwrap();
        assert!(rng.get_seed() == 0xfffc0000)
    }

    #[test]
    fn all_a_code() {
        let rng = VHRandom::from_code("AAAAAAAAAA").unwrap();
        assert!(rng.get_seed() == 0x84214a40)
    }

    #[test]
    fn same_seed_codes() {
        let rng1 = VHRandom::from_code("♀♀♀♀♀♀♀♀♀♀").unwrap();
        let rng2 = VHRandom::from_code("TTTQBBBBBB").unwrap();
        assert!(rng1.get_seed() == rng2.get_seed());
    }

    #[test]
    fn random_seed_code_gen() {
        for _ in 0..1000 {
            let rng1 = VHRandom::from_seed(rand::random());
            let rng2 = VHRandom::from_code(&rng1.get_code()).unwrap();
            assert!(rng1.get_seed() == rng2.get_seed());
        }
    }

    #[test]
    fn known_seed_sequence() {
        let mut rng = VHRandom::from_code("CCCCCCCCBB").unwrap();
        assert!(rng.get_seed() == 0x11111111);
        let seed_sequence: Vec<u32> = vec![
            0x11111111, 0xEEED82B6, 0x3C451065, 0x6C8fB3B2, 0x35ff7C59, 0xEAAE428E, 0x9B1449ED,
            0x2389CE4A, 0x97E39621, 0x180D6DE6,
        ];
        for i in 0..10 {
            assert!(rng.get_seed() == seed_sequence[i]);
            let _ = rng.rand(4);
        }
    }
}
