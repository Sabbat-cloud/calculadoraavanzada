pub mod eval;
pub mod lexer;
pub mod plot;
pub mod token;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Calculator {
    pub memory_stack: Vec<f64>,
    pub vars: HashMap<String, f64>,
    pub history_file: String,
    pub is_radians: bool,
    pub last_result: f64,
}

impl Calculator {
    pub fn new() -> Self {
        let mut calc = Self {
            memory_stack: Vec::new(),
            vars: HashMap::new(),
            history_file: "historial.txt".to_string(),
            is_radians: false,
            last_result: 0.0,
        };
        calc.reset();
        calc
    }

    pub fn reset(&mut self) {
        self.memory_stack.clear();
        self.vars.clear();

        // Constantes Matemáticas
        self.vars.insert("pi".to_string(), std::f64::consts::PI);
        self.vars.insert("e".to_string(), std::f64::consts::E);
        self.vars
            .insert("tau".to_string(), 2.0 * std::f64::consts::PI);

        // Número Áureo
        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
        self.vars.insert("golden".to_string(), phi);
        self.vars.insert("phi".to_string(), phi);

        // Constantes físicas
        self.vars.insert("c".to_string(), 299_792_458.0);

        self.last_result = 0.0;
    }

    pub(crate) fn parse_number_str(&self, s: &str) -> Result<f64, String> {
        s.parse::<f64>().map_err(|_| format!("Número inválido: {s}"))
    }
}

#[cfg(test)]
mod tests;

