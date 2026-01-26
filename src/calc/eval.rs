use super::{token::Token, Calculator};
use num_complex::{Complex64, ComplexFloat};
use crate::math_ext::{comb_u128, is_prime_u128, next_prime_u128, perm_u128};
use num_integer::gcd as num_gcd;
use num_integer::lcm as num_lcm;

fn push_checked(vals: &mut Vec<Complex64>, v: Complex64) -> Result<(), String> {
    if !v.re.is_finite() || !v.im.is_finite() {
        Err("Resultado no finito (NaN/Inf). Revisa dominio/división por 0.".to_string())
    } else {
        vals.push(v);
        Ok(())
    }
}

fn safe_i64(x: Complex64) -> Result<i64, String> {
    let xr = x.re.round();
    if !xr.is_finite() || xr > i64::MAX as f64 || xr < i64::MIN as f64 {
        return Err("Valor fuera de rango para operación entera".to_string());
    }
    Ok(xr as i64)
}

fn apply_func(vals: &mut Vec<Complex64>, func: &str, is_rad: bool) -> Result<(), String> {
    let b = vals.pop().ok_or("Faltan argumentos")?;

    match func {
        // --- Trigonométricas ---
        "sin" => push_checked(vals, if is_rad { b.sin() } else { Complex64::new(b.re.to_radians(), b.im).sin() })?,
        "cos" => push_checked(vals, if is_rad { b.cos() } else { Complex64::new(b.re.to_radians(), b.im).cos() })?,
        "tan" => push_checked(vals, if is_rad { b.tan() } else { Complex64::new(b.re.to_radians(), b.im).tan() })?,
        "asin" => {
            let res = b.asin();
            push_checked(vals, if is_rad { res } else { Complex64::new(res.re.to_degrees(), res.im) })?
        }
        "acos" => {
            let res = b.acos();
            push_checked(vals, if is_rad { res } else { Complex64::new(res.re.to_degrees(), res.im) })?
        }
        "atan" => {
            let res = b.atan();
            push_checked(vals, if is_rad { res } else { Complex64::new(res.re.to_degrees(), res.im) })?
        }
        "atan2" => {
            let a = vals.pop().ok_or("atan2 requiere 2 argumentos (y, x)")?;
            let res = Complex64::new(a.re.atan2(b.re), 0.0);
            push_checked(vals, if is_rad { res } else { Complex64::new(res.re.to_degrees(), 0.0) })?
        }
        "hypot" => {
            let a = vals.pop().ok_or("hypot requiere 2 argumentos (x, y)")?;
            push_checked(vals, Complex64::new(a.re.hypot(b.re), 0.0))?
        }

        // --- Hiperbólicas ---
        "sinh" => push_checked(vals, b.sinh())?,
        "cosh" => push_checked(vals, b.cosh())?,
        "tanh" => push_checked(vals, b.tanh())?,
        "asinh" => push_checked(vals, b.asinh())?,
        "acosh" => push_checked(vals, b.acosh())?,
        "atanh" => push_checked(vals, b.atanh())?,

        // --- Raíces y Logaritmos ---
        "sqrt" => push_checked(vals, b.sqrt())?,
        "cbrt" => push_checked(vals, b.powf(1.0/3.0))?,
        "root" => {
            let a = vals.pop().ok_or("root requiere 2 argumentos (n, x)")?;
            push_checked(vals, b.powc(Complex64::new(1.0/a.re, 0.0)))?
        }
        "ln" => {
            if b == Complex64::new(0.0, 0.0) { return Err("ln(0) indefinido".to_string()); }
            push_checked(vals, b.ln())?
        }
        "log10" => push_checked(vals, Complex64::new(b.re.log10(), 0.0))?, // Basado en parte real
        "log2" => push_checked(vals, Complex64::new(b.re.log2(), 0.0))?,
        "log" => {
            let a = vals.pop().ok_or("log requiere 2 argumentos (base, n)")?;
            push_checked(vals, b.ln() / a.ln())?
        }
        "exp" => push_checked(vals, b.exp())?,
        "pow" | "^" => {
            let a = vals.pop().ok_or("Falta operando base")?;
            push_checked(vals, a.powc(b))?
        }

        // --- Redondeo y Signo (sobre parte Real) ---
        "floor" => push_checked(vals, Complex64::new(b.re.floor(), 0.0))?,
        "ceil" => push_checked(vals, Complex64::new(b.re.ceil(), 0.0))?,
        "round" => push_checked(vals, Complex64::new(b.re.round(), 0.0))?,
        "trunc" => push_checked(vals, Complex64::new(b.re.trunc(), 0.0))?,
        "sign" => push_checked(vals, Complex64::new(b.re.signum(), 0.0))?,

        // --- Conversiones (sobre parte Real) ---
        "deg2rad" => push_checked(vals, Complex64::new(b.re.to_radians(), 0.0))?,
        "rad2deg" => push_checked(vals, Complex64::new(b.re.to_degrees(), 0.0))?,
        "cm2in" => push_checked(vals, Complex64::new(b.re / 2.54, 0.0))?,
        "in2cm" => push_checked(vals, Complex64::new(b.re * 2.54, 0.0))?,
        "m2ft" => push_checked(vals, Complex64::new(b.re * 3.280_839_895, 0.0))?,
        "ft2m" => push_checked(vals, Complex64::new(b.re / 3.280_839_895, 0.0))?,

        // --- Complejos ---
        "abs"  => push_checked(vals, Complex64::new(b.abs(), 0.0))?,
        "arg"  => push_checked(vals, Complex64::new(b.arg(), 0.0))?,
        "conj" => push_checked(vals, b.conj())?,
        "re"   => push_checked(vals, Complex64::new(b.re, 0.0))?,
        "im"   => push_checked(vals, Complex64::new(b.im, 0.0))?,

        // --- Teoría de Números (parte Real) ---
        "isprime" => {
            let n = b.re.round();
            let res = if n >= 0.0 && is_prime_u128(n as u128) { 1.0 } else { 0.0 };
            push_checked(vals, Complex64::new(res, 0.0))?
        }
        "nextprime" => {
            let n = b.re.round();
            if n < 0.0 { return Err("nextprime requiere n >= 0".to_string()); }
            push_checked(vals, Complex64::new(next_prime_u128(n as u128) as f64, 0.0))?
        }
        "mcd" => {
            let a = vals.pop().ok_or("mcd requiere 2 argumentos")?;
            push_checked(vals, Complex64::new(num_gcd(safe_i64(a)?, safe_i64(b)?) as f64, 0.0))?
        }
        "mcm" => {
            let a = vals.pop().ok_or("mcm requiere 2 argumentos")?;
            push_checked(vals, Complex64::new(num_lcm(safe_i64(a)?, safe_i64(b)?) as f64, 0.0))?
        }

        // --- Combinatoria ---
        "fact" => {
            let n = b.re.round() as u64;
            let mut acc = 1.0f64;
            for i in 2..=n { acc *= i as f64; }
            push_checked(vals, Complex64::new(acc, 0.0))?
        }
        "comb" | "nCr" => {
            let a = vals.pop().ok_or("nCr requiere 2 argumentos")?;
            let res = comb_u128(a.re.round() as u64, b.re.round() as u64)?;
            push_checked(vals, Complex64::new(res as f64, 0.0))?
        }
        "perm" | "nPr" => {
            let a = vals.pop().ok_or("nPr requiere 2 argumentos")?;
            let res = perm_u128(a.re.round() as u64, b.re.round() as u64)?;
            push_checked(vals, Complex64::new(res as f64, 0.0))?
        }

        // --- Estadística / Random / Pct ---
        "rand" => {
            let a = vals.pop().ok_or("rand requiere 2 argumentos (min, max)")?;
            let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
            let factor = ((nanos % 1_000_000_007) as f64) / 1_000_000_007.0;
            push_checked(vals, Complex64::new(a.re + factor * (b.re - a.re), 0.0))?
        }
        "min" => {
            let a = vals.pop().ok_or("min requiere 2 argumentos")?;
            push_checked(vals, if a.re < b.re { a } else { b })?
        }
        "max" => {
            let a = vals.pop().ok_or("max requiere 2 argumentos")?;
            push_checked(vals, if a.re > b.re { a } else { b })?
        }
        "pct" => {
            let a = vals.pop().ok_or("pct requiere 2 argumentos (parte, total)")?;
            push_checked(vals, Complex64::new((a.re / b.re) * 100.0, 0.0))?
        }
        "applypct" => {
            let a = vals.pop().ok_or("applypct requiere 2 argumentos (%, valor)")?;
            push_checked(vals, Complex64::new((a.re / 100.0) * b.re, 0.0))?
        }

        // --- Regla de tres ---
        "r3d" => {
            let c = b;
            let b_val = vals.pop().ok_or("r3d requiere 3 argumentos (a, b, c)")?;
            let a = vals.pop().ok_or("r3d requiere 3 argumentos (a, b, c)")?;
            push_checked(vals, (c * b_val) / a)?
        }
        "r3i" => {
            let c = b;
            let b_val = vals.pop().ok_or("r3i requiere 3 argumentos (a, b, c)")?;
            let a = vals.pop().ok_or("r3i requiere 3 argumentos (a, b, c)")?;
            push_checked(vals, (a * b_val) / c)?
        }

        // --- Conversiones de Base ---
        "bin" => {
            let n = safe_i64(b)?;
            println!("0b{:b}", n); // Imprime en consola el valor binario
            push_checked(vals, b)?
        }
        "oct" => {
            let n = safe_i64(b)?;
            println!("0o{:o}", n);
            push_checked(vals, b)?
        }
        "hex" => {
            let n = safe_i64(b)?;
            println!("0x{:x}", n);
            push_checked(vals, b)?
        }
        // Para convertir de base a decimal, usaremos 2 argumentos: de_base(base, número_en_esa_base)
        //"frombase" => {
        //    let num_str = safe_i64(b)?.to_string();
        //    let base = safe_i64(vals.pop().ok_or("frombase requiere 2 argumentos (base, número)")?)? as u32;
        //    let res = i64::from_str_radix(&num_str, base).map_err(|_| "Número inválido para la base especificada")?;
        //    push_checked(vals, num_complex::Complex64::new(res as f64, 0.0))?
        //}

        // --- Operadores ---
        "+" => { let a = vals.pop().ok_or("Falta operando")?; push_checked(vals, a + b)?; }
        "-" => { let a = vals.pop().ok_or("Falta operando")?; push_checked(vals, a - b)?; }
        "*" => { let a = vals.pop().ok_or("Falta operando")?; push_checked(vals, a * b)?; }
        "/" => {
            let a = vals.pop().ok_or("Falta operando")?;
            if b == Complex64::new(0.0, 0.0) { return Err("División por cero".to_string()); }
            push_checked(vals, a / b)?;
        }
        "%" | "mod" => {
            let a = vals.pop().ok_or("mod requiere 2 argumentos")?;
            push_checked(vals, Complex64::new(((a.re % b.re) + b.re) % b.re, 0.0))?
        }

        _ => return Err(format!("Función '{}' no implementada", func)),
    }
    Ok(())
}

impl Calculator {
    pub fn evaluate(&mut self, expr: &str) -> Result<Complex64, String> {
        let tokens = self.tokenize(expr)?;
        let mut values: Vec<Complex64> = Vec::new();
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
                        values.push(*self.vars.get(&name).ok_or(format!("Variable '{}' no existe", name))?);
                    }
                }
                Token::Func(f) => ops.push(f),
                Token::LParen => ops.push("(".to_string()),
                Token::Comma => {
                    while let Some(op) = ops.last() {
                        if op == "(" { break; }
                        let op = ops.pop().unwrap();
                        apply_func(&mut values, &op, self.is_radians)?;
                    }
                }
                Token::RParen => {
                    while let Some(op) = ops.pop() {
                        if op == "(" { break; }
                        apply_func(&mut values, &op, self.is_radians)?;
                    }
                }
                Token::Op(c) => {
                    let s = c.to_string();
                    let (p_curr, right_assoc) = prec(&s);
                    while let Some(top) = ops.last() {
                        if top == "(" { break; }
                        let (p_top, _) = prec(top);
                        if p_top > p_curr || (p_top == p_curr && !right_assoc) {
                            let op = ops.pop().unwrap();
                            apply_func(&mut values, &op, self.is_radians)?;
                        } else { break; }
                    }
                    ops.push(s);
                }
            }
        }
        while let Some(op) = ops.pop() { apply_func(&mut values, &op, self.is_radians)?; }
        let res = values.pop().ok_or("Error en expresión")?;
        self.last_result = res;
        Ok(res)
    }
}
