use std::cmp::max;
use std::cmp::min;

#[derive(Clone, Debug)]
struct Polynomial {
    n: i32,
    coeficients: Vec<i32>,
}

fn print_poly(a: Polynomial) {
    let mut i = (a.n - 1) as usize;
    while i > 0 {
        if a.coeficients[i] != 0 {
            print!("{:?}x^{:?}", a.coeficients[i], i);
            print!(" + ");
        }
        i -= 1;
    }
    println!("{:?}", a.coeficients[0]);
}

fn add_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut c = vec![0; max(a.n, b.n) as usize];
    let ini = min(a.n, b.n) as usize;
    for i in 0..ini {
        c[i] = a.coeficients[i] + b.coeficients[i];
    }

    for i in ini..(a.n as usize) {
        c[i] = a.coeficients[i];
    }

    for i in ini..(b.n as usize) {
        c[i] = b.coeficients[i];
    }

    let poly = Polynomial{n: max(a.n, b.n), coeficients: c};
    return poly;
}

fn sub_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut minus = b.clone();
    for i in 0..(b.n as usize) {
        minus.coeficients[i] *= -1; 
    }
    return add_poly(a, minus);
}

fn mul_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let tam = a.n + b.n - 1;
    let mut ret = Polynomial{n: tam, coeficients: vec![0; tam as usize]};
    for i in 0..(a.n as usize) {
        for j in 0..(b.n as usize) {
            ret.coeficients[i + j] += a.coeficients[i] * b.coeficients[j];
        }
    }
    return ret;
}

fn add_eval(alpha: i32, a: Polynomial, b: Polynomial) -> bool {
    let c = add_poly(a.clone(), b.clone());
    let mut value_ab = 0i32;
    let mut value_c = 0i32;
    let mut x = 1i32;
    let mut y = 1i32;
    let mut z = 1i32;

    for i in 0..(c.n as usize) {
        value_c += c.coeficients[i] * x;
        x *= alpha;
    }
    for i in 0..(a.n as usize) {
        value_ab += a.coeficients[i] * y;
        y *= alpha;
    }
    for i in 0..(b.n as usize) {
        value_ab += b.coeficients[i] * z;
        z *= alpha;
    }

    if value_ab == value_c {
        return true;
    } else {
        return false;
    }
}

fn mul_eval(alpha: i32, a: Polynomial, b: Polynomial) -> bool {
    let c = mul_poly(a.clone(), b.clone());
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

    if value_a * value_b == value_c {
        return true;
    } else {
        return false;
    }
}

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
}