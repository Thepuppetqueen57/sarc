fn split_amount(input: &str, delimiter: &str, n: usize) -> Vec<String> {
    input.splitn(n, delimiter)
        .map(|s| s.to_string())
        .collect()
}

pub fn parse_code(code: String) {
    let lines: Vec<&str> = code.split(';').map(|l| l.trim()).collect();

    let mut index: usize = 0;
    while index + 1 < lines.len() {
        let line: &str = lines[index];
        let mut func_args: Vec<String> = split_amount(line, "(", 1);

        // Borrow checker
        // Oh borrow checker
        // Why do you exist?
        // Borrow checker
        // Oh borrow checker
        // die now
        let why_me_why_me: usize = func_args.len();
        func_args[why_me_why_me - 1].pop();

        let func_name: String = func_args[0].clone();
        let arguments_str: String = func_args[1].clone();
        let arguments: Vec<&str> = arguments_str.split(", ").collect();

        if func_name == "print" {
            let mut str_to_print: String = "".to_string();
            let mut reading_str: bool = false;

            for char in arguments[0].chars() {
                if char == '"' {
                    if reading_str {
                        reading_str = false
                    } else {
                        reading_str = true
                    }
                }

                if reading_str {
                    str_to_print.push(char);
                }
            }

            println!("{}", str_to_print);
        }

        index += 1
    }
}