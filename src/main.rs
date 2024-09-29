mod polynomial;
use crate::polynomial::*;

fn main() {
    let a = rand_poly();
    let b = rand_poly();

    print!("Polinômio a: ");
    print_poly(a.clone());
    print!("Polinômio b: ");
    print_poly(b.clone());

    let c = add_poly(a.clone(), b.clone());
    print!("Polinômio a + b: ");
    print_poly(c.clone());

    let d = sub_poly(c.clone(), b.clone());
    print!("Polinômio a - b: ");
    print_poly(d.clone());

    let e = mul_poly(a.clone(), b.clone());
    print!("Polinômio a * b: ");
    print_poly(e.clone());

    println!("{:?}", add_eval(100, a.clone(), b.clone()));
    println!("{:?}", mul_eval(100, a.clone(), b.clone()));
}