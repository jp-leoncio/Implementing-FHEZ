#![allow(dead_code)]
#![allow(unused_imports)]
use num_bigint::{BigInt, Sign, RandBigInt, ToBigInt};
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
            coeficients: vec![BigInt::from(0); qt_coeficients],
        }
    }

    pub fn rand(qt_coeficients: usize, size: u64, degree: u32) -> BigPolynomial {
        let mut rng = rand::thread_rng();
        let mut vec = vec![BigInt::from(0); qt_coeficients];

        for coef in vec.iter_mut() {
            *coef = RandBigInt::gen_bigint(&mut rng, size);
        }

        let mut poly = BigPolynomial { 
            len: qt_coeficients as u32, 
            n: qt_coeficients as u32 - 1, 
            coeficients: vec
        };
        poly.module(degree)
    }

    pub fn module(&mut self, degree: u32) -> BigPolynomial {
        let mut cont = 1u32;
        let mut valor = 1u32;
        let mut poly = BigPolynomial::new(degree as usize);

        for i in 0..degree as usize {
            poly.coeficients[i] = self.coeficients[i].clone();
        }

        for i in degree..self.len {
            poly.coeficients[(i % degree) as usize] += 
            BigInt::from(-1).pow(valor) * self.coeficients[i as usize].clone();
            cont += 2;

            if cont == (2 * degree + 1) {
                cont = 0;
                valor = 0;
            } else if cont == (2 * degree) {
                cont = 1;
                valor = 1;
            }
        }
        poly
    }
}