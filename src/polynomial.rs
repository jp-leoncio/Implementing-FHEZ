use std::cmp::max;
use std::cmp::min;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub n: i128,
    pub coeficients: Vec<i128>,
}

pub fn _print_poly(a: Polynomial) {
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

pub fn add_poly(a: Polynomial, b: Polynomial) -> Polynomial {
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

fn add_eval(alpha: i128, a: Polynomial, b: Polynomial) {
    let c = add_poly(a.clone(), b.clone());
    let mut value_ab = 0i128;
    let mut value_c = 0i128;
    let mut x = 1i128;

    for i in 0..(c.n as usize) {
        value_c += c.coeficients[i] * x;
        x *= alpha;
    }
    x = 1i128;
    for i in 0..(a.n as usize) {
        value_ab += a.coeficients[i] * x;
        x *= alpha;
    }
    x = 1i128;
    for i in 0..(b.n as usize) {
        value_ab += b.coeficients[i] * x;
        x *= alpha;
    }
    assert_eq!(value_ab, value_c);
}

pub fn _sub_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let mut minus = b.clone();
    for i in 0..(b.n as usize) {
        minus.coeficients[i] *= -1; 
    }
    return add_poly(a, minus);
}

fn sub_eval(alpha: i128, a: Polynomial, b: Polynomial) {
    let mut minus = b.clone();
    for i in 0..(b.n as usize) {
        minus.coeficients[i] *= -1; 
    }

    return add_eval(alpha, a, minus);
}

pub fn mul_poly(a: Polynomial, b: Polynomial) -> Polynomial {
    let tam = a.n + b.n - 1;
    let mut ret = Polynomial{n: tam, coeficients: vec![0; tam as usize]};
    for i in 0..(a.n as usize) {
        for j in 0..(b.n as usize) {
            ret.coeficients[i + j] += a.coeficients[i] * b.coeficients[j];
        }
    }
    return ret;
}

fn mul_eval(alpha: i128, a: Polynomial, b: Polynomial) {
    let c = mul_poly(a.clone(), b.clone());
    let mut value_a = 0i128;
    let mut value_b = 0i128;
    let mut value_c = 0i128;
    let mut x = 1i128;
    let mut y = 1i128;
    let mut z = 1i128;

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

pub fn eval(alpha: i128, a: Polynomial, b: Polynomial, operation: i32) {
    match operation {
        1 => add_eval(alpha, a, b),
            
        2 => sub_eval(alpha, a, b),

        3 => mul_eval(alpha, a, b),

        _ => println!("Don't care"),
    }
}

pub fn rand_poly() -> Polynomial {
    let mut rng = rand::thread_rng();
    let tam: i128 = rng.gen_range(1..15);
    let mut v = vec![0; tam as usize];
    
    for i in 0..tam as usize {
        let coef = rng.gen_range(-1000..1000);
        v[i] = coef;
    }
    let poly = Polynomial{n: tam, coeficients: v};
    return poly;
}