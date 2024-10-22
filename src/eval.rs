use crate::polynomial::*;

fn add_eval(alpha: i32, a: &Polynomial, b: &Polynomial) {
    let c = add_poly(a, b);
    let mut value_ab = 0i32;
    let mut value_c = 0i32;
    let mut x = 1i32;

    for i in 0..(c.n as usize) {
        value_c += c.coeficients[i] * x;
        x *= alpha;
    }
    x = 1i32;
    for i in 0..(a.n as usize) {
        value_ab += a.coeficients[i] * x;
        x *= alpha;
    }
    x = 1i32;
    for i in 0..(b.n as usize) {
        value_ab += b.coeficients[i] * x;
        x *= alpha;
    }
    assert_eq!(value_ab, value_c);
}

fn sub_eval(alpha: i32, a: &Polynomial, b: &Polynomial) {
    let mut minus = b.clone();
    for i in 0..(b.n as usize) {
        minus.coeficients[i] *= -1; 
    }
    return add_eval(alpha, a, &minus);
}

fn mul_eval(alpha: i32, a: &Polynomial, b: &Polynomial) {
    let c = mul_poly_fast(a, b);
    let mut value_a = 0i32;
    let mut value_b = 0i32;
    let mut value_c = 0i32;
    let mut x = 1i32;
    let mut y = 1i32;
    let mut z = 1i32;

    for i in 0..(c.n as usize) {
        value_c += c.coeficients[i] * x;
        x *= alpha;
    }
    for i in 0..(a.n as usize) {
        value_a += a.coeficients[i] * y;
        y *= alpha;
    }
    for i in 0..(b.n as usize) {
        value_b += b.coeficients[i] * z;
        z *= alpha;
    }
    assert_eq!(value_a * value_b, value_c);
}

pub fn eval(alpha: i32, a: &Polynomial, b: &Polynomial, operation: i32) {
    match operation {
        1 => add_eval(alpha, a, b),
            
        2 => sub_eval(alpha, a, b),

        3 => mul_eval(alpha, a, b),

        _ => println!("Don't care"),
    }
}