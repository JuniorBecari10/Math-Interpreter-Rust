mod lexer;
mod token;

use std::io::{self, Write};

fn main() {
    loop {
        let mut s: String = String::new();

        input("> ", &mut s);

        let res = lexer::lex(s);
        if res.1 { continue; }
        let tokens = res.0;

        println!("{:?}", tokens);
    }
}

fn input(prompt: &str, output: &mut String) {
    print!("{}", prompt);

    io::stdout().flush().expect("not!");
    io::stdin().read_line(output).expect("not!");
    *output = output.trim().into();
}
