use crate::polynomial::*;
use crate::polynomial_old::*;
//use rand::Rng;
use std::time::Instant;

pub fn comparision() {
    let mut _rng = rand::thread_rng();
    let tam = 1000;

    let now = Instant::now();
    for _ in 0..(tam as usize) {
        let a = rand_poly();
        let b = rand_poly();
        //let ope = rng.gen_range(1..4);
        let ope = 3;
        eval(a, b, ope);
    }
    let elapsed_time = now.elapsed();
    println!("Running the cycle took {} seconds.", elapsed_time.as_secs());


    let now = Instant::now();
    for _ in 0..(tam as usize) {
        let a = rand_old();
        let b = rand_old();
        //let ope = rng.gen_range(1..4);
        let ope = 3;
        eval_old(a, b, ope);
    }
    let elapsed_time = now.elapsed();
    println!("Running the old cycle took {} seconds.", elapsed_time.as_secs());
}