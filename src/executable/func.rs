use regex::{self, Regex};
use crate::arithmetic::*;
use super::datatype::*;
use super::execution;
use super::display;

pub(super) fn as_algebra(raw_input: &str) -> Result<Algebra, ParseError> {
    let input_string = raw_input.to_ascii_uppercase();
    let input = input_string.as_str();
    
    let re_f = Regex::new(r"^F\(([0-9]+)\)$").unwrap();
    let re_fpol = Regex::new(r"^FPOL\(([0-9]+)\)$").unwrap();

    // simple situations
    if input == "Z" {
        return Ok(Algebra::Z);
    }
    if input == "Q" {
        return Ok(Algebra::Q);
    }
    if input == "QPOL" {
        return Ok(Algebra::QPol);
    }
    if input == "ZI" {
        return Ok(Algebra::Zi);
    }

    // F(p) case
    if re_f.is_match(input){
        let cap = re_f.captures(input).unwrap();
        let (_, [p_str]) = cap.extract();
        
        let p: int = p_str.parse().unwrap();

        return Ok(Algebra::F(p));
    }

    // FPol(p) case 
    if re_fpol.is_match(input){
        let cap = re_fpol.captures(input).unwrap();
        let (_, [p_str]) = cap.extract();
        
        let p: int = p_str.parse().unwrap();

        return Ok(Algebra::FPol(p));
    }

    Err(ParseError::Algebra)
}

pub(super) fn as_operation(input: &str) -> Result<Operation, ParseError> {
    match input.to_lowercase().as_str() {
        "add" => Ok(Operation::Add),
        "sub" => Ok(Operation::Sub),
        "mul" => Ok(Operation::Mul),
        "div" => Ok(Operation::Div),
        "mod" => Ok(Operation::Mod),
        "divmod" => Ok(Operation::DivMod),
        "bezout" => Ok(Operation::Bezout),
        _ => Err(ParseError::Op),
    }
}

pub(super) fn raw_parse(raw_input: &str) -> Result<RawCommand, ParseError>{
    if !raw_input.is_ascii() {return Err(ParseError::NotAscii)};
    let trimmed_string: String = raw_input.split_ascii_whitespace().collect();
    let input = trimmed_string.as_str();

    let re = Regex::new(r"^\[(.*)\](.*)\{(.*)\}\{(.*)\}$").unwrap();
    let Some(caps) = re.captures(input) else {
        return Err(ParseError::Format);
    };

    let (_, [raw_alg, raw_op, raw_arg1, raw_arg2]) = caps.extract();

    let alg = as_algebra(raw_alg)?;

    let op = as_operation(raw_op)?;

    let operand1 = String::from(raw_arg1);

    let operand2 = String::from(raw_arg2);

    Ok(RawCommand{
        alg: alg,
        op: op,
        operand1: operand1,
        operand2: operand2,
        
    })
}

pub(super) fn execute(cmd: &RawCommand) -> Result<RawResult, ParseError> {
    match cmd.alg {
        Algebra::Z => execution::execute_ring::<Integer>(cmd),
        Algebra::Q => execution::execute_field::<Rational>(cmd),
        Algebra::QPol => execution::execute_ring::<Polynomial<Rational>>(cmd),
        Algebra::F(_) => execution::execute_f(cmd),
        Algebra::FPol(_) => execution::execute_fpol(cmd),
        Algebra::Zi => execution::execute_ring::<GaussInteger>(cmd)
    }
}

pub(super) fn display(cmd: RawCommand, res: RawResult) -> String {
    match cmd.alg {
        Algebra::Z => display::display_ring(cmd, res),
        Algebra::Q => display::display_field(cmd, res),
        Algebra::QPol => display::display_ring(cmd, res),
        Algebra::F(_) => display::display_f(cmd, res),
        Algebra::FPol(_) => display::display_fpol(cmd, res),
        Algebra::Zi => display::display_ring(cmd, res)
    }
}



