mod polynomial;
use polynomial::Polynomial;

fn main() {
    divan::main()
}

fn rand_poly_pair(num_coeficients: usize) -> (Polynomial, Polynomial) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let a = Polynomial::_new_rand(&mut rng, num_coeficients);
    let b = Polynomial::_new_rand(&mut rng, num_coeficients);
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
            polynomial::_add_poly(a, b);
        })
}

#[divan::bench(consts = [0,1,2,3,4,5,6,7])]
fn sub_poly<const SCALE: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| rand_poly_pair(pow_10(SCALE)))
        .bench_refs(|(a, b)| {
            polynomial::_sub_poly(a, b);
        })
}

mod mutiplication {
    use super::*;
    #[divan::bench(consts = [0,1,2,3,4])]
    fn mult_poly_fast<const SCALE: usize>(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| rand_poly_pair(pow_10(SCALE)))
            .bench_refs(|(a, b)| {
                polynomial::_mul_poly_fast(a, b);
            })
    }
    #[divan::bench(consts = [0,1,2,3,4])]
    fn par_mult_poly<const SCALE: usize>(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| rand_poly_pair(pow_10(SCALE)))
            .bench_refs(|(a, b)| {
                polynomial::_par_mul_poly(a, b);
            })
    }
}
