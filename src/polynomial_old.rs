use std::cmp::max;
use std::convert::TryInto;

#[derive(Clone, Debug, Copy)]
struct Monomial {
    coef: i32,
    expo: i32,
}
#[derive(Clone, Debug)]
struct Polynomial {
    n: i32,
    monomial: Vec<Monomial>,
}

fn print_poly(poly: Polynomial) {
    let mut real_count = 1;
    let mut count = poly.n;
    for mono in &poly.monomial {
        if mono.coef == 0 {
            count -= 1;
        }
        else if mono.coef != 0 {
            print!("{:?}x^{:?} ", mono.coef, mono.expo);
            if real_count < count {
                print!("+ "); 
            }
            real_count += 1;
        }
    }
    println!(" ");
}

fn add_poly(a: Polynomial, b: Polynomial) -> Polynomial {
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
    let c = Polynomial{n: max(a.n, b.n), monomial: v.clone()};
    return c;
}

fn sub_poly(a: Polynomial, mut b: Polynomial) -> Polynomial {
    for mono in &mut b.monomial {
        mono.coef = mono.coef * (-1);
    }
    return add_poly(a, b);
}

fn mul_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut v = vec!();
    let mut mono = Monomial{coef: 0, expo: 0};

    for a_mono in &a.monomial {
        for b_mono in &b.monomial {
            mono.coef = a_mono.coef * b_mono.coef;
            mono.expo = a_mono.expo + b_mono.expo;
            v.push(mono.clone());
        }
    }
    let c = Polynomial{n: max(a.n, b.n), monomial: v.clone()};
    return c;
}

fn main() {
    let coef = 5;
    let expo = 3;
    let a = Monomial{coef, expo};

    let coef = -8;
    let expo = 2;
    let b = Monomial{coef, expo};

    let mut vector = vec!(a.clone());
    let mut poly = Polynomial{n:1, monomial: vector.clone()};
    vector.push(b.clone());
    let poly2 = Polynomial{n:2, monomial: vector.clone()};

    print_poly(poly.clone());
    print_poly(poly2.clone());

    poly = sub_poly(poly, poly2.clone());
    poly = mul_poly(poly, poly2);
    print_poly(poly.clone());
}