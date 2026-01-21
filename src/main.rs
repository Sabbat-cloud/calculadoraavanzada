mod calc;
mod help;
mod history;
mod math_ext;
mod repl;

use std::env;
use crate::calc::Calculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Modo una sola ejecución (CLI)
        let mut calc = Calculator::new();
        // Unimos todos los argumentos por si el usuario no usó comillas: mod(12, 14)
        let input = args[1..].join("");

        match calc.evaluate(&input) {
            Ok(res) => {
                if res.im == 0.0 {
                    println!("{}", res.re);
                } else {
                    println!("{} {} {}i", res.re, if res.im >= 0.0 { "+" } else { "-" }, res.im.abs());
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Modo interactivo por defecto
        repl::run();
    }
}
