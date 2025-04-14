use modinverse::modinverse;
use concrete_fft::c64;
use concrete_fft::ordered::{Plan, Method};
use dyn_stack::{PodStack, GlobalPodBuffer, ReborrowMut};
use num_complex::Complex;
use std::time::{Duration, Instant};
use crate::polynomial::*;
use crate::util::*;
use crate::N;

const PRIME_LEN: usize = 4;
static PRIME: &[i64] = &[3, 5, 7, 11];
static PRIME_15: &[i64] = &[32771,32779,32783,32789,32797,32801,32803,32831,32833,
    32839,32843,32869,32887,32909,32911,32917,32933,32939, 
    32941,32957,32969,32971,32983,32987,32993,32999,33013,
    33023,33029,33037,33049,33053,33071,33073,33083,33091,
    33107,33113,33119,33149,33151,33161,33179,33181,33191,
    33199,33203,33211,33223,33247]; // first 50 primes with 15-bits

static PRIME_20: &[i64] = &[1048583, 1048589, 1048601, 1048609, 1048613, 
    1048627, 1048633, 1048661, 1048681, 1048703, 
    1048709, 1048717, 1048721, 1048759, 1048783, 
    1048793, 1048799, 1048807, 1048829, 1048837, 
    1048847, 1048867, 1048877, 1048889, 1048891, 
    1048897, 1048909, 1048919, 1048963, 1048991]; // first 30 primes with 20-bits

pub fn crt(a: &mut [Complex<f64>]) -> i64 {
    let mut m = 1;
    for p in PRIME.iter() {
        m *= p;
    }
    let mut solution = 0;
    for (i, p) in PRIME.iter().enumerate() {
        let a_i = a[i].re as i64;
        let m_i = m / p;
        let n_i = modinverse(m_i, PRIME[i]);
        solution = (solution + a_i * m_i * n_i.unwrap()) % m;
    }
    (solution + m) % m
}

pub fn to_fft<'a, const N: usize>(poly: &'a mut [Complex<f64>], plan: &'a mut Plan) -> &'a mut [Complex<f64>] {
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);

    plan.fwd(poly, stack.rb_mut());
    poly
}

pub fn from_fft<'a, const N: usize>(poly: &'a mut [Complex<f64>], plan: &'a mut Plan) -> &'a mut [Complex<f64>] {
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);

    plan.inv(poly, stack.rb_mut());
    for coef in poly.iter_mut() {
        *coef /= N as f64;
    }
    poly
}

pub fn to_crt<const N: usize>(a: &mut [Complex<f64>], plan: &mut Plan) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut res = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for (i, poly) in res.iter_mut().enumerate() {
        // MOD
        for (j, coef) in a.iter_mut().enumerate() {
            poly[j] = *coef % PRIME[i] as f64;
        }
        to_fft::<N>(poly, plan);
    }
    res
}

pub fn from_crt<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], plan: &mut Plan) -> [Complex<f64>; N] {
    for poly in a.iter_mut() {
        from_fft::<N>(poly, plan);
    }

    let mut trans = [[c64::new(0.0, 0.0); PRIME_LEN]; N];
    for i in 0..N {
        for j in 0..PRIME_LEN {
            trans[i][j] = a[j][i];
        }
    }

    let mut res = [c64::new(0.0, 0.0); N];
    for (i, line) in trans.iter_mut().enumerate() {
        res[i].re = crt(line) as f64;
    }
    res
}

pub fn to_dcrt<const N: usize>(a: &mut Polynomial, plan: &mut Plan) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut new_a = from_poly::<N>(a);
    to_crt::<N>(&mut new_a, plan)
}

pub fn from_dcrt<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], plan: &mut Plan) -> Polynomial {
    let new_a = from_crt(a, plan);
    to_poly(new_a)
}

pub fn add_crt<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], b: &mut [[Complex<f64>; N]; PRIME_LEN]) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut res = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..PRIME_LEN {
        for j in 0..N {
            res[i][j] = a[i][j] + b[i][j];
        }
    }
    res
}

pub fn add_crt_to_poly<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], b: &mut [[Complex<f64>; N]; PRIME_LEN], plan: &mut Plan) -> Polynomial {
    let mut res = add_crt(a, b);
    from_dcrt(&mut res, plan)
}

pub fn add_crt_from_poly<const N: usize>(a: &mut Polynomial, b: &mut [[Complex<f64>; N]; PRIME_LEN], plan: &mut Plan) -> Polynomial {
    let mut new_a = to_dcrt(a, plan);
    add_crt_to_poly(&mut new_a, b, plan)
}

pub fn mul_crt<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], b: &mut [[Complex<f64>; N]; PRIME_LEN]) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut res = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..PRIME_LEN {
        for j in 0..N {
            res[i][j] = a[i][j] * b[i][j];
        }
    }
    res
}

pub fn mul_crt_to_poly<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], b: &mut [[Complex<f64>; N]; PRIME_LEN], plan: &mut Plan) -> Polynomial {
    let mut res = mul_crt(a, b);
    from_dcrt(&mut res, plan)
}

pub fn mul_crt_from_poly<const N: usize>(a: &mut Polynomial, b: &mut [[Complex<f64>; N]; PRIME_LEN], plan: &mut Plan) -> Polynomial {
    let mut new_a = to_dcrt(a, plan);
    mul_crt_to_poly(&mut new_a, b, plan)
}

pub fn inner_product<const N: usize, const L: usize>(a: &mut [Polynomial; L], b: &mut [Polynomial; L], plan: &mut Plan) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut res = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..L {
        let mut new_a = to_dcrt::<N>(&mut a[i], plan);
        let mut new_b = to_dcrt::<N>(&mut b[i], plan);
        let mut mul = mul_crt(&mut new_a, &mut new_b);
        res = add_crt(&mut res, &mut mul);
    }
    res
}

pub fn inner_product_precomp<const N: usize, const L: usize>(a: &mut [Polynomial; L], b: &mut [[[Complex<f64>; N]; PRIME_LEN]; L], plan: &mut Plan) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut res = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..L {
        let mut new_a = to_dcrt::<N>(&mut a[i], plan);
        let mut mul = mul_crt(&mut new_a, &mut b[i]);
        res = add_crt(&mut res, &mut mul);
    }
    res
}

pub fn inner_product_2precomp<const N: usize, const L: usize>(a: &mut [[[Complex<f64>; N]; PRIME_LEN]; L], b: &mut [[[Complex<f64>; N]; PRIME_LEN]; L]) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut res = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..L {
        let mut mul = mul_crt(&mut a[i], &mut b[i]);
        res = add_crt(&mut res, &mut mul);
    }
    res
}

pub fn inner_product_to_poly<const N: usize, const L: usize>(a: &mut [[[Complex<f64>; N]; PRIME_LEN]; L], b: &mut [[[Complex<f64>; N]; PRIME_LEN]; L], plan: &mut Plan) -> Polynomial {
    let mut res = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..L {
        let mut mul = mul_crt(&mut a[i], &mut b[i]);
        res = add_crt(&mut res, &mut mul);
    }
    from_dcrt(&mut res, plan)
}