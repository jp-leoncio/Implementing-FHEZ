use crate::prelude::*;

pub fn to_fft<'a, const N: usize>(poly: &'a mut [Complex<f64>], plan: &'a mut Plan) -> &'a mut [Complex<f64>] {
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);

    plan.fwd(poly, stack.rb_mut());
    poly
}

pub fn from_fft<'a, const N: usize>(poly: &'a mut [Complex<f64>], plan: &'a mut Plan) -> &'a mut [Complex<f64>] {
    let mut scratch_memory = GlobalPodBuffer::new(plan.fft_scratch().unwrap());
    let mut stack = PodStack::new(&mut scratch_memory);

    plan.inv(poly, stack.rb_mut());
    for coef in poly.iter_mut() {
        *coef /= N as f64;
    }
    poly
}

pub fn sym_mod(a: BigInt, n: i64) -> BigInt {
    let valor = a % n;
    if 2*valor.clone() > BigInt::from(n) {
        return valor - n;
    }
    valor
}

pub fn inv_g_zz(a: BigInt, g: Vec<f64>, q: f64, l: usize) -> Vec<BigInt> {
    let mut res = vec![BigInt::ZERO; l];
    let mut copy = sym_mod(a, q as i64);
    for i in 0..l { 
        let valor = copy.clone() / (g[l-i-1] as i64);
        let rem = copy % (g[l-i-1] as i64); 
        res[l-i-1] = valor;
        copy = rem.clone();
        if rem == BigInt::ZERO {
            break;
        }
    }
    res
}

pub fn inv_g_poly(a: &mut BigPolynomial, q: f64) -> Vec<BigPolynomial> {
    let l = q.log(B).ceil() as usize;
    let mut g = vec![0.0; l];
    for i in 0..l {
        g[i] = B.powi(i as i32);
    }
    let mut res = vec![BigPolynomial::new(N); N];
    for i in 0..N/2 as usize {
        let reduc = inv_g_zz(a.coeficients[i].clone(), g.clone(), q, l);
        res[i].coeficients = reduc;
    }
    res
}

/*
pub fn red_base_poly_hilder<const l: usize>(a: &mut BigPolynomial) -> [BigPolynomial; l] {
    let mut g = [0.0; l];
    for i in 0..l {
        g[i] = B.powi(i as i32);
    }
    let mut res: [BigPolynomial; N] = core::array::from_fn(|_| BigPolynomial::new(&[0; l].to_vec(), l as u32));
    for i in 0..N/2 as usize {
        let reduc = red_base_zz(a.coeficients[i], g);
        res[i] = BigPolynomial::new(&reduc.to_vec(), l as u32);
    }

    let mut trans: [BigPolynomial; l] = core::array::from_fn(|_| BigPolynomial::new(&[0; N].to_vec(), N as u32));
    for i in 0..N {
        for j in 0..l {
            trans[j].coeficients[i] = res[i].coeficients[j];
        }
    }

    trans
}
*/

/**
 * TODO: Gerar {v0, .., vl}: estão na DCRT, tendo l itens na base b
 */
// pub fn generation(b: f64, gamma: f64, n: usize, l: usize) ->     {
//     let u = BigPolynomial::rand(n, gamma as u32, n as u32);
    
// }

pub fn sample_poly_uniform_bound(bound: &BigInt, degree: usize) -> BigPolynomial {
    let mut rng = rand::thread_rng();
    let coeficients = (0..degree)
        .map(|_| rng.gen_bigint_range(&BigInt::zero(), bound))
        .collect();
    
    BigPolynomial { 
        coeficients 
    }
}

pub fn sample_poly_signed_bound(bound: &BigInt, degree: usize) -> BigPolynomial {
    let mut rng = rand::thread_rng();
    let neg_bound = -bound;
    let coeficients = (0..degree)
        .map(|_| rng.gen_bigint_range(&neg_bound, bound))
        .collect();

    BigPolynomial { 
        coeficients 
    }
}

pub fn centered_rem_poly(poly: &BigPolynomial, modulus: &BigInt) -> BigPolynomial {
    let m_half = modulus >> 1;
    let coeficients = poly
        .coeficients
        .iter()
        .map(|c| {
            let shifted: BigInt = c + &m_half;
            shifted.rem_euclid(modulus) - &m_half
        })
        .collect();

    BigPolynomial { 
        coeficients 
    }
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
    let coeficients = poly.coeficients.iter()
        .map(|c| round_bigint_division(c, den))
        .collect();

    BigPolynomial { 
        coeficients 
    }
}

pub fn print_poly(poly: &BigPolynomial) {
    // Verifica se é o polinômio nulo
    if poly.coeficients.iter().all(|x| *x == BigInt::ZERO) {
        println!("0");
        return;
    }

    let mut is_first = true;

    // Itera do maior grau para o menor
    for i in (0..poly.degree()).rev() {
        let coef = &poly.coeficients[i as usize];

        if *coef == BigInt::ZERO {
            continue;
        }

        let sign = coef.sign();
        let abs_coef = coef.magnitude();

        // Imprime o sinal para todos, exceto para o primeiro termo positivo
        if !is_first {
            print!("{}", if sign == Sign::Minus { " - " } else { " + " });
        } else if sign == Sign::Minus {
            print!("-");
        }
        
        // Omite o '1' se não for o termo constante
        if *abs_coef != 1.to_biguint().expect("error converting 1") || i == 0 {
            print!("{}", abs_coef);
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