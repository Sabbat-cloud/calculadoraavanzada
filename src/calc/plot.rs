use super::Calculator;
use num_complex::Complex64;
use std::char;
use std::fs::File;
use std::io::Write;

impl Calculator {
    /// Renderizado Braille en terminal y guardado de comando para exportación
    pub fn plot(&mut self, input: &str) {
        // 1. Guardar el comando para poder exportarlo después
        self.last_plot_cmd = input.to_string();

        let saved_last = self.last_result;
        let saved_x = self.vars.get("x").copied();

        let mut parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Uso: plot <exprs> [xmin xmax] [ymin ymax] [width height]");
            return;
        }

        // --- Parsing de Argumentos (igual que antes) ---
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

        // --- Configuración Braille ---
        let pixel_width = width * 2;
        let pixel_height = height * 4;

        // --- Auto-escala Y ---
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

        // --- Renderizado Grid Braille ---
        let mut grid = vec![vec![0u8; width]; height];

        // Closure sin 'mut' (corregido warning anterior)
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

        // Imprimir Resultado
        println!("\nPlot (Braille): {:?} [X: {:.2} a {:.2}, Y: {:.2} a {:.2}]", exprs, x_min, x_max, y_min, y_max);
        println!("       ┌{}┐", "─".repeat(width));
        for (i, row) in grid.iter().enumerate() {
            let mut line = String::with_capacity(width);
            for &mask in row {
                let c = char::from_u32(0x2800 + mask as u32).unwrap_or(' ');
                line.push(c);
            }
            let label = if i == 0 { format!("{:>6.2} ", y_max) }
                       else if i == height - 1 { format!("{:>6.2} ", y_min) }
                       else if i == height / 2 { format!("{:>6.2} ", (y_max + y_min)/2.0) }
                       else { "       ".to_string() };
            println!("{}│{}│", label, line);
        }
        println!("       └{}┘", "─".repeat(width));
        println!("       {:<width$}{:.2}", format!("{:.2}", x_min), x_max, width=width - format!("{:.2}", x_max).len());

