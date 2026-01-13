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

// Lista de comandos REPL -consola-
const COMMANDS: &[&str] = &[
    "help", "exit", "new", "mode", "vars", "mem",
    "sum", "avg", "min", "max", "std",
    "push", "pop", "dup", "swap", "clearstack",
    "hist", "clear",
    "plot",
];

// Lista de funciones del parser -ya soportadas-
const FUNCS: &[&str] = &[
    "sin","cos","tan","asin","acos","atan","atan2","hypot",
    "sqrt","cbrt","root","log","ln",
    "mcd","mcm",
    "floor","ceil","abs","round","trunc","sign",
    "sinh","cosh","tanh","asinh","acosh","atanh",
    "deg2rad","rad2deg","cm2in","in2cm","m2ft","ft2m",
    "fact","comb","perm","nCr","nPr",
    "exp","log10","log2","pow","min","max","mod","isprime",
    "nextprime","rand","pct","applypct","r3d","r3i",
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
        // extras útiles
        for extra in ["ans", "last", "x"] {
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
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}
impl Highlighter for CalcHelper {}
impl Validator for CalcHelper {}

impl Completer for CalcHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // Encuentra el "token" a completar mirando hacia atrás desde el cursor
        let bytes = line.as_bytes();
        let mut start = pos;
        while start > 0 {
            let c = bytes[start - 1] as char;
            if c.is_ascii_alphanumeric() || c == '_' {
                start -= 1;
            } else {
                break;
            }
        }
        let prefix = &line[start..pos];
        if prefix.is_empty() {
            return Ok((start, vec![]));
        }

        let mut out: Vec<Pair> = Vec::new();

        // Autocomplete de comandos
        for &c in COMMANDS {
            if c.starts_with(prefix) {
                out.push(Pair {
                    display: c.to_string(),
                    replacement: c.to_string(),
                });
            }
        }

        // Autocomplete de funciones (si se escriben tipo "si" -> "sin")
        for &f in FUNCS {
            if f.starts_with(prefix) {
                out.push(Pair {
                    display: format!("{f}()"),
                    replacement: f.to_string(),
                });
            }
        }

        // Autocomplete de variables (dinámico)
        for v in &self.vars {
            if v.starts_with(prefix) {
                out.push(Pair {
                    display: v.clone(),
                    replacement: v.clone(),
                });
            }
        }

        // Ordena y limita un poco para no saturar
        out.sort_by(|a, b| a.display.cmp(&b.display));
        out.dedup_by(|a, b| a.display == b.display);

        Ok((start, out))
    }
}

