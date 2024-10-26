use crate::polynomial::*;
use std::time::Instant;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha;
use rand_chacha::ChaCha8Rng; 

fn random_coeficients(num_coeficients: usize, num_polynomials: usize, mut rng: ChaCha8Rng) -> Vec<Vec<i32>> {
    let mut pols = vec![vec![0; num_coeficients]; num_polynomials];
    for i in 0..num_polynomials  {
        for j in 0..num_coeficients {
            pols[i][j] = rng.gen_range(-1000..1000);
        }
    }
    return pols;
}

fn random_degree(num_polynomials: usize, mut rng: ChaCha8Rng) -> Vec<u32> {
    let mut degrees = vec![0; num_polynomials];
    for i in 0..num_polynomials {
        degrees[i] = rng.gen_range(1..1000);
    }
    return degrees;
}

fn coeficients_to_polynomials(pols: &Vec<Vec<i32>>, qt_coeficients: usize) -> Vec<Polynomial> {
    return pols.iter()
        .map(|coeficients| Polynomial::new(&coeficients, qt_coeficients as u32))
        .collect()
}

pub fn comparision() {
    let rng = ChaCha8Rng::seed_from_u64(1);
    let num_samples = 100;
    let num_coeficients = 5000;
    println!("Running brenchmark with the parameters");
    println!("Number of samples = {num_samples}");
    println!("Number of coefficients = {num_coeficients}");

    println!("Started : Coeficient Generation");
    let pols = random_coeficients(num_coeficients, 2*num_samples, rng.clone());
    println!("Finished: Coeficient Generation");

    println!("Started : Initialization of polynomials");
    let polsnew  = coeficients_to_polynomials(&pols, num_coeficients);
    println!("Finished: Initialization of polynomials");

    let now = Instant::now();
    for i in 0..num_samples {
        mul_poly_fast(&polsnew[2*i], &polsnew[2*i+1]);
    }
    let elapsed_time = now.elapsed();
    println!("Fast: {} miliseconds.", elapsed_time.as_millis());


    let now = Instant::now();
    for i in 0..num_samples {
        mul_poly_mid(&polsnew[2*i], &polsnew[2*i+1]);
    }
    let elapsed_time = now.elapsed();
    println!("Mid: {} miliseconds.", elapsed_time.as_millis());


    let now = Instant::now();
    for i in 0..num_samples {
        unsafe {mul_poly_naive_unsafe(&polsnew[2*i], &polsnew[2*i+1])};   
    }
    let elapsed_time = now.elapsed();
    println!("Naive unsafe: {} miliseconds.", elapsed_time.as_millis());


    let now = Instant::now();
    for i in 0..num_samples {
        mul_poly_naive(&polsnew[2*i], &polsnew[2*i+1]);   
    }
    let elapsed_time = now.elapsed();
    println!("Naive safe: {} miliseconds.", elapsed_time.as_millis());


    println!("######################################");
    let num_samples = 10000;
    let num_coeficients = 50000;
    println!("Running brenchmark with the parameters");
    println!("Number of samples = {num_samples}");
    println!("Number of coefficients = {num_coeficients}");

    println!("Started : Coeficient Generation");
    let pols = random_coeficients(num_coeficients, 2*num_samples, rng.clone());
    println!("Finished: Coeficient Generation");

    println!("Started : Degrees Generation");
    let degrees = random_degree(2*num_samples, rng);
    println!("Finished: Degrees Generation");

    println!("Started : Initialization of polynomials");
    let polsnew  = coeficients_to_polynomials(&pols, num_coeficients);
    println!("Finished: Initialization of polynomials");

    let now = Instant::now();
    for i in 0..num_samples {
        module_poly(&polsnew[2*i], degrees[i]);   
    }
    let elapsed_time = now.elapsed();
    println!("Mod X^n + 1: {} milliseconds.", elapsed_time.as_millis());
}
