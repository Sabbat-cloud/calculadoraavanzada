use std::borrow::Cow;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

use crate::calc::Calculator;
use crate::help::show_help;
use crate::history::{load_history_expr, HistoryPick};

const CMD_HISTORY_FILE: &str = "historial_cmds.txt";

// Lista de comandos REPL
const COMMANDS: &[&str] = &[
    "help", "exit", "new", "mode", "vars", "mem", "hist", "clear", "plot",
    "push", "pop", "dup", "swap", "clearstack", "mem", "sum", "avg", "min", "max", "std", "ayuda",
];

// Lista de funciones soportadas
const FUNCS: &[&str] = &[
    "exp", "sqrt", "cbrt", "ln", "abs", "floor", "ceil", "round", "trunc", "sign", "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "deg2rad", "rad2deg", "cm2in", "in2cm", "m2ft", "ft2m", "fact", "log10", "log2", "isprime", "nextprime", "atan2", "hypot", "root", "log", "mcd", "mcm", "comb", "nCr", "perm", "nPr", "pow", "min", "max", "mod", "rand", "pct", "applypct", "r3d", "r3i", "abs", "arg", "conj", "re", "im", "bin", "oct", "hex",
];

#[derive(Clone)]
struct CalcHelper {
    vars: Vec<String>,
}

impl CalcHelper {
    fn new() -> Self {
        Self { vars: Vec::new() }
    }
    fn update_vars(&mut self, keys: impl Iterator<Item = String>) {
        self.vars = keys.collect();
        self.vars.sort();
        self.vars.dedup();
        for extra in ["ans", "last", "i", "x"] {
            if !self.vars.iter().any(|v| v == extra) {
                self.vars.push(extra.to_string());
            }
        }
        self.vars.sort();
    }
}

impl Helper for CalcHelper {}
impl Hinter for CalcHelper {
    type Hint = String;
    fn hint(&self, _: &str, _: usize, _: &Context<'_>) -> Option<Self::Hint> { None }
}
impl Highlighter for CalcHelper {}
impl Validator for CalcHelper {}

impl Completer for CalcHelper {
    type Candidate = Pair;
    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let bytes = line.as_bytes();
        let mut start = pos;
        while start > 0 {
            let c = bytes[start - 1] as char;
            if c.is_ascii_alphanumeric() || c == '_' { start -= 1; } else { break; }
        }
        let prefix = &line[start..pos];
        if prefix.is_empty() { return Ok((start, vec![])); }

        let mut out: Vec<Pair> = Vec::new();
        for &c in COMMANDS {
            if c.starts_with(prefix) {
                out.push(Pair { display: c.to_string(), replacement: c.to_string() });
            }
        }
        for &f in FUNCS {
            if f.starts_with(prefix) {
                out.push(Pair { display: format!("{f}()"), replacement: f.to_string() });
            }
        }
        for v in &self.vars {
            if v.starts_with(prefix) {
                out.push(Pair { display: v.clone(), replacement: v.clone() });
            }
        }
        Ok((start, out))
    }
}

