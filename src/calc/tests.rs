use super::Calculator;
use num_complex::Complex64;

// --- Helpers para pruebas ---

/// Compara dos números complejos con un margen de error (epsilon).
fn approx_eq(a: Complex64, b: Complex64) -> bool {
    (a.re - b.re).abs() < 1e-9 && (a.im - b.im).abs() < 1e-9
}

/// Constructor rápido de complejos para no escribir Complex64::new a cada rato
fn c(re: f64, im: f64) -> Complex64 {
    Complex64::new(re, im)
}

// --- Tests de Aritmética Básica ---

#[test]
fn test_basic_arithmetic() {
    let mut calc = Calculator::new();
    
    // Suma
    let res = calc.evaluate("10 + 5").unwrap();
    assert!(approx_eq(res, c(15.0, 0.0)));

    // Resta
    let res = calc.evaluate("10 - 2.5").unwrap();
    assert!(approx_eq(res, c(7.5, 0.0)));

    // Multiplicación
    let res = calc.evaluate("4 * 2.5").unwrap();
    assert!(approx_eq(res, c(10.0, 0.0)));

    // División
    let res = calc.evaluate("10 / 4").unwrap();
    assert!(approx_eq(res, c(2.5, 0.0)));

    // Módulo (resto)
    let res = calc.evaluate("10 % 3").unwrap();
    assert!(approx_eq(res, c(1.0, 0.0)));

    // Potencia
    let res = calc.evaluate("2 ^ 3").unwrap();
    assert!(approx_eq(res, c(8.0, 0.0)));
}

#[test]
fn test_precedence_and_associativity() {
    let mut calc = Calculator::new();

    // Multiplicación (2) > Suma (1)
    let res = calc.evaluate("2 + 3 * 4").unwrap();
    assert!(approx_eq(res, c(14.0, 0.0)));

    // Paréntesis alteran precedencia
    let res = calc.evaluate("(2 + 3) * 4").unwrap();
    assert!(approx_eq(res, c(20.0, 0.0)));

    // Potencia (4) > Multiplicación (2)
    let res = calc.evaluate("2 * 3 ^ 2").unwrap();
    assert!(approx_eq(res, c(18.0, 0.0))); // 2 * 9 = 18

    // NOTA: Tu lexer captura el signo negativo junto al número si es un dígito.
    // Por tanto, "-2" es un token Number(-2).
    // "-2^2" se interpreta como "(-2) ^ 2" = 4.
    // Si fuera aritmética escrita estricta sería -(2^2) = -4, pero en programación
    // es común tratar los literales negativos como unidades atómicas.
    let res = calc.evaluate("-2^2").unwrap();
    assert!(approx_eq(res, c(4.0, 0.0))); 

    // Para forzar el comportamiento matemático (-4), el usuario debe usar paréntesis:
    let res = calc.evaluate("-(2^2)").unwrap();
    // Aquí el lexer ve "-" (operador unario/resta) y luego el paréntesis.
    // Como tu lexer convierte "-" a "-1 *" cuando no hay dígito, esto será -1 * 4
    assert!(approx_eq(res, c(-4.0, 0.0)));
}

// --- Tests de Constantes y Variables ---

#[test]
fn test_constants() {
    let mut calc = Calculator::new();
    
    // PI
    let res = calc.evaluate("pi").unwrap();
    assert!((res.re - std::f64::consts::PI).abs() < 1e-9);

    // E
    let res = calc.evaluate("e").unwrap();
    assert!((res.re - std::f64::consts::E).abs() < 1e-9);

    // I (Imaginario)
    let res = calc.evaluate("i").unwrap();
    assert!(approx_eq(res, c(0.0, 1.0)));
}

// --- Tests de Trigonometría (Modos RAD y DEG) ---

#[test]
fn test_trigonometry_modes() {
    let mut calc = Calculator::new();

    // 1. Modo Radianes (default)
    calc.is_radians = true;
    assert!(approx_eq(calc.evaluate("sin(pi/2)").unwrap(), c(1.0, 0.0)));
    assert!(approx_eq(calc.evaluate("cos(pi)").unwrap(), c(-1.0, 0.0)));
    assert!(approx_eq(calc.evaluate("tan(0)").unwrap(), c(0.0, 0.0)));

    // 2. Modo Grados
    calc.is_radians = false;
    assert!(approx_eq(calc.evaluate("sin(90)").unwrap(), c(1.0, 0.0)));
    assert!(approx_eq(calc.evaluate("cos(180)").unwrap(), c(-1.0, 0.0)));
    // tan(45) = 1
    assert!(approx_eq(calc.evaluate("tan(45)").unwrap(), c(1.0, 0.0)));
}

// --- Tests de Números Complejos ---

#[test]
fn test_complex_operations() {
    let mut calc = Calculator::new();

    // Raíz de número negativo
    let res = calc.evaluate("sqrt(-4)").unwrap();
    assert!(approx_eq(res, c(0.0, 2.0))); // 2i

    // Multiplicación compleja: i * i = -1
    let res = calc.evaluate("i * i").unwrap();
    assert!(approx_eq(res, c(-1.0, 0.0)));

    // Suma compleja
    let res = calc.evaluate("(2 + 3*i) + (1 - i)").unwrap();
    assert!(approx_eq(res, c(3.0, 2.0)));

    // Valor absoluto (módulo)
    let res = calc.evaluate("abs(3 + 4*i)").unwrap(); // sqrt(9+16) = 5
    assert!(approx_eq(res, c(5.0, 0.0)));

    // Argumento (fase)
    let res = calc.evaluate("arg(i)").unwrap(); // 90 grados o pi/2
    assert!((res.re - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
}

// --- Tests de Funciones Científicas ---

#[test]
fn test_scientific_funcs() {
    let mut calc = Calculator::new();

    // Logaritmos
    assert!(approx_eq(calc.evaluate("log10(100)").unwrap(), c(2.0, 0.0)));
    assert!(approx_eq(calc.evaluate("log2(8)").unwrap(), c(3.0, 0.0)));
    assert!(approx_eq(calc.evaluate("ln(e^2)").unwrap(), c(2.0, 0.0)));

    // Redondeo
    assert!(approx_eq(calc.evaluate("round(3.6)").unwrap(), c(4.0, 0.0)));
    assert!(approx_eq(calc.evaluate("floor(3.9)").unwrap(), c(3.0, 0.0)));
    assert!(approx_eq(calc.evaluate("ceil(3.1)").unwrap(), c(4.0, 0.0)));

    // Funciones de 2 argumentos
    assert!(approx_eq(calc.evaluate("hypot(3, 4)").unwrap(), c(5.0, 0.0)));
    assert!(approx_eq(calc.evaluate("min(10, 5)").unwrap(), c(5.0, 0.0)));
    assert!(approx_eq(calc.evaluate("max(10, 5)").unwrap(), c(10.0, 0.0)));
}

// --- Tests de Combinatoria y Enteros ---

#[test]
fn test_combinatorics() {
    let mut calc = Calculator::new();

    assert!(approx_eq(calc.evaluate("fact(5)").unwrap(), c(120.0, 0.0)));
    assert!(approx_eq(calc.evaluate("comb(5, 2)").unwrap(), c(10.0, 0.0)));
    assert!(approx_eq(calc.evaluate("perm(5, 2)").unwrap(), c(20.0, 0.0)));
    
    // MCD y MCM
    assert!(approx_eq(calc.evaluate("mcd(12, 18)").unwrap(), c(6.0, 0.0)));
    assert!(approx_eq(calc.evaluate("mcm(4, 6)").unwrap(), c(12.0, 0.0)));
}

// --- Tests de Parsing y Formato ---

#[test]
fn test_parser_formats() {
    let mut calc = Calculator::new();

    // Espacios extra
    assert!(approx_eq(calc.evaluate("  1   + 2  ").unwrap(), c(3.0, 0.0)));

    // Decimales sin cero inicial
    assert!(approx_eq(calc.evaluate(".5 + .25").unwrap(), c(0.75, 0.0)));

    // Notación científica
    assert!(approx_eq(calc.evaluate("1e3").unwrap(), c(1000.0, 0.0)));
    assert!(approx_eq(calc.evaluate("2.5E-1").unwrap(), c(0.25, 0.0)));
}

// --- Tests de Manejo de Errores ---

#[test]
fn test_errors() {
    let mut calc = Calculator::new();

    // División por cero
    assert!(calc.evaluate("1 / 0").is_err());

    // Paréntesis desbalanceados
    assert!(calc.evaluate("(1 + 2").is_err());
    assert!(calc.evaluate("1 + 2)").is_err());

    // Función desconocida o variable no definida
    // (El lexer detectará 'funcion_fake' como variable y evaluate fallará al buscarla)
    assert!(calc.evaluate("funcion_fake(10)").is_err());

    // Argumentos incorrectos
    assert!(calc.evaluate("ln(0)").is_err()); // Indefinido
    assert!(calc.evaluate("mod(10)").is_err()); // Faltan argumentos (requiere 2)
}

#[test]
fn test_readme_examples() {
    // Validamos que los ejemplos prometidos en el README funcionen
    let mut calc = Calculator::new();
    
    // sqrt(16) + log10(100) = 4 + 2 = 6
    assert!(approx_eq(calc.evaluate("sqrt(16) + log10(100)").unwrap(), c(6.0, 0.0)));

    // fact(7) / (fact(3) * fact(4)) = 5040 / (6 * 24) = 5040 / 144 = 35
    assert!(approx_eq(calc.evaluate("fact(7) / (fact(3) * fact(4))").unwrap(), c(35.0, 0.0)));
}
