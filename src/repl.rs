use std::borrow::Cow;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use colored::Colorize;

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
    "exp", "sqrt", "cbrt", "ln", "abs", "floor", "ceil", "round", "trunc", "sign", "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "deg2rad", "rad2deg", "cm2in", "in2cm", "m2ft", "ft2m", "fact", "log10", "log2", "isprime", "nextprime", "atan2", "hypot", "root", "log", "mcd", "mcm", "comb", "nCr", "perm", "nPr", "pow", "min", "max", "mod", "rand", "pct", "applypct", "r3d", "r3i", "abs", "arg", "conj", "re", "im", "bin", "oct", "hex","integ",
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

impl Highlighter for CalcHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let mut colored_line = String::new();
        let mut chars = line.chars().peekable();

        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() || c == '.' {
                // Números -> Azul brillante
                let mut num = String::new();
                while let Some(&n) = chars.peek() {
                    if n.is_ascii_digit() || n == '.' {
                        num.push(n);
                        chars.next();
                    } else { break; }
                }
                colored_line.push_str(&num.bright_blue().to_string());

            } else if "+-*/^%=!&|".contains(c) {
                // Operadores -> Rojo
                colored_line.push_str(&c.to_string().red().to_string());
                chars.next();

            } else if c.is_ascii_alphabetic() || c == '_' {
                // Palabras (Funciones, Comandos o Variables)
                let mut word = String::new();
                while let Some(&w) = chars.peek() {
                    if w.is_ascii_alphanumeric() || w == '_' {
                        word.push(w);
                        chars.next();
                    } else { break; }
                }

                if FUNCS.contains(&word.as_str()) || COMMANDS.contains(&word.as_str()) {
                    // Funciones y Comandos -> Verde
                    colored_line.push_str(&word.green().to_string());
                } else if self.vars.iter().any(|v| v == &word) {
                    // Variables conocidas -> Cyan
                    colored_line.push_str(&word.cyan().to_string());
                } else {
                    // Texto desconocido -> Default (Blanco)
                    colored_line.push_str(&word);
                }

            } else {
                // Paréntesis, espacios, otros -> Default
                colored_line.push(c);
                chars.next();
            }
        }

        Cow::Owned(colored_line)
    }

    // CORRECCIÓN E0050: Se añadió el parámetro `_forced: bool` que faltaba
    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        true 
    }
}

impl Validator for CalcHelper {}

// CORRECCIÓN E0119: Se eliminó el bloque duplicado de `impl Completer` que había aquí abajo.
// Solo queda esta implementación (la versión mejorada):

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
        // Sugerir Comandos
        for &c in COMMANDS {
            if c.starts_with(prefix) {
                out.push(Pair { display: format!("{} (cmd)", c), replacement: c.to_string() });
            }
        }
        // Sugerir Funciones
        for &f in FUNCS {
            if f.starts_with(prefix) {
                out.push(Pair { display: format!("{f}()"), replacement: format!("{f}(") }); // Mejora UX: añade '('
            }
        }
        // Sugerir Variables
        for v in &self.vars {
            if v.starts_with(prefix) {
                out.push(Pair { display: format!("{} (var)", v), replacement: v.clone() });
            }
        }
        Ok((start, out))
    }
}

