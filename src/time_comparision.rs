use crate::polynomial::*;
use crate::polynomial_old::*;
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
        .map(|coeficients| Polynomial::new(&coeficients, qt_coeficients as i32))
        .collect()
}

fn coeficients_to_polynomials_old(pols: Vec<Vec<i32>>, qt_coeficients: usize) -> Vec<PolynomialOld> {
    pols.iter()
        .map(|coeficients| PolynomialOld::new(&coeficients, qt_coeficients as i32))
        .collect()
}

pub fn comparision() {
    let ope = 3;
    let mut _rng = ChaCha8Rng::seed_from_u64(1);
    let num_samples = 10;
    let num_coeficients = 4000;
    println!("Running brenchmark with the parameters");
    println!("number of samples = {num_samples}");
    println!("number of coefficients = {num_coeficients}");
    println!("started : Coeficient Generation");
    let pols = random_coeficients(num_coeficients, 2*num_samples, _rng);
    println!("finished: Coeficient Generation");

    println!("started : Initialization of polynomials");
    let polsnew  = coeficients_to_polynomials(&pols, num_coeficients);
    let polsold = coeficients_to_polynomials_old(pols, num_coeficients);
    println!("finished: Initialization of polynomials");

    /*let now = Instant::now();
    for i in 0..num_samples {
        eval_old(&polsold[2*i], &polsold[2*i+1],ope);
    }
    let elapsed_time = now.elapsed();
    //println!("Running the old cycle took {} seconds.", elapsed_time.as_millis());*/

    let now = Instant::now();
    for i in 0..num_samples {
        eval(&polsnew[2*i], &polsnew[2*i+1], ope);
    }
    let elapsed_time = now.elapsed();
    println!("Running the cycle took {} seconds.", elapsed_time.as_millis());
}
