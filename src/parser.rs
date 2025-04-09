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

        index += 1
    }
}