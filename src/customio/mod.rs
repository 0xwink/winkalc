pub mod display;
pub mod parse;

pub use parse::{Parse, ParseError};

use crate::arithmetic::*;
use crate::{Field, EuclideanRing, Duo, Trio};