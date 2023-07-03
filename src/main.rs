mod lexer;
mod token;
mod parser;
mod ast;
mod runtime;

use std::io::{self, Write};

fn main() {
    loop {
        let mut s: String = String::new();

        input("> ", &mut s);

        if s == "exit" { break }

        let l_res = lexer::lex(s.clone());
        if l_res.1 { continue }
        let tokens = l_res.0;

        let p_res = parser::parse(tokens, s.clone());
        if p_res.1 { continue }
        let exp = p_res.0;

        println!("{:?}", exp);
    }
}

fn input(prompt: &str, output: &mut String) {
    print!("{}", prompt);

    io::stdout().flush().expect("not!");
    io::stdin().read_line(output).expect("not!");
    *output = output.trim().into();
}
