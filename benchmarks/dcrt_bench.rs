use implementing_fhez::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

// Primos de 20 bits para garantir que o módulo M seja grande o suficiente
static ARTICLE_PRIMES: &[u32] = &[
    1048583, 1048589, 1048601, 1048609, 1048613, 1048627, 1048633, 1048661,
    1048681, 1048703, 1048709, 1048717, 1048721, 1048759, 1048783,
];

/// Função de benchmark genérica que agora aceita todos os parâmetros do artigo
fn run_benchmarks_for_n<const N_POLY: usize>(
    c: &mut Criterion,
    rho: u32,
    gamma: f64,
    big_l: usize,      // Parâmetro L da tabela (para produto interno)
    l_param: usize,    // Parâmetro ℓ da tabela (para produto externo)
    log_b_param: u32,  // Parâmetro log b da tabela (para produto externo)
    id: &str,
) {
    let mut group = c.benchmark_group("FHEZ Operations (Article Params)");

    // --- Setup ---
    let mut plan = Plan::new(N_POLY, Method::Measure(Duration::from_millis(10)));

    // --- CORREÇÃO APLICADA AQUI ---
    // A potência é calculada sobre um inteiro (u64) e depois convertida para f64
    let b_as_f64 = (1u64 << log_b_param) as f64;
    let context = DcrtContext::new(ARTICLE_PRIMES, gamma, l_param as f64, b_as_f64, N_POLY as f64);

    let poly_a = BigPolynomial::rand(N_POLY, rho, N_POLY as u32);
    let poly_b = BigPolynomial::rand(N_POLY, rho, N_POLY as u32);

    // --- Benchmarks de Conversão ---
    group.bench_with_input(BenchmarkId::new("to_dcrt", id), &poly_a, |b, p| {
        b.iter(|| to_dcrt::<N_POLY>(black_box(p), black_box(&context), black_box(&mut plan)))
    });

    let dcrt_a = to_dcrt::<N_POLY>(&poly_a, &context, &mut plan);
    let dcrt_b = to_dcrt::<N_POLY>(&poly_b, &context, &mut plan);

    group.bench_with_input(BenchmarkId::new("from_dcrt", id), &dcrt_a, |b, p| {
        b.iter_batched(
            || p.clone(),
            |mut p_clone| from_dcrt::<N_POLY>(black_box(&mut p_clone), black_box(&context), black_box(&mut plan)),
            BatchSize::SmallInput,
        )
    });

    // --- Benchmarks de Aritmética DCRT ---
    group.bench_with_input(BenchmarkId::new("Add (DCRT)", id), &(&dcrt_a, &dcrt_b), |b, (p1, p2)| {
        b.iter(|| black_box(*p1) + black_box(*p2))
    });
    group.bench_with_input(BenchmarkId::new("Mul (DCRT)", id), &(&dcrt_a, &dcrt_b), |b, (p1, p2)| {
        b.iter(|| black_box(*p1) * black_box(*p2))
    });
    
    // --- Benchmark de Produto Interno ---
    let dcrt_vec_a: Vec<Dcrt> = (0..big_l).map(|_| dcrt_a.clone()).collect();
    let dcrt_vec_b: Vec<Dcrt> = (0..big_l).map(|_| dcrt_b.clone()).collect();
    
    group.bench_with_input(BenchmarkId::new("Inner Product (DCRT)", id), &(&dcrt_vec_a, &dcrt_vec_b), |b, (v1, v2)| {
        b.iter(|| inner_product(black_box(v1), black_box(v2)))
    });

    // --- Benchmark do Produto Externo ---
    let b_param_u64 = 1u64 << log_b_param;
    let vector_ct: Vec<Dcrt> = (0..l_param).map(|_| dcrt_a.clone()).collect();
    let scalar_ct = dcrt_b.clone();

    group.bench_function(BenchmarkId::new("External Product (DCRT)", id), |b| {
        b.iter_batched(
            || (vector_ct.clone(), scalar_ct.clone(), plan.clone()),
            |(v_ct, mut s_ct, mut p)| {
                external_product::<N_POLY>(
                    black_box(&v_ct),
                    black_box(&mut s_ct),
                    black_box(&context),
                    black_box(&mut p),
                    l_param,
                    b_param_u64,
                )
            },
            BatchSize::SmallInput,
        )
    });
    
    group.finish();
}

// Funções "trampolim" agora passam todos os parâmetros corretos para cada linha
fn benchmark_row1(c: &mut Criterion) {
    // Parâmetros da Linha 1: L=114, ℓ=10, log b=24
    run_benchmarks_for_n::<256>(c, 56, 206.0, 114, 10, 24, "Row 1 (N=256, rho=56)");
}

fn benchmark_row3(c: &mut Criterion) {
    // Parâmetros da Linha 3: L=86, ℓ=21, log b=11
    run_benchmarks_for_n::<128>(c, 69, 204.0, 86, 21, 11, "Row 3 (N=128, rho=69)");
}

// Agrupa os benchmarks para execução
criterion_group!(benches, benchmark_row1, benchmark_row3);
criterion_main!(benches);