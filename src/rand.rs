pub trait Rand {
    fn rand_max() -> u64;
    fn rand(&mut self) -> u64;
}

// xoshiro256++ by David Blackman and Sebastiano Vigna (vigna@acm.org), 2019

pub struct Xoshiro256PlusPlus {
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
}

impl Xoshiro256PlusPlus {
    pub fn default() -> Xoshiro256PlusPlus {
        Xoshiro256PlusPlus {
            s0: 0x9a_22_3f_a0_34_56_ab_28,
            s1: 0xf2_c8_2b_9a_af_0a_6f_20,
            s2: 0x63_08_ed_5d_a2_00_3a_ea,
            s3: 0x97_47_dd_6f_ba_3a_89_1f,
        }
    }

    fn rtol(x: u64, k: i32) -> u64 {
        (x << k) | (x >> (64 - k))
    }
}

impl Rand for Xoshiro256PlusPlus {
    fn rand_max() -> u64 {
        u64::MAX
    }

    fn rand(&mut self) -> u64 {
        let result = Xoshiro256PlusPlus::rtol(self.s0 + self.s3, 23) + self.s0;

        let t = self.s1 << 17;

        self.s2 ^= self.s0;
        self.s3 ^= self.s1;
        self.s1 ^= self.s2;
        self.s0 ^= self.s3;

        self.s2 ^= t;

        self.s3 = Xoshiro256PlusPlus::rtol(self.s3, 45);

        result
    }
}