use crate::Polynomial;
use crate::N;
use crate::B;
use concrete_fft::ordered::Plan;
use dyn_stack::{PodStack, GlobalPodBuffer, ReborrowMut};
use num_bigint::BigUint;
use num_complex::Complex;
use concrete_fft::c64;

pub fn from_poly<const N: usize>(a: &Polynomial) -> [Complex<f64>; N] {
    let mut new_a = [c64::new(0.0, 0.0); N];  
    for (i, coef) in a.coeficients.iter().enumerate().take(N) {
        new_a[i].re = *coef as f64;
    }
    new_a
}

pub fn to_poly<const N: usize>(a: [Complex<f64>; N]) -> Polynomial {
    let mut new_a = Polynomial::new(&[0; N].to_vec(), N as u32);
    for (i, coef) in new_a.coeficients.iter_mut().enumerate() {
        *coef = a[i].re as i64;
    }
    new_a
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

pub fn sym_mod(a: i64, n: i64) -> i64 {
    let valor = a % n;
    if 2*valor > n {
        return valor - n;
    }
    valor
}

pub fn red_base_zz<const l: usize>(a: i64, g: [f64; l]) -> [i64; l] {
    // let l = q.log(B).ceil();
    let mut res = [0; l];
    let mut copy = a;
    // let mut valor = 0;
    // let mut rem = 0;
    for i in 0..l {
        let valor = copy / (g[l-i-1] as i64);
        let rem = copy % (g[l-i-1] as i64);
        res[l-i-1] = valor;
        copy = rem;
        if rem == 0 {
            break;
        }
    }
    res
}

pub fn red_base_poly<const l: usize>(a: &mut Polynomial) -> [Polynomial; N] {
    let mut g = [0.0; l];
    for i in 0..l {
        g[i] = B.powi(i as i32);
    }
    let mut res: [Polynomial; N] = core::array::from_fn(|_| Polynomial::new(&[0; l].to_vec(), l as u32));
    for i in 0..N/2 as usize {
        let reduc = red_base_zz(a.coeficients[i], g);
        res[i] = Polynomial::new(&reduc.to_vec(), l as u32);
    }
    res
}

pub fn red_base_poly_hilder<const l: usize>(a: &mut Polynomial) -> [Polynomial; l] {
    let mut g = [0.0; l];
    for i in 0..l {
        g[i] = B.powi(i as i32);
    }
    let mut res: [Polynomial; N] = core::array::from_fn(|_| Polynomial::new(&[0; l].to_vec(), l as u32));
    for i in 0..N/2 as usize {
        let reduc = red_base_zz(a.coeficients[i], g);
        res[i] = Polynomial::new(&reduc.to_vec(), l as u32);
    }

    let mut trans: [Polynomial; l] = core::array::from_fn(|_| Polynomial::new(&[0; N].to_vec(), N as u32));
    for i in 0..N {
        for j in 0..l {
            trans[j].coeficients[i] = res[i].coeficients[j];
        }
    }

    trans
}

// pub fn red_base_poly<const l: usize, const N: usize>(a: &mut Polynomial) -> [Polynomial; l] {
//     let mut g = [0.0; l];
//     for i in 0..l {
//         g[i] = B.powi(i as i32);
//     }
//     let mut res: [Polynomial; l] = core::array::from_fn(|_| Polynomial::new(&[0; N].to_vec(), (N as u32) / 2));
//     for i in 0..N/2 as usize {
//         let reduc = red_base_zz::<N>(a.coeficients[i], g);
//         res[i] = Polynomial::new(&reduc.to_vec(), (N as u32) / 2);
//         // println!("{:?}", reduc);
//     }
//     res
// }

/**
 *! Não testei
*/
pub fn egcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
    if *a == BigUint::new([0].to_vec()) {
        (b.clone(), BigUint::new([0].to_vec()), BigUint::new([1].to_vec()))
    }
    else {
        let (g, x, y) = egcd(&(b % a), &a);
        (g, y - (b / a) * x.clone(), x)
    }
}
/**
 *! Não testei
*/
pub fn modinvers(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let (g, x, _) = egcd(a, m);
    if g != BigUint::new([1].to_vec()) {
        None
    }
    else {
        Some((x % m + m) % m)
    }
}

pub fn print_poly(a: &Polynomial) {
    let mut i = a.n as usize;
    
    if is_null(a) {
        println!("Null Polynomial");
    } else if !is_null(a) && a.n == 0 {
        println!("{:?}", a.coeficients[i]);
    } else {
        print!("{:?}x^{:?}", a.coeficients[i], i);
        i -= 1;

        while i > 0 {
            if a.coeficients[i] < 0 {
                print!(" - ");
                print!("{:?}x^{:?}", -a.coeficients[i], i);
            } else if a.coeficients[i] > 0 {
                print!(" + ");
                print!("{:?}x^{:?}", a.coeficients[i], i);
            }
            i -= 1;
        }
    
        if a.coeficients[i] < 0 {
            print!(" - ");
            print!("{:?}", -a.coeficients[i]);
        } else if a.coeficients[i] > 0 {
            print!(" + ");
            print!("{:?}", a.coeficients[i]);
        }
        println!();
    }

}

pub fn is_null(a: &Polynomial) -> bool {
    for a_coef in a.coeficients.iter() {
        if a_coef != &0 {
            return false;
        }
    }
    true
}

pub fn extension(a: &Polynomial, len: u32) -> Polynomial {
    let mut poly = Polynomial {
        len, 
        n: len - 1, 
        coeficients: vec![0; len as usize]
    };

    for i in 0..a.len as usize {
        poly.coeficients[i] = a.coeficients[i];
    }
    for j in (a.len as usize)..poly.len as usize {
        poly.coeficients[j] = 0;
    }
    poly
}