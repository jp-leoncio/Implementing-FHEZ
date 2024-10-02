mod polynomial;
use crate::polynomial::*;
use std::time::Instant;
use rand::Rng;

fn main() {
    let now = Instant::now();

    let mut rng = rand::thread_rng();
    let tam = 100000;
    for _ in 0..(tam as usize) {
        let a = rand_poly();
        let b = rand_poly();
        let x = rng.gen_range(-10..10);
        let ope = rng.gen_range(1..4);
        eval(x, a, b, ope);
    }

    let elapsed_time = now.elapsed();
    println!("Running the cycle took {} milliseconds.", elapsed_time.as_millis());
}