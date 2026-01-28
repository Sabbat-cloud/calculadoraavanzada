use super::Calculator;
use num_complex::Complex64;
use std::char;

impl Calculator {
    /// plot <exprs> [xmin xmax] [ymin ymax] [width height]
    /// Renderizado de Alta Resolución usando caracteres Braille (2x4 puntos por caracter).
    pub fn plot(&mut self, input: &str) {
        let saved_last = self.last_result;
        let saved_x = self.vars.get("x").copied();

        let mut parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Uso: plot <exprs> [xmin xmax] [ymin ymax] [width height]");
            return;
        }

        // --- 1. Parsing de Argumentos ---
        let mut x_min = -10.0;
        let mut x_max = 10.0;
        let mut y_min_opt: Option<f64> = None;
        let mut y_max_opt: Option<f64> = None;
        let mut width = 80usize;
        let mut height = 24usize;

        if parts.len() >= 2 {
            if let (Ok(w), Ok(h)) = (parts[parts.len()-2].parse::<usize>(), parts[parts.len()-1].parse::<usize>()) {
                if w > 10 && h > 5 { 
                    width = w;
                    height = h;
                    parts.truncate(parts.len() - 2);
                }
            }
        }

        if parts.len() >= 2 {
            let y1 = parts[parts.len()-2].parse::<f64>();
            let y2 = parts[parts.len()-1].parse::<f64>();
            if let (Ok(val1), Ok(val2)) = (y1, y2) {
                y_min_opt = Some(val1);
                y_max_opt = Some(val2);
                parts.truncate(parts.len() - 2);
            }
        }

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
            println!("Error: No se especificaron expresiones.");
            return;
        }

        // --- 2. Configuración de Resolución Braille ---
        let pixel_width = width * 2;
        let pixel_height = height * 4;

        // --- 3. Auto-escala Y ---
        let (y_min, y_max) = if let (Some(y1), Some(y2)) = (y_min_opt, y_max_opt) {
            (y1.min(y2), y1.max(y2))
        } else {
            let mut all_y = Vec::new();
            for &expr in &exprs {
                for px in 0..pixel_width {
                    let t = px as f64 / (pixel_width - 1) as f64;
                    let x = x_min + t * (x_max - x_min);
                    self.vars.insert("x".to_string(), Complex64::new(x, 0.0));
                    
                    if let Ok(res) = self.evaluate(expr) {
                        if res.re.is_finite() { all_y.push(res.re); }
                    }
                }
            }
            if all_y.is_empty() {
                println!("Error: No hay valores reales en el rango.");
                self.restore_x(saved_x);
                return;
            }
            let min = all_y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = all_y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            if (max - min).abs() < 1e-9 { (min - 1.0, max + 1.0) } else { (min, max) }
        };

        let y_range = y_max - y_min;

        // --- 4. Renderizado a Grid ---
        let mut grid = vec![vec![0u8; width]; height];

        let set_pixel = |px: usize, py: usize, grid: &mut Vec<Vec<u8>>| {
            if px >= pixel_width || py >= pixel_height { return; }
            let col_char = px / 2;
            let row_char = (pixel_height - 1 - py) / 4; 

            if row_char >= height || col_char >= width { return; }

            let dx = px % 2;
            let dy = (pixel_height - 1 - py) % 4;

            let mask = match (dx, dy) {
                (0, 0) => 0x01, (1, 0) => 0x08,
                (0, 1) => 0x02, (1, 1) => 0x10,
                (0, 2) => 0x04, (1, 2) => 0x20,
                (0, 3) => 0x40, (1, 3) => 0x80,
                _ => 0,
            };
            grid[row_char][col_char] |= mask;
        };

        // Ejes
        let zero_y_ratio = (0.0 - y_min) / y_range;
        if zero_y_ratio >= 0.0 && zero_y_ratio <= 1.0 {
            let py_zero = (zero_y_ratio * (pixel_height - 1) as f64).round() as usize;
            for px in 0..pixel_width { set_pixel(px, py_zero, &mut grid); }
        }

        let zero_x_ratio = (0.0 - x_min) / (x_max - x_min);
        if zero_x_ratio >= 0.0 && zero_x_ratio <= 1.0 {
            let px_zero = (zero_x_ratio * (pixel_width - 1) as f64).round() as usize;
            for py in 0..pixel_height { set_pixel(px_zero, py, &mut grid); }
        }

        // Funciones
        for &expr in &exprs {
            for px in 0..pixel_width {
                let t = px as f64 / (pixel_width - 1) as f64;
                let x = x_min + t * (x_max - x_min);
                self.vars.insert("x".to_string(), Complex64::new(x, 0.0));

                if let Ok(res) = self.evaluate(expr) {
                    let y = res.re;
                    if y >= y_min && y <= y_max {
                        let y_ratio = (y - y_min) / y_range;
                        let py = (y_ratio * (pixel_height - 1) as f64).round() as usize;
                        set_pixel(px, py, &mut grid);
                    }
                }
            }
        }

        // --- 5. Imprimir Resultado ---
        println!("\nPlot (Braille): {:?} [X: {:.2} a {:.2}, Y: {:.2} a {:.2}]", exprs, x_min, x_max, y_min, y_max);
        
        // CORRECCIÓN: Indentación de 7 espacios para alinear con la etiqueta del eje Y
        println!("       ┌{}┐", "─".repeat(width));
        
        for (i, row) in grid.iter().enumerate() {
            let mut line = String::with_capacity(width);
            for &mask in row {
                let c = char::from_u32(0x2800 + mask as u32).unwrap_or(' ');
                line.push(c);
            }
            
            // Etiqueta eje Y a la izquierda (ancho fijo de 7 chars: 6 nums + 1 espacio)
            let label = if i == 0 { format!("{:>6.2} ", y_max) }
                       else if i == height - 1 { format!("{:>6.2} ", y_min) }
                       else if i == height / 2 { format!("{:>6.2} ", (y_max + y_min)/2.0) }
                       else { "       ".to_string() };
            
            println!("{}│{}│", label, line);
        }
        
        // CORRECCIÓN: Indentación de 7 espacios para el borde inferior
        println!("       └{}┘", "─".repeat(width));
        
        // Etiquetas eje X (ya tenían 7 espacios, se mantienen igual)
        println!("       {:<width$}{:.2}", format!("{:.2}", x_min), x_max, width=width - format!("{:.2}", x_max).len());

        self.last_result = saved_last;
        self.restore_x(saved_x);
    }

    fn restore_x(&mut self, saved: Option<Complex64>) {
        if let Some(old) = saved {
            self.vars.insert("x".to_string(), old);
        } else {
            self.vars.remove("x");
        }
    }
}
