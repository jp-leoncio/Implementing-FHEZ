use implementing_fhez::*;

fn to_sage_string(poly: &BigPolynomial) -> String {
    if poly.coeficients.iter().all(|c| c.is_zero()) { return "0".to_string(); }
    let mut result = String::new();
    for (i, coef) in poly.coeficients.iter().enumerate().rev() {
        if coef.is_zero() { continue; }
        let abs_coef = coef.abs();
        let is_first_term = result.is_empty();
        if !is_first_term {
            result.push_str(if coef.sign() == Sign::Plus { " + " } else { " - " });
        } else if coef.sign() == Sign::Minus {
            result.push('-');
        }
        let term_str = match i {
            0 => format!("{}", abs_coef),
            1 => if abs_coef.is_one() { "x".to_string() } else { format!("{}*x", abs_coef) },
            _ => if abs_coef.is_one() { format!("x^{}", i) } else { format!("{}*x^{}", abs_coef, i) },
        };
        result.push_str(&term_str);
    }
    result
}

fn to_sage_string_for_parsing(poly: &BigPolynomial) -> String {
    if poly.coeficients.is_empty() || poly.coeficients.iter().all(|c| c.is_zero()) { return "0".to_string(); }
    let mut parts = Vec::new();
    for (i, coef) in poly.coeficients.iter().enumerate() {
        if coef.is_zero() { continue; }
        let term_str = match i {
            0 => format!("({})", coef),
            1 => format!("({}) * x", coef),
            _ => format!("({}) * x^{}", coef, i),
        };
        parts.push(term_str);
    }
    parts.join(" + ")
}

pub fn semi_to_dcrt<const N: usize>(a: &BigPolynomial, context: &DcrtContext, plan: &mut Plan) -> Dcrt {
    let mut res = Dcrt::new(N, context.primes.len());

    for (i, p_u32) in context.primes.iter().enumerate() {
        let p_big = p_u32.to_bigint().unwrap();

        for (j, coef) in a.coeficients.iter().enumerate() {
            // Usa `rem_euclid` para garantir que o resto seja sempre positivo
            let reduced_coef = coef.rem_euclid(&p_big);
            // let reduced_coef = coef % &p_big;
            
            let coef_f64 = reduced_coef.to_f64().unwrap_or(0.0);
            res.poly[i][j] = c64::new(coef_f64, 0.0);
        }
        println!("{:?}", res);
        // Aplica a FFT
        to_fft::<N>(&mut res.poly[i], plan);
    }
    res
}


// static TEST_PRIMES: &[u32] = &[32771, 32779, 32783, 32789, 32797, 32801, 32803, 32831, 32833, 32839];
static TEST_PRIMES: &[u32] = &[3, 5, 7, 11, 13, 17, 23, 31, 37];
fn test_dcrt_inner_product_against_sage() {
    const N_POLY: usize = 2;
    const VECTOR_SIZE: usize = 2;
    const RAND_BITS: u32 = 5;

    let mut plan = Plan::new(N_POLY, Method::Measure(Duration::from_millis(10)));
    let context = DcrtContext::new(TEST_PRIMES, 5.0, 2.0, B, N_POLY as f64);

    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    for _ in 0..VECTOR_SIZE {
        vec_a.push(BigPolynomial::rand(N_POLY, RAND_BITS, N_POLY as u32));
        vec_b.push(BigPolynomial::rand(N_POLY, RAND_BITS, N_POLY as u32));
    }

    // --- Operações em Rust ---
    let dcrt_a: Vec<Dcrt> = vec_a.iter().map(|p| to_dcrt::<N_POLY>(p, &context, &mut plan)).collect();
    let dcrt_b: Vec<Dcrt> = vec_b.iter().map(|p| to_dcrt::<N_POLY>(p, &context, &mut plan)).collect();

    let teste = vec_a[0].clone();
    let new_teste = to_dcrt::<N_POLY>(&teste, &context, &mut plan);
    assert_eq!(new_teste, dcrt_a[0]);
    let vapo = vec_a[0].clone() * &vec_b[0];
    let new_vapo = to_dcrt::<N_POLY>(&vapo, &context, &mut plan);
    println!("New vapo: {:?}", new_vapo);

    // Produtos Parciais
    let mut partial_prod_dcrt_0 = &dcrt_a[0] * &dcrt_b[0];
    // Tenta implementar esse produto de dcrt na mão aqui, sem usar a operação mesmo
    let mut partial_prod_dcrt_1 = &dcrt_a[1] * &dcrt_b[1];

    println!("Produto: {:?}", partial_prod_dcrt_0);

    
    // Soma Final
    let mut final_sum_dcrt = &partial_prod_dcrt_0 + &partial_prod_dcrt_1;

    // Conversão de volta
    let rust_prod0 = from_dcrt::<N_POLY>(&mut partial_prod_dcrt_0, &context, &mut plan);
    let rust_prod1 = from_dcrt::<N_POLY>(&mut partial_prod_dcrt_1, &context, &mut plan);
    let rust_final_sum = from_dcrt::<N_POLY>(&mut final_sum_dcrt, &context, &mut plan);

    print_poly(&vec_a[0]);
    print_poly(&vec_b[0]);

    print_poly(&(vec_a[0].clone() * &vec_b[0]));
    print_poly(&rust_prod0);
}

