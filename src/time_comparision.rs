use crate::polynomial::*;
//use crate::polynomial_old::*;
//use rand::Rng;
use std::time::Instant;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha;
use rand_chacha::ChaCha8Rng; 

fn random_coeficients(num_coeficients: usize, num_polynomials: usize, mut _rng: ChaCha8Rng) -> Vec<Vec<i32>> {
    let mut pols = vec![vec![0; num_coeficients]; num_polynomials];
    for i in 0..num_polynomials  {
        for j in 0..num_coeficients {
            pols[i][j] = _rng.gen_range(-100..100);
        }
    }
    pols
}

fn coeficients_to_polynomials(pols: &Vec<Vec<i32>>, qt_coeficients: usize) -> Vec<Polynomial> {
    pols.iter()
        .map(|coeficients| Polynomial::_new(&coeficients, qt_coeficients as i32))
        .collect()
}

/*fn _coeficients_to_polynomials_old(pols: Vec<Vec<i32>>, qt_coeficients: usize) -> Vec<PolynomialOld> {
    pols.iter()
        .map(|coeficients| PolynomialOld::new(&coeficients, qt_coeficients as i32))
        .collect()
}*/

pub fn comparision() {
    let _ope = 3;
    let mut _rng = ChaCha8Rng::seed_from_u64(1);
    let num_samples = 100;
    let num_coeficients = 10000;
    println!("Running brenchmark with the parameters");
    println!("number of samples = {num_samples}");
    println!("number of coefficients = {num_coeficients}");
    println!("started : Coeficient Generation");
    let pols = random_coeficients(num_coeficients, 2*num_samples, _rng);
    println!("finished: Coeficient Generation");

    println!("started : Initialization of polynomials");
    let polsnew  = coeficients_to_polynomials(&pols, num_coeficients);
    println!("finished: Initialization of polynomials");

    let now = Instant::now();
    for i in 0..num_samples {
        _mul_poly_fast(&polsnew[2*i], &polsnew[2*i+1]);
    }
    let elapsed_time = now.elapsed();
    println!("Fast: {} miliseconds.", elapsed_time.as_millis());

    let now = Instant::now();
    for i in 0..num_samples {
        _mul_poly_mid(&polsnew[2*i], &polsnew[2*i+1]);
    }
    let elapsed_time = now.elapsed();
    println!("Mid: {} miliseconds.", elapsed_time.as_millis());

    let now = Instant::now();
    for i in 0..num_samples {
        _mul_poly_naive(&polsnew[2*i], &polsnew[2*i+1]);
    }
    let elapsed_time = now.elapsed();
    println!("Naive: {} miliseconds.", elapsed_time.as_millis());
}
