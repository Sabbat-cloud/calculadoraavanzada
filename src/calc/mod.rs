pub mod eval;
pub mod lexer;
pub mod plot;
pub mod token;

use std::collections::HashMap;
use num_complex::Complex64;

#[derive(Debug)]
pub struct Calculator {
    pub memory_stack: Vec<Complex64>, // Cambio a Complex
    pub vars: HashMap<String, Complex64>, // Cambio a Complex
    pub history_file: String,
    pub is_radians: bool,
    pub last_result: Complex64, // Cambio a Complex
}

impl Calculator {
    pub fn new() -> Self {
        let mut calc = Self {
            memory_stack: Vec::new(),
            vars: HashMap::new(),
            history_file: "historial.txt".to_string(),
            is_radians: false,
            last_result: Complex64::new(0.0, 0.0),
        };
        calc.reset();
        calc
    }

    pub fn reset(&mut self) {
        self.memory_stack.clear();
        self.vars.clear();

        // Constantes (se insertan como complejos con parte imaginaria 0)
        self.vars.insert("pi".to_string(), Complex64::new(std::f64::consts::PI, 0.0));
        self.vars.insert("e".to_string(), Complex64::new(std::f64::consts::E, 0.0));
        self.vars.insert("tau".to_string(), Complex64::new(2.0 * std::f64::consts::PI, 0.0));
        self.vars.insert("i".to_string(), Complex64::new(0.0, 1.0)); // Nueva constante i

        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
        self.vars.insert("phi".to_string(), Complex64::new(phi, 0.0));
        self.vars.insert("c".to_string(), Complex64::new(299_792_458.0, 0.0));

        self.last_result = Complex64::new(0.0, 0.0);
    }

    pub(crate) fn parse_number_str(&self, s: &str) -> Result<Complex64, String> {
        s.parse::<f64>()
            .map(|n| Complex64::new(n, 0.0))
            .map_err(|_| format!("Número inválido: {s}"))
    }
}