        self.last_result = saved_last;
        self.restore_x(saved_x);
    }

    /// Genera un archivo SVG con la última gráfica realizada
    pub fn export_svg(&mut self, filename: &str) -> Result<String, String> {
        if self.last_plot_cmd.is_empty() {
            return Err("No hay gráfico anterior para exportar.".to_string());
        }

        // Re-parseamos el comando guardado (duplicación necesaria para no refactorizar todo el parser ahora)
        let input = self.last_plot_cmd.clone();
        let mut parts: Vec<&str> = input.split_whitespace().collect();
        
        // Variables por defecto para SVG
        let mut x_min = -10.0;
        let mut x_max = 10.0;
        let mut y_min_opt: Option<f64> = None;
        let mut y_max_opt: Option<f64> = None;
        
        // Ignoramos width/height de terminal, usaremos resolución fija para SVG (800x600)
        let svg_w = 800.0;
        let svg_h = 600.0;
        let margin = 40.0;

        // Limpieza de argumentos (igual que en plot)
        if parts.len() >= 2 {
            // Intentar consumir dimensiones terminal (las ignoramos para SVG pero hay que sacarlas del vec)
            if let (Ok(_), Ok(_)) = (parts[parts.len()-2].parse::<usize>(), parts[parts.len()-1].parse::<usize>()) {
                parts.truncate(parts.len() - 2);
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
        let exprs: Vec<&str> = exprs_str.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

        // Recalcular rango Y (necesario si era automático)
        let saved_x = self.vars.get("x").copied();
        let steps = 1000; // Resolución interna para calcular min/max y trazar líneas
        let (y_min, y_max) = if let (Some(y1), Some(y2)) = (y_min_opt, y_max_opt) {
            (y1.min(y2), y1.max(y2))
        } else {
            let mut all_y = Vec::new();
            for &expr in &exprs {
                for i in 0..=steps {
                    let t = i as f64 / steps as f64;
                    let x = x_min + t * (x_max - x_min);
                    self.vars.insert("x".to_string(), Complex64::new(x, 0.0));
                    if let Ok(res) = self.evaluate(expr) {
                        if res.re.is_finite() { all_y.push(res.re); }
                    }
                }
            }
            if all_y.is_empty() {
                self.restore_x(saved_x);
                return Err("Rango vacío".to_string());
            }
            let min = all_y.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = all_y.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            if (max - min).abs() < 1e-9 { (min - 1.0, max + 1.0) } else { (min, max) }
        };

        // --- Generar SVG ---
        let mut svg_content = String::new();
        svg_content.push_str(&format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, svg_w, svg_h));
        svg_content.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);
        
        // Estilos
        let colors = ["#e74c3c", "#3498db", "#2ecc71", "#9b59b6", "#f1c40f", "#34495e"];
        
        // Transformada de coordenadas (Mundo -> Píxel SVG)
        // SVG Y va hacia abajo, Matemático Y va hacia arriba
        let map_x = |x: f64| -> f64 { margin + (x - x_min) / (x_max - x_min) * (svg_w - 2.0*margin) };
        let map_y = |y: f64| -> f64 { svg_h - margin - (y - y_min) / (y_max - y_min) * (svg_h - 2.0*margin) };

        // Dibujar Ejes
        let y0_px = map_y(0.0);
        let x0_px = map_x(0.0);
        
        // Eje X (si está visible)
        if y_min <= 0.0 && y_max >= 0.0 {
            svg_content.push_str(&format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1"/>"#, 
                margin, y0_px, svg_w - margin, y0_px));
        }
        // Eje Y (si está visible)
        if x_min <= 0.0 && x_max >= 0.0 {
            svg_content.push_str(&format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1"/>"#, 
                x0_px, margin, x0_px, svg_h - margin));
        }

        // Dibujar Curvas
        for (idx, &expr) in exprs.iter().enumerate() {
            let color = colors[idx % colors.len()];
            let mut points_str = String::new();
            
            for i in 0..=steps {
                let t = i as f64 / steps as f64;
                let x = x_min + t * (x_max - x_min);
                self.vars.insert("x".to_string(), Complex64::new(x, 0.0));
                
                if let Ok(res) = self.evaluate(expr) {
                    let y = res.re;
                    // Solo dibujar si está (aprox) dentro del rango visual (con un margen de seguridad)
                    // para evitar líneas locas que cruzan todo el gráfico en asíntotas
                    if y.is_finite() && y >= y_min - (y_max-y_min)*2.0 && y <= y_max + (y_max-y_min)*2.0 {
                        let px = map_x(x);
                        let py = map_y(y);
                        points_str.push_str(&format!("{},{} ", px, py));
                    }
                }
            }
            
            svg_content.push_str(&format!(r#"<polyline points="{}" fill="none" stroke="{}" stroke-width="2"/>"#, points_str.trim(), color));
        }

        // Marco y Texto
        svg_content.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="gray"/>"#, 
            margin, margin, svg_w - 2.0*margin, svg_h - 2.0*margin));
        
        // Etiquetas simples de esquinas
        svg_content.push_str(&format!(r#"<text x="{}" y="{}" font-family="Arial" font-size="12">({:.2}, {:.2})</text>"#, margin, svg_h - margin + 15.0, x_min, y_min));
        svg_content.push_str(&format!(r#"<text x="{}" y="{}" font-family="Arial" font-size="12" text-anchor="end">({:.2}, {:.2})</text>"#, svg_w - margin, margin - 5.0, x_max, y_max));

        svg_content.push_str("</svg>");

        self.restore_x(saved_x);

        // Guardar archivo
        let mut file = File::create(filename).map_err(|e| e.to_string())?;
        file.write_all(svg_content.as_bytes()).map_err(|e| e.to_string())?;
        
        Ok(format!("Gráfico guardado en '{}' (800x600)", filename))
    }

    fn restore_x(&mut self, saved: Option<Complex64>) {
        if let Some(old) = saved {
            self.vars.insert("x".to_string(), old);
        } else {
            self.vars.remove("x");
        }
    }
}
