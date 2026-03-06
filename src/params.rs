use crate::prelude::*;

/// Primes of 15 bits
pub static PRIMES_15: &[u32] = &[
    32771, 32779, 32783, 32789, 32797, 32801, 32803, 32831, 32833, 32839, 32843, 32869, 32887,
    32909, 32911, 32917, 32933, 32939, 32941, 32957,
];

/// Primes of 20 bits
pub static PRIMES_20: &[u32] = &[
    1048583, 1048589, 1048601, 1048609, 1048613, 1048627, 1048633, 1048661, 1048681, 1048703,
    1048709, 1048717, 1048721, 1048759, 1048783, 1048793, 1048807, 1048819, 1048829, 1048849,
];

/// Parameters of the FHEZ scheme
#[derive(Debug, Clone)]
pub struct FhezParameters {
    /// Degree of the ring Z[X] / <X^n + 1>
    pub n: usize,

    /// Base of the gadget decomposition (ex: 2^24)
    pub b: u64,

    /// Number of levels of the gadget decomposition: ceil(log_B(2^gamma))
    pub l: usize,

    /// Bit length of the module q: the ciphertext module has ~2^gamma bits
    pub gamma: f64,

    /// Bit length of the noise r in the encryption: r belongs to the interval negative 2^rho to 2^rho
    pub rho: u64,

    /// Module of the message space
    pub t: u64,

    /// Slice of primes used in the DCRT representation
    pub primes: &'static [u32],
}

impl FhezParameters {
    pub fn article_line1() -> Self {
        Self {
            n: 256,
            b: 1 << 24, // 2^24 = 16.777.216
            l: 10,
            gamma: 206.0,
            rho: 16,
            t: 4,
            primes: PRIMES_20,
        }
    }

    pub fn small_test() -> Self {
        Self {
            n: 128,
            b: 1 << 20,
            l: 8,
            gamma: 150.0,
            rho: 16,
            t: 2,
            primes: PRIMES_20,
        }
    }

    pub fn b_f64(&self) -> f64 {
        self.b as f64
    }

    pub fn l_f64(&self) -> f64 {
        self.l as f64
    }

    pub fn n_f64(&self) -> f64 {
        self.n as f64
    }

    pub fn t_bigint(&self) -> num_bigint::BigInt {
        num_bigint::BigInt::from(self.t)
    }
}
