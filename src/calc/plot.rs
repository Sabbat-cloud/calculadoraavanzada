use super::Calculator;
use num_complex::{Complex64}; // Importación necesaria para manejar complejos
//use num_complex::{Complex64, ComplexFloat}; // Importación necesaria para manejar complejos

impl Calculator {
    /// plot <exprs> [xmin xmax] [ymin ymax] [width height]
    /// El gráfico utiliza la parte REAL de los resultados complejos.
    pub fn plot(&mut self, input: &str) {
        let saved_last = self.last_result;

        let mut parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Uso: plot <exprs> [xmin xmax] [ymin ymax] [width height]");
            return;
        }

        // Valores por defecto
        let mut x_min = -10.0;
        let mut x_max = 10.0;
        let mut y_min_opt: Option<f64> = None;
        let mut y_max_opt: Option<f64> = None;
        let mut width = 80usize;
        let mut height = 24usize;

        // Intentar extraer dimensiones (width height) si hay suficientes argumentos
        if parts.len() >= 2 {
            if let (Ok(w), Ok(h)) = (parts[parts.len()-2].parse::<usize>(), parts[parts.len()-1].parse::<usize>()) {
                if w > 10 && h > 5 { 
                    width = w;
                    height = h;
                    parts.truncate(parts.len() - 2);
                }
            }
        }

        // Extraer rangos Y
        if parts.len() >= 2 {
            let y1 = parts[parts.len()-2].parse::<f64>();
            let y2 = parts[parts.len()-1].parse::<f64>();
            if let (Ok(val1), Ok(val2)) = (y1, y2) {
                y_min_opt = Some(val1);
                y_max_opt = Some(val2);
                parts.truncate(parts.len() - 2);
            }
        }

        // Extraer rangos X
        if parts.len() >= 2 {
            let x1 = parts[parts.len()-2].parse::<f64>();
            let x2 = parts[parts.len()-1].parse::<f64>();
            if let (Ok(val1), Ok(val2)) = (x1, x2) {
                x_min = val1;
                x_max = val2;
                parts.truncate(parts.len() - 2);
            }
        }

        let exprs_str = parts.join(" ");
        let exprs: Vec<&str> = exprs_str
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if exprs.is_empty() {
            println!("Error: No se especificaron expresiones para graficar.");
            return;
        }

        // --- Lógica de Escalado y Generación ---
        
        // Auto-escala Y basada en la parte REAL de la función
        let (y_min, y_max) = if let (Some(y1), Some(y2)) = (y_min_opt, y_max_opt) {
            (y1.min(y2), y1.max(y2))
        } else {
            let mut y_values = Vec::new();
            for &expr in &exprs {
                for w in 0..width {
                    let x = x_min + (w as f64 / (width - 1) as f64) * (x_max - x_min);
                    // Insertamos x como un número complejo (parte imaginaria 0)
                    self.vars.insert("x".to_string(), Complex64::new(x, 0.0));
                    if let Ok(y_comp) = self.evaluate(expr) {
                        let y = y_comp.re; // Solo graficamos la parte real
                        if y.is_finite() { y_values.push(y); }
                    }
                }
            }
            if y_values.is_empty() {
                println!("Error: No hay valores reales finitos en este rango.");
                return;
            }
            let min = *y_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let max = *y_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            if (max - min).abs() < 1e-9 { (min - 1.0, max + 1.0) } else { (min, max) }
        };

        let y_range = y_max - y_min;
        let mut grid = vec![vec![' '; width]; height];
        let symbols = ['•', 'x', '*', '+', 'o', '#', '@'];

        // Dibujar ejes
        let zero_row = (height as f64 - 1.0 - ((-y_min / y_range) * (height as f64 - 1.0))) as i32;
        let zero_col = ((-x_min / (x_max - x_min)) * (width as f64 - 1.0)) as i32;

        for r in 0..height {
            for c in 0..width {
                if r as i32 == zero_row { grid[r][c] = '-'; }
                if c as i32 == zero_col {
                    grid[r][c] = if grid[r][c] == '-' { '+' } else { '|' };
                }
            }
        }

        // Dibujar funciones
        for (idx, &expr) in exprs.iter().enumerate() {
            let sym = symbols[idx % symbols.len()];
            for w in 0..width {
                let x = x_min + (w as f64 / (width - 1) as f64) * (x_max - x_min);
                self.vars.insert("x".to_string(), Complex64::new(x, 0.0));
                if let Ok(y_comp) = self.evaluate(expr) {
                    let y = y_comp.re; // Eje Y representa la parte real
                    if y >= y_min && y <= y_max {
                        let row_f = (y - y_min) / y_range * (height as f64 - 1.0);
                        let r = (height as i32 - 1) - row_f.round() as i32;
                        if r >= 0 && r < height as i32 {
                            grid[r as usize][w] = sym;
                        }
                    }
                }
            }
        }

        // --- Renderizado con Etiquetas ---
        println!("\nPlot (Parte Real): {:?} ({}x{})", exprs, width, height);
        println!("{:>10} ┌{}┐", format!("{:.2}", y_max), "-".repeat(width));
        for (i, row) in grid.iter().enumerate() {
            let side_label = if i == height / 2 { format!("{:.2}", (y_max + y_min) / 2.0) } else { String::new() };
            println!("{:>10} |{}|", side_label, row.iter().collect::<String>());
        }
        println!("{:>10} └{}┘", format!("{:.2}", y_min), "-".repeat(width));
        println!("{:>11}{}{}", format!("{:.2}", x_min), " ".repeat(width.saturating_sub(10)), format!("{:.2}", x_max));

        self.last_result = saved_last;
        self.vars.remove("x"); 
    }
}

