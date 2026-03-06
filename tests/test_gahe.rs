use implementing_fhez::*;

// Generates test key parameters that satisfy the correctness condition
// condition necessary for the decrypt to work properly
// 2^rho plus half of t minus 1 must be less than p divided by 2t
fn make_test_key(n: usize) -> (GaheSecretKey, GaheContext) {
    let p = BigInt::from(19u32);
    let t = BigInt::from(3u32);
    let x0 = BigInt::from(38003u32);

    let mut k_coeffs = vec![BigInt::zero(); n];
    k_coeffs[0] = BigInt::from(3u32);
    let k = BigPolynomial {
        coefficients: k_coeffs,
    };

    let mut k_inv_coeffs = vec![BigInt::zero(); n];
    k_inv_coeffs[0] = BigInt::from(12668u32);
    let k_inv = BigPolynomial {
        coefficients: k_inv_coeffs,
    };

    let sk = GaheSecretKey { p, k, k_inv, x0 };
    let ctx = GaheContext {
        t,
        n,
        gamma: 10,
        rho: 1,
    };
    (sk, ctx)
}

fn encrypt_deterministic(
    sk: &GaheSecretKey,
    ctx: &GaheContext,
    message: &BigPolynomial,
    q: &BigPolynomial,
    r: &BigPolynomial,
) -> BigPolynomial {
    let p_q_plus_r = (&sk.p * q) + r;
    let x = (p_q_plus_r * &sk.k) % &sk.x0;
    let alpha = round_bigint_division(&sk.p, &ctx.t);
    let msg_term = (message * &(&alpha * &sk.k)) % &sk.x0;
    (&x + &msg_term) % &sk.x0
}

#[test]
fn test_encrypt_decrypt_roundtrip() {
    let n = 4;
    let (sk, ctx) = make_test_key(n);
    let message = BigPolynomial {
        coefficients: vec![
            BigInt::from(1i32),
            BigInt::from(2i32),
            BigInt::from(1i32),
            BigInt::from(0i32),
        ],
    };
    let q = BigPolynomial {
        coefficients: vec![
            BigInt::from(2u32),
            BigInt::from(1u32),
            BigInt::from(3u32),
            BigInt::from(0u32),
        ],
    };
    let r = BigPolynomial {
        coefficients: vec![
            BigInt::from(1i32),
            BigInt::from(-1i32),
            BigInt::from(0i32),
            BigInt::from(1i32),
        ],
    };

    let ciphertext = encrypt_deterministic(&sk, &ctx, &message, &q, &r);
    let decrypted = decrypt_scalar(&sk, &ctx, &ciphertext);

    assert_eq!(
        decrypted.coefficients, message.coefficients,
        "Expected {:?} but got {:?}",
        message.coefficients, decrypted.coefficients
    );
}

#[test]
fn test_encrypt_decrypt_zero_message() {
    let n = 4;
    let (sk, ctx) = make_test_key(n);
    let zero_msg = BigPolynomial::new(n);
    let q = BigPolynomial {
        coefficients: vec![
            BigInt::from(3u32),
            BigInt::from(0u32),
            BigInt::from(2u32),
            BigInt::from(1u32),
        ],
    };
    let r = BigPolynomial {
        coefficients: vec![
            BigInt::from(-1i32),
            BigInt::from(1i32),
            BigInt::from(0i32),
            BigInt::from(-1i32),
        ],
    };

    let ciphertext = encrypt_deterministic(&sk, &ctx, &zero_msg, &q, &r);
    let decrypted = decrypt_scalar(&sk, &ctx, &ciphertext);

    assert!(
        decrypted.coefficients.iter().all(|c| c.is_zero()),
        "Zero message must decrypt to zero but got {:?}",
        decrypted.coefficients
    );
}

#[test]
fn test_encrypt_decrypt_max_message() {
    let n = 4;
    let (sk, ctx) = make_test_key(n);
    let t_minus_1 = &ctx.t - BigInt::one();
    let message = BigPolynomial {
        coefficients: vec![t_minus_1.clone(); n],
    };
    let q = BigPolynomial {
        coefficients: vec![
            BigInt::from(1u32),
            BigInt::from(2u32),
            BigInt::from(0u32),
            BigInt::from(1u32),
        ],
    };
    let r = BigPolynomial {
        coefficients: vec![
            BigInt::from(1i32),
            BigInt::from(-1i32),
            BigInt::from(1i32),
            BigInt::from(0i32),
        ],
    };

    let ciphertext = encrypt_deterministic(&sk, &ctx, &message, &q, &r);
    let decrypted = decrypt_scalar(&sk, &ctx, &ciphertext);

    assert_eq!(
        decrypted.coefficients, message.coefficients,
        "Expected {:?} but got {:?}",
        message.coefficients, decrypted.coefficients
    );
}

#[test]
fn test_ciphertext_internal_structure() {
    let n = 4;
    let (sk, ctx) = make_test_key(n);
    let message = BigPolynomial {
        coefficients: vec![
            BigInt::from(2u32),
            BigInt::from(0u32),
            BigInt::from(1u32),
            BigInt::from(1u32),
        ],
    };
    let q = BigPolynomial {
        coefficients: vec![
            BigInt::from(1u32),
            BigInt::from(0u32),
            BigInt::from(2u32),
            BigInt::from(0u32),
        ],
    };
    let r = BigPolynomial {
        coefficients: vec![
            BigInt::from(0i32),
            BigInt::from(1i32),
            BigInt::from(-1i32),
            BigInt::from(0i32),
        ],
    };
    let alpha = round_bigint_division(&sk.p, &ctx.t);
    let expected_inner: Vec<BigInt> = (0..n)
        .map(|i| {
            &sk.p * &q.coefficients[i] + &r.coefficients[i] + &message.coefficients[i] * &alpha
        })
        .collect();

    let ciphertext = encrypt_deterministic(&sk, &ctx, &message, &q, &r);
    let c_prime = (ciphertext * &sk.k_inv) % &sk.x0;

    for i in 0..n {
        let expected = expected_inner[i].rem_euclid(&sk.x0);
        assert_eq!(
            c_prime.coefficients[i], expected,
            "Index {i} got {} but expected {} modulo x0",
            c_prime.coefficients[i], expected
        );
    }
}

// encrypts random data and decrypts to check stability across 50 rounds
#[test]
fn test_random_encrypt_decrypt_multiple_rounds() {
    let n = 4;
    let (sk, ctx) = make_test_key(n);

    for round in 0..50u64 {
        let msg_coeffs: Vec<BigInt> = (0..n)
            .map(|i| BigInt::from((i as u64 + round) % 3))
            .collect();
        let message = BigPolynomial {
            coefficients: msg_coeffs,
        };

        let ciphertext = encrypt_scalar(&sk, &ctx, &message);
        let decrypted = decrypt_scalar(&sk, &ctx, &ciphertext);

        assert_eq!(
            decrypted.coefficients, message.coefficients,
            "Round {round} failed decrypt did not match original message"
        );
    }
}
