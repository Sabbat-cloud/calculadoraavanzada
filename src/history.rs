#[derive(Debug, Clone, Copy)]
pub enum HistoryPick {
    Last,
    Index1(usize),
}

/// Devuelve solo la expresión (sin "= resultado") desde el historial.
///
/// Formato típico por línea: "<expr> = <res>".
pub fn load_history_expr(path: &str, which: HistoryPick) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("No se pudo leer {}: {}", path, e))?;
    let text = String::from_utf8_lossy(&bytes);
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return Err("Historial vacío.".to_string());
    }

    let line = match which {
        HistoryPick::Last => *lines.last().unwrap(),
        HistoryPick::Index1(i1) => {
            let idx = i1.saturating_sub(1);
            *lines
                .get(idx)
                .ok_or_else(|| format!("No existe la línea {} en el historial.", i1))?
        }
    };

    let expr = line.split(" = ").next().unwrap_or(line).trim().to_string();
    if expr.is_empty() {
        Err("Línea de historial inválida.".to_string())
    } else {
        Ok(expr)
    }
}

