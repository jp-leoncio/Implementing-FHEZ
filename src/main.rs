mod polynomial;
use crate::polynomial::*;
mod polynomial_old;
use crate::polynomial_old::*;
use std::time::Instant;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let tam = 500000;

    let now = Instant::now();
    for _ in 0..(tam as usize) {
        let a = rand_poly();
        let b = rand_poly();
        //let x = rng.gen_range(-15..15);
        //let ope = rng.gen_range(1..4);
        let ope = 3;
        eval(a, b, ope);
    }
    let elapsed_time = now.elapsed();
    println!("Running the cycle took {} milliseconds.", elapsed_time.as_millis());


    let now = Instant::now();
    for _ in 0..(tam as usize) {
        let a = rand_old();
        let b = rand_old();
        //let ope = rng.gen_range(1..4);
        let ope = 3;
        eval_old(a, b, ope);
    }
    let elapsed_time = now.elapsed();
    println!("Running the old cycle took {} milliseconds.", elapsed_time.as_millis());
}