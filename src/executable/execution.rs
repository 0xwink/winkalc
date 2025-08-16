use super::datatype::*;
use crate::arithmetic::{Integer, Prime, ZPol};
use crate::customio::display::SimpleDisplay;
use crate::{unwrapped_bezout, EuclideanRing, Field, Parse};

pub(super) fn execute_ring<R>(cmd: &RawCommand) -> Result<RawResult, ParseError>
where R: EuclideanRing + Parse + SimpleDisplay {
    let op = cmd.op;
    let str_operand1 = cmd.operand1.as_str();
    let str_operand2 = cmd.operand2.as_str();
    let operand1 = R::parse(str_operand1)?;
    let operand2 = R::parse(str_operand2)?;
    
    if op == Operation::Bezout {
        let (u, v, gcd) = unwrapped_bezout(&operand1, &operand2);

        Ok(RawResult{
            oper1: operand1.simple_display(),
            oper2: operand2.simple_display(),
            main: gcd.simple_display(),
            sub1: Some(u.simple_display()),
            sub2: Some(v.simple_display()),
        })
    }
    else if op == Operation::DivMod {
        if operand2 == R::zero() {
            return Err(ParseError::ZeroDenom);
        }
        let duo = R::divmod(&operand1, &operand2);
        let quo = duo.first; let rem= duo.second;

        Ok(RawResult {
            oper1: operand1.simple_display(),
            oper2: operand2.simple_display(),
            main: quo.simple_display(),
            sub1: Some(rem.simple_display()),
            sub2: None,
        })
    }
    else {
        let res = match op {
            Operation::Add => R::add(&operand1, &operand2),
            Operation::Sub => R::subtract(&operand1, &operand2),
            Operation::Mul => R::multiply(&operand1, &operand2),
            Operation::Mod => R::divmod(&operand1, &operand2).second,
            _ => return Err(ParseError::Op),
        };

        Ok(RawResult {
            oper1: operand1.simple_display(),
            oper2: operand2.simple_display(),
            main: res.simple_display(),
            sub1: None,
            sub2: None
        })
    }
}

pub(super) fn execute_field<F>(cmd: &RawCommand) -> Result<RawResult, ParseError> 
where F: Field + Parse + SimpleDisplay {
    let op = cmd.op;
    let str_operand1 = cmd.operand1.as_str();
    let str_operand2 = cmd.operand2.as_str();
    let operand1 = F::parse(str_operand1)?;
    let operand2 = F::parse(str_operand2)?;

    //pay special attention to division. zero cannot be a divisor.
    let res = match op {
        Operation::Add => F::add(&operand1, &operand2),
        Operation::Sub => F::subtract(&operand1, &operand2),
        Operation::Mul => F::multiply(&operand1, &operand2),
        Operation::Div => {
            if F::equal(&operand2, &F::zero()){
                return Err(ParseError::ZeroDenom);
            }
            F::divide(&operand1, &operand2)
        }
        _ => return Err(ParseError::Op),
    };

    Ok(RawResult {
        oper1: operand1.simple_display(),
        oper2: operand2.simple_display(),
        main: res.simple_display(),
        sub1: None,
        sub2: None
    })
}

pub(super) fn execute_f(cmd: &RawCommand) -> Result<RawResult, ParseError> {
    let Algebra::F(p_int) = cmd.alg else {
        panic!()
    };
    
    let Some(p) = Prime::try_new(p_int) else {
        return Err(ParseError::NotPrime);
    };

    let op = cmd.op;
    let str_operand1 = cmd.operand1.as_str();
    let str_operand2 = cmd.operand2.as_str();
    let raw_operand1 = Integer::parse(str_operand1)?;
    let raw_operand2 = Integer::parse(str_operand2)?;

    let operand1 = p.modulo(&raw_operand1);
    let operand2 = p.modulo(&raw_operand2);

    //pay special attention to division. zero cannot be a divisor.
    let res = match op {
        Operation::Add => p.add(&operand1, &operand2),
        Operation::Sub => p.subtract(&operand1, &operand2),
        Operation::Mul => p.multiply(&operand1, &operand2),
        Operation::Div => {
            if operand2 == Integer::zero() {
                return Err(ParseError::ZeroDenom);
            }
            p.divide(&operand1, &operand2)
        }
        _ => return Err(ParseError::Op),
    };

    Ok(RawResult {
        oper1: operand1.simple_display(),
        oper2: operand2.simple_display(),
        main: res.simple_display(),
        sub1: None,
        sub2: None
    })    
}

pub(super) fn execute_fpol(cmd: &RawCommand) -> Result<RawResult, ParseError> {
    let Algebra::FPol(p_int) = cmd.alg else {
        panic!()
    };
    
    let Some(p) = Prime::try_new(p_int) else {
        return Err(ParseError::NotPrime);
    };

    let op = cmd.op;
    let str_operand1 = cmd.operand1.as_str();
    let str_operand2 = cmd.operand2.as_str();
    let raw_operand1 = ZPol::parse(str_operand1)?;
    let raw_operand2 = ZPol::parse(str_operand2)?;

    let operand1 = p.modpol(&raw_operand1);
    let operand2 = p.modpol(&raw_operand2);

    if op == Operation::Bezout {
        let (u, v, gcd) = p.unwrapped_bezoutpol(&operand1, &operand2);

        Ok(RawResult{
            oper1: operand1.simple_display(),
            oper2: operand2.simple_display(),
            main: gcd.simple_display(),
            sub1: Some(u.simple_display()),
            sub2: Some(v.simple_display()),
        })
    }
    else if op == Operation::DivMod {
        if operand2 == ZPol::zero() {
            return Err(ParseError::ZeroDenom);
        }
        let duo = p.divmodpol(&operand1, &operand2);
        let quo = duo.first; let rem= duo.second;

        Ok(RawResult {
            oper1: operand1.simple_display(),
            oper2: operand2.simple_display(),
            main: quo.simple_display(),
            sub1: Some(rem.simple_display()),
            sub2: None,
        })
    }
    else {
        let res = match op {
            Operation::Add => p.addpol(&operand1, &operand2),
            Operation::Sub => p.subpol(&operand1, &operand2),
            Operation::Mul => p.mulpol(&operand1, &operand2),
            Operation::Mod => p.divmodpol(&operand1, &operand2).second,
            _ => return Err(ParseError::Op),
        };

        Ok(RawResult {
            oper1: operand1.simple_display(),
            oper2: operand2.simple_display(),
            main: res.simple_display(),
            sub1: None,
            sub2: None
        })
    }
}