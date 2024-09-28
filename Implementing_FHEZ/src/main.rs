mod polynomial;
use crate::polynomial::*;

fn main() {
    let mut coef = vec![2, 6, 7, 0, 13];
    let a = Polynomial{n: 5, coeficients: coef};
    coef = vec![8, 3, 4, 14];
    let b = Polynomial{n: 4, coeficients: coef};

    let c = add_poly(a.clone(), b.clone());
    print_poly(c.clone());
    let d = sub_poly(c.clone(), b.clone());
    print_poly(d.clone());
    let e = mul_poly(a.clone(), b.clone());
    print_poly(e.clone());
    println!("{:?}", add_eval(2, a.clone(), b.clone()));
    println!("{:?}", mul_eval(2, a.clone(), b.clone()));
    rand_poly();
}