use std::cmp::max;

#[derive(Clone, Debug)]
struct Polynomial {
    n: i32,
    coeficients: Vec<i32>,
}

fn print_poly(a: Polynomial) {
    let mut i = a.n;
    for mono in a.coeficients.iter() {
        print!("{:?}x^{:?}", mono, i);
        if i != 1 {
            print!(" + ");
        }
        i -= 1;
    }
    println!("");
}
/*
fn add_poly(a: Polynomial, b: Polynomial) -> Polynomial {

}

fn mul_poly(a: Polynomial, b: Polynomial) -> Polynomial {

}
*/
fn main() {
    let mut coef = vec![2, 6, 7, 0, 13];
    let a = Polynomial{n: 5, coeficients: coef};

    coef = vec![8, 3, 4, 14];
    let b = Polynomial{n: 4, coeficients: coef};

    print_poly(a);
    print_poly(b);
}