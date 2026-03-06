use crate::prelude::*;

pub fn to_fft<'a>(poly: &'a mut [Complex<f64>], plan: &'a mut Plan) -> &'a mut [Complex<f64>] {
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);
    plan.fwd(poly, stack.rb_mut());
    poly
}

pub fn from_fft<'a>(
    poly: &'a mut [Complex<f64>],
    plan: &'a mut Plan,
    params: &FhezParameters,
) -> &'a mut [Complex<f64>] {
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);
    plan.inv(poly, stack.rb_mut());
    for coeff in poly.iter_mut() {
        *coeff /= params.n as f64;
    }
    poly
}

// D_{γ,ρ}(p)
pub fn sample_d(gamma: u64, rho: u64, p: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();

    // Sample q from [0, ⌊2^γ/p⌋]
    let two_gamma = BigInt::one() << gamma;
    let q_bound = &two_gamma / p;
    let q = rng.gen_bigint_range(&BigInt::zero(), &(&q_bound + BigInt::one()));

    // Sample r from [−2^ρ, 2^ρ]
    let two_rho = BigInt::one() << rho;
    let r = rng.gen_bigint_range(&(-&two_rho), &(&two_rho + BigInt::one()));

    p * q + r
}

// P_{N,γ,ρ}(p)
pub fn sample_p(gamma: u64, rho: u64, p: &BigInt, n: usize) -> BigPolynomial {
    let coefficients: Vec<BigInt> = (0..n).map(|_| sample_d(gamma, rho, p)).collect();
    BigPolynomial { coefficients }
}

pub fn sample_r(
    gamma: u64,
    rho: u64,
    p: &BigInt,
    k: &BigPolynomial,
    x_0: &BigInt,
    n: usize,
) -> BigPolynomial {
    let c = sample_p(gamma, rho, p, n);
    (c * k) % x_0
}

/// Symmetric reduction of a modulo n
/// Returns the unique integer r such that r ≡ a (mod n) and r ∈ (-n/2, n/2]
pub fn sym_mod(a: BigInt, n: i64) -> BigInt {
    let n_big = BigInt::from(n);
    // rem_euclid guarantees result in [0, n) regardless of the sign of a
    let value = a.rem_euclid(&n_big);
    if 2 * &value > n_big {
        value - n_big
    } else {
        value
    }
}

/// Gadget decomposition of an integer a into l digits in base b with sign
/// Guarantees that Σ res[i] * g[i] == sym_mod(a, q) and that each digit
/// is in [-(b/2), b/2]
pub fn inv_g_zz(a: BigInt, g: Vec<f64>, q: f64, l: usize) -> Vec<BigInt> {
    let b = if l >= 2 {
        (g[1] / g[0]).round() as i64
    } else {
        1
    };
    let b_big = BigInt::from(b);

    let mut res = vec![BigInt::ZERO; l];
    let mut copy_val = sym_mod(a, q as i64);

    for num in res.iter_mut().take(l) {
        // rem_euclid guarantees remainder in [0, b) regardless of the sign of copy_val
        let rem = copy_val.rem_euclid(&b_big);
        // centers the digit in [-b/2, b/2]
        let digit = if 2 * &rem > b_big {
            &rem - &b_big
        } else {
            rem.clone()
        };
        *num = digit.clone();
        copy_val = (copy_val - &digit) / &b_big;
    }
    res
}

/// Gadget decomposition of each coefficient of polynomial a into l polynomials
/// The j th output polynomial contains the j th gadget digit of each coefficient
pub fn inv_g_poly(a: &BigPolynomial, q: f64, params: &FhezParameters) -> Vec<BigPolynomial> {
    let l = params.l;
    let mut g = vec![0.0f64; l];
    for (i, coeff) in g.iter_mut().enumerate().take(l) {
        *coeff = params.b_f64().powi(i as i32);
    }

    // l polynomials of degree n all initialized with zeros
    let mut res = vec![BigPolynomial::new(params.n); l];

    // Iterates ALL n coefficients not only n/2
    for i in 0..params.n {
        let digits = inv_g_zz(a.coefficients[i].clone(), g.clone(), q, l);
        // Transposes the j th digit of coefficient i goes to res[j].coefficients[i]
        for j in 0..l {
            res[j].coefficients[i] = digits[j].clone();
        }
    }
    res
}

/// Decomposes a BigInt val into l digits in base b with sign
/// Σ d[i] * b^i == val without modular reduction
pub fn signed_base_b_decomposition(val: &BigInt, b: u64, l: usize) -> Vec<BigInt> {
    let mut digits = Vec::with_capacity(l);
    let mut current_val = val.clone();
    let b_big = BigInt::from(b);

    for _ in 0..l {
        let remainder = current_val.rem_euclid(&b_big);
        let centered = if 2 * &remainder > b_big {
            &remainder - &b_big
        } else {
            remainder
        };
        digits.push(centered.clone());
        current_val = (current_val - &centered) / &b_big;
    }
    digits
}

pub fn sample_poly_uniform_bound(bound: &BigInt, degree: usize) -> BigPolynomial {
    let mut rng = rand::thread_rng();
    let coefficients = (0..degree)
        .map(|_| rng.gen_bigint_range(&BigInt::zero(), bound))
        .collect();
    BigPolynomial { coefficients }
}

pub fn sample_poly_signed_bound(bound: &BigInt, degree: usize) -> BigPolynomial {
    let mut rng = rand::thread_rng();
    let neg_bound = -bound;
    let coefficients = (0..degree)
        .map(|_| rng.gen_bigint_range(&neg_bound, bound))
        .collect();
    BigPolynomial { coefficients }
}

pub fn centered_rem_poly(poly: &BigPolynomial, modulus: &BigInt) -> BigPolynomial {
    let m_half = modulus >> 1;
    let coefficients = poly
        .coefficients
        .iter()
        .map(|c| {
            let r = c.rem_euclid(modulus);
            if r > m_half {
                r - modulus
            } else {
                r
            }
        })
        .collect();
    BigPolynomial { coefficients }
}

pub fn round_bigint_division(num: &BigInt, den: &BigInt) -> BigInt {
    let half_den = den >> 1;
    if num.sign() == Sign::Plus {
        (num + &half_den) / den
    } else {
        (num - &half_den) / den
    }
}

pub fn round_poly_division(poly: &BigPolynomial, den: &BigInt) -> BigPolynomial {
    let coefficients = poly
        .coefficients
        .iter()
        .map(|c| round_bigint_division(c, den))
        .collect();
    BigPolynomial { coefficients }
}

pub fn print_poly(poly: &BigPolynomial) {
    if poly.coefficients.iter().all(|x| *x == BigInt::ZERO) {
        println!("0");
        return;
    }
    let mut is_first = true;
    for i in (0..poly.degree()).rev() {
        let coeff = &poly.coefficients[i];
        if *coeff == BigInt::ZERO {
            continue;
        }
        let sign = coeff.sign();
        let abs_coeff = coeff.magnitude();
        if !is_first {
            print!("{}", if sign == Sign::Minus { " - " } else { " + " });
        } else if sign == Sign::Minus {
            print!("-");
        }
        if *abs_coeff != 1.to_biguint().expect("error converting 1") || i == 0 {
            print!("{}", abs_coeff);
        }
        if i > 0 {
            print!("x");
            if i > 1 {
                print!("^{}", i);
            }
        }
        is_first = false;
    }
    println!();
}
