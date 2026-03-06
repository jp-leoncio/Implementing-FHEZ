use crate::prelude::*;

pub struct GaheContext {
    pub t: BigInt,  // Message space module
    pub n: usize,   // Polynomial degree or ring dimension
    pub gamma: u64, // Bit length of q
    pub rho: u64,   // Bit length of r noise
}

pub struct GaheSecretKey {
    pub p: BigInt,            // Secret prime
    pub k: BigPolynomial,     // Secret polynomial
    pub k_inv: BigPolynomial, // Inverse of k mod x0
    pub x0: BigInt,           // Private module
}

/// Encrypts a message polynomial using the GAHE scheme for scalar ciphertext
pub fn encrypt_scalar(
    sk: &GaheSecretKey,
    context: &GaheContext,
    message: &BigPolynomial,
) -> BigPolynomial {
    let q_bound = (BigInt::one() << context.gamma) / &sk.p;
    let r_bound = BigInt::one() << context.rho;

    let q = sample_poly_uniform_bound(&q_bound, context.n);
    let r = sample_poly_signed_bound(&r_bound, context.n);

    // x := (p * q + r) * k mod x0
    let p_q_plus_r = (&sk.p * &q) + &r;
    let x = (p_q_plus_r * &sk.k) % &sk.x0;

    // c := x + m * round(p / t) * k mod x0
    let alpha = round_bigint_division(&sk.p, &context.t);
    let msg_term = (message * &(&alpha * &sk.k)) % &sk.x0;

    (&x + &msg_term) % &sk.x0
}

/// Decrypts a polynomial using the GAHE scheme
///
/// c prime := c * k inverse mod x0
/// output round(t * [c prime]_p / p) mod t
pub fn decrypt_scalar(
    sk: &GaheSecretKey,
    context: &GaheContext,
    ciphertext: &BigPolynomial,
) -> BigPolynomial {
    let c_prime = (ciphertext * &sk.k_inv) % &sk.x0;
    let c_prime_mod_p = centered_rem_poly(&c_prime, &sk.p);
    let scaled = round_poly_division(&(&context.t * &c_prime_mod_p), &sk.p);
    scaled % &context.t
}