pub fn run() {
    let mut calc = Calculator::new();
    // Importante: Habilitar el comportamiento de rustyline con colores
    let config = rustyline::Config::builder()
        .auto_add_history(true)
        .build();

    let mut rl: Editor<CalcHelper, rustyline::history::DefaultHistory> = Editor::with_config(config).expect("rustyline editor");

    rl.set_helper(Some(CalcHelper::new()));
    let _ = rl.load_history(CMD_HISTORY_FILE);

    println!("Calculadora Avanzada (Complejos) en Rust. Escribe 'help'. (Ctrl+D para salir)");

    loop {
        // Actualizamos las variables conocidas en el helper para el coloreado y autocompletado
        if let Some(h) = rl.helper_mut() {
            h.update_vars(calc.vars.keys().cloned());
        }

        let mode_str = if calc.is_radians { "RAD" } else { "DEG" };
        let prompt = format!("[{}] >> ", mode_str).bold().to_string(); // Prompt en negrita

        let line = match rl.readline(&prompt) {
            Ok(s) => s,
            Err(ReadlineError::Interrupted) => { println!("^C"); continue; },
            Err(ReadlineError::Eof) => break,
            Err(e) => { eprintln!("Error: {e}"); break; }
        };

        let raw0 = line.trim();
        if raw0.is_empty() { continue; }
        rl.add_history_entry(Cow::Borrowed(raw0)).ok();

        // Lógica de historial (!! y !N)
        let line_to_eval = if raw0 == "!!" {
            match load_history_expr(&calc.history_file, HistoryPick::Last) {
                Ok(expr) => { println!("(hist !!) {}", expr.dimmed()); expr } // Dimmed para feedback visual
                Err(e) => { println!("Error: {}", e); continue; }
            }
        } else if raw0.starts_with('!') && raw0.len() > 1 && raw0[1..].chars().all(|c: char| c.is_ascii_digit()) {
            let n: usize = raw0[1..].parse().unwrap_or(0);
            match load_history_expr(&calc.history_file, HistoryPick::Index1(n)) {
                Ok(expr) => { println!("(hist !{}) {}", n, expr.dimmed()); expr }
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

            s if s.starts_with("integ") => {
                // Sintaxis: integ <expr> <min> <max> <pasos>
                let args: Vec<&str> = s[6..].split_whitespace().collect();
    
                if args.len() < 3 {
                    println!("Uso: integ <expr> <min> <max> [pasos]");
                    println!("Ejemplo: integ x^2 0 1 1000");
                } else {
                    let expr = args[0];
                    let min_res = calc.evaluate(args[1]);
                    let max_res = calc.evaluate(args[2]);
        
                    // Parseo seguro de los límites
                    if let (Ok(min_c), Ok(max_c)) = (min_res, max_res) {
                        let a = min_c.re;
                        let b = max_c.re;
            
                        // Pasos por defecto: 1000 si no se especifica
                        let n = if args.len() >= 4 {
                            args[3].parse::<usize>().unwrap_or(1000)
                        } else {
                            1000
                        };

                        if n == 0 {
                            println!("Error: El número de pasos debe ser > 0");
                        } else {
                            let h = (b - a) / (n as f64); // Delta x
                            let mut sum = 0.0;
                
                        // Guardamos la variable 'x' del usuario si existe
                        let saved_x = calc.vars.get("x").copied();
                        let mut error = false;

                        // --- Regla del Trapecio ---
                        // f(a)
                        calc.vars.insert("x".to_string(), num_complex::Complex64::new(a, 0.0));
                        match calc.evaluate(expr) {
                            Ok(y) => sum += y.re, // Primer término
                            Err(e) => { println!("Error evaluando en {}: {}", a, e); error = true; }
                        }

                        // Suma intermedia: 2 * sum(f(x_i))
                        if !error {
                            for i in 1..n {
                                let x = a + (i as f64) * h;
                                calc.vars.insert("x".to_string(), num_complex::Complex64::new(x, 0.0));
                                match calc.evaluate(expr) {
                                    Ok(y) => sum += 2.0 * y.re,
                                    Err(_) => { error = true; break; } // Ignoramos error individual en bucle grande, o abortamos
                                }
                            }
                        }

                        // f(b)
                        if !error {
                            calc.vars.insert("x".to_string(), num_complex::Complex64::new(b, 0.0));
                            match calc.evaluate(expr) {
                                    Ok(y) => sum += y.re, // Último término
                                    Err(e) => { println!("Error evaluando en {}: {}", b, e); error = true; }
                            }
                        }

                        // Restauramos 'x'
                        if let Some(old_val) = saved_x {
                            calc.vars.insert("x".to_string(), old_val);
                        } else {
                            calc.vars.remove("x");
                        }

                        if !error {
                            let integral = (h / 2.0) * sum;
                            calc.last_result = num_complex::Complex64::new(integral, 0.0);
                            println!("Integral definida de '{}' entre {} y {} ({}, n={})", expr, a, b, integral, n);
                            println!("= {}", integral);
                        }
                }
            } else {
                println!("Error: Los límites de integración deben ser números válidos.");
            }
        }
    }

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
                let parts: Vec<&str> = s.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let var_name = parts[0].trim();
                    let expr = parts[1].trim();

                    // Validación manual básica (sin regex para no añadir deps pesadas x ahora)
                    let is_valid_name = !var_name.is_empty()
                        && var_name.chars().next().unwrap().is_ascii_alphabetic()
                        && var_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');

                    // Lista negra de constantes protegidas
                    let reserved = ["pi", "e", "tau", "phi", "c", "i", "ans", "last"];

                    if !is_valid_name {
                        println!("Error: '{}' no es un nombre de variable válido.", var_name);
                    } else if reserved.contains(&var_name) {
                        println!("Error: '{}' es una constante reservada.", var_name);
                    } else {
                        // Proceder a evaluar
                        match calc.evaluate(expr) {
                            Ok(r) => {
                                calc.vars.insert(var_name.to_string(), r);
                                println!("{} = {}", var_name, r);
                            }
                            Err(e) => println!("Error al asignar: {}", e),
                        }
                    }
                } else {
                    println!("Error de sintaxis en asignación.");
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
