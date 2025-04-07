use modinverse::modinverse;
use concrete_fft::c64;
use concrete_fft::ordered::{Plan, Method};
use dyn_stack::{PodStack, GlobalPodBuffer, ReborrowMut};
use num_complex::Complex;
use std::time::{Duration, Instant};
use crate::polynomial::*;
use crate::util::*;
use crate::N;

const PRIME_LEN: usize = 3;
static PRIME: &'static [i64] = &[3, 5, 7];
static PRIME_15: &'static [i64] = &[32771,32779,32783,32789,32797,32801,32803,32831,32833,
    32839,32843,32869,32887,32909,32911,32917,32933,32939, 
    32941,32957,32969,32971,32983,32987,32993,32999,33013,
    33023,33029,33037,33049,33053,33071,33073,33083,33091,
    33107,33113,33119,33149,33151,33161,33179,33181,33191,
    33199,33203,33211,33223,33247]; // first 50 primes with 15-bits

static PRIME_20: &'static [i64] = &[1048583, 1048589, 1048601, 1048609, 1048613, 
    1048627, 1048633, 1048661, 1048681, 1048703, 
    1048709, 1048717, 1048721, 1048759, 1048783, 
    1048793, 1048799, 1048807, 1048829, 1048837, 
    1048847, 1048867, 1048877, 1048889, 1048891, 
    1048897, 1048909, 1048919, 1048963, 1048991]; // first 30 primes with 20-bits


pub fn fft<const N: usize>(a: &mut [Complex<f64>], b: &mut [Complex<f64>]) -> [Complex<f64>; N] {
    let plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);
    let mut res = [c64::new(0.0, 0.0); N];

    plan.fwd(a, stack.rb_mut());
    plan.fwd(b, stack.rb_mut());
    
    for i in 0..N {
        res[i] = a[i] * b[i];
    }
    
    plan.inv(&mut res, stack.rb_mut());

    for i in 0..N {
        res[i] /= N as f64;
    }
    return res;
}

pub fn crt(a: &mut [Complex<f64>]) -> f64 {
    let mut m = 1.0;
    for i in 0..PRIME.len() {
        m *= PRIME[i] as f64;
    }
    let mut solution = 0.0;
    for i in 0..PRIME.len() {
        let a_i = a[i].re;
        let m_i = m / PRIME[i] as f64;
        let n_i = modinverse(m_i as i64, PRIME[i] as i64);
        solution = (solution + a_i * m_i * n_i.unwrap() as f64) % m;
    }
    return (solution + m) % m;
}

pub fn double_crt<const N: usize>(a: &mut [Complex<f64>], b: &mut [Complex<f64>]) -> [Complex<f64>; N] {
    let mut c = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..PRIME.len() {
        let mut a_i = [c64::new(0.0, 0.0); N];
        let mut b_i = [c64::new(0.0, 0.0); N];
        
        // MOD
        for j in 0..N {
            a_i[j] = a[j] % PRIME[i] as f64;
            b_i[j] = b[j] % PRIME[i] as f64;
        }

        c[i] = fft(&mut a_i, &mut b_i);

        for j in 0..N {
            c[i][j] %= PRIME[i] as f64; 
        }
    }

    let mut trans = [[c64::new(0.0, 0.0); PRIME_LEN]; N];
    for i in 0..N {
        for j in 0..PRIME_LEN {
            trans[i][j] = c[j][i];
        }
    }

    let mut answer = [c64::new(0.0, 0.0); N];
    for i in 0..N {
        answer[i].re = crt(&mut trans[i]);
    }

    return answer;
}

pub fn to_fft<const N: usize>(a: &mut [Complex<f64>]) -> &mut [Complex<f64>] {
    let plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);

    plan.fwd(a, stack.rb_mut());
    return a;
}

pub fn from_fft<const N: usize>(a: &mut [Complex<f64>]) -> &mut [Complex<f64>] {
    let plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);

    plan.inv(a, stack.rb_mut());
    for i in 0..N {
        a[i] /= N as f64;
    }
    return a;
}

pub fn to_crt<'a, const N: usize>(a: &'a mut [Complex<f64>]) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut c = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..PRIME_LEN {
        // MOD
        for j in 0..N {
            c[i][j] = a[j] % PRIME[i] as f64;
        }
        to_fft::<N>(&mut c[i]);
    }
    return c;
}

pub fn from_crt<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN]) -> [Complex<f64>; N] {
    for i in 0..PRIME_LEN {
        from_fft::<N>(&mut a[i]);
    }

    let mut trans = [[c64::new(0.0, 0.0); PRIME_LEN]; N];
    for i in 0..N {
        for j in 0..PRIME_LEN {
            trans[i][j] = a[j][i];
        }
    }

    let mut answer = [c64::new(0.0, 0.0); N];
    for i in 0..N {
        answer[i].re = crt(&mut trans[i]);
    }

    return answer;
}

pub fn add_crt<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], b: &mut [[Complex<f64>; N]; PRIME_LEN]) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut c = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..PRIME_LEN {
        for j in 0..N {
            c[i][j] = a[i][j] + b[i][j];
        }
    }
    return c;
}

pub fn mul_crt<const N: usize>(a: &mut [[Complex<f64>; N]; PRIME_LEN], b: &mut [[Complex<f64>; N]; PRIME_LEN]) -> [[Complex<f64>; N]; PRIME_LEN] {
    let mut c = [[c64::new(0.0, 0.0); N]; PRIME_LEN];
    for i in 0..PRIME_LEN {
        for j in 0..N {
            c[i][j] = a[i][j] * b[i][j];
        }
    }
    return c;
}

pub fn mul_poly_crt<const N: usize>(a: &mut Polynomial, b: &mut [[Complex<f64>; N]; PRIME_LEN]) -> Polynomial {
    let mut conv = from_poly::<N>(&a);
    let mut new_a = to_crt::<N>(&mut conv);
    let mut c = mul_crt(&mut new_a, b);
    let conv = from_crt(&mut c);
    return to_poly(conv);
}