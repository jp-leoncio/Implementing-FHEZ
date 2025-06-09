use crate::big_polynomial::*;
use crate::N;
use crate::B;

use num_bigint::*;
use num_complex::*;
use concrete_fft::c64;
use concrete_fft::ordered::Plan;
use dyn_stack::{PodStack, GlobalPodBuffer, ReborrowMut};

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

pub fn sym_mod(a: BigInt, n: i64) -> BigInt {
    let valor = a % n;
    if 2*valor.clone() > BigInt::from(n) {
        return valor - n;
    }
    valor
}

pub fn inv_g_zz(a: BigInt, g: Vec<f64>, q: f64, l: usize) -> Vec<BigInt> {
    let mut res = vec![BigInt::ZERO; l];
    let mut copy = sym_mod(a, q as i64);
    for i in 0..l { 
        let valor = copy.clone() / (g[l-i-1] as i64);
        let rem = copy % (g[l-i-1] as i64); 
        res[l-i-1] = valor;
        copy = rem.clone();
        if rem == BigInt::ZERO {
            break;
        }
    }
    res
}

pub fn inv_g_poly(a: &mut BigPolynomial, q: f64) -> Vec<BigPolynomial> {
    let l= q.log(B).ceil() as usize;
    let mut g = vec![0.0; l];
    for i in 0..l {
        g[i] = B.powi(i as i32);
    }
    let mut res = vec![BigPolynomial::new(N); N];
    for i in 0..N/2 as usize {
        let reduc = inv_g_zz(a.coeficients[i].clone(), g.clone(), q, l);
        res[i].coeficients = reduc;
    }
    res
}

pub fn print_poly(a: &BigPolynomial) {
    let mut i = a.n as usize;

    if is_null(a) {
        println!("Null BigPolynomial");
    } else if !is_null(a) && a.n == 0 {
        println!("{:?}", a.coeficients[i]);
    } else {
        print!("{:?}x^{:?}", a.coeficients[i], i);
        i -= 1;

        while i > 0 {
            if a.coeficients[i] < BigInt::ZERO {
                print!(" - ");
                print!("{:?}x^{:?}", -a.coeficients[i].clone(), i);
            } else if a.coeficients[i] > BigInt::ZERO {
                print!(" + ");
                print!("{:?}x^{:?}", a.coeficients[i], i);
            }
            i -= 1;
        }
    
        if a.coeficients[i] < BigInt::ZERO {
            print!(" - ");
            print!("{:?}", -a.coeficients[i].clone());
        } else if a.coeficients[i] > BigInt::ZERO {
            print!(" + ");
            print!("{:?}", a.coeficients[i]);
        }
        println!();
    }

}

pub fn is_null(a: &BigPolynomial) -> bool {
    for a_coef in a.coeficients.iter() {
        if a_coef != &BigInt::ZERO {
            return false;
        }
    }
    true
}