pub fn run() {
    let mut calc = Calculator::new();

    let mut rl: Editor<CalcHelper, rustyline::history::DefaultHistory> =
        Editor::new().expect("rustyline editor");
    rl.set_helper(Some(CalcHelper::new()));

    // Carga historial de comandos (↑↓)
    let _ = rl.load_history(CMD_HISTORY_FILE);

    println!("Calculadora Avanzada en Rust. Escribe 'help'. (Ctrl+D para salir)");

    loop {
        // Actualiza variables para autocompletar
        if let Some(h) = rl.helper_mut() {
            h.update_vars(calc.vars.keys().cloned());
        }

        let mode_str = if calc.is_radians { "RAD" } else { "DEG" };
        let prompt = format!("[{}] >> ", mode_str);

        let line = match rl.readline(&prompt) {
            Ok(s) => s,
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C: no salimos, solo línea nueva
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(e) => {
                eprintln!("Error readline: {e}");
                break;
            }
        };

        let raw0 = line.trim();
        if raw0.is_empty() {
            continue;
        }

        // Añade al historial de comandos (↑↓)
        rl.add_history_entry(Cow::Borrowed(raw0)).ok();

        // Soporte !! y !N (sobre historial de expresiones "historial.txt")
        let mut line2 = raw0.to_string();
        if line2 == "!!" {
            match load_history_expr(&calc.history_file, HistoryPick::Last) {
                Ok(expr) => {
                    println!("(hist !!) {}", expr);
                    line2 = expr;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }
        } else if line2.starts_with('!')
            && line2.len() > 1
            && line2[1..].chars().all(|c| c.is_ascii_digit())
        {
            let n: usize = match line2[1..].parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Error: uso !N donde N es un entero.");
                    continue;
                }
            };
            match load_history_expr(&calc.history_file, HistoryPick::Index1(n)) {
                Ok(expr) => {
                    println!("(hist !{}) {}", n, expr);
                    line2 = expr;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }
        }

        let raw = line2.trim();

        match raw {
            "exit"|"quit"|"salir" => break,
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

            "sum" => {
                if calc.memory_stack.is_empty() {
                    println!("Pila vacía.");
                } else {
                    let v: f64 = calc.memory_stack.iter().sum();
                    calc.last_result = v;
                    println!("= {}", v);
                }
            }
            "avg" => {
                if calc.memory_stack.is_empty() {
                    println!("Pila vacía.");
                } else {
                    let v: f64 =
                        calc.memory_stack.iter().sum::<f64>() / (calc.memory_stack.len() as f64);
                    calc.last_result = v;
                    println!("= {}", v);
                }
            }
            "min" => {
                if let Some(m) = calc.memory_stack.iter().cloned().reduce(f64::min) {
                    calc.last_result = m;
                    println!("= {}", m);
                } else {
                    println!("Pila vacía.");
                }
            }
            "max" => {
                if let Some(m) = calc.memory_stack.iter().cloned().reduce(f64::max) {
                    calc.last_result = m;
                    println!("= {}", m);
                } else {
                    println!("Pila vacía.");
                }
            }
            "std" => {
                let n = calc.memory_stack.len();
                if n == 0 {
                    println!("Pila vacía.");
                } else {
                    let mean = calc.memory_stack.iter().sum::<f64>() / (n as f64);
                    let var = calc
                        .memory_stack
                        .iter()
                        .map(|x| {
                            let d = x - mean;
                            d * d
                        })
                        .sum::<f64>()
                        / (n as f64);
                    let v = var.sqrt();
                    calc.last_result = v;
                    println!("= {}", v);
                }
            }

            "clearstack" => {
                calc.memory_stack.clear();
                println!("Pila vaciada.");
            }

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
                    println!(
                        "SWAP -> top={} (size={})",
                        calc.memory_stack[n - 2],
                        calc.memory_stack.len()
                    );
                }
            }

            "clear" => match std::fs::write(&calc.history_file, b"") {
                Ok(_) => println!("Historial borrado: {}", calc.history_file),
                Err(e) => eprintln!("No se pudo borrar {}: {}", calc.history_file, e),
            },

            "hist" => match std::fs::read(&calc.history_file) {
                Ok(btes) => print!("{}", String::from_utf8_lossy(&btes)),
                Err(e) => eprintln!("No se pudo leer {}: {}", calc.history_file, e),
            },

            "push" => {
                let v = calc.last_result;
                calc.memory_stack.push(v);
                println!("PUSH(last) -> {} (size={})", v, calc.memory_stack.len());
            }

            s if s.starts_with("push ") => {
                let rest = s[5..].trim();
                if rest.is_empty() {
                    println!("Uso: push <expr> [expr2 ...]");
                    continue;
                }
                let parts: Vec<&str> = rest.split_whitespace().collect();
                for p in parts {
                    match calc.evaluate(p) {
                        Ok(v) => {
                            calc.memory_stack.push(v);
                            println!("PUSH -> {} (size={})", v, calc.memory_stack.len());
                        }
                        Err(e) => {
                            println!("Error en '{}': {}", p, e);
                            break;
                        }
                    }
                }
            }

            "pop" => match calc.memory_stack.pop() {
                Some(v) => {
                    calc.last_result = v;
                    println!("POP  -> {} (size={})", v, calc.memory_stack.len());
                }
                None => println!("Pila vacía."),
            },

            s if s.starts_with("plot ") => calc.plot(&s[5..]),

            s if s.contains('=') => {
                let p: Vec<&str> = s.split('=').collect();
                let var_name = p[0].trim().to_string();
                match calc.evaluate(p[1]) {
                    Ok(r) => {
                        calc.vars.insert(var_name.clone(), r);
                        println!("{} = {}", var_name, r);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }

            _ => match calc.evaluate(raw) {
                Ok(res) => {
                    println!("= {}", res);

                    if let Ok(mut file) = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&calc.history_file)
                    {
                        use std::io::Write;
                        let _ = writeln!(file, "{} = {}", raw, res);
                    }
                }
                Err(e) => println!("Error: {}", e),
            },
        }
    }

    // Guarda historial de comandos
    let _ = rl.save_history(CMD_HISTORY_FILE);
}