fn main() {
    let mut plan = Plan::new(N, Method::Measure(Duration::from_millis(10)));
    let contexto = DcrtContext::new(PRIME_15, gamma, l as f64, B, N as f64);

    static PRIME_15: &[u32] = &[
    32771, 32779, 32783, 32789, 32797, 32801, 32803, 32831, 32833, 32839, 32843,
    32869, 32887, 32909, 32911, 32917, 32933, 32939, 32941, 32957,
    ];

    /*
     * Teste das operações de soma e multiplicação do DCRT
     */
    let a = BigPolynomial::rand(3, 5 as u32, (3-1) as u32);
    let b = BigPolynomial::rand(3, 5 as u32, (3-1) as u32);
    let c = BigPolynomial::rand(3, 5 as u32, (3-1) as u32);
    let d = BigPolynomial::rand(3, 5 as u32, (3-1) as u32);

    // print_poly(&a);
    // print_poly(&b);
    // print_poly(&c);
    // print_poly(&d);
    
    // let a_conv = to_dcrt::<N>(&a, &contexto, &mut plan);
    // let b_conv = to_dcrt::<N>(&b, &contexto, &mut plan);
    // let c_conv = to_dcrt::<N>(&c, &contexto, &mut plan);
    // let d_conv = to_dcrt::<N>(&d, &contexto, &mut plan);

    // let mut sum0 = a_conv.mul(&c_conv);
    // let mut sum1 = b_conv.mul(&d_conv);
    // let mut mul = sum0.add(&sum1);

    // let res_sum0 = from_dcrt::<N>(&mut sum0, &contexto, &mut plan);
    // let res_sum1 = from_dcrt::<N>(&mut sum1, &contexto, &mut plan);
    // let res_mul = from_dcrt::<N>(&mut mul, &contexto, &mut plan);
    // println!("-------------");
    // print_poly(&res_sum0);
    // print_poly(&res_sum1);
    // print_poly(&res_mul);

    // let res = BigPolynomial {
    //     coeficients: vec![BigInt::new(Sign::Minus, vec![752]), BigInt::new(Sign::Minus, vec![2200]),
    //     BigInt::new(Sign::Minus, vec![594])],
    // };
    // let modu = res_mul.module(2);
    // print_poly(&modu);

    // /**
    //  ** Teste do Teorema Chinês do Resto
    //  */
    // let m = BigInt::from(105);
    // let m_i = vec![BigInt::from(35), BigInt::from(21), BigInt::from(15)];
    // let mut vetor = vec![c64::new(-2.0, 0.0), c64::new(1.0, 0.0), c64::new(2.0, 0.0)];
    // let res = crt(&mut vetor, &m, &m_i);
    // println!("Valor do CRT: {}", res);

    // /**
    //  ** Teste da geração de polinômios aleatórios com gamma bits 
    //  */
    // let mut teste = BigPolynomial::rand(4, 5, N as u32);
    // print_poly(&teste);
    // teste = teste.module(2);
    // print_poly(&teste);    

    /*
     * Teste das tranformações de e para DCRT
     */
    // let poly = BigPolynomial::rand(N, gamma as u32, (N-1) as u32);
    // let mut vapo = to_dcrt::<N>(&poly, &contexto, &mut plan);
    // let _rebeca = from_dcrt::<N>(&mut vapo, &contexto, &mut plan);
    // print_poly(&poly);
    // print_poly(&rebeca);

    // /**
    //  ** Teste da redução de base em ZZ e em poly
    //  */
    // let mut poly = BigPolynomial::rand(N, 15, (N-1) as u32);
    // let q = 15.0_f64.exp2(); // As coisas estão entre -128 e 128
    // let numsei = vec![1.0, 4.0, 16.0, 64.0, 128.0, 256.0, 512.0, 1024.0];
    // let vapo = inv_g_poly(&mut poly, q);


    // let context = GaheContext {
    //     t: 64.to_bigint().unwrap(),     // Mensagens com coeficientes de 0 a 15
    //     n: 4,                           // Polinômios de grau 4
    //     gamma: 128,                     // Parâmetro de segurança para `q`
    //     rho: 40,                        // Ruído `r` terá ~40 bits
    // };

    // // Primo secreto `p` com 61 bits, muito maior que o ruído de 40 bits
    // let p: BigInt = (BigInt::one() << 61) - 1; 
    // let x0 = &p * BigInt::from(123456789); // Módulo privado

    // // A escolha mais simples é k=1, pois sua inversa k_inv=1
    // let k = BigPolynomial {
    //     coeficients: vec![One::one(), Zero::zero(), Zero::zero(), Zero::zero()],
    // };
    // let k_inv = k.clone(); // k_inv de 1 é 1
    // let sk = GaheSecretKey { 
    //     p: p.clone(), 
    //     k, 
    //     k_inv, 
    //     x0 
    // };

    // println!("Módulo da Mensagem (t): {}", context.t);
    // println!("Primo Secreto (p) com bit-length: {}", sk.p.bits());
    // println!("Ruído (rho) com bit-length: {}", context.rho);
    // println!("--------------------\n");

    // let message = BigPolynomial {
    //     coeficients: vec![7.to_bigint().unwrap(), 13.to_bigint().unwrap(), 37.to_bigint().unwrap(), 4.to_bigint().unwrap()],
    // };
    // println!("Mensagem Original: {:?}", message.coeficients);

    // let ciphertext = encrypt_scalar(&sk, &context, &message);
    // println!("\nCiphertext Gerado (um polinômio com coeficientes grandes):");
    // println!("Grau: {}, Exemplo de Coeficiente: {}...", ciphertext.degree(), ciphertext.coeficients[0]);

    // let decrypted_message = decrypt_scalar(&sk, &context, &ciphertext);
    // println!("\nMensagem Decifrada: {:?}", decrypted_message.coeficients);

    // assert_eq!(message, decrypted_message);
    // println!("\nSucesso: A mensagem decifrada é igual à original");

    println!("---------------------");
    test_dcrt_inner_product_against_sage();
}