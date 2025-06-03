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

// pub fn module(a: &mut BigPolynomial, degree: u32) -> BigPolynomial {
//     // Fazer mod x^n + 1, n = degree 
//     let mut cont = 1u32;
//     let mut valor = 1u32;
//     let mut p = BigPolynomial::new(degree as usize);

//     for i in 0..degree as usize {
//         p.coeficients[i] = a.coeficients[i];
//     }

//     for i in degree..a.len {
//         p.coeficients[(i % degree) as usize] += 
//         (BigInt::new(Sign::Minus, [1].to_vec())).modpow(degree, 2) * a.coeficients[i as usize];
//         cont += 2; // Oq caralhos esse modpow faz cara.....
//         if cont == (2 * degree + 1) {
//             cont = 0;
//             valor = 0;
//         } else if cont == (2 * degree) {
//             cont = 1;
//             valor = 1;
//         }
//     }
//     p
// }

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