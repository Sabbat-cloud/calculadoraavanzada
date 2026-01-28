pub mod eval;
pub mod lexer;
pub mod plot;
pub mod token;

use std::collections::HashMap;
use num_complex::Complex64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Decimal,
    Scientific,
}

pub struct Calculator {
    pub memory_stack: Vec<Complex64>, // Cambio a Complex
    pub vars: HashMap<String, Complex64>, // Cambio a Complex
    pub history_file: String,
    pub is_radians: bool,
    pub last_result: Complex64, // Cambio a Complex
    pub output_format: OutputFormat,
}

impl Calculator {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert("pi".to_string(), Complex64::new(std::f64::consts::PI, 0.0));
        vars.insert("e".to_string(), Complex64::new(std::f64::consts::E, 0.0));
        vars.insert("tau".to_string(), Complex64::new(std::f64::consts::TAU, 0.0));
        vars.insert("phi".to_string(), Complex64::new(1.618033988749895, 0.0));
        vars.insert("golden".to_string(), Complex64::new(1.618033988749895, 0.0));
        vars.insert("c".to_string(), Complex64::new(299792458.0, 0.0));
        vars.insert("i".to_string(), Complex64::new(0.0, 1.0));

        Calculator {
            memory_stack: Vec::new(),
            vars,
            history_file: "historial.txt".to_string(),
            is_radians: true, // Por defecto en Radianes
            last_result: Complex64::new(0.0, 0.0),
            // 3. Inicializamos en Decimal por defecto
            output_format: OutputFormat::Decimal, 
        }
    }

    pub fn reset(&mut self) {
        self.memory_stack.clear();
        self.vars.clear();
        // Re-inicializar constantes
        self.vars.insert("pi".to_string(), Complex64::new(std::f64::consts::PI, 0.0));
        self.vars.insert("e".to_string(), Complex64::new(std::f64::consts::E, 0.0));
        self.vars.insert("tau".to_string(), Complex64::new(std::f64::consts::TAU, 0.0));
        self.vars.insert("phi".to_string(), Complex64::new(1.618033988749895, 0.0));
        self.vars.insert("golden".to_string(), Complex64::new(1.618033988749895, 0.0));
        self.vars.insert("c".to_string(), Complex64::new(299792458.0, 0.0));
        self.vars.insert("i".to_string(), Complex64::new(0.0, 1.0));
        
        self.last_result = Complex64::new(0.0, 0.0);
        self.is_radians = true;
        self.output_format = OutputFormat::Decimal; // Reset también el formato
    }
    
    pub(crate) fn parse_number_str(&self, s: &str) -> Result<Complex64, String> {
        s.parse::<f64>()
            .map(|n| Complex64::new(n, 0.0))
            .map_err(|_| format!("Número inválido: {s}"))
    }
}
