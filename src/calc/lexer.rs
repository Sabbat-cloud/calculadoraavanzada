use super::{token::Token, Calculator};

/// Lee un número en formato:
///   123
///   123.45
///   .5
///   1e6
///   2.5E-3
/// con un prefijo opcional (por ejemplo '-' para unarios)
fn read_number<I>(chars: &mut std::iter::Peekable<I>, prefix: Option<char>) -> String
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    if let Some(p) = prefix {
        s.push(p);
    }

    let mut saw_digit = false;
    let mut saw_dot = false;
    let mut saw_exp = false;

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            saw_digit = true;
            s.push(chars.next().unwrap());
        } else if c == '.' && !saw_dot && !saw_exp {
            saw_dot = true;
            s.push(chars.next().unwrap());
        } else if (c == 'e' || c == 'E') && !saw_exp && saw_digit {
            saw_exp = true;
            s.push(chars.next().unwrap());
            if let Some(&sign) = chars.peek() {
                if sign == '+' || sign == '-' {
                    s.push(chars.next().unwrap());
                }
            }
        } else {
            break;
        }
    }

    s
}

impl Calculator {
    pub fn tokenize(&self, expr: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let cleaned = expr.replace(' ', "");
        let mut chars = cleaned.chars().peekable();
        let mut last_token_was_op = true;

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    let s = read_number(&mut chars, None);
                    let n = self.parse_number_str(&s)?;
                    tokens.push(Token::Number(n));
                    last_token_was_op = false;
                }

                '-' if last_token_was_op => {
                    chars.next();
                    if let Some(&nextc) = chars.peek() {
                        if nextc.is_ascii_digit() || nextc == '.' {
                            let s = read_number(&mut chars, Some('-'));
                            let n = self.parse_number_str(&s)?;
                            tokens.push(Token::Number(n));
                            last_token_was_op = false;
                        } else {
                            tokens.push(Token::Number(num_complex::Complex64::new(-1.0, 0.0)));
                            tokens.push(Token::Op('*'));
                            last_token_was_op = true;
                        }
                    } else {
                        tokens.push(Token::Number(num_complex::Complex64::new(-1.0, 0.0)));
                        tokens.push(Token::Op('*'));
                        last_token_was_op = true;
                    }
                }

                '+' | '-' | '*' | '/' | '^' | '%' => {
                    tokens.push(Token::Op(chars.next().unwrap()));
                    last_token_was_op = true;
                }

                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                    last_token_was_op = true;
                }

                '(' => {
                    tokens.push(Token::LParen);
                    chars.next();
                    last_token_was_op = true;
                }

                ')' => {
                    tokens.push(Token::RParen);
                    chars.next();
                    last_token_was_op = false;
                }

                'a'..='z' | 'A'..='Z' => {
                    let mut name = String::new();
                    while let Some(&c2) = chars.peek() {
                        if c2.is_alphanumeric() {
                            name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    let funcs = [
                        "sin","cos","tan","asin","acos","atan","atan2","hypot",
                        "sqrt","cbrt","root","log","ln",
                        "mcd","mcm","exp","arg","conj","re","im","pow",
                        "floor","ceil","abs","round","trunc","sign",
                        "sinh","cosh","tanh","asinh","acosh","atanh",
                        "deg2rad","rad2deg","cm2in","in2cm","m2ft","ft2m",
                        "fact","comb","perm","nCr","nPr",
                        "exp","log10","log2","pow","min","max","mod","isprime",
                        "nextprime","rand","pct","applypct","r3d","r3i",
                        "bin", "oct", "hex",
                    ];

                    if funcs.contains(&name.as_str()) {
                        tokens.push(Token::Func(name));
                        last_token_was_op = true;
                    } else {
                        tokens.push(Token::Var(name));
                        last_token_was_op = false;
                    }
                }

                _ => return Err(format!("Carácter inválido: '{c}'")),
            }
        }

        Ok(tokens)
    }
}

