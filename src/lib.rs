#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

pub mod big_polynomial;
pub mod prelude;
pub mod class;
pub mod gahe;
pub mod util;

pub use big_polynomial::*;
pub use class::*;
pub use gahe::*;
pub use util::*;

pub use std::{ops::{Add, Div, Mul, Rem, Sub, AddAssign, MulAssign}, time::{Duration, Instant}};
pub use num_traits::{FromPrimitive, ToPrimitive, One, Zero, Signed, Euclid};
pub use dyn_stack::{PodStack, GlobalPodBuffer, ReborrowMut};
pub use concrete_fft::ordered::{Plan, Method};
pub use rand::rngs::ThreadRng;
pub use concrete_fft::c64;
pub use num_complex::*;
pub use num_bigint::*;
pub use rand::Rng;

pub const N: usize = 256;            // N of X^N + 1
pub const B: f64 = 16777216.0;      // Fixed Base (2^24)
// pub const gamma: f64 = 206.0;        // Module 2^gamma
pub const l: usize = 10;            // ceil(log(gamma) with B base)