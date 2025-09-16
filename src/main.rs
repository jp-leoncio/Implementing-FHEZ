use implementing_fhez::*;

/// Calcula o produto interno de dois vetores de BigPolynomials
/// Usado para calcular o "ground truth" no nosso teste de corretude
fn inner_product_big_poly(a: &[BigPolynomial], b: &[BigPolynomial]) -> BigPolynomial {
    assert_eq!(a.len(), b.len(), "Os vetores devem ter o mesmo comprimento.");
    if a.is_empty() {
        return BigPolynomial::new(0);
    }
    let mut res = &a[0] * &b[0];
    for i in 1..a.len() {
        res = &res + &(&a[i] * &b[i]);
    }
    res
}

/// Testa a corretude matemática da função `external_product` usando os parâmetros do artigo
fn test_external_product_correctness() {
    println!("\n--- Teste de Corretude do Produto Externo (com Parâmetros do Artigo) ---");

    // --- Parâmetros (baseado na Linha 1 da Tabela 2 do artigo) ---
    const N_POLY: usize = 256;
    const RAND_BITS: u32 = 56;
    const L_PARAM: usize = 10;
    const B_PARAM: u64 = 1 << 24;
    const GAMMA: f64 = 206.0;
    
    static TEST_PRIMES_ARTICLE: &[u32] = &[
        1048583, 1048589, 1048601, 1048609, 1048613, 1048627, 1048633, 1048661,
        1048681, 1048703, 1048709, 1048717, 1048721, 1048759, 1048783,
    ];
    
    let mut plan = Plan::new(N_POLY, Method::Measure(Duration::from_millis(10)));
    let context = DcrtContext::new(TEST_PRIMES_ARTICLE, GAMMA, L_PARAM as f64, B, N_POLY as f64);
    
    println!("A gerar dados de teste com os parâmetros do artigo (pode demorar um pouco)...");
    // Geração de Dados
    let vector_messages: Vec<BigPolynomial> = (0..L_PARAM)
        .map(|_| BigPolynomial::rand(N_POLY, RAND_BITS, N_POLY as u32))
        .collect();
    let scalar_message = BigPolynomial::rand(N_POLY, RAND_BITS, N_POLY as u32);

    // Cálculo do "Ground Truth" (Resultado Exato)
    println!("A calcular o resultado exato...");
    let decomposed_scalar_messages = gadget_decompose::<N_POLY>(&scalar_message, L_PARAM, B_PARAM);
    let exact_result = inner_product_big_poly(&vector_messages, &decomposed_scalar_messages);

    // Cálculo com o Pipeline DCRT/FFT
    println!("A calcular o resultado via DCRT/FFT...");
    let vector_ciphertext_dcrt: Vec<Dcrt> = vector_messages.iter()
        .map(|p| to_dcrt::<N_POLY>(p, &context, &mut plan))
        .collect();
    let mut scalar_ciphertext_dcrt = to_dcrt::<N_POLY>(&scalar_message, &context, &mut plan);

    let result_dcrt = external_product::<N_POLY>(
        &vector_ciphertext_dcrt, 
        &mut scalar_ciphertext_dcrt, 
        &context, 
        &mut plan, 
        L_PARAM, 
        B_PARAM
    );

    let mut result_dcrt_mut = result_dcrt.clone();
    let approx_result = from_dcrt::<N_POLY>(&mut result_dcrt_mut, &context, &mut plan);

    // Verificação (com prints mantidos)
    println!("\n--- Resultados da Verificação ---");
    println!("Resultado Exato (BigPoly):");
    // print_poly(&exact_result);
    println!("Resultado Obtido (DCRT):");
    // print_poly(&approx_result);

    let error_poly = &exact_result - &approx_result;
    let max_error = error_poly.coeficients.iter().map(|c| c.abs()).max().unwrap_or_else(BigInt::zero);

    println!("\nErro Máximo Encontrado: {}", max_error);
    assert!(max_error <= BigInt::from(1), "FALHA: O erro do produto externo ({}) excedeu a tolerância.", max_error);
    println!("\n✅ SUCESSO: A função de produto externo é correta dentro de uma pequena tolerância de erro.");
}

fn test_dcrt_multiplication_with_article_params() {
    println!("\n--- Teste de Multiplicação DCRT com Parâmetros do Artigo ---");

    // --- 1. Configuração de Parâmetros (baseado na Tabela 2 do artigo) ---
    const N_POLY: usize = 256;         // N = 256 
    const RAND_BITS: u32 = 56;         // Relacionado a ρ (rho) = 16 
    const GAMMA: f64 = 206.0;          // γ = 206
    
    // O erro da FFT cresce com N e com a magnitude dos coeficientes.
    // Para N=256, um limiar de erro pequeno ainda é esperado.
    // Se o teste falhar, este valor pode ser aumentado.
    let error_threshold = BigInt::from(10); 
    
    // Usamos os primos de 20 bits para garantir que o módulo M seja grande o suficiente
    static TEST_PRIMES_ARTICLE: &[u32] = &[
        1048583, 1048589, 1048601, 1048609, 1048613, 1048627, 1048633, 1048661,
        1048681, 1048703, 1048709, 1048717, 1048721, 1048759, 1048783,
    ];

    let mut plan = Plan::new(N_POLY, Method::Measure(Duration::from_millis(10)));
    // O contexto DCRT é criado com base no gamma do artigo
    let context = DcrtContext::new(TEST_PRIMES_ARTICLE, GAMMA, l as f64, B, N_POLY as f64);

    // --- 2. Geração de Dados ---
    println!("A gerar polinómios aleatórios (N={}, bits={})...", N_POLY, RAND_BITS);
    let poly_a = BigPolynomial::rand(N_POLY, RAND_BITS, N_POLY as u32);
    let poly_b = BigPolynomial::rand(N_POLY, RAND_BITS, N_POLY as u32);

    // --- 3. Cálculo dos Resultados ---
    println!("A calcular o produto exato...");
    let exact_product = &poly_a * &poly_b;

    println!("A calcular o produto via DCRT/FFT...");
    let dcrt_a = to_dcrt::<N_POLY>(&poly_a, &context, &mut plan);
    let dcrt_b = to_dcrt::<N_POLY>(&poly_b, &context, &mut plan);
    let mut dcrt_product = &dcrt_a * &dcrt_b;
    let approx_product = from_dcrt::<N_POLY>(&mut dcrt_product, &context, &mut plan);

    // --- 4. Análise do Erro ---
    let error_poly = &exact_product - &approx_product;
    let max_error = error_poly.coeficients.iter()
        .map(|c| c.abs())
        .max()
        .unwrap_or_else(BigInt::zero);

    // --- 5. Verificação ---
    println!("\n--- Resultados da Verificação ---");
    println!("Erro Máximo Encontrado: {}", max_error);
    println!("Limite de Erro Aceitável: {}", error_threshold);
    
    assert!(
        max_error < error_threshold,
        "FALHA: O erro da computação DCRT ({}) excedeu o limite aceitável ({}).",
        max_error,
        error_threshold
    );

    println!("\n✅ SUCESSO: O erro da computação DCRT está dentro do limite aceitável para os parâmetros do artigo.");
}

fn main() {
    println!("---------------------");
    test_dcrt_multiplication_with_article_params();
    test_external_product_correctness();
}