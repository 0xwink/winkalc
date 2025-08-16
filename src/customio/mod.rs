mod display;
pub use display::SimpleDisplay;

mod parse;
pub use parse::{Parse, ParseError};

use super::arithmetic::*;
use crate::{Duo, Trio};