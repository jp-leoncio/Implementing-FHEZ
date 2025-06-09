use rand::rngs::ThreadRng;
use rand::Rng;
use num_bigint::*;

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
            coeficients: vec![BigInt::ZERO; qt_coeficients],
        }
    }

    pub fn rand(qt_coeficients: usize, size: u32, degree: u32) -> BigPolynomial {
        let mut rng = rand::thread_rng();
        let vec = (0..qt_coeficients)
            .map(|_| RandBigInt::gen_bigint(&mut rng, size.into()))
            .collect();

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
        let slice = &self.coeficients[degree as usize..self.len as usize];

        poly.coeficients = self.coeficients.clone().into_iter().take(degree as usize).collect();

        for (i, coef) in slice.iter().enumerate() {
            poly.coeficients[(i as u32 % degree) as usize] += BigInt::from(-1).pow(valor) * coef;
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