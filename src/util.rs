use crate::Polynomial;
use crate::N;
use crate::B;
use num_complex::Complex;
use concrete_fft::c64;

pub fn from_poly<const N: usize>(a: &Polynomial) -> [Complex<f64>; N] {
    let mut new_a = [c64::new(0.0, 0.0); N];  
    for (i, coef) in a.coeficients.iter().enumerate(){
        new_a[i].re = *coef as f64;
    }
    new_a
}

pub fn to_poly<const N: usize>(a: [Complex<f64>; N]) -> Polynomial {
    let mut new_a = Polynomial::new(&[0; N].to_vec(), N as u32);
    for (i, coef) in new_a.coeficients.iter_mut().enumerate() {
        *coef = a[i].re as i32;
    }
    new_a
}

pub fn sym_mod(a: i64, n: i64) -> i64 {
    let valor = a % n;
    if 2*valor > n {
        return valor - n;
    }
    valor
}

pub fn red_base_ZZ<const l: usize>(a: i32, g: [f64; l]) -> [f64; l] {
    // let l = q.log(B).ceil();
    let mut res = [0.0; l];
    let mut copy = a;
    let mut valor = 0;
    let mut rem = 0;
    for i in 0..l {
        valor = copy / (g[l-i-1] as i32);
        rem = copy % (g[l-i-1] as i32);
        res[l-i-1] = valor as f64;
        copy = rem;
        if rem == 0 {
            break;
        }
    }
    res
}

pub fn red_base_poly<const l: usize>(a: &mut Polynomial) -> [[f64; l]; N] {
    let mut g = [0.0; l];
    for i in 0..l {
        g[i] = B.powi(i as i32);
    }
    let mut res = [[0.0; l]; N];
    for i in 0..a.len as usize {
        let reduc = red_base_ZZ(a.coeficients[i], g);
        res[i] = reduc;
    }
    res
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