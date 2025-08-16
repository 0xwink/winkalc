pub use crate::customio::ParseError;
pub use crate::arithmetic::int;

#[derive(PartialEq, Copy, Clone, Debug)]
pub(super) enum Algebra {
    Z, Q, QPol, F(int), FPol(int), Zi
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(super) enum Operation {
    Add, Sub, Mul, Div,
    Mod, DivMod, Bezout,
}

#[derive(Debug)]
pub(super) struct RawCommand {
    pub alg: Algebra,
    pub op: Operation,
    pub operand1: String,
    pub operand2: String,
}

#[derive(Debug)]
pub(super) struct RawResult {
    pub oper1: String,
    pub oper2: String,
    pub main: String,
    pub sub1: Option<String>,
    pub sub2: Option<String>,
}

