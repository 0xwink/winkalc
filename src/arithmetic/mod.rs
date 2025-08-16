mod rational;
pub use rational::{Rational, ComplexRational};

mod integer;
pub use integer::{Integer, GaussInteger};

mod polynomial;
pub use polynomial::{Polynomial, Z, ZPol, QPol};

mod prime;
pub use prime::Prime;

mod ring;
pub use ring::{Field, EuclideanRing, Ring};

pub use i64 as int;


use crate::{Duo, Trio};