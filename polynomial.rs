#![allow(dead_code)]
#[derive(Clone, Debug)]
struct Monomial {
    coef: i32,
    expo: i32,
}
#[derive(Clone)]
struct Polynomial {
    n: i32,
    monomial: [Monomial; 2],
}

fn main() {
    let coef = 5;
    let expo = 3;
    let mono = Monomial{coef, expo};

    let coef = 8;
    let expo = 10;
    let mono2 = Monomial{coef, expo};

    let poly = Polynomial{n:2, monomial:[mono.clone(), mono2.clone()]};
    println!("The monomial is {0}x^{1}", mono.coef, mono.expo);
    println!("The monomial is {0}x^{1}", mono2.coef, mono2.expo);
    println!("{:?} {:?}", poly.n, poly.monomial);
}
