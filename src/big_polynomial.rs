use crate::prelude::*;
const PI: f64 = std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigPolynomial {
    pub coefficients: Vec<BigInt>,
}

impl BigPolynomial {
    pub fn new(qt_coefficients: usize) -> BigPolynomial {
        BigPolynomial {
            coefficients: vec![BigInt::ZERO; qt_coefficients],
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len()
    }

    pub fn rand(qt_coefficients: usize, size: u32, degree: u32) -> BigPolynomial {
        let mut rng = rand::thread_rng();
        let vec = (0..qt_coefficients)
            .map(|_| RandBigInt::gen_bigint(&mut rng, size.into()))
            .collect();

        let poly = BigPolynomial { coefficients: vec };

        poly.module(degree as usize)
    }

    pub fn module(&self, n: usize) -> BigPolynomial {
        let mut res = BigPolynomial::new(n);

        for (i, coef) in self.coefficients.iter().enumerate() {
            let degree = i % n;
            let wrap_count = i / n;

            if wrap_count % 2 == 1 {
                res.coefficients[degree] -= coef;
            } else {
                res.coefficients[degree] += coef;
            }
        }
        res
    }
}

// Adição
impl Add for BigPolynomial {
    type Output = BigPolynomial;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'a> Add<&'a BigPolynomial> for BigPolynomial {
    type Output = BigPolynomial;
    fn add(self, rhs: &'a BigPolynomial) -> Self::Output {
        &self + rhs
    }
}

impl<'a> Add<BigPolynomial> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn add(self, rhs: BigPolynomial) -> Self::Output {
        self + &rhs
    }
}

impl<'a, 'b> Add<&'b BigPolynomial> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn add(self, rhs: &'b BigPolynomial) -> Self::Output {
        let degree = self.degree().max(rhs.degree());
        let mut coefficients = Vec::with_capacity(degree);
        let zero_val = BigInt::zero();

        for i in 0..degree {
            let c1 = self.coefficients.get(i).unwrap_or(&zero_val);
            let c2 = rhs.coefficients.get(i).unwrap_or(&zero_val);
            coefficients.push(c1 + c2);
        }

        BigPolynomial { coefficients }
    }
}

// Subtração
impl Sub for BigPolynomial {
    type Output = BigPolynomial;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<'a> Sub<&'a BigPolynomial> for BigPolynomial {
    type Output = BigPolynomial;
    fn sub(self, rhs: &'a BigPolynomial) -> Self::Output {
        &self - rhs
    }
}

impl<'a> Sub<BigPolynomial> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn sub(self, rhs: BigPolynomial) -> Self::Output {
        self - &rhs
    }
}

impl<'a, 'b> Sub<&'b BigPolynomial> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn sub(self, rhs: &'b BigPolynomial) -> Self::Output {
        let degree = self.degree().max(rhs.degree());
        let mut coefficients = Vec::with_capacity(degree);
        let zero_val = BigInt::zero();

        for i in 0..degree {
            let c1 = self.coefficients.get(i).unwrap_or(&zero_val);
            let c2 = rhs.coefficients.get(i).unwrap_or(&zero_val);
            coefficients.push(c1 - c2);
        }

        BigPolynomial { coefficients }
    }
}

// Multiplicação (Polinômio * Polinômio)
impl Mul for BigPolynomial {
    type Output = BigPolynomial;
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<'a> Mul<&'a BigPolynomial> for BigPolynomial {
    type Output = BigPolynomial;
    fn mul(self, rhs: &'a BigPolynomial) -> Self::Output {
        &self * rhs
    }
}

impl<'a> Mul<BigPolynomial> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn mul(self, rhs: BigPolynomial) -> Self::Output {
        self * &rhs
    }
}

impl<'a, 'b> Mul<&'b BigPolynomial> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn mul(self, rhs: &'b BigPolynomial) -> Self::Output {
        let n = self.degree().max(rhs.degree());
        let mut out = BigPolynomial::new(n);
        for i in 0..self.degree() {
            for j in 0..rhs.degree() {
                let term = &self.coefficients[i] * &rhs.coefficients[j];
                if i + j < n {
                    out.coefficients[i + j] += &term;
                } else {
                    out.coefficients[i + j - n] -= &term;
                }
            }
        }
        out
    }
}

// Multiplicação (BigInt * Polinômio)
impl<'a> Mul<&'a BigPolynomial> for &'a BigInt {
    type Output = BigPolynomial;
    fn mul(self, rhs: &'a BigPolynomial) -> Self::Output {
        let coefficients = rhs.coefficients.iter().map(|c| self * c).collect();

        BigPolynomial { coefficients }
    }
}

// Divisão (Polinômio / BigInt)
impl Div<&BigInt> for BigPolynomial {
    type Output = BigPolynomial;
    fn div(self, rhs: &BigInt) -> Self::Output {
        &self / rhs
    }
}

impl<'a> Div<&'a BigInt> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn div(self, rhs: &'a BigInt) -> Self::Output {
        let coefficients = self.coefficients.iter().map(|c| c / rhs).collect();

        BigPolynomial { coefficients }
    }
}

// Resto (Polinômio % BigInt)
impl Rem<&BigInt> for BigPolynomial {
    type Output = BigPolynomial;
    fn rem(self, rhs: &BigInt) -> Self::Output {
        &self % rhs
    }
}

impl<'a> Rem<&'a BigInt> for &'a BigPolynomial {
    type Output = BigPolynomial;
    fn rem(self, rhs: &'a BigInt) -> Self::Output {
        let coefficients = self
            .coefficients
            .iter()
            .map(|c| c.rem_euclid(rhs))
            .collect();
        BigPolynomial { coefficients }
    }
}
