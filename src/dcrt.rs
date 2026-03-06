use crate::prelude::*;
const PI: f64 = std::f64::consts::PI;

/// Represents a polynomial in Double CRT form
/// Each inner vector Vec Complex f64 represents the polynomial in FFT modulo one of the context primes
#[derive(Clone, Debug, PartialEq)]
pub struct Dcrt {
    pub poly: Vec<Vec<Complex<f64>>>, // poly i is the transform of the polynomial modulo primes i
    pub n: usize,                     // The degree of the polynomial
}

/// Stores pre computed parameters for Dcrt operations
#[derive(Debug)]
pub struct DcrtContext {
    pub primes: &'static [u32],      // The primes used
    pub m: BigInt,                   // The product M of all primes
    pub m_i: Vec<BigInt>,            // Vector with values m i equals M divided by p i
    pub m_i_inv_mod_pi: Vec<BigInt>, // Vector with the inverse mod equals m i inverse mod p i
}

/// Decomposes a BigPolynomial scalar ciphertext into a vector of l polynomials
/// This function implements the g inverse operation described in the article
pub fn gadget_decompose(
    scalar_poly: &BigPolynomial,
    params: &FhezParameters,
) -> Vec<BigPolynomial> {
    let mut decomposed_polys = vec![BigPolynomial::new(params.n); params.l];

    // Decomposes each coefficient of the input polynomial
    let mut decomposed_coeffs = Vec::with_capacity(params.n);
    for coef in &scalar_poly.coefficients {
        decomposed_coeffs.push(signed_base_b_decomposition(coef, params.b, params.l));
    }

    // Transposes the results to form the output polynomials
    // The i th output polynomial is formed by the i th digit of each original coefficient
    for i in 0..params.l {
        for j in 0..params.n {
            if i < decomposed_coeffs[j].len() {
                decomposed_polys[i].coefficients[j] = decomposed_coeffs[j][i].clone();
            }
        }
    }

    decomposed_polys
}

/// Reconstructs a coefficient from its congruences using the Chinese Remainder Theorem
pub fn crt(congruences: &[BigInt], context: &DcrtContext) -> BigInt {
    let mut solution = BigInt::zero();

    // The formula is solution equals Sum a i times m i times m i inverse mod p i mod M
    for i in 0..congruences.len() {
        let a_i = &congruences[i];
        let mi = &context.m_i[i];
        let inv = &context.m_i_inv_mod_pi[i];
        solution += a_i * mi * inv;
    }

    solution % &context.m
}

/// Converts a BigPolynomial to the Dcrt form
pub fn to_dcrt(
    a: &BigPolynomial,
    context: &DcrtContext,
    plan: &mut Plan,
    params: &FhezParameters,
) -> Dcrt {
    let mut res = Dcrt::new(params.n, context.primes.len());

    for (i, p_u32) in context.primes.iter().enumerate() {
        let p_big = p_u32.to_bigint().unwrap();

        for (j, coef) in a.coefficients.iter().enumerate() {
            // Uses rem euclid to ensure the remainder is always positive
            let reduced_coef = coef.rem_euclid(&p_big);

            let coef_f64 = reduced_coef.to_f64().unwrap_or(0.0);

            let theta = PI * j as f64 / params.n as f64;
            let twiddle = c64::new(theta.cos(), theta.sin());

            res.poly[i][j] = c64::new(coef_f64, 0.0) * twiddle;
        }
        // Applies FFT
        to_fft(&mut res.poly[i], plan);
    }
    res
}

/// Converts a Dcrt back to a BigPolynomial
///
/// After IFFT and undoing the twiddle a poly i j contains the j th coefficient
/// of the polynomial reduced modulo the i th prime For the CRT of the j th coefficient
/// simply collect a poly 0 j a poly 1 j a poly k minus 1 j directly
/// without needing to transpose the entire matrix
pub fn from_dcrt(
    a: &mut Dcrt,
    context: &DcrtContext,
    plan: &mut Plan,
    params: &FhezParameters,
) -> BigPolynomial {
    // Applies inverse FFT and undoes the twiddle for each prime layer
    for poly_mod_p in a.poly.iter_mut() {
        from_fft(poly_mod_p, plan, params);

        for j in 0..params.n {
            let theta = PI * j as f64 / params.n_f64();
            let inv_twiddle = c64::new(theta.cos(), -theta.sin()); // Conjugate
            poly_mod_p[j] *= inv_twiddle;
        }
    }

    let m_half = &context.m >> 1;
    let num_primes = context.primes.len();

    let mut res = BigPolynomial::new(params.n);
    for j in 0..params.n {
        // Congruence of the j th coefficient in each prime round a poly i j re
        let congruences: Vec<BigInt> = (0..num_primes)
            .map(|i| BigInt::from_f64(a.poly[i][j].re.round()).unwrap_or_else(BigInt::zero))
            .collect();

        // Reconstructs the original coefficient via CRT and centers it in negative M 2 M 2
        let crt_result = crt(&congruences, context);
        let shifted: BigInt = crt_result + &m_half;
        let remainder = shifted.rem_euclid(&context.m);
        res.coefficients[j] = remainder - &m_half;
    }
    res
}

/// Computes the dot product inner product of two vectors of DCRT polynomials
///
/// The computed operation is res equals sum a i times b i
pub fn inner_product(a: &[Dcrt], b: &[Dcrt]) -> Dcrt {
    assert_eq!(
        a.len(),
        b.len(),
        "Input vectors for the inner product must have the same length"
    );

    if a.is_empty() {
        panic!("Cannot compute inner product of empty vectors");
    }

    let mut res = &a[0] * &b[0];

    for (poly_a, poly_b) in a.iter().zip(b.iter()).skip(1) {
        res += &*poly_a * &*poly_b;
    }

    res
}

/// Computes the external product homomorphic mixed product between a ciphertext vector
/// and a scalar ciphertext both in DCRT format
pub fn external_product(
    vector_ciphertext: &[Dcrt],
    scalar_ciphertext: &mut Dcrt,
    context: &DcrtContext,
    plan: &mut Plan,
    params: &FhezParameters,
) -> Dcrt {
    // Converts the scalar ciphertext from the DCRT FFT domain back to the integer domain
    // to perform the exact decomposition
    let scalar_poly = from_dcrt(scalar_ciphertext, context, plan, params);

    // Decomposes the scalar BigPolynomial into a vector of l BigPolynomials
    let decomposed_scalar_polys = gadget_decompose(&scalar_poly, params);

    // Converts each of the decomposed polynomials back to DCRT format
    let decomposed_scalar_dcrt: Vec<Dcrt> = decomposed_scalar_polys
        .iter()
        .map(|p| to_dcrt(p, context, plan, params))
        .collect();

    // Computes the inner product between the original vector ciphertext and
    // the decomposed vector of the scalar ciphertext
    inner_product(vector_ciphertext, &decomposed_scalar_dcrt)
}

impl Dcrt {
    /// Creates a new null Dcrt polynomial
    pub fn new(n: usize, prime_count: usize) -> Self {
        Self {
            poly: vec![vec![c64::new(0.0, 0.0); n]; prime_count],
            n,
        }
    }
}

impl<'a, 'b> Add<&'b Dcrt> for &'a Dcrt {
    type Output = Dcrt;

    /// Adds two Dcrt polynomials returning a new one
    fn add(self, rhs: &'b Dcrt) -> Self::Output {
        assert_eq!(
            self.poly.len(),
            rhs.poly.len(),
            "Mismatch in the number of primes"
        );
        let mut res = Dcrt::new(self.n, self.poly.len());
        for i in 0..self.poly.len() {
            for j in 0..self.n {
                res.poly[i][j] = self.poly[i][j] + rhs.poly[i][j];
            }
        }
        res
    }
}

impl AddAssign for Dcrt {
    /// Adds another Dcrt polynomial to this one in place
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.poly.len(),
            rhs.poly.len(),
            "Mismatch in the number of primes"
        );
        for i in 0..self.poly.len() {
            for j in 0..self.n {
                self.poly[i][j] += rhs.poly[i][j];
            }
        }
    }
}

impl<'a, 'b> Mul<&'b Dcrt> for &'a Dcrt {
    type Output = Dcrt;

    /// Multiplies component by component two Dcrt polynomials returning a new one
    fn mul(self, rhs: &'b Dcrt) -> Self::Output {
        assert_eq!(
            self.poly.len(),
            rhs.poly.len(),
            "Mismatch in the number of primes"
        );
        let mut res = Dcrt::new(self.n, self.poly.len());
        for i in 0..self.poly.len() {
            for j in 0..self.n {
                res.poly[i][j] = self.poly[i][j] * rhs.poly[i][j];
            }
        }
        res
    }
}

impl MulAssign for Dcrt {
    /// Multiplies another Dcrt polynomial to this one in place
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(
            self.poly.len(),
            rhs.poly.len(),
            "Mismatch in the number of primes"
        );
        for i in 0..self.poly.len() {
            for j in 0..self.n {
                self.poly[i][j] *= rhs.poly[i][j];
            }
        }
    }
}

impl DcrtContext {
    /// Creates a new Dcrt context from a list of primes and security parameters
    pub fn new(params: &FhezParameters) -> Self {
        // Calculation of required size
        let size = params.gamma
            + f64::ceil(f64::log2(params.l_f64()))
            + f64::log2(params.b_f64())
            + f64::log2(params.n_f64());
        let size_bits = size.ceil() as u64;

        // Selects the minimum amount of primes from the provided list
        let mut m = BigInt::one();
        let mut primes_to_use = Vec::new();
        for i in 0..params.primes.len() {
            if m.bits() >= size_bits {
                break;
            }
            m *= params.primes[i];
            primes_to_use.push(i);
        }

        if m.bits() < size_bits {
            panic!(
                "The provided prime list is too small for the calculated size of {} \
                Consider adding more primes",
                size_bits
            );
        }

        let selected_primes: &'static [u32] = &params.primes[..primes_to_use.len()];

        // Calculation of m i and its inverses
        let mut m_i = Vec::with_capacity(selected_primes.len());
        let mut m_i_inv_mod_pi = Vec::with_capacity(selected_primes.len());

        for &prime in selected_primes.iter() {
            let p_i = prime
                .to_bigint()
                .expect("Failed to convert prime to BigInt");
            let mi = &m / &p_i;
            let inv = mi
                .modinv(&p_i)
                .expect("Modular inverse must exist for distinct primes");

            m_i.push(mi);
            m_i_inv_mod_pi.push(inv);
        }

        Self {
            primes: selected_primes,
            m,
            m_i,
            m_i_inv_mod_pi,
        }
    }
}
