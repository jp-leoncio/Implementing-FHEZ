use crate::Polynomial;
use crate::N;
use num_complex::Complex;
use concrete_fft::c64;

pub fn converter(a: &Polynomial) -> [Complex<f64>; N] {
    let mut new_a = [c64::new(0.0, 0.0); N];  

    for i in 0..a.len as usize{
        new_a[i].re = a.coeficients[i] as f64;
    }

    return new_a;
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
                print!("{:?}x^{:?}", (-1) * a.coeficients[i], i);
            } else if a.coeficients[i] > 0 {
                print!(" + ");
                print!("{:?}x^{:?}", a.coeficients[i], i);
            }
            i -= 1;
        }
    
        if a.coeficients[i] < 0 {
            print!(" - ");
            print!("{:?}", (-1) * a.coeficients[i]);
        } else if a.coeficients[i] > 0 {
            print!(" + ");
            print!("{:?}", a.coeficients[i]);
        }
        println!("");
    }

}

pub fn is_null(a: &Polynomial) -> bool {
    for (_, a_coef) in a.coeficients.iter().enumerate()  {
        if a_coef != &0 {
            return false;
        }
    }
    return true;
}

pub fn extension(a: &Polynomial, len: u32) -> Polynomial {
    let mut p = Polynomial{
        len, 
        n: len - 1, 
        coeficients: vec![0; len as usize]
    };

    for i in 0..a.len as usize {
        p.coeficients[i] = a.coeficients[i];
    }
    for j in (a.len as usize)..p.len as usize {
        p.coeficients[j] = 0;
    }
    return p;
}