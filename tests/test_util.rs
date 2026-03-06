use implementing_fhez::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sym_mod_positive() {
        assert_eq!(sym_mod(BigInt::from(7), 5), BigInt::from(2));
    }

    #[test]
    fn test_sym_mod_negative_simple() {
        assert_eq!(sym_mod(BigInt::from(-7), 5), BigInt::from(-2));
    }

    #[test]
    fn test_sym_mod_negative_should_become_positive() {
        assert_eq!(sym_mod(BigInt::from(-3), 5), BigInt::from(2));
    }

    #[test]
    fn test_sym_mod_positive_becomes_negative() {
        assert_eq!(sym_mod(BigInt::from(3), 5), BigInt::from(-2));
    }

    #[test]
    fn test_sym_mod_zero() {
        assert_eq!(sym_mod(BigInt::from(0), 5), BigInt::from(0));
    }

    #[test]
    fn test_sym_mod_divisible() {
        assert_eq!(sym_mod(BigInt::from(10), 5), BigInt::from(0));
        assert_eq!(sym_mod(BigInt::from(-10), 5), BigInt::from(0));
    }

    #[test]
    fn test_sym_mod_boundary() {
        assert_eq!(sym_mod(BigInt::from(6), 12), BigInt::from(6));
        assert_eq!(sym_mod(BigInt::from(-6), 12), BigInt::from(6));
    }

    // =========================================================================
    // inv_g_zz
    // =========================================================================
    fn make_g(b: f64, l: usize) -> Vec<f64> {
        (0..l).map(|i| b.powi(i as i32)).collect()
    }

    fn reconstruct(digits: &[BigInt], b: i64) -> BigInt {
        let mut result = BigInt::zero();
        let mut power = BigInt::one();
        let b_big = BigInt::from(b);
        for d in digits {
            result += d * &power;
            power *= &b_big;
        }
        result
    }

    #[test]
    fn test_inv_g_zz_basic_reconstruction() {
        let g = make_g(4.0, 3);
        let digits = inv_g_zz(BigInt::from(13), g, 16.0, 3);
        let recon = reconstruct(&digits, 4);
        assert_eq!(
            recon,
            sym_mod(BigInt::from(13), 16),
            "Reconstruction must equal sym_mod(13, 16)"
        );
        for d in &digits {
            assert!(
                d >= &BigInt::from(-2) && d <= &BigInt::from(2),
                "Digit {d} out of range [-2, 2]"
            );
        }
    }

    #[test]
    fn test_inv_g_zz_negative_value() {
        let g = make_g(4.0, 3);
        let digits = inv_g_zz(BigInt::from(-3), g, 16.0, 3);
        let recon = reconstruct(&digits, 4);
        assert_eq!(recon, sym_mod(BigInt::from(-3), 16));
        for d in &digits {
            assert!(
                d >= &BigInt::from(-2) && d <= &BigInt::from(2),
                "Digit {d} out of range [-2, 2]"
            );
        }
    }

    #[test]
    fn test_inv_g_zz_zero() {
        let g = make_g(4.0, 3);
        let digits = inv_g_zz(BigInt::from(0), g, 16.0, 3);
        assert!(digits.iter().all(|d| d.is_zero()));
    }

    #[test]
    fn test_inv_g_zz_multiple_values() {
        let b = 4i64;
        let l = 3usize;
        let q = 16.0f64;
        let g = make_g(b as f64, l);

        for a in [-7i64, -3, 0, 5, 7, 13, 15, 100] {
            let digits = inv_g_zz(BigInt::from(a), g.clone(), q, l);
            let recon = reconstruct(&digits, b);
            let expected = sym_mod(BigInt::from(a), q as i64);
            assert_eq!(
                recon, expected,
                "inv_g_zz({a}): reconstruction {recon} != sym_mod({a},{q})={expected}"
            );
            for d in &digits {
                assert!(
                    d >= &BigInt::from(-(b / 2)) && d <= &BigInt::from(b / 2),
                    "inv_g_zz({a}): digit {d} out of bounds [-{}, {}]",
                    b / 2,
                    b / 2
                );
            }
        }
    }

    // =========================================================================
    // inv_g_poly
    // =========================================================================
    #[test]
    fn test_inv_g_poly_full_reconstruction() {
        let params = FhezParameters {
            n: 4,
            b: 4,
            l: 3,
            gamma: 20.0,
            rho: 4,
            t: 17,
            primes: &[],
        };
        let q = 16.0f64;
        let b = params.b as i64;
        let l = params.l;

        let input = BigPolynomial {
            coefficients: vec![
                BigInt::from(13),
                BigInt::from(5),
                BigInt::from(-3),
                BigInt::from(7),
            ],
        };

        let decomposed = inv_g_poly(&input, q, &params);

        // Must return l polynomials not n
        assert_eq!(
            decomposed.len(),
            l,
            "inv_g_poly must return {} polynomials (l), not {}",
            l,
            decomposed.len()
        );

        // Each polynomial must have n coefficients
        for poly in &decomposed {
            assert_eq!(poly.coefficients.len(), params.n);
        }

        // Reconstruction for each coefficient i, sum_j(decomposed[j].coef[i] * b^j) == sym_mod(input[i], q)
        for i in 0..params.n {
            let recon: BigInt = (0..l)
                .map(|j| &decomposed[j].coefficients[i] * BigInt::from(b).pow(j as u32))
                .sum();
            let expected = sym_mod(input.coefficients[i].clone(), q as i64);
            assert_eq!(
                recon, expected,
                "coefficient {i}: reconstruction {recon} != sym_mod(input[{i}], {q}) = {expected}"
            );
        }
    }

    #[test]
    fn test_inv_g_poly_processes_all_coefficients() {
        let params = FhezParameters {
            n: 4,
            b: 4,
            l: 3,
            gamma: 20.0,
            rho: 4,
            t: 17,
            primes: &[],
        };
        let input = BigPolynomial {
            coefficients: vec![
                BigInt::from(0),
                BigInt::from(0),
                BigInt::from(13),
                BigInt::from(7),
            ],
        };
        let decomposed = inv_g_poly(&input, 16.0, &params);

        let any_nonzero_in_last_half =
            (2..params.n).any(|i| (0..params.l).any(|j| !decomposed[j].coefficients[i].is_zero()));
        assert!(
            any_nonzero_in_last_half,
            "Coefficients 2 and 3 were ignored (n/2 bug)"
        );
    }

    // =========================================================================
    // signed_base_b_decomposition
    // =========================================================================
    #[test]
    fn test_signed_base_b_decomposition_reconstruction() {
        let b = 4u64;
        let l = 6usize;
        let b_big = BigInt::from(b);

        for val in [0i64, 1, 13, -13, 100, -100, 255, -255] {
            let digits = signed_base_b_decomposition(&BigInt::from(val), b, l);
            assert_eq!(digits.len(), l);

            let recon: BigInt = digits
                .iter()
                .enumerate()
                .map(|(i, d)| d * b_big.pow(i as u32))
                .sum();
            assert_eq!(
                recon,
                BigInt::from(val),
                "signed_base_b_decomposition({val}): reconstruction {recon} != {val}"
            );

            let half = (b / 2) as i64;
            for d in &digits {
                let d_i64 = d.to_i64().unwrap();
                assert!(
                    d_i64 >= -half && d_i64 <= half,
                    "digit {d_i64} out of bounds [-{half}, {half}] for val={val}"
                );
            }
        }
    }

    // =========================================================================
    // centered_rem_poly
    // =========================================================================
    #[test]
    fn test_centered_rem_poly() {
        let modulus = BigInt::from(10);
        let poly = BigPolynomial {
            coefficients: vec![
                BigInt::from(0),
                BigInt::from(1),
                BigInt::from(5),
                BigInt::from(6),  // 6 > 5 -> 6 - 10 = -4
                BigInt::from(-1), // -1 % 10 = 9 via euclid -> 9 - 10 = -1
                BigInt::from(-5), // -5 % 10 = 5 via euclid -> 5
                BigInt::from(-6), // -6 % 10 = 4 via euclid -> 4
                BigInt::from(10), // 10 % 10 = 0
            ],
        };
        let result = centered_rem_poly(&poly, &modulus);
        let expected = vec![0i64, 1, 5, -4, -1, 5, 4, 0];
        for (i, (got, exp)) in result.coefficients.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                got,
                &BigInt::from(*exp),
                "coefficient {i}: got {got}, expected {exp}"
            );
        }
    }

    // =========================================================================
    // round_bigint_division
    // =========================================================================
    #[test]
    fn test_round_bigint_division() {
        let cases = [
            (7i64, 2i64, 4i64), // 3.5 -> 4
            (6, 2, 3),          // 3.0 -> 3
            (5, 2, 3),          // 2.5 -> 3 half up
            (-7, 2, -4),        // -3.5 -> -4
            (10, 3, 3),         // 3.33 -> 3
            (11, 3, 4),         // 3.67 -> 4
            (0, 5, 0),
        ];
        for (num, den, expected) in cases {
            let result = round_bigint_division(&BigInt::from(num), &BigInt::from(den));
            assert_eq!(
                result,
                BigInt::from(expected),
                "round({num}/{den}): got {result}, expected {expected}"
            );
        }
    }

    // =========================================================================
    // sample_d
    // =========================================================================
    #[test]
    fn test_sample_d_limit() {
        let gamma = 6u64;
        let rho = 2u64;
        let p = BigInt::from(7);

        let two_gamma = BigInt::one() << gamma;
        let q_bound = &two_gamma / &p;
        let two_rho = BigInt::one() << rho;

        let x_min = &p * BigInt::zero() + (-&two_rho);
        let x_max = &p * &q_bound + &two_rho;

        for _ in 0..1000 {
            let x = sample_d(gamma, rho, &p);
            assert!(
                x >= x_min && x <= x_max,
                "sample {x} out of bounds [{x_min}, {x_max}]"
            );
        }
    }

    // =========================================================================
    // sample_p
    // =========================================================================
    #[test]
    fn test_sample_p() {
        let gamma = 6u64;
        let rho = 2u64;
        let p = BigInt::from(7);
        let n = 8usize;

        let poly = sample_p(gamma, rho, &p, n);
        assert_eq!(
            poly.coefficients.len(),
            n,
            "Polynomial must have {n} coefficients"
        );

        let two_gamma = BigInt::one() << gamma;
        let q_bound = &two_gamma / &p;
        let two_rho = BigInt::one() << rho;
        let x_min = -&two_rho;
        let x_max = &p * &q_bound + &two_rho;

        for c in &poly.coefficients {
            assert!(
                c >= &x_min && c <= &x_max,
                "coefficient {c} out of bounds [{x_min}, {x_max}]"
            );
        }
    }
}