pub fn run() {
    let mut calc = Calculator::new();
    let mut rl: Editor<CalcHelper, rustyline::history::DefaultHistory> = Editor::new().expect("rustyline editor");
    rl.set_helper(Some(CalcHelper::new()));
    let _ = rl.load_history(CMD_HISTORY_FILE);

    println!("Calculadora Avanzada (Complejos) en Rust. Escribe 'help'. (Ctrl+D para salir)");

    loop {
        if let Some(h) = rl.helper_mut() {
            h.update_vars(calc.vars.keys().cloned());
        }

        let mode_str = if calc.is_radians { "RAD" } else { "DEG" };
        let prompt = format!("[{}] >> ", mode_str);

        let line = match rl.readline(&prompt) {
            Ok(s) => s,
            Err(ReadlineError::Interrupted) => { println!("^C"); continue; },
            Err(ReadlineError::Eof) => break,
            Err(e) => { eprintln!("Error: {e}"); break; }
        };

        let raw0 = line.trim();
        if raw0.is_empty() { continue; }
        rl.add_history_entry(Cow::Borrowed(raw0)).ok();

        // Lógica de historial (!! y !N) sobre historial.txt
        let line_to_eval = if raw0 == "!!" {
            match load_history_expr(&calc.history_file, HistoryPick::Last) {
                Ok(expr) => { println!("(hist !!) {}", expr); expr }
                Err(e) => { println!("Error: {}", e); continue; }
            }
        } else if raw0.starts_with('!') && raw0.len() > 1 && raw0[1..].chars().all(|c: char| c.is_ascii_digit()) {
            let n: usize = raw0[1..].parse().unwrap_or(0);
            match load_history_expr(&calc.history_file, HistoryPick::Index1(n)) {
                Ok(expr) => { println!("(hist !{}) {}", n, expr); expr }
                Err(e) => { println!("Error: {}", e); continue; }
            }
        } else {
            raw0.to_string()
        };

        let raw = line_to_eval.trim();
        match raw {
            "exit" | "quit" | "salir" => break,
            "help" => show_help(),
            "new" => {
                calc.reset();
                println!("Sistema reseteado.");
            }
            "mode" => {
                calc.is_radians = !calc.is_radians;
                println!("Modo: {}", if calc.is_radians { "RAD" } else { "DEG" });
            }
            "vars" => println!("Vars: {:?}", calc.vars),
            "mem" => println!("Pila: {:?}", calc.memory_stack),
            "clearstack" => {
                calc.memory_stack.clear();
                println!("Pila vaciada.");
            }
            "pop" => match calc.memory_stack.pop() {
                Some(v) => {
                    calc.last_result = v;
                    println!("POP -> {} (size={})", v, calc.memory_stack.len());
                }
                None => println!("Pila vacía."),
            },
            "dup" => match calc.memory_stack.last().copied() {
                Some(v) => {
                    calc.memory_stack.push(v);
                    println!("DUP -> {} (size={})", v, calc.memory_stack.len());
                }
                None => println!("Pila vacía."),
            },
            "swap" => {
                let n = calc.memory_stack.len();
                if n < 2 {
                    println!("Se necesitan al menos 2 valores en la pila.");
                } else {
                    calc.memory_stack.swap(n - 1, n - 2);
                    println!("SWAP -> top={} (size={})", calc.memory_stack[n - 1], n);
                }
            },
            
            // --- Gestión de Historial Físico ---
            "hist" => match std::fs::read(&calc.history_file) {
                Ok(btes) => print!("{}", String::from_utf8_lossy(&btes)),
                Err(e) => eprintln!("No se pudo leer {}: {}", calc.history_file, e),
            },
            "clear" => match std::fs::write(&calc.history_file, b"") {
                Ok(_) => println!("Historial vaciado: {}", calc.history_file),
                Err(e) => eprintln!("No se pudo vaciar {}: {}", calc.history_file, e),
            },

            // --- Estadísticas sobre la pila ---
            "sum" => {
                if calc.memory_stack.is_empty() { println!("Pila vacía."); }
                else {
                    let v: num_complex::Complex64 = calc.memory_stack.iter().sum();
                    calc.last_result = v;
                    println!("= {}", v);
                }
            },
            "avg" => {
                if calc.memory_stack.is_empty() { println!("Pila vacía."); }
                else {
                    let sum: num_complex::Complex64 = calc.memory_stack.iter().sum();
                    let v = sum / (calc.memory_stack.len() as f64);
                    calc.last_result = v;
                    println!("= {}", v);
                }
            },

            "min" => {
                if let Some(m) = calc.memory_stack.iter().min_by(|a, b| a.re.partial_cmp(&b.re).unwrap()) {
                    println!("= {}", m);
                } else { println!("Pila vacía."); }
            },
            "max" => {
                if let Some(m) = calc.memory_stack.iter().max_by(|a, b| a.re.partial_cmp(&b.re).unwrap()) {
                    println!("= {}", m);
                } else { println!("Pila vacía."); }
            },

            "author" => {
                println!("By Oscar Gimenez Blasco.\nhttps://sabbat.cloud\nhttps://github.com/Sabat-cloud");
            }


            s if s.starts_with("ayuda ") => {
                let target = s[6..].trim();
                crate::help::show_specific_help(target);
                }   
            // --- Otros Comandos ---
            s if s.starts_with("plot ") => {
                calc.plot(&s[5..]);
            }

            s if s.starts_with("push ") => {
                let rest = s[5..].trim();
                let cleaned = rest.replace(',', " ");
                let parts: Vec<&str> = cleaned.split_whitespace().collect();
                for p in parts {
                    match calc.evaluate(p) {
                        Ok(v) => {
                            calc.memory_stack.push(v);
                            println!("PUSH -> {} (size={})", v, calc.memory_stack.len());
                        }
                        Err(e) => { println!("Error en '{}': {}", p, e); break; }
                    }
                }
            }

            s if s.contains('=') => {
                let p: Vec<&str> = s.split('=').collect();
                if p.len() >= 2 {
                    let var_name = p[0].trim().to_string();
                    match calc.evaluate(p[1]) {
                        Ok(r) => {
                            calc.vars.insert(var_name.clone(), r);
                            if r.im == 0.0 { println!("{} = {}", var_name, r.re); }
                            else { println!("{} = {} {} {}i", var_name, r.re, if r.im >= 0.0 { "+" } else { "-" }, r.im.abs()); }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }

            _ => match calc.evaluate(raw) {
                Ok(res) => {
                    if res.im == 0.0 { println!("= {}", res.re); }
                    else { println!("= {} {} {}i", res.re, if res.im >= 0.0 { "+" } else { "-" }, res.im.abs()); }

                    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&calc.history_file) {
                        use std::io::Write;
                        let _ = writeln!(file, "{} = {}", raw, res);
                    }
                }
                Err(e) => println!("Error: {}", e),
            },
        }
    }
    let _ = rl.save_history(CMD_HISTORY_FILE);
}
