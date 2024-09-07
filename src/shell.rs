use lalrpop_util::lalrpop_mod;
use std::io::{self, stdout, Write};

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub grammar
);

macro_rules! flush_stdout {
    () => {
        stdout().lock().flush().unwrap()
    };
}

pub fn run() {
    let mut line = String::new();
    let stdin = io::stdin();

    print!("λ ");
    flush_stdout!();
    while stdin.read_line(&mut line).is_ok() {
        let token_vec = match grammar::ExprParser::new().parse(&line) {
            Ok(token_vec) => token_vec,
            Err(err) => {
                println!("[ERROR] Couldn't parse line. {err}");
                std::process::exit(1);
            }
        };

        println!("{:#?}", token_vec);
        print!("λ ");
        flush_stdout!();
        line.clear();
    }
}
