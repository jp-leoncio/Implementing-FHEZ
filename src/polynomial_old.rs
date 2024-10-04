use std::cmp::max;
use std::convert::TryInto;
use rand::Rng;

#[derive(Clone, Debug, Copy)]
pub struct Monomial {
    coef: i32,
    expo: i32,
}
#[derive(Clone, Debug)]
pub struct PolynomialOld {
    n: i32,
    monomial: Vec<Monomial>,
}

pub fn _print_old(poly: PolynomialOld) {
    let mut i = (poly.n - 1) as usize;
    while i > 0 {
        if poly.monomial[i].coef != 0 {
            print!("{:?}x^{:?} ", poly.monomial[i].coef, poly.monomial[i].expo);
            print!(" + ");
        }
        i -= 1;
    }
    println!("{:?} ", poly.monomial[0].coef);
}

pub fn add_old(a: PolynomialOld, b: PolynomialOld) -> PolynomialOld {
    let mut v = vec!();

    let mut mon1 = a.monomial.clone();
    let mon2 = b.monomial.clone();
    let mut i = 0usize;
    let mut j = 0usize;

    while i < a.n.try_into().unwrap() && j < b.n.try_into().unwrap() {
        if mon1[i].expo > mon2[j].expo {
            v.push(mon1[i].clone());
            i += 1;
        }
        else if mon1[i].expo < mon2[j].expo {
            v.push(mon2[j].clone());
            j += 1;
        }
        else {
            mon1[i].coef +=  mon2[j].coef;
            v.push(mon1[i].clone());
            i += 1;
            j += 1;
        }        
    }
    if i == a.n.try_into().unwrap() {
        for k in j..b.n.try_into().unwrap() {
            v.push(mon2[k].clone());
        }
    }

    if j == b.n.try_into().unwrap() {
        for k in i..a.n.try_into().unwrap() {
            v.push(mon2[k].clone());
        }
    }
    let c = PolynomialOld{n: max(a.n, b.n), monomial: v.clone()};
    return c;
}

pub fn sub_old(a: PolynomialOld, mut b: PolynomialOld) -> PolynomialOld {
    for mono in &mut b.monomial {
        mono.coef = mono.coef * (-1);
    }
    return add_old(a, b);
}

pub fn mul_old(a: PolynomialOld, b: PolynomialOld) -> PolynomialOld {
    let mut v = vec!();
    let mut mono = Monomial{coef: 0, expo: 0};

    for a_mono in &a.monomial {
        for b_mono in &b.monomial {
            mono.coef = a_mono.coef * b_mono.coef;
            mono.expo = a_mono.expo + b_mono.expo;
            v.push(mono.clone());
        }
    }
    let c = PolynomialOld{n: max(a.n, b.n), monomial: v.clone()};
    return c;
}

pub fn eval_old(a: PolynomialOld, b: PolynomialOld, operation: i32) -> PolynomialOld {
    match operation {
        1 => add_old(a, b),
            
        2 => sub_old(a, b),

        3 => mul_old(a, b),

        _ => return a,
    }
}

pub fn rand_old() -> PolynomialOld {
    let mut rng = rand::thread_rng();
    let n = 1000;
    
    let mut v = vec!();
    let mut mono = Monomial{coef: 0, expo: 0};
    
    for i in 0..n as usize {
        mono.coef = rng.gen_range(-100..100);
        mono.expo = i as i32;
        v.push(mono.clone());
    }
    let poly = PolynomialOld{n, monomial: v.clone()};
    return poly;
}