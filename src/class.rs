#![allow(unused_imports)]
#![allow(unused_doc_comments)]
use modinverse::modinverse;
use concrete_fft::c64;
use concrete_fft::ordered::{Plan, Method};
use dyn_stack::{PodStack, GlobalPodBuffer, ReborrowMut};
use num_bigint::{BigInt, BigUint, Sign, ToBigInt, ToBigUint};
use num_complex::Complex;
use rand::Rng;
use std::ops::Div;
use std::time::{Duration, Instant};
use crate:: big_polynomial::*;
use crate::util::*;
use crate::N;

const PRIME_LEN: usize = 3; 

static TESTE: &[u32] = &[3,5,7];

static PRIME_15: &[u32] = &[32771,32779,32783,32789,32797,32801,32803,32831,32833,
    32839,32843,32869,32887,32909,32911,32917,32933,32939, 
    32941,32957,32969,32971,32983,32987,32993,32999,33013,
    33023,33029,33037,33049,33053,33071,33073,33083,33091,
    33107,33113,33119,33149,33151,33161,33179,33181,33191,
    33199,33203,33211,33223,33247]; // first 50 primes with 15-bits

static PRIME_20: &[u32] = &[1048583, 1048589, 1048601, 1048609, 1048613, 
    1048627, 1048633, 1048661, 1048681, 1048703, 
    1048709, 1048717, 1048721, 1048759, 1048783, 
    1048793, 1048799, 1048807, 1048829, 1048837, 
    1048847, 1048867, 1048877, 1048889, 1048891, 
    1048897, 1048909, 1048919, 1048963, 1048991]; // first 30 primes with 20-bits


#[derive(Clone, Debug)]
pub struct DCRT {
    pub poly: [[Complex<f64>; N]; PRIME_LEN],
    // pub a: Vec<[Complex<f64>; N]>,
    pub n: usize,
    pub primes: usize,
}

/**
 * Funciona!!
*/
/// Calcula o valor esperado utilizando a congruências do array de `Complex<64>`
/// no Teorema Chinês do Resto.
/// 
/// # Argumentos
/// 
/// * `a` - congruências dos valores com os primos definidos em `PRIME`.
pub fn new_crt(a: &mut Vec<Complex<f64>>, m: &BigInt, m_i: &Vec<BigInt>) -> BigInt {
    let mut solution = 0.to_bigint().unwrap();
    for (i, primes) in m_i.iter().enumerate() { // uso m_i.iter() pq o len de m_i é o prime_qnt
        let mut a_i = BigInt::ZERO;
        if a[i].re < 0.0 {
            a_i = BigInt::from(a[i].re as i32);
        } else {
            a_i = BigInt::from(a[i].re as i32);            
        }
        let n_i = modinvers(primes, &TESTE[i].into());
        solution = (solution + a_i * primes * n_i.unwrap()) % m;
    }
    solution
}

/**
 * Funciona!!
*/
pub fn precomp_crt(b: f64, gamma: f64, n: usize, l: usize, prime_len: usize) -> (BigUint, Vec<BigUint>) {
    let size = gamma + f64::ceil(f64::log2(l as f64)) 
    + f64::log2(b) + f64::log2(n as f64);
    
    let prime_qnt = (size as usize) / prime_len;
    let mut m = BigUint::new([1].to_vec());
    let mut m_i = vec![BigUint::new([1].to_vec()); prime_qnt];

    if prime_len == 15 {
        for i in 0..prime_qnt {
            m *= BigUint::new([PRIME_15[i]].to_vec());
        }
        for i in 0..prime_qnt {
            m_i[i] = m.clone() / PRIME_15[i];
        }
    }

    else if prime_len == 20 {   
        for i in 0..prime_qnt {
            m *= BigUint::new([PRIME_20[i]].to_vec());
        }
        for i in 0..prime_qnt {
            m_i[i] = m.clone() / PRIME_20[i];
        }
    }
    (m, m_i)
}


/// Função que converte um polinômio `a` da estrutura `Polynomial` para o Double-CRT.
/// 
/// # Argumentos
/// 
/// * `a` - polinômio da struct Polynomial.
/// * `plan` - variável utilizada na funções "forward" e "inverse" da concrete-fft.
pub fn to_dcrt<const N: usize>(a: &mut BigPolynomial, plan: &mut Plan) -> DCRT {
    let mut res = DCRT::new(N, PRIME_LEN);
    for (i, poly) in res.poly.iter_mut().enumerate() {
        for (j, coef) in a.coeficients.iter_mut().enumerate() {
            let valor = coef.clone() % TESTE[i];
            poly[j] = c64::new(valor.to_string().parse().unwrap(), 0.0);
        }
        to_fft::<N>(poly, plan);
    }
    res
}

/// Função que converte um polinômio `a` da forma Double-CRT para a struct `Polynomial`.
/// 
/// # Argumentos
/// 
/// * `a` - polinômio na forma DCRT.
/// * `plan` - variável utilizada na funções "forward" e "inverse" da concrete-fft.
pub fn from_dcrt<const N: usize>(a: &mut DCRT, plan: &mut Plan) -> BigPolynomial {
    for poly in a.poly.iter_mut() {
        from_fft::<N>(poly, plan);
    }

    let mut trans = vec![vec![c64::new(0.0, 0.0); PRIME_LEN]; N];
    for i in 0..N {
        for j in 0..PRIME_LEN {
            trans[i][j] = a.poly[j][i];
        }
    }
    // precomp_crt(b, gamma, n, l, prime_len);
    let m = BigInt::from(105);
    let m_i = vec![BigInt::from(35), BigInt::from(21), BigInt::from(15)];
    let mut ans = BigPolynomial::new(N);
    for (i, line) in trans.iter_mut().enumerate() {
        ans.coeficients[i] = new_crt(line, &m, &m_i);
    }
    ans
}


pub fn inner_product<const N: usize, const L: usize>(a: &mut [DCRT; L], b: &mut [DCRT; L]) -> DCRT {
    /**
    * TODO: Documentar as funções!!!
    */
    let mut res = DCRT::new(N, PRIME_LEN);
    for i in 0..L {
        let mut mul = a[i].mul::<N>(&mut b[i]);
        res = res.add::<N>(&mut mul);
    }
    res
}

// pub fn external_product<const N: usize, const L: usize>(u: &mut Polynomial, v: &mut [[[Complex<f64>; N]; PRIME_LEN]; L], plan: &mut Plan) -> Polynomial {
//     let mut new_u = red_base_poly::<L>(u);
//     let mut res = inner_product_precomp::<N, L>(&mut new_u, v, plan);
//     from_dcrt(&mut res, plan)
// }

impl DCRT {
    /// Gera um polinômio na forma DCRT com os coeficiente nulos.
    /// ```
    /// codigo
    /// 
    /// ```
    pub fn new(n: usize, primes: usize) -> DCRT {
        /**
        * TODO: Documentar as funções!!!
        */
        DCRT {
            poly: [[c64::new(0.0, 0.0); N]; PRIME_LEN], 
            n,
            primes,
        }
    }

    pub fn new_rand(n: usize, primes: usize) -> DCRT {
        /**
        * TODO: Documentar as funções!!!
        */
        let mut rng = rand::thread_rng();
        let mut dcrt = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
        for poly in dcrt.iter_mut() {
            for i in 0..poly.len() {
                poly[i] = c64::new(rng.gen_range(0..100) as f64, 0.0);
            }
        }
        DCRT {
            // poly: [[c64::new(rng.gen_range(0..100) as f64, 0.0); N]; PRIME_LEN], 
            poly: dcrt, 
            n,
            primes,
        }
    }

    pub fn add<const N: usize>(&mut self, other: &mut DCRT) -> DCRT {
        /**
        * TODO: Documentar as funções!!!
        */
        assert_eq!(self.poly.len(), other.poly.len());

        let mut res = DCRT::new(N, PRIME_LEN);
        for (i, poly) in res.poly.iter_mut().enumerate() {
            for j in 0..N {
                poly[j] = self.poly[i][j] + other.poly[i][j];
            }
        }
        res
    }

    pub fn mul<const N: usize>(&mut self, other: &mut DCRT) -> DCRT {
        /**
        * TODO: Documentar as funções!!!
        */
        assert_eq!(self.poly.len(), other.poly.len());

        let mut res = DCRT::new(N, PRIME_LEN);
        for (i, poly) in res.poly.iter_mut().enumerate() {
            for j in 0..N {
                poly[j] = self.poly[i][j] * other.poly[i][j];
            }
        }
        res
    } 
}