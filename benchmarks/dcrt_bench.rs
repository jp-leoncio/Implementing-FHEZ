use implementing_fhez::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion,};
type C64 = Complex<f64>;

const POLY_DEGREE: usize = 32;
const BENCH_PRIMES: &[u32] = &[
    32771, 32779, 32783, 32789, 32797, 32801, 32803, 32831, 32833, 32839, 32843,
    32869, 32887, 32909, 32911, 32917, 32933, 32939, 32941, 32957,
    ];
const PRIME_COUNT: usize = 10;

/// Função auxiliar para gerar um polinômio DCRT aleatório para os testes
fn create_random_dcrt() -> Dcrt {
    let mut rng = rand::thread_rng();
    Dcrt {
        poly: (0..PRIME_COUNT)
            .map(|_| {
                (0..POLY_DEGREE)
                    .map(|_| C64::new(rng.gen(), rng.gen()))
                    .collect()
            })
            .collect(),
        n: POLY_DEGREE,
    }
}

fn create_random_big_poly(bit_size: u64) -> BigPolynomial {
    let mut rng = rand::thread_rng();
    BigPolynomial {
        coeficients: (0..POLY_DEGREE)
            .map(|_| rng.gen_bigint(bit_size))
            .collect(),
    }
}

fn dcrt_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("DCRT Operations (Complex Coeffs)");

    // Setup 
    let poly_a = create_random_dcrt();
    let poly_b = create_random_dcrt();
    let context = DcrtContext::new(BENCH_PRIMES, 15.0, 10.0, 2.0, POLY_DEGREE as f64);
    let mut plan = Plan::new(POLY_DEGREE, Method::Measure(Duration::from_millis(10)));

    group.bench_function("Addition", |b| {
        b.iter(|| {
            let _result = black_box(&poly_a) + black_box(&poly_b);
        })
    });

    group.bench_function("Multiplication", |b| {
        b.iter(|| {
            let _result = black_box(&poly_a) * black_box(&poly_b);
        })
    });

    group.bench_function("from_dcrt Conversion", |b| {
        b.iter_batched(
            || poly_a.clone(),
            |mut p| from_dcrt::<N>(black_box(&mut p), black_box(&context), black_box(&mut plan)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

/// Benchmark para operações em polinômios com coeficientes BigInt
fn big_polynomial_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigPolynomial Operations (Integer Coeffs)");

    // Setup
    let bit_sizes_to_test = vec![50];
    let mut plan = Plan::new(POLY_DEGREE, Method::Measure(Duration::from_millis(10)));
    let context = DcrtContext::new(BENCH_PRIMES, 15.0, 10.0, 2.0, POLY_DEGREE as f64);

    for bit_size in bit_sizes_to_test {
        let poly_a = create_random_big_poly(bit_size);

        group.bench_with_input(BenchmarkId::new("to_dcrt Conversion", bit_size), &poly_a, 
            |b, poly| {
                b.iter(|| to_dcrt::<N>(black_box(poly), black_box(&context), black_box(&mut plan)))
            }
        );

        let new_a = to_dcrt::<N>(&poly_a, &context, &mut plan);
        group.bench_function("Addition fixed", |b| {
            b.iter(|| {
                let _result = black_box(&poly_a) + black_box(&poly_a);
            })
        });
    }
    group.finish();
}

fn vector_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vector Operations");

    let vector_lengths = vec![2, 4, 8, 10];

    for l_var in vector_lengths {
        // Setup:
        let vec_a: Vec<Dcrt> = (0..l_var).map(|_| create_random_dcrt()).collect();
        let vec_b: Vec<Dcrt> = (0..l_var).map(|_| create_random_dcrt()).collect();
        
        group.bench_with_input(BenchmarkId::new("inner_product", l_var), &l_var, 
            |b, _| {
                b.iter(|| inner_product(black_box(&vec_a), black_box(&vec_b)))
            }
        );
    }
    group.finish();
}

criterion_group!(
    benches, 
    dcrt_operations_benchmark, 
    big_polynomial_operations_benchmark,
    vector_operations_benchmark
);
criterion_main!(benches);
