use std::cmp::max;
use std::cmp::min;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub n: i32,
    pub coeficients: Vec<i32>,
}

pub fn _print_poly(a: &Polynomial) {
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

pub fn add_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
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

fn _add_eval(alpha: i32, a: &Polynomial, b: &Polynomial) {
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

pub fn sub_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut minus = b.clone();
    for i in 0..(b.n as usize) {
        minus.coeficients[i] *= -1; 
    }
    return add_poly(a, &minus);
}

fn _sub_eval(alpha: i32, a: &Polynomial, b: &Polynomial) {
    let mut minus = b.clone();
    for i in 0..(b.n as usize) {
        minus.coeficients[i] *= -1; 
    }

    return _add_eval(alpha, a, &minus);
}

pub fn mul_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let n = a.n + b.n - 1;
    let mut ret = Polynomial {
        n,
        coeficients: vec![0; n as usize],
    };
    a.coeficients
        .iter()
        .enumerate()
        .flat_map(move |(i, a_coef)| {
            b.coeficients
                .iter()
                .enumerate()
                .map(move |(j, b_coef)| (i + j, a_coef * b_coef))
        })
        .for_each(|(i, coef)| {
            ret.coeficients[i] += coef;
        });
    return ret;
}
pub fn par_mul_poly(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let n = a.n + b.n - 1;
    let coeficients = a
        .coeficients
        .par_iter()
        .enumerate()
        .flat_map(|(i, a_coef)| {
            b.coeficients
                .par_iter()
                .enumerate()
                .map(move |(j, b_coef)| (i + j, a_coef * b_coef))
        })
        .fold_with(vec![0; n as usize], |mut vec, (i, coef)| {
            vec[i] += coef;
            vec
        }).find_first(|_| true).unwrap();

    Polynomial { n, coeficients }
}
fn _mul_eval(alpha: i32, a: &Polynomial, b: &Polynomial) {
    let c = mul_poly(a, b);
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

/*pub fn eval(alpha: i32, a: Polynomial, b: Polynomial, operation: i32) {
    match operation {
        1 => add_eval(alpha, a, b),
            
        2 => sub_eval(alpha, a, b),

        3 => mul_eval(alpha, a, b),

        _ => println!("Don't care"),
    }
}*/

pub fn eval(a: &Polynomial, b: &Polynomial, operation: i32) -> Polynomial {
    match operation {
        1 => add_poly(a, b),
            
        2 => sub_poly(a, b),

        3 => mul_poly(a, b),

        _ => return a.clone(),
    }
}




impl Polynomial {
    pub fn new(coeficients: &Vec<i32>, qt_coeficients: i32) -> Polynomial {
        return Polynomial {
            n: qt_coeficients,
            coeficients: coeficients.clone(),
        };
    }
    pub fn new_rand(rng: &mut ThreadRng, num_coeficients: usize) -> Self {
        let mut coeficients = vec![0; num_coeficients];
        for i in 0..num_coeficients {
            coeficients[i] = rng.gen_range(-100..100);
        }
        return Polynomial {
            coeficients,
            n: num_coeficients as i32,
        };
    }
}
