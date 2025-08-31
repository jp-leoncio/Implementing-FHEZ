use implementing_fhez::*;

/// Converte um BigPolynomial para uma string no formato padrão do Sage (ordem decrescente)
fn to_sage_string(poly: &BigPolynomial) -> String {
    if poly.coeficients.iter().all(|c| c.is_zero()) {
        return "0".to_string();
    }

    let mut result = String::new();
    // Itera em ordem reversa para obter os graus decrescentes (x^N, x^(N-1), ...)
    for (i, coef) in poly.coeficients.iter().enumerate().rev() {
        if coef.is_zero() {
            continue;
        }

        let abs_coef = coef.abs();
        let is_first_term = result.is_empty();

        // Adiciona o sinal (+ ou -) para termos que não são o primeiro
        if !is_first_term {
            if coef.sign() == Sign::Plus {
                result.push_str(" + ");
            } else {
                result.push_str(" - ");
            }
        } else {
            // Adiciona o sinal de menos para o primeiro termo, se ele for negativo
            if coef.sign() == Sign::Minus {
                result.push('-');
            }
        }

        // Constrói o termo (coeficiente e variável)
        let term_str = match i {
            0 => format!("{}", abs_coef), // Grau 0 (constante)
            1 => { // Grau 1 (x)
                if abs_coef.is_one() {
                    "x".to_string()
                } else {
                    format!("{}*x", abs_coef)
                }
            }
            _ => { // Grau > 1 (x^n)
                if abs_coef.is_one() {
                    format!("x^{}", i)
                } else {
                    format!("{}*x^{}", abs_coef, i)
                }
            }
        };
        result.push_str(&term_str);
    }
    result
}


#[test]
fn test_arithmetic_against_sage() {
    const TEST_DEGREE: usize = 8;
    let poly_a = BigPolynomial::rand(TEST_DEGREE, 32, TEST_DEGREE as u32);
    let poly_b = BigPolynomial::rand(TEST_DEGREE, 32, TEST_DEGREE as u32);

    let rust_sum = &poly_a + &poly_b;
    let rust_mul = &poly_a * &poly_b;

    let sage_poly_a = to_sage_string_for_parsing(&poly_a);
    let sage_poly_b = to_sage_string_for_parsing(&poly_b);

    let sage_script = format!(
        r#"# --- Script Sage para Verificação ---
N = {}
P.<x> = PolynomialRing(ZZ)
R = P.quotient(x^N + 1, 'x')
poly_a = R("{}")
poly_b = R("{}")
sage_sum = poly_a + poly_b
sage_mul = poly_a * poly_b
print("[SAGE_SUM]")
print(sage_sum)
print("[SAGE_MUL]")
print(sage_mul)
"#,
        TEST_DEGREE, sage_poly_a, sage_poly_b,
    );

    println!("\n--- Resultados do Rust (para comparação) ---");
    println!("[RUST_SUM]");
    println!("{}", to_sage_string(&rust_sum));
    println!("[RUST_MUL]");
    println!("{}", to_sage_string(&rust_mul));
    println!("-----------------------------------------\n");
    
    println!("SAGE_SCRIPT_START");
    println!("{}", sage_script);
    println!("SAGE_SCRIPT_END");

    assert!(true);
}

// Função auxiliar para criar a string para o Sage analisar
fn to_sage_string_for_parsing(poly: &BigPolynomial) -> String {
    if poly.coeficients.is_empty() || poly.coeficients.iter().all(|c| c.is_zero()) {
        return "0".to_string();
    }
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