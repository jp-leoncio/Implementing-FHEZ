use crate::prelude::*;
const PI: f64 = std::f64::consts::PI;

/// Primos de 15 bits
static PRIME_15: &[u32] = &[
    32771, 32779, 32783, 32789, 32797, 32801, 32803, 32831, 32833, 32839, 32843,
    32869, 32887, 32909, 32911, 32917, 32933, 32939, 32941, 32957,
];

/// Primos de 20 bits
static PRIME_20: &[u32] = &[
    1048583, 1048589, 1048601, 1048609, 1048613, 1048627, 1048633, 1048661,
    1048681, 1048703, 1048709, 1048717, 1048721, 1048759, 1048783,
];

/// Representa um polinômio na forma Double-CRT
/// Cada vetor interno `Vec<Complex<f64>>` representa o polinômio na FFT módulo um dos primos do contexto
#[derive(Clone, Debug, PartialEq)]
pub struct Dcrt {
    /// `poly[i]` é a transformada do polinômio módulo `primes[i]`
    pub poly: Vec<Vec<Complex<f64>>>,
    /// O grau do polinômio
    pub n: usize,
}

/// Armazena os parâmetros pré-calculados para as operações Dcrt
#[derive(Debug)]
pub struct DcrtContext {
    /// Os primos utilizados
    pub primes: &'static [u32],
    /// O produto (M) de todos os primos
    pub m: BigInt,
    /// Vetor com os valores m_i = M / p_i
    pub m_i: Vec<BigInt>,
    /// Vetor com o mod inverso = (m_i)^-1 mod p_i
    pub m_i_inv_mod_pi: Vec<BigInt>,
}

/// Decompõe um BigPolynomial (criptograma escalar) em um vetor de `l` polinômios
/// Esta função implementa a operação `g⁻¹` descrita no artigo
pub fn gadget_decompose<const N: usize>(scalar_poly: &BigPolynomial, l: usize, b: u64) -> Vec<BigPolynomial> {
    let mut decomposed_polys = vec![BigPolynomial::new(N); l];

    // Decompõe cada coeficiente do polinômio de entrada
    let mut decomposed_coeffs = Vec::with_capacity(N);
    for coef in &scalar_poly.coeficients {
        decomposed_coeffs.push(signed_base_b_decomposition(coef, b, l));
    }

    // "Transpõe" os resultados para formar os polinômios de saída
    // O polinômio `i` na saída é formado pelo `i`-ésimo dígito de cada coeficiente original
    for i in 0..l {
        for j in 0..N {
            if i < decomposed_coeffs[j].len() {
                decomposed_polys[i].coeficients[j] = decomposed_coeffs[j][i].clone();
            }
        }
    }

    decomposed_polys
}

/// Reconstitui um coeficiente a partir de suas congruências usando o Teorema Chinês do Resto
///
/// # Argumentos
/// *  `congruences` - Vetor de coeficientes `a_i`, um para cada primo no contexto
/// *  `context` - O contexto Dcrt com os valores pré-calculados
pub fn crt(congruences: &[BigInt], context: &DcrtContext) -> BigInt {
    let mut solution = BigInt::zero();
    
    // A fórmula é: solution = Sum(a_i * m_i * (m_i^-1 mod p_i)) mod M
    for i in 0..congruences.len() {
        let a_i = &congruences[i];
        let mi = &context.m_i[i];
        let inv = &context.m_i_inv_mod_pi[i];
        solution += a_i * mi * inv;
    }
    
    solution % &context.m
}


/// Converte um `BigPolynomial` para a forma `Dcrt`
pub fn to_dcrt<const N: usize>(a: &BigPolynomial, context: &DcrtContext, plan: &mut Plan) -> Dcrt {
    let mut res = Dcrt::new(N, context.primes.len());

    for (i, p_u32) in context.primes.iter().enumerate() {
        let p_big = p_u32.to_bigint().unwrap();

        for (j, coef) in a.coeficients.iter().enumerate() {
            // Usa `rem_euclid` para garantir que o resto seja sempre positivo
            let reduced_coef = coef.rem_euclid(&p_big);
            // let reduced_coef = coef % &p_big;
            
            let coef_f64 = reduced_coef.to_f64().unwrap_or(0.0);

            let theta = PI * j as f64 / N as f64;
            let twiddle = c64::new(theta.cos(), theta.sin());
            
            res.poly[i][j] = c64::new(coef_f64, 0.0) * twiddle;
        }
        // Aplica a FFT
        to_fft::<N>(&mut res.poly[i], plan);
    }
    res
}


/// Converte um `Dcrt` de volta para um `BigPolynomial
pub fn from_dcrt<const N: usize>(a: &mut Dcrt, context: &DcrtContext, plan: &mut Plan) -> BigPolynomial {
    // Aplica a FFT inversa para cada "camada" de primo
    for poly_mod_p in a.poly.iter_mut() {
        from_fft::<N>(poly_mod_p, plan);

        for j in 0..N {
            let theta = PI * j as f64 / N as f64;
            let inv_twiddle = c64::new(theta.cos(), -theta.sin()); // Conjugado
            poly_mod_p[j] *= inv_twiddle;
        }
    }

    // Transpõe os dados para agrupar os coeficientes
    let mut transposed_coeffs = vec![vec![c64::new(0.0, 0.0); context.primes.len()]; N];    
    for i in 0..context.primes.len() {
        for j in 0..N {
            transposed_coeffs[j][i] = a.poly[i][j];
        }
    }

    let mut res = BigPolynomial::new(N);
    for i in 0..N {
        // Arredondamento de Complex<f64> para BigInt
        let rounded_congruences: Vec<BigInt> = transposed_coeffs[i]
            .iter()
            .map(|c| BigInt::from_f64(c.re.round()).unwrap_or_else(BigInt::zero))
            .collect();

        // Reconstitui o coeficiente original via CRT
        let crt_result = crt(&rounded_congruences, context);
        
        let m_half = &context.m >> 1;
        // Soma com deslocamento
        let shifted_val:BigInt = crt_result + &m_half;
        // Calcula o resto euclidiano
        let remainder = shifted_val.rem_euclid(&context.m);
        // Subtrai o deslocamento para centrar o resultado
        let centered_coef = remainder - &m_half;

        res.coeficients[i] = centered_coef;
    }
    res
}

/// Calcula o produto interno (dot product) de dois vetores de polinômios DCRT
///
/// A operação computada é: `res = Σ(a[i] * b[i])`
pub fn inner_product(a: &[Dcrt], b: &[Dcrt]) -> Dcrt {
    // Garante que os vetores de entrada tenham o mesmo comprimento
    assert_eq!(a.len(), b.len(),
        "Os vetores de entrada para o produto interno devem ter o mesmo comprimento"
    );

    // Garante que os vetores não estejam vazios
    if a.is_empty() {
        panic!("Não é possível calcular o produto interno de vetores vazios");
    }

    let mut res = &a[0] * &b[0];

    // Itera sobre o restante dos elementos e acumula a soma
    for (poly_a, poly_b) in a.iter().zip(b.iter()).skip(1) {
        res += &*poly_a * &*poly_b;
    }

    res
}

/// Calcula o produto externo (produto misto homomórfico) entre um vetor de criptogramas
/// e um criptograma escalar, ambos no formato DCRT
pub fn external_product<const N: usize>(vector_ciphertext: &[Dcrt], scalar_ciphertext: &mut Dcrt, context: &DcrtContext, plan: &mut Plan, l: usize, b: u64) -> Dcrt {
    // 1. Converter o criptograma escalar do domínio DCRT/FFT de volta para o domínio
    //    de inteiros para realizar a decomposição de forma exata
    let scalar_poly = from_dcrt::<N>(scalar_ciphertext, context, plan);

    // 2. Decompor o BigPolynomial escalar em um vetor de `l` BigPolynomials
    let decomposed_scalar_polys = gadget_decompose::<N>(&scalar_poly, l, b);

    // 3. Converter cada um dos polinômios decompostos de volta para o formato DCRT
    let decomposed_scalar_dcrt: Vec<Dcrt> = decomposed_scalar_polys
        .iter()
        .map(|p| to_dcrt::<N>(p, context, plan))
        .collect();

    // 4. Calcular o produto interno entre o criptograma de vetor original e
    //    o vetor decomposto do criptograma escalar
    inner_product(vector_ciphertext, &decomposed_scalar_dcrt)
}

impl Dcrt {
    /// Cria um novo polinômio Dcrt nulo
    pub fn new(n: usize, prime_count: usize) -> Self {
        Self {
            poly: vec![vec![c64::new(0.0, 0.0); n]; prime_count],
            n,
        }
    }
}

impl<'a, 'b> Add<&'b Dcrt> for &'a Dcrt {
    type Output = Dcrt;

    /// Soma dois polinômios Dcrt, retornando um novo
    fn add(self, rhs: &'b Dcrt) -> Self::Output {
        assert_eq!(self.poly.len(), rhs.poly.len(), "Incompatibilidade no número de primos.");
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
    /// Soma outro polinômio Dcrt a este (in-place)
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.poly.len(), rhs.poly.len(), "Incompatibilidade no número de primos.");
        for i in 0..self.poly.len() {
            for j in 0..self.n {
                self.poly[i][j] += rhs.poly[i][j];
            }
        }
    }
}

impl<'a, 'b> Mul<&'b Dcrt> for &'a Dcrt {
    type Output = Dcrt;

    /// Multiplica (componente a componente) dois polinômios Dcrt, retornando um novo
    fn mul(self, rhs: &'b Dcrt) -> Self::Output {
        assert_eq!(self.poly.len(), rhs.poly.len(), "Incompatibilidade no número de primos.");
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
    /// Multiplica outro polinômio Dcrt a este (in-place)
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.poly.len(), rhs.poly.len(), "Incompatibilidade no número de primos.");
        for i in 0..self.poly.len() {
            for j in 0..self.n {
                self.poly[i][j] *= rhs.poly[i][j];
            }
        }
    }
}

impl DcrtContext {
    /// Cria um novo contexto Dcrt a partir de uma lista de primos e parâmetros de segurança
    pub fn new(primes: &'static [u32], gamma: f64, l: f64, b: f64, n_poly: f64) -> Self {
        // Cálculo do `size` necessário
        let size = gamma + f64::ceil(f64::log2(l)) + f64::log2(b) + f64::log2(n_poly);
        let size_bits = size.ceil() as u64;

        // Seleciona a quantidade mínima de primos da lista fornecida
        let mut m = BigInt::one();
        let mut primes_to_use = Vec::new();
        for i in 0..primes.len() {
            if m.bits() >= size_bits {
                break;
            }
            m *= primes[i];
            primes_to_use.push(i);
        }

        if m.bits() < size_bits {
            panic!(
                "A lista de primos fornecida é muito pequena para o `size` calculado de {}.
                Considere adicionar mais primos.", size_bits
            );
        }

        let selected_primes: &'static [u32] = &primes[..primes_to_use.len()];

        // Cálculo de m_i e suas inversas
        let mut m_i = Vec::with_capacity(selected_primes.len());
        let mut m_i_inv_mod_pi = Vec::with_capacity(selected_primes.len());

        for (_, &prime) in selected_primes.iter().enumerate() {
            let p_i = prime.to_bigint().expect("Falha ao converter primo para BigInt");
            let mi = &m / &p_i;
            let inv = mi.modinv(&p_i)
                .expect("Inversa modular deve existir para primos distintos.");

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