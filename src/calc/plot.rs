use super::Calculator;

impl Calculator {
    /// plot <exprs> [xmin xmax] [ymin ymax]
    pub fn plot(&mut self, input: &str) {
        let saved_last = self.last_result;

        let mut parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Uso: plot <exprs> [xmin xmax] [ymin ymax]");
            return;
        }

        let mut x_min = -10.0;
        let mut x_max = 10.0;
        let mut y_min_opt: Option<f64> = None;
        let mut y_max_opt: Option<f64> = None;

        // 4 nums => xmin xmax ymin ymax
        if parts.len() >= 4 {
            let a = parts[parts.len() - 4].parse::<f64>();
            let b = parts[parts.len() - 3].parse::<f64>();
            let c = parts[parts.len() - 2].parse::<f64>();
            let d = parts[parts.len() - 1].parse::<f64>();
            if let (Ok(x1), Ok(x2), Ok(y1), Ok(y2)) = (a, b, c, d) {
                x_min = x1;
                x_max = x2;
                y_min_opt = Some(y1);
                y_max_opt = Some(y2);
                parts.truncate(parts.len() - 4);
            }
        }

        // 2 nums => xmin xmax
        if y_min_opt.is_none() && parts.len() >= 2 {
            let a = parts[parts.len() - 2].parse::<f64>();
            let b = parts[parts.len() - 1].parse::<f64>();
            if let (Ok(x1), Ok(x2)) = (a, b) {
                x_min = x1;
                x_max = x2;
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
            println!("No hay expresiones para plot.");
            self.last_result = saved_last;
            return;
        }

        if x_max == x_min {
            println!("Rango X inválido (xmin == xmax).");
            self.last_result = saved_last;
            return;
        }

        let width = 80usize;
        let height = 24usize;
        let mut grid = vec![vec![' '; width]; height];
        let symbols = ['•', 'x', '*', '+', 'o', '#', '@'];

        // auto-escala Y si no viene fija
        let (y_min, y_max) = if let (Some(y1), Some(y2)) = (y_min_opt, y_max_opt) {
            if y2 == y1 {
                println!("Rango Y inválido (ymin == ymax).");
                self.last_result = saved_last;
                return;
            }
            (y1.min(y2), y1.max(y2))
        } else {
            let mut y_values = Vec::new();
            for &expr in &exprs {
                for w in 0..width {
                    let x = x_min + (w as f64 / (width - 1) as f64) * (x_max - x_min);
                    self.vars.insert("x".to_string(), x);
                    if let Ok(y) = self.evaluate(expr) {
                        if y.is_finite() {
                            y_values.push(y);
                        }
                    }
                }
            }
            if y_values.is_empty() {
                println!("No se pudo graficar (sin valores finitos).");
                self.last_result = saved_last;
                return;
            }
            let y_min = *y_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let y_max = *y_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            if y_max == y_min {
                (y_min - 1.0, y_max + 1.0)
            } else {
                (y_min, y_max)
            }
        };

        let y_range = y_max - y_min;

        // ejes
        let zero_row =
            (height as f64 - ((0.0 - y_min) / y_range * (height - 1) as f64)) as i32;
        let zero_col = (width as f64 * (0.0 - x_min) / (x_max - x_min)) as i32;

        for r in 0..height {
            for c in 0..width {
                if r as i32 == zero_row {
                    grid[r][c] = '-';
                }
                if c as i32 == zero_col {
                    grid[r][c] = if grid[r][c] == '-' { '+' } else { '|' };
                }
            }
        }

        // funciones
        for (idx, &expr) in exprs.iter().enumerate() {
            let sym = symbols[idx % symbols.len()];
            for w in 0..width {
                let x = x_min + (w as f64 / (width - 1) as f64) * (x_max - x_min);
                self.vars.insert("x".to_string(), x);
                if let Ok(y) = self.evaluate(expr) {
                    if !y.is_finite() {
                        continue;
                    }
                    let row = ((y - y_min) / y_range * (height - 1) as f64) as i32;
                    let r = height as i32 - 1 - row;
                    if r >= 0 && r < height as i32 {
                        grid[r as usize][w] = sym;
                    }
                }
            }
        }

        println!(
            "\nPlot: X=[{:.3}, {:.3}] Y=[{:.3}, {:.3}]",
            x_min, x_max, y_min, y_max
        );
        println!(" +{}", "-".repeat(width));
        for row in grid {
            println!(" |{}|", row.iter().collect::<String>());
        }
        println!(" +{}", "-".repeat(width));

        self.last_result = saved_last;
    }
}

