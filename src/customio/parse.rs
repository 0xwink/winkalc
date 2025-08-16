use super::*;
use regex::{self, Regex};

#[derive(PartialEq, Debug)]
pub enum ParseError {
    NotAscii, Format, Algebra, Op, Operand, NotPrime, ZeroDenom,

    //only for debug use
    Debug, Debug1, Debug2, Debug3, Debug4
}
impl ParseError {
    pub fn print(&self){
        println!("Error: {self:?}\n");
    }
}

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<Self, ParseError>;
}

impl Parse for Rational {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let re_int = Regex::new(r"^ *([0-9]+) *$").unwrap();
        let re_neg_int = Regex::new(r"^ *\- *([0-9]+) *$").unwrap();
        let re_wrapped_neg_int = Regex::new(r"^ *\( *\- *([0-9]+) *\) *$").unwrap();
        let re_q = Regex::new(r"^ *([0-9]+) */ *([0-9]+) *$").unwrap();
        let re_wrapped_q = Regex::new(r"^ *\( *([0-9]+) */ *([0-9]+) *\) *$").unwrap();
        let re_neg_wrapped_q = Regex::new(r"^ *\- *\( *([0-9]+) */ *([0-9]+) *\) *$").unwrap();
        let re_wrapped_neg_q = Regex::new(r"^ *\( *\- *([0-9]+) */ *([0-9]+) *\) *$").unwrap();

        if re_int.is_match(input) {
            let caps = re_int.captures(input).unwrap();
            let (_, [int]) = caps.extract();
            let output: int = int.parse().unwrap();
            Ok(Rational::new(output, 1))
        }
        else if re_neg_int.is_match(input){
            let caps = re_int.captures(input).unwrap();
            let (_, [int]) = caps.extract();
            let output: int = int.parse().unwrap();
            Ok(Rational::new(-output, 1))
        }
        else if re_wrapped_neg_int.is_match(input) {
            let caps = re_wrapped_neg_int.captures(input).unwrap();
            let (_, [int]) = caps.extract();
            let output: int = int.parse().unwrap();
            Ok(Rational::new(-output, 1))
        }
        else if re_q.is_match(input) {
            let caps = re_q.captures(input).unwrap();
            let (_, [num_str, denom_str]) = caps.extract();
            let num: int = num_str.parse().unwrap();
            let denom: int = denom_str.parse().unwrap();
            if denom == 0 {
                return Err(ParseError::ZeroDenom);
            }
            Ok(Rational::new(num, denom))
        }
        else if re_wrapped_q.is_match(input) {
            let caps = re_wrapped_q.captures(input).unwrap();
            let (_, [num_str, denom_str]) = caps.extract();
            let num: int = num_str.parse().unwrap();
            let denom: int = denom_str.parse().unwrap();
            if denom == 0 {
                return Err(ParseError::ZeroDenom);
            }
            Ok(Rational::new(num, denom))
        }
        else if re_neg_wrapped_q.is_match(input) {
            let caps = re_neg_wrapped_q.captures(input).unwrap();
            let (_, [num_str, denom_str]) = caps.extract();
            let num: int = num_str.parse().unwrap();
            let denom: int = denom_str.parse().unwrap();
            if denom == 0 {
                return Err(ParseError::ZeroDenom);
            }
            Ok(Rational::new(-num, denom))
        }
        else if re_wrapped_neg_q.is_match(input) {
            let caps = re_wrapped_neg_q.captures(input).unwrap();
            let (_, [num_str, denom_str]) = caps.extract();
            let num: int = num_str.parse().unwrap();
            let denom: int = denom_str.parse().unwrap();
            if denom == 0 {
                return Err(ParseError::ZeroDenom);
            }
            Ok(Rational::new(-num, denom))
        }
        else {
            Err(ParseError::Operand)
        }
    }
}

impl Parse for Integer {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let re_int = Regex::new(r"^ *([0-9]+) *$").unwrap();
        let re_neg_int = Regex::new(r"^ *\- *([0-9]+) *$").unwrap();
        let re_wrapped_neg_int = Regex::new(r"^ *\( *\- *([0-9]+) *\) *$").unwrap();

        if re_int.is_match(input) {
            let caps = re_int.captures(input).unwrap();
            let (_, [int]) = caps.extract();
            let output: int = int.parse().unwrap();
            Ok(Integer::new(output))
        }
        else if re_neg_int.is_match(input){
            let caps = re_neg_int.captures(input).unwrap();
            let (_, [int]) = caps.extract();
            let output: int = int.parse().unwrap();
            Ok(Integer::new(-output))
        }
        else if re_wrapped_neg_int.is_match(input) {
            let caps = re_wrapped_neg_int.captures(input).unwrap();
            let (_, [int]) = caps.extract();
            let output: int = int.parse().unwrap();
            Ok(Integer::new(-output))
        }
        else {
            Err(ParseError::Operand)
        }
    }
}

impl Parse for Polynomial<Rational> {
    fn parse(raw_input: &str) -> Result<Self, ParseError> {
        let re_mon = Regex::new(r"^(.*)x\^?([0-9]*)$").unwrap();
        // drop all whitespace
        let input: String = raw_input.split_ascii_whitespace().collect();

        // replace "+" with "+-" in order to split the polynomial into monomials
        let string_ = input.replace("-", "+-");
        
        let str = string_.as_str();

        let vec_str: Vec<&str> = str.split('+').collect();

        let mut vec: Vec<Self> = Vec::new();

        // monomial parsing. if empty then skip; if it contains "x" then use regex;
        // if it has no "x" then treat as constant.
        for str_mon in vec_str {
            if str_mon.is_empty() {
                // do nothing
            }

            else if re_mon.is_match(str_mon) {
                let caps = re_mon.captures(str_mon).unwrap();
                let (_, [str_coeff, str_expon]) = caps.extract();
                let coeff: Rational;

                // pay attention to special cases as in "-x" and "x", 
                // where str_coeff cant be parsed normally
                if str_coeff == "" {
                    coeff = Rational::new(1,1);
                } else if str_coeff == "-" {
                    coeff = Rational::new(-1, 1)
                } else {
                    let res_coeff = Rational::parse(str_coeff);
                    if res_coeff.is_err() {return Err(ParseError::Operand);}
                    coeff = res_coeff.unwrap();
                }
                
                // pay attention to "2x", as the exponential term is omitted 
                let expon: usize = if str_expon.is_empty() {1} else {
                    str_expon.parse::<usize>().unwrap()
                };

                let mon = Self::monomial(&coeff, expon);
                
                vec.push(mon);
            }

            else {
                let res_con = Rational::parse(str_mon);
                if res_con.is_err() { return Err(ParseError::Operand); }
                
                let con = res_con.unwrap();
                let mon = Self::monomial(&con, 0);
                
                vec.push(mon);
            }
        } 

        if vec.is_empty() {
            return Err(ParseError::Operand);
        }

        let mut sum = Self::zero();

        for p in vec {
            sum = Self::add(&sum, &p);
        }

        Ok(sum)
    }
}

impl Parse for ZPol {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let qpol = Polynomial::<Rational>::parse(input)?;
        
        let Some(zpol) = qpol.to_zpol() else {
            return Err(ParseError::Operand);
        };

        Ok(zpol)
    }
}

// trick: use Polynomial Parse for Complex parse
impl Parse for ComplexRational {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let pol_str = input.replace("i", "x");
        let pol = QPol::parse(&pol_str)?;

        if pol.norm() >= 2 {
            return Err(ParseError::Operand);
        }

        let res = ComplexRational {
            real: pol.coefficient(0),
            imag: pol.coefficient(1)
        };

        Ok(res)
    }
}

impl Parse for GaussInteger {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let rat = ComplexRational::parse(input)?;

        let Some(res) = rat.to_gauss_integer() else {
            return Err(ParseError::Operand);
        };

        Ok(res)
    }
}