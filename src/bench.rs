#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod dcrt;
mod util;
mod class;
mod polynomial;
use polynomial::*;
use dcrt::*;
// use class::*;

use concrete_fft::c64;
use std::time::Duration;
use concrete_fft::ordered::{Plan, Method};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const N: usize = 256;   // N of X^N + 1
const L: usize = 10; 
const B: f64 = 16777216.0;     // Fixed Base
const gama: f64 = 206.0;   // Module q used in (R)LWE
const l: usize = 10;     // ceil(log(q) with B base)
const PRIME_LEN: usize = 4;

// fn teste(a: &mut Criterion) {
//     let mut cri = Criterion::default();
//     cri = cri.sample_size(100);
//     let mut rng = rand::thread_rng();
//     let mut j = 1;
//     let vapo = Polynomial::new_mod(&mut rng, N, N as u32);
//     println!("{:?}", vapo);
//     cri.bench_function("será?", |b| {
//         b.iter(|| {black_box(for i in 1..2 {
//             let teste = Polynomial::new_mod(&mut rng, N, N as u32);
//             // let vapo = Polynomial::new_mod(&mut rng, 2*i, i as u32);
//             let p = add_poly(&teste, &vapo);
//             // println!("{:?}", p);
//             j += 1;
//             });
//         });
//     });
//     println!("{}", j);
// }

// pub fn class_add(c: &mut Criterion) {
//     let mut plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
//     let mut rng = rand::thread_rng();

//     let mut poly = Polynomial::new_mod(&mut rng, N, (N as u32) / 2);
//     let mut teste = Dcrt::new(N, PRIME_LEN);
//     teste.poly = to_dcrt(&mut poly, &mut plan);
//     let mut v_array: [Dcrt; l] = core::array::from_fn(|_| Dcrt::new_mod(N, PRIME_LEN));

//     c.bench_function("Add within in a class with DCRT:", |b| {
//         b.iter(|| {
//             black_box(for i in 0..l {
//                 teste.add_crt::<N>(&mut v_array[i]);
//             })
//         });
//     });
// }

pub fn dcrt_add(c: &mut Criterion) {
    let mut plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let mut rng = rand::thread_rng();

    let mut u = Polynomial::new_mod(&mut rng, N, (N as u32) / 2);
    let mut v_array: [Polynomial; l] = core::array::from_fn(|_| Polynomial::new_mod(&mut rng, N, (N as u32) / 2));
    let mut new_u = to_dcrt(&mut u, &mut plan);
    let mut new_v = [[[c64::new(0.0, 0.0); N]; PRIME_LEN]; L];

    for (i, poly) in v_array.iter_mut().enumerate().take(l) {
        new_v[i] = to_dcrt(poly, &mut plan);
    }

    c.bench_function("Add with DCRT:", |b| {
        b.iter(|| {
            black_box(for i in 0..l {
                add_crt(&mut new_v[i], &mut new_u);
            })
        });
    });
}

pub fn dcrt_mul(c: &mut Criterion) {
    let mut plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let mut rng = rand::thread_rng();

    let mut u = Polynomial::new_mod(&mut rng, N, (N as u32) / 2);
    let mut v_array: [Polynomial; l] = core::array::from_fn(|_| Polynomial::new_mod(&mut rng, N, (N as u32) / 2));
    let mut new_u = to_dcrt(&mut u, &mut plan);
    let mut new_v = [[[c64::new(0.0, 0.0); N]; PRIME_LEN]; L];

    for (i, poly) in v_array.iter_mut().enumerate().take(l) {
        new_v[i] = to_dcrt(poly, &mut plan);
    }

    c.bench_function("Mul with DCRT:", |b| {
        b.iter(|| {
            black_box(for i in 0..l {
                mul_crt(&mut new_v[i], &mut new_u);
            })
        });
    });
}

pub fn dcrt_inner(c: &mut Criterion) {
    let mut plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let mut rng = rand::thread_rng();

    let mut u: [Polynomial; L] = core::array::from_fn(|_| Polynomial::new_mod(&mut rng, N, (N as u32) / 2));
    let mut v: [Polynomial; L] = core::array::from_fn(|_| Polynomial::new_mod(&mut rng, N, (N as u32) / 2));
    let mut new_v = [[[c64::new(0.0, 0.0); N]; PRIME_LEN]; L];
    let mut new_u = [[[c64::new(0.0, 0.0); N]; PRIME_LEN]; L];

    for (i, poly) in v.iter_mut().enumerate() {
        new_v[i] = to_dcrt(poly, &mut plan);
    }

    for (i, poly) in u.iter_mut().enumerate() {
        new_u[i] = to_dcrt(poly, &mut plan);
    }

    c.bench_function("Inner product with DCRT:", |b| {
        b.iter(|| {
            black_box(inner_product_2precomp(&mut new_u, &mut new_v))
        });
    });
}

pub fn dcrt_outer(c: &mut Criterion) {
    let mut plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let mut rng = rand::thread_rng();

    let mut u = Polynomial::new_mod(&mut rng, N, (N as u32) / 2);
    let mut v: [Polynomial; L] = core::array::from_fn(|_| Polynomial::new_mod(&mut rng, N, (N as u32) / 2));
    let mut new_v = [[[c64::new(0.0, 0.0); N]; PRIME_LEN]; L];

    for (i, poly) in v.iter_mut().enumerate() {
        new_v[i] = to_dcrt(poly, &mut plan);
    }

    c.bench_function("Outer product with DCRT:", |b| {
        b.iter(|| {
            black_box(outer_product(&mut u, &mut new_v, &mut plan))
        });
    });
}

criterion_group!(benches, dcrt_outer);
criterion_main!(benches);
