use crate::prelude::*;

pub struct GaheContext {
    pub t: BigInt,  // Módulo do espaço de mensagens
    pub n: usize,   // Grau do polinômio/dimensão do anel
    pub gamma: u64, // Bit-length para `q`
    pub rho: u64,   // Bit-length para o ruído `r`
}

/// Chave secreta do esquema GAHE
pub struct GaheSecretKey {
    pub p: BigInt,              // Primo secreto
    pub k: BigPolynomial,       // Polinômio secreto
    pub k_inv: BigPolynomial,   // Inversa de `k` mod p
    pub x0: BigInt,             // Módulo privado
}

/// Cifra um polinômio de mensagem usando o esquema GAHE (EncScalar)
pub fn encrypt_scalar(sk: &GaheSecretKey, context: &GaheContext, message: &BigPolynomial) -> BigPolynomial {
    // O artigo define c := [x + m * ⌊p/t⌉ * k]_x0
    // x := (pq + r)k

    // Amostra `q` e `r` para criar o termo de erro
    // q de [0, 2^gamma / p] e r de [-2^rho, 2^rho]
    let q_bound = (BigInt::one() << context.gamma) / &sk.p;
    let r_bound = BigInt::one() << context.rho;

    let q = sample_poly_uniform_bound(&q_bound, context.n);
    let r = sample_poly_signed_bound(&r_bound, context.n);

    // Calcula pq + r
    let p_q_plus_r = (&sk.p * &q) + &r;

    // Calcula x = (pq + r)k
    let x = (&p_q_plus_r + &sk.k) % &sk.x0;

    // Calcula o fator de escala α = ⌊p/t⌉
    let alpha = round_bigint_division(&sk.p, &context.t);

    // Calcula o termo da mensagem: m * α * k
    let msg_term = (message * &(&alpha * &sk.k)) % &sk.x0;

    // Combina tudo para obter o cifrotexto
    // c := [x + msg_term]_x0
    let ciphertext = (&x + &msg_term) % &sk.x0;

    ciphertext
}

/// Decifra um polinômio usando o esquema GAHE (DecScalar)
pub fn decrypt_scalar(sk: &GaheSecretKey, context: &GaheContext, ciphertext: &BigPolynomial) -> BigPolynomial {
    // Decifragem := ⌊(t * [c']_p) / p⌉ mod t
    // c' := c * k⁻¹ mod x0

    // Calcula c' = c * k⁻¹ (mod x0)
    let c_prime = (ciphertext * &sk.k_inv) % &sk.x0;

    // Reduz c' módulo p, com resto centrado em zero
    let c_prime_mod_p = centered_rem_poly(&c_prime, &sk.p);

    // Calcula a etapa final: ⌊(t * [c']_p) / p⌉
    let scaled_poly = round_poly_division(&(&context.t * &c_prime_mod_p), &sk.p);

    // A mensagem final é o resultado módulo t
    let message = scaled_poly % &context.t;

    message
}