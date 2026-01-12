use super::{token::Token, Calculator};

use crate::math_ext::{comb_u128, is_prime_u128, next_prime_u128, perm_u128};
use num_integer::gcd as num_gcd;
use num_integer::lcm as num_lcm;

fn push_checked(vals: &mut Vec<f64>, v: f64) -> Result<(), String> {
    if !v.is_finite() {
        Err("Resultado no finito (NaN/Inf). Revisa dominio/división por 0.".to_string())
    } else {
        vals.push(v);
        Ok(())
    }
}

fn safe_i64_for_gcd(x: f64) -> Result<i64, String> {
    if !x.is_finite() {
        return Err("mcd/mcm requiere números finitos".to_string());
    }
    let xr = x.round();
    let max = i64::MAX as f64;
    if xr > max || xr < -max {
        return Err("mcd/mcm fuera de rango para i64".to_string());
    }
    Ok(xr as i64)
}

fn apply_func(vals: &mut Vec<f64>, func: &str, is_rad: bool) -> Result<(), String> {
    let b = vals.pop().ok_or("Faltan argumentos")?;

    match func {
        "sin" => push_checked(vals, if is_rad { b.sin() } else { b.to_radians().sin() })?,
        "cos" => push_checked(vals, if is_rad { b.cos() } else { b.to_radians().cos() })?,
        "tan" => push_checked(vals, if is_rad { b.tan() } else { b.to_radians().tan() })?,
        "asin" => {
            if !((-1.0..=1.0).contains(&b)) {
                return Err("asin(x) requiere x en [-1, 1]".to_string());
            }
            let v = if is_rad { b.asin() } else { b.asin().to_degrees() };
            push_checked(vals, v)?;
        }
        "acos" => {
            if !((-1.0..=1.0).contains(&b)) {
                return Err("acos(x) requiere x en [-1, 1]".to_string());
            }
            let v = if is_rad { b.acos() } else { b.acos().to_degrees() };
            push_checked(vals, v)?;
        }
        "atan" => {
            let v = if is_rad { b.atan() } else { b.atan().to_degrees() };
            push_checked(vals, v)?;
        }

        "atan2" => {
            let a = vals.pop().ok_or("atan2 requiere 2 argumentos: atan2(y, x)")?;
            let v = a.atan2(b);
            let v = if is_rad { v } else { v.to_degrees() };
            push_checked(vals, v)?;
        }
        "hypot" => {
            let a = vals.pop().ok_or("hypot requiere 2 argumentos: hypot(x, y)")?;
            push_checked(vals, a.hypot(b))?;
        }

        "sqrt" => {
            if b < 0.0 {
                return Err("sqrt(x) requiere x >= 0".to_string());
            }
            push_checked(vals, b.sqrt())?;
        }
        "cbrt" => {
            let v = if b == 0.0 { 0.0 } else { b.signum() * b.abs().powf(1.0 / 3.0) };
            push_checked(vals, v)?;
        }
        "root" => {
            let a = vals.pop().ok_or("root requiere 2 argumentos: root(n, x)")?;
            let ni = a.round();
            if !a.is_finite() || (a - ni).abs() > 1e-12 {
                return Err("root(n, x): n debe ser entero".to_string());
            }
            let ni64 = ni as i64;
            if ni64 == 0 {
                return Err("root(n, x): n no puede ser 0".to_string());
            }
            if b < 0.0 && (ni64.abs() % 2 == 0) {
                return Err("root(n, x): raíz par de negativo no es real".to_string());
            }
            let inv = 1.0 / (ni64 as f64);
            let mag = b.abs().powf(inv);
            let v = if b < 0.0 { -mag } else { mag };
            push_checked(vals, v)?;
        }
        "ln" => {
            if b <= 0.0 {
                return Err("ln(x) requiere x > 0".to_string());
            }
            push_checked(vals, b.ln())?;
        }
        "log" => {
            let a = vals.pop().ok_or("log requiere 2 argumentos: log(base, n)")?;
            let base = a;
            let n = b;
            if base <= 0.0 || base == 1.0 {
                return Err("log(base, n) requiere base > 0 y base != 1".to_string());
            }
            if n <= 0.0 {
                return Err("log(base, n) requiere n > 0".to_string());
            }
            push_checked(vals, n.log(base))?;
        }

        "sinh" => push_checked(vals, b.sinh())?,
        "cosh" => push_checked(vals, b.cosh())?,
        "tanh" => push_checked(vals, b.tanh())?,
        "asinh" => push_checked(vals, b.asinh())?,
        "acosh" => {
            if b < 1.0 {
                return Err("acosh(x) requiere x >= 1".to_string());
            }
            push_checked(vals, b.acosh())?;
        }
        "atanh" => {
            if b <= -1.0 || b >= 1.0 {
                return Err("atanh(x) requiere |x| < 1".to_string());
            }
            push_checked(vals, b.atanh())?;
        }

        "floor" => push_checked(vals, b.floor())?,
        "ceil" => push_checked(vals, b.ceil())?,
        "abs" => push_checked(vals, b.abs())?,
        "round" => push_checked(vals, b.round())?,
        "trunc" => push_checked(vals, b.trunc())?,
        "sign" => {
            let v = if b > 0.0 { 1.0 } else if b < 0.0 { -1.0 } else { 0.0 };
            push_checked(vals, v)?;
        }

        "deg2rad" => push_checked(vals, b.to_radians())?,
        "rad2deg" => push_checked(vals, b.to_degrees())?,
        "cm2in" => push_checked(vals, b / 2.54)?,
        "in2cm" => push_checked(vals, b * 2.54)?,
        "m2ft" => push_checked(vals, b * 3.280_839_895_013_123)?,
        "ft2m" => push_checked(vals, b / 3.280_839_895_013_123)?,

        "fact" => {
            if !b.is_finite() {
                return Err("fact(n) requiere n finito".to_string());
            }
            let r = b.round();
            if (b - r).abs() > 1e-12 {
                return Err("fact(n) requiere n entero".to_string());
            }
            if r < 0.0 {
                return Err("fact(n) requiere n >= 0".to_string());
            }
            let n = r as u64;

            let mut acc = 1.0f64;
            for i in 2..=n {
                acc *= i as f64;
                if !acc.is_finite() {
                    return Err("fact overflow (demasiado grande para f64)".to_string());
                }
            }
            push_checked(vals, acc)?;
        }
        "comb" | "nCr" => {
            let a = vals.pop().ok_or("comb requiere 2 argumentos: comb(n, k)")?;
            let n = a.round();
            let k = b.round();
            if !a.is_finite() || (a - n).abs() > 1e-12 || n < 0.0 {
                return Err("comb(n,k): n debe ser entero >= 0".to_string());
            }
            if !b.is_finite() || (b - k).abs() > 1e-12 || k < 0.0 {
                return Err("comb(n,k): k debe ser entero >= 0".to_string());
            }
            let res_u = comb_u128(n as u64, k as u64)?;
            push_checked(vals, res_u as f64)?;
        }
        "perm" | "nPr" => {
            let a = vals.pop().ok_or("perm requiere 2 argumentos: perm(n, k)")?;
            let n = a.round();
            let k = b.round();
            if !a.is_finite() || (a - n).abs() > 1e-12 || n < 0.0 {
                return Err("perm(n,k): n debe ser entero >= 0".to_string());
            }
            if !b.is_finite() || (b - k).abs() > 1e-12 || k < 0.0 {
                return Err("perm(n,k): k debe ser entero >= 0".to_string());
            }
            let res_u = perm_u128(n as u64, k as u64)?;
            push_checked(vals, res_u as f64)?;
        }

        "mcd" => {
            let a = vals.pop().ok_or("mcd requiere 2 argumentos")?;
            let ai = safe_i64_for_gcd(a)?;
            let bi = safe_i64_for_gcd(b)?;
            push_checked(vals, num_gcd(ai, bi) as f64)?;
        }
        "mcm" => {
            let a = vals.pop().ok_or("mcm requiere 2 argumentos")?;
            let ai = safe_i64_for_gcd(a)?;
            let bi = safe_i64_for_gcd(b)?;
            push_checked(vals, num_lcm(ai, bi) as f64)?;
        }

        "exp" => push_checked(vals, b.exp())?,
        "log10" => {
            if b <= 0.0 {
                return Err("log10(x) requiere x > 0".to_string());
            }
            push_checked(vals, b.log10())?
        }
        "log2" => {
            if b <= 0.0 {
                return Err("log2(x) requiere x > 0".to_string());
            }
            push_checked(vals, b.log2())?
        }
        "pow" => {
            let a = vals.pop().ok_or("pow requiere 2 argumentos: pow(base, exp)")?;
            push_checked(vals, a.powf(b))?
        }
        "min" => {
            let a = vals.pop().ok_or("min requiere 2 argumentos")?;
            push_checked(vals, a.min(b))?
        }
        "max" => {
            let a = vals.pop().ok_or("max requiere 2 argumentos")?;
            push_checked(vals, a.max(b))?
        }
        "mod" => {
            let a = vals.pop().ok_or("mod requiere 2 argumentos")?;
            if b == 0.0 {
                return Err("Módulo por cero".to_string());
            }
            push_checked(vals, ((a % b) + b) % b)?
        }

        "isprime" => {
            let n = b.round();
            if n < 0.0 || !n.is_finite() {
                push_checked(vals, 0.0)?;
            } else {
                push_checked(vals, if is_prime_u128(n as u128) { 1.0 } else { 0.0 })?;
            }
        }
        "nextprime" => {
            let n = b.round();
            if n < 0.0 || !n.is_finite() {
                return Err("next_prime requiere un número positivo".to_string());
            }
            push_checked(vals, next_prime_u128(n as u128) as f64)?;
        }

        "rand" => {
            let min = vals.pop().ok_or("rand requiere 2 argumentos: rand(min, max)")?;
            let max = b;
            if min >= max {
                return Err("rand(min, max): min debe ser menor que max".to_string());
            }

            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();

            let factor = ((nanos % 1_000_000_007) as f64) / 1_000_000_007.0;
            push_checked(vals, min + factor * (max - min))?
        }

        "pct" => {
            let total = b;
            let parte = vals.pop().ok_or("pct requiere 2 argumentos: pct(parte, total)")?;
            if total == 0.0 {
                return Err("División por cero en pct".to_string());
            }
            push_checked(vals, (parte / total) * 100.0)?
        }
        "applypct" => {
            let valor = b;
            let porcentaje = vals
                .pop()
                .ok_or("applypct requiere 2 argumentos: applypct(%, valor)")?;
            push_checked(vals, (porcentaje / 100.0) * valor)?
        }

        "r3d" => {
            let c = b;
            let b_val = vals.pop().ok_or("r3d requiere 3 argumentos (a, b, c)")?;
            let a = vals.pop().ok_or("r3d requiere 3 argumentos (a, b, c)")?;
            if a == 0.0 {
                return Err("División por cero en r3d".to_string());
            }
            push_checked(vals, (c * b_val) / a)?
        }
        "r3i" => {
            let c = b;
            let b_val = vals.pop().ok_or("r3i requiere 3 argumentos (a, b, c)")?;
            let a = vals.pop().ok_or("r3i requiere 3 argumentos (a, b, c)")?;
            if c == 0.0 {
                return Err("División por cero en r3i".to_string());
            }
            push_checked(vals, (a * b_val) / c)?
        }

        "+" => {
            let a = vals.pop().ok_or("Falta operando")?;
            push_checked(vals, a + b)?;
        }
        "-" => {
            let a = vals.pop().ok_or("Falta operando")?;
            push_checked(vals, a - b)?;
        }
        "*" => {
            let a = vals.pop().ok_or("Falta operando")?;
            push_checked(vals, a * b)?;
        }
        "/" => {
            let a = vals.pop().ok_or("Falta operando")?;
            if b == 0.0 {
                return Err("División por cero".to_string());
            }
            push_checked(vals, a / b)?;
        }
        "^" => {
            let a = vals.pop().ok_or("Falta operando")?;
            push_checked(vals, a.powf(b))?;
        }
        "%" => {
            let a = vals.pop().ok_or("Falta operando")?;
            if b == 0.0 {
                return Err("Módulo por cero".to_string());
            }
            push_checked(vals, a % b)?;
        }

        _ => return Err(format!("Función/operador {} no implementado", func)),
    }

    Ok(())
}

impl Calculator {
    pub fn evaluate(&mut self, expr: &str) -> Result<f64, String> {
        let tokens = self.tokenize(expr)?;
        let mut values: Vec<f64> = Vec::new();
        let mut ops: Vec<String> = Vec::new();

        let prec = |op: &str| match op {
            "+" | "-" => (1, false),
            "*" | "/" | "%" => (2, false),
            "^" => (4, true),
            _ => (3, false),
        };

        for token in tokens {
            match token {
                Token::Number(n) => values.push(n),

                Token::Var(name) => {
                    if name == "last" || name == "ans" {
                        values.push(self.last_result);
                    } else {
                        values.push(
                            *self
                                .vars
                                .get(&name)
                                .ok_or(format!("Variable '{}' no existe", name))?,
                        );
                    }
                }

                Token::Func(f) => ops.push(f),
                Token::LParen => ops.push("(".to_string()),

                Token::Comma => {
                    while let Some(op) = ops.last() {
                        if op == "(" {
                            break;
                        }
                        let op = ops.pop().unwrap();
                        apply_func(&mut values, &op, self.is_radians)?;
                    }
                }

                Token::RParen => {
                    while let Some(op) = ops.pop() {
                        if op == "(" {
                            break;
                        }
                        apply_func(&mut values, &op, self.is_radians)?;
                    }
                }

                Token::Op(c) => {
                    let s = c.to_string();
                    let (p_curr, right_assoc) = prec(&s);

                    while let Some(top) = ops.last() {
                        if top == "(" {
                            break;
                        }
                        let (p_top, _) = prec(top);

                        if p_top > p_curr || (p_top == p_curr && !right_assoc) {
                            let op = ops.pop().unwrap();
                            apply_func(&mut values, &op, self.is_radians)?;
                        } else {
                            break;
                        }
                    }

                    ops.push(s);
                }
            }
        }

        while let Some(op) = ops.pop() {
            apply_func(&mut values, &op, self.is_radians)?;
        }

        let res = values.pop().ok_or("Error en expresión".to_string())?;
        if !res.is_finite() {
            return Err("Resultado no finito (NaN/Inf).".to_string());
        }
        self.last_result = res;
        Ok(res)
    }
}

