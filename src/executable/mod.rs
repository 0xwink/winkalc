mod execution;
mod display;
mod datatype;
mod func;
mod input;

pub fn go(){
    let mut reader = input::new_editor();

    loop {
        let Ok(input) = reader.readline("> ") else {
            println!("Goodbye!\n");
            break;
        };

        let input_str = input.trim();
        if input_str.to_ascii_lowercase() == "exit" {
            println!("Goodbye!\n");
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