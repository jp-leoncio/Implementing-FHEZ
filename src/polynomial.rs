#![allow(dead_code)]
#![allow(unused_imports)]
use std::cmp::{max, min};
use std::mem::swap;
use num_complex::Complex;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
static PI: f64 = 3.1415926535;

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub len: u32,
    pub n: u32,
    pub coeficients: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct Congruence {
    pub a: i32,
    pub m: i32,
}

pub fn add_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let len = max(a.n, b.n) + 1;
    let ini = (min(a.n, b.n) + 1) as usize;

    let mut p = Polynomial{
        len,
        n: max(a.n, b.n), 
        coeficients: vec![0; len as usize]
    };

    for i in 0..ini {
        p.coeficients[i] = a.coeficients[i] + b.coeficients[i];
    }
    for i in ini..((a.n + 1) as usize) {
        p.coeficients[i] = a.coeficients[i];
    }
    for i in ini..((b.n + 1) as usize) {
        p.coeficients[i] = b.coeficients[i];
    }

    return p;
}

pub fn sub_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut minus = b.clone();
    for i in 0..(b.len as usize) {
        minus.coeficients[i] *= -1; 
    }
    return add_poly(a, &minus);
}

pub fn mul_poly_naive(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut ret = Polynomial{
        len: a.n + b.n + 1, 
        n: a.n + b.n, 
        coeficients: vec![0; (a.n + b.n + 1) as usize]
    };

    for i in 0..((a.n + 1) as usize) {
        for j in 0..((b.n + 1) as usize) {
            ret.coeficients[i + j] += a.coeficients[i] * b.coeficients[j];
            /* O compilador não sabe se a.n é do mesmo tamanho doq o a.coef.len(), então 
             * em cada iteração ele precisa verificar se a memória explode */ 
        }
    }    
    return ret;
}

pub fn mul_poly_mid(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut ret = Polynomial{
        len: a.n + b.n + 1, 
        n: a.n + b.n, 
        coeficients: vec![0; (a.n + b.n + 1) as usize]
    };

    for (i, a_coef) in a.coeficients.iter().enumerate() {
        for (j, b_coef) in b.coeficients.iter().enumerate() {
            ret.coeficients[i + j] += a_coef * b_coef;
            // Esse faz 2 loads de informação por iteração, o antigo (naive) faz 3
        }
    }    
    return ret;
}

pub fn mul_poly_fast(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut ret = Polynomial{
        len: a.n + b.n + 1, 
        n: a.n + b.n, 
        coeficients: vec![0; (a.n + b.n + 1) as usize]
    };

    a.coeficients
        .iter()
        .enumerate()
        .flat_map(move |(i, a_coef)| {
            b.coeficients
                .iter()
                .enumerate()
                .map(move |(j, b_coef)| (i + j, a_coef * b_coef))
        })
        .for_each(|(i, coef)| {
            ret.coeficients[i] += coef;
        });
    return ret;
}

pub fn par_mul_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let len = a.len + b.len - 1;

    let coeficients = a
        .coeficients
        .par_iter()
        .enumerate()
        .flat_map(|(i, a_coef)| {
            b.coeficients
                .par_iter()
                .enumerate()
                .map(move |(j, b_coef)| (i + j, a_coef * b_coef))
        })
        .fold_with(vec![0; len as usize], |mut vec, (i, coef)| {
            vec[i] += coef;
            vec
        }).find_first(|_| true).unwrap();

    Polynomial{len, n: len - 1, coeficients}
}

pub fn module_poly(a: &Polynomial, degree: u32) -> Polynomial {
    /* Fazer mod x^n + 1, n = degree 
     *  */
    let mut cont = 1u32;
    let mut valor = 1u32;
    let mut p = Polynomial {
        len: degree,
        n: degree - 1,
        coeficients: vec![0; degree as usize]
    };

    for i in 0..degree as usize {
        p.coeficients[i] = a.coeficients[i];
    }

    for i in degree..a.len {
        p.coeficients[(i % degree) as usize] += (-1 as i32).pow(valor) * a.coeficients[i as usize];
        cont += 2;
        if cont == (2 * degree + 1) {
            cont = 0;
            valor = 0;
        } else if cont == (2 * degree) {
            cont = 1;
            valor = 1;
        }
    }
    return p;
}

impl Polynomial {
    pub fn new(coeficients: &Vec<i32>, qt_coeficients: u32) -> Polynomial {
        return Polynomial {
            len: qt_coeficients,
            n: qt_coeficients - 1,
            coeficients: coeficients.clone(),
        };
    }

    pub fn new_rand(rng: &mut ThreadRng, num_coeficients: usize) -> Self {
        let mut coeficients = vec![0; num_coeficients];

        for i in 0..num_coeficients {
            coeficients[i] = rng.gen_range(0..100);
        }

        return Polynomial {
            len: num_coeficients as u32,
            n: (num_coeficients - 1) as u32,
            coeficients,
        };
    }

    pub fn new_mod(rng: &mut ThreadRng, num_coeficients: usize, degree: u32) -> Self {
        let mut coeficients = vec![0; num_coeficients];

        for i in 0..num_coeficients {
            coeficients[i] = rng.gen_range(0..100);
        }

        let a = Polynomial {
            len: num_coeficients as u32,
            n: (num_coeficients - 1) as u32,
            coeficients,
        };
        return module_poly(&a, degree);
    }

}

impl Congruence {
    pub fn new(a: i32, m: i32) -> Congruence {
        return Congruence {
            a: a,
            m: m,
        };
    }
}