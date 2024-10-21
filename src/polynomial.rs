#![allow(dead_code)]
use std::cmp::max;
use std::cmp::min;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub len: i32,
    pub n: i32,
    pub coeficients: Vec<i32>,
}

pub fn print_poly(a: &Polynomial) {
    let mut i = (a.n - 1) as usize;
    while i > 0 {
        if a.coeficients[i] != 0 {
            print!("{:?}x^{:?}", a.coeficients[i], i);
            print!(" + ");
        }
        i -= 1;
    }
    println!("{:?}", a.coeficients[0]);
}

pub fn is_null(a: &Polynomial) -> bool {
    for i in 0..a.len as usize {
        if a.coeficients[i] != 0 {
            return false;
        }
    }
    return true;
}

pub fn extension(a: &Polynomial, len: i32) -> Polynomial {
    let mut p = Polynomial{
        len, 
        n: len - 1, 
        coeficients: vec![0; len as usize]
    };

    for i in 0..a.len as usize{
        p.coeficients[i] = a.coeficients[i];
    }
    for j in (a.len as usize)..p.len as usize{
        p.coeficients[j] = 0;
    }

    return p;
}

pub fn add_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let len = max(a.len, b.len);
    let ini = min(a.len, b.len) as usize;

    let mut p = Polynomial{
        len,
        n: len - 1, 
        coeficients: vec![0; len as usize]
    };

    for i in 0..ini {
        p.coeficients[i] = a.coeficients[i] + b.coeficients[i];
    }
    for i in ini..(a.len as usize) {
        p.coeficients[i] = a.coeficients[i];
    }
    for i in ini..(b.len as usize) {
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

pub unsafe fn mul_poly_naive(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut ret = Polynomial{
        len: a.n + b.n + 1, 
        n: a.n + b.n, 
        coeficients: vec![0; (a.n + b.n + 1) as usize]
    };

    for i in 0..(a.len as usize) {
        for j in 0..(b.len as usize) {
            ret.coeficients[i + j] += a.coeficients[i] * b.coeficients[j]; 
        }
    }    
    return ret;
}

pub fn mul_poly_naive_safe(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut ret = Polynomial{
        len: a.n + b.n + 1, 
        n: a.n + b.n, 
        coeficients: vec![0; (a.n + b.n + 1) as usize]
    };

    for i in 0..(a.len as usize) {
        for j in 0..(b.len as usize) {
            ret.coeficients[i + j] += a.coeficients[i] * b.coeficients[j]; 
            // O compilador não sabe se a.n é do mesmo tamanho doq o a.coef.len(), então em cada iteração ele precisa verificar se a memória explode
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

impl Polynomial {
    pub fn new(coeficients: &Vec<i32>, qt_coeficients: i32) -> Polynomial {
        return Polynomial {
            len: qt_coeficients,
            n: qt_coeficients - 1,
            coeficients: coeficients.clone(),
        };
    }

    pub fn new_rand(rng: &mut ThreadRng, num_coeficients: usize) -> Self {
        let mut coeficients = vec![0; num_coeficients];

        for i in 0..num_coeficients {
            coeficients[i] = rng.gen_range(-100..100);
        }

        return Polynomial {
            len: num_coeficients as i32,
            n: (num_coeficients - 1) as i32,
            coeficients,
        };
    }
}