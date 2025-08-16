use super::datatype::*;

pub(super) fn display_ring (cmd: RawCommand, res: RawResult) -> String {
    let op  = cmd.op;
    if op == Operation::Bezout {
        let main_str = "U * F + V * G = H, where";
        let u_str = format!("U = {}", res.sub1.unwrap());
        let f_str = format!("F = {}", res.oper1);
        let v_str = format!("V = {}", res.sub2.unwrap());
        let g_str = format!("G = {}", res.oper2);
        let h_str = format!("[GCD] H = {}", res.main);

        return format!("{main_str}\n{u_str},\n{f_str},\n{v_str},\n{g_str},\n{h_str}.");
    }
    else if op == Operation::DivMod {
        let main_str = "F / G = Q ... R, where";
        let f_str = format!("F = {}", res.oper1);
        let g_str = format!("G = {}", res.oper2);
        let q_str = format!("Q = {}", res.main);
        let r_str = format!("R = {}", res.sub1.unwrap());

        return format!("{main_str}\n{f_str},\n{g_str},\n{q_str},\n{r_str}.");
    }
    else {
        return format!("{}", res.main);
    }
}

pub(super) fn display_fpol(cmd: RawCommand, res: RawResult) -> String{
    let op  = cmd.op;

    let Algebra::FPol(p) = cmd.alg else{
        panic!();
    };

    if op == Operation::Bezout {
        let main_str = format!("U * F + V * G = H mod {p}, where");
        let u_str = format!("U = {}", res.sub1.unwrap());
        let f_str = format!("F = {}", res.oper1);
        let v_str = format!("V = {}", res.sub2.unwrap());
        let g_str = format!("G = {}", res.oper2);
        let h_str = format!("[GCD] H = {}", res.main);

        return format!("{main_str}\n{u_str},\n{f_str},\n{v_str},\n{g_str},\n{h_str}.");
    }
    else if op == Operation::DivMod {
        let main_str = format!("F / G = Q ... R mod {p}, where");
        let f_str = format!("F = {}", res.oper1);
        let g_str = format!("G = {}", res.oper2);
        let q_str = format!("Q = {}", res.main);
        let r_str = format!("R = {}", res.sub1.unwrap());

        return format!("{main_str}\n{f_str},\n{g_str},\n{q_str},\n{r_str}.");
    }
    else {
        let operator = match op {
            Operation::Add => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Mod => "%",
            _ => panic!(),
        };

        let main_str = format!("F {operator} G = H mod {p}, where");
        let f_str = format!("F = {}", res.oper1);
        let g_str = format!("G = {}", res.oper2);
        let h_str = format!("H = {}", res.main);

        return format!("{main_str}\n{f_str},\n{g_str},\n{h_str}.")
    }    
}

pub(super) fn display_field(_cmd: RawCommand, res: RawResult) -> String {
    format!("{}", res.main)
}

pub(super) fn display_f(cmd: RawCommand, res: RawResult) -> String {
    let op = cmd.op;

    let Algebra::F(p) = cmd.alg else{
        panic!();
    };

    let operator = match op {
        Operation::Add => "+",
        Operation::Sub => "-",
        Operation::Mul => "*",
        Operation::Div => "/",
        _ => panic!(),
    };

    format!("{} {} {} = {} mod {}.", res.oper1, operator, res.oper2, res.main, p)
}