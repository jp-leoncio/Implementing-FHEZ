use crate::prelude::*;
const PI: f64 = std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigPolynomial {
    pub coeficients: Vec<BigInt>,
}

impl BigPolynomial {
    pub fn new(qt_coeficients: usize) -> BigPolynomial {
        BigPolynomial {
            coeficients: vec![BigInt::ZERO; qt_coeficients],
        }
    }

    pub fn degree(&self) -> usize {
        self.coeficients.len()
    }

    pub fn rand(qt_coeficients: usize, size: u32, degree: u32) -> BigPolynomial {
        let mut rng = rand::thread_rng();
        let vec = (0..qt_coeficients)
            .map(|_| RandBigInt::gen_bigint(&mut rng, size.into()))
            .collect();

        let poly = BigPolynomial { 
            coeficients: vec
        };
        
        poly.module(degree as usize)
    }

    pub fn module(&self, n: usize) -> BigPolynomial {
        let mut res = BigPolynomial::new(n);
        
        for (i, coef) in self.coeficients.iter().enumerate() {
            let degree = i % n;
            let wrap_count = i / n;

            if wrap_count % 2 == 1 {
                res.coeficients[degree] -= coef;
            } else {
                res.coeficients[degree] += coef;
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
        let mut coeficients = Vec::with_capacity(degree);
        let zero_val = BigInt::zero(); 

        for i in 0..degree {
            let c1 = self.coeficients.get(i).unwrap_or(&zero_val);
            let c2 = rhs.coeficients.get(i).unwrap_or(&zero_val);
            coeficients.push(c1 + c2);
        }

        BigPolynomial { 
            coeficients 
        }
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
        let mut coeficients = Vec::with_capacity(degree);
        let zero_val = BigInt::zero();

        for i in 0..degree {
            let c1 = self.coeficients.get(i).unwrap_or(&zero_val);
            let c2 = rhs.coeficients.get(i).unwrap_or(&zero_val);
            coeficients.push(c1 - c2);
        }

        BigPolynomial { 
            coeficients 
        }
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
                let term = &self.coeficients[i] * &rhs.coeficients[j];
                if i + j < n {
                    out.coeficients[i + j] += &term;
                } else {
                    out.coeficients[i + j - n] -= &term;
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
        let coeficients = rhs.coeficients.iter().map(|c| self * c).collect();
        
        BigPolynomial { 
            coeficients 
        }
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
        let coeficients = self.coeficients.iter().map(|c| c / rhs).collect();
        
        BigPolynomial { 
            coeficients 
        }
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
        let coeficients = self.coeficients.iter().map(|c| c.rem_euclid(rhs)).collect();
        BigPolynomial { 
            coeficients 
        }
    }
}