pub mod execution;
pub mod display;
pub mod datatype;
pub mod func;

use std::io::{self, Write};

pub fn go(){
    loop {
        let mut input = String::new();
        print!("> ");
        
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Invalid input.");

        let input_str = input.trim();
        if input_str.to_ascii_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }

        let res_cmd = func::raw_parse(input_str);
        let Ok(cmd) = res_cmd else {
            res_cmd.unwrap_err().print();
            continue;
        };

        let res_res = func::execute(&cmd);
        let Ok(res) = res_res else {
            res_res.unwrap_err().print();
            continue;
        };

        let output_str = func::display(cmd, res);

        println!("Result: {}\n", output_str);
    }
}