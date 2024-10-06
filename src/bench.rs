mod polynomial;
use polynomial::Polynomial;

fn main() {
    divan::main()
}
fn rand_poly_pair(num_coeficients: usize) -> (Polynomial, Polynomial) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let a = Polynomial::new_rand(&mut rng, num_coeficients);
    let b = Polynomial::new_rand(&mut rng, num_coeficients);
    return (a, b);
}
fn pow_10(n: usize) -> usize {
    let mut x = 1;
    for _ in 0..n {
        x *= 10;
    }
    x
}
#[divan::bench(consts = [0,1,2,3,4,5,6,7])]
fn add_poly<const SCALE: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| rand_poly_pair(pow_10(SCALE)))
        .bench_refs(|(a, b)| {
            polynomial::add_poly(a, b);
        })
}

#[divan::bench(consts = [0,1,2,3,4,5,6,7])]
fn sub_poly<const SCALE: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| rand_poly_pair(pow_10(SCALE)))
        .bench_refs(|(a, b)| {
            polynomial::sub_poly(a, b);
        })
}
mod mutiplication {
    use super::*;
    #[divan::bench(consts = [0,1,2,3,4,5])]
    fn mult_poly<const SCALE: usize>(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| rand_poly_pair(pow_10(SCALE)))
            .bench_refs(|(a, b)| {
                polynomial::mul_poly(a, b);
            })
    }
    #[divan::bench(consts = [0,1,2,3,4,5,6,7])]
    fn par_mult_poly<const SCALE: usize>(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| rand_poly_pair(pow_10(SCALE)))
            .bench_refs(|(a, b)| {
                polynomial::par_mul_poly(a, b);
            })
    }
}
