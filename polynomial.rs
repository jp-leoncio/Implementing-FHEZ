use std::cmp::max;
use std::convert::TryInto;

#[derive(Clone, Debug)]
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
    let mut count = 1;
    for mono in &poly.monomial {
        print!("{:?}x^{:?}", mono.coef, mono.expo);
        if count < poly.n {
            print!(" + "); 
        }
        count += 1;
    }
    println!(" ");
}

fn add_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut v = vec!();

    let mut mon1 = a.monomial.clone();
    let mon2 = b.monomial.clone();
    let mut i = 0usize;
    let mut j = 0usize;
    let mut _gerson = Monomial{coef: 0, expo: 0};

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

fn main() {
    let coef = 5;
    let expo = 3;
    let a = Monomial{coef, expo};

    let coef = 8;
    let expo = 2;
    let b = Monomial{coef, expo};

    let mut vector = vec!(a.clone());
    let mut poly = Polynomial{n:1, monomial: vector.clone()};
    vector.push(b.clone());
    let poly2 = Polynomial{n:2, monomial: vector.clone()};

    print_poly(poly.clone());
    print_poly(poly2.clone());

    poly = add_poly(poly, poly2);
    print_poly(poly.clone());
}