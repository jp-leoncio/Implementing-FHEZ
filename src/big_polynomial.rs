#![allow(dead_code)]
// #![allow(unused_imports)]
use num_bigint::{BigInt, Sign, RandBigInt};
use rand::rngs::ThreadRng;
use rand::Rng;

const PI: f64 = std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct BigPolynomial {
    pub len: u32,
    pub n: u32,
    pub coeficients: Vec<BigInt>,
}

impl BigPolynomial {
    pub fn new(qt_coeficients: usize) -> BigPolynomial {
        BigPolynomial {
            len: qt_coeficients as u32,
            n: qt_coeficients as u32 - 1,
            coeficients: vec![BigInt::new(Sign::Plus, [0].to_vec()); qt_coeficients],
        }
    }

    pub fn rand(qt_coeficients: usize, size: u64) -> BigPolynomial {
        let mut rng = rand::thread_rng();
        let mut ret = vec![BigInt::new(Sign::NoSign, [0].to_vec()); qt_coeficients];

        for coef in ret.iter_mut() {
            *coef = RandBigInt::gen_bigint(&mut rng, size);
        }

        BigPolynomial { 
            len: qt_coeficients as u32, 
            n: qt_coeficients as u32 - 1, 
            coeficients: ret
        }
    }
}