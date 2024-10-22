use crate::polynomial::*;
use std::time::Instant;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha;
use rand_chacha::ChaCha8Rng; 

fn random_coeficients(num_coeficients: usize, num_polynomials: usize, mut _rng: ChaCha8Rng) -> Vec<Vec<i32>> {
    let mut pols = vec![vec![0; num_coeficients]; num_polynomials];
    for i in 0..num_polynomials  {
        for j in 0..num_coeficients {
            pols[i][j] = _rng.gen_range(-1000..1000);
        }
    }
    return pols;
}

fn coeficients_to_polynomials(pols: &Vec<Vec<i32>>, qt_coeficients: usize) -> Vec<Polynomial> {
    return pols.iter()
        .map(|coeficients| Polynomial::new(&coeficients, qt_coeficients as i32))
        .collect()
}

pub fn comparision() {
    let mut _rng = ChaCha8Rng::seed_from_u64(1);
    let num_samples = 100;
    let num_coeficients = 5000;
    println!("Running brenchmark with the parameters");
    println!("Number of samples = {num_samples}");
    println!("Number of coefficients = {num_coeficients}");

    println!("Started : Coeficient Generation");
    let pols = random_coeficients(num_coeficients, 2*num_samples, _rng);
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

}
