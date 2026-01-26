pub fn show_help() {
    println!(
        r#"---CALCULADORA AVANZADA EN RUST----
Operaciones: +, -, *, /, ^, % Constantes: pi, e, phi/golden, tau, c
Comando <expr>              : Evalúa        var = <expr>        : Guarda variable
Funciones (1 arg):
  exp(x), sqrt(x), cbrt(x), ln(x), abs(x), floor(x), ceil(x), round(x), trunc(x), sign(x), sin(x), cos(x), tan(x), asin(x), acos(x), atan(x) sinh(x), cosh(x), tanh(x), asinh(x), acosh(x), atanh(x), deg2rad(x), rad2deg(x), cm2in(x), in2cm(x), m2ft(x), ft2m(x), fact(n), log10(x), log2(x), isprime(n), nextprime(n)
Funciones (2 args):
  atan2(y, x), hypot(x, y), root(n, x), log(base, n), mcd(a,b), mcm(a,b), comb/nCr(n,k), perm/nPr(n,k), pow(base, exp), min(a, b), max(a, b), mod(a,b), rand(min,max), pct(parte, total), applypct(%,val)
Funciones (3 args):
  r3d(a,b,c), r3i(a,b,c)
Complejos:
    abs, arg, conj, re, im
Funciones de Base:
  bin(n), oct(n), hex(n)        : Muestra n en binario, octal o hexa.
Pila:
  push <expr> [expr2 ...]       : Evalúa y mete (varios separados por espacios) o sin arg mete 'last'
  pop                           : Saca el último (lo deja en last)
  dup / swap / clearstack       : Operaciones de pila
  mem                           : Ver pila
  sum / avg / min / max / std   : Estadística sobre la pila
Historial:
  hist      : Muestra historial.txt                        clear    : Vacía historial.txt
  !!        : Repite última línea del historial            !N       : Repite la línea N del historial (1-index)
  last/ans  : Último valor                                 ayuda    : Muestra la ayuda de una funcion ejem: ayuda cos
Otros:
  plot <exprs> [xmin xmax] [ymin ymax] [w h]
        Ej 1: plot sin(x);cos(x) -10 10 -2 2        Ej 2: plot tan(x) -5 5 -5 5 120 40 (Gráfico grande)
  mode  : Alterna RAD/DEG       new  : Reset        help/"exit","quit","salir"  : Ayuda / salir
"#
    );
}

pub fn show_specific_help(cmd: &str) {
    let help_text = match cmd {
        // --- Funciones (1 arg) ---
        "exp" => "exp(x): Calcula e elevado a la potencia x. Ej: exp(1) -> 2.718",
        "sqrt" => "sqrt(x): Raíz cuadrada. Ej: sqrt(16) -> 4",
        "cbrt" => "cbrt(x): Raíz cúbica. Ej: cbrt(27) -> 3",
        "ln" => "ln(x): Logaritmo natural (base e). Ej: ln(e) -> 1",
        "abs" => "abs(x): Valor absoluto o módulo de un complejo. Ej: abs(-5) -> 5",
        "floor" => "floor(x): Redondea hacia abajo al entero más cercano. Ej: floor(3.9) -> 3",
        "ceil" => "ceil(x): Redondea hacia arriba al entero más cercano. Ej: ceil(3.1) -> 4",
        "round" => "round(x): Redondea al entero más próximo. Ej: round(3.5) -> 4",
        "trunc" => "trunc(x): Elimina la parte decimal. Ej: trunc(-3.7) -> -3",
        "sign" => "sign(x): Retorna el signo del número (1, -1 o 0). Ej: sign(-10) -> -1",
        "sin" => "sin(x): Seno (según modo RAD/DEG). Ej: sin(pi/2) -> 1",
        "cos" => "cos(x): Coseno (según modo RAD/DEG). Ej: cos(0) -> 1",
        "tan" => "tan(x): Tangente (según modo RAD/DEG). Ej: tan(pi/4) -> 1",
        "asin" => "asin(x): Arcoseno. Ej: asin(1) -> 1.57 (rad)",
        "acos" => "acos(x): Arcocoseno. Ej: acos(0) -> 1.57 (rad)",
        "atan" => "atan(x): Arcotangente. Ej: atan(1) -> 0.78 (rad)",
        "sinh" => "sinh(x): Seno hiperbólico. Ej: sinh(0) -> 0",
        "cosh" => "cosh(x): Coseno hiperbólico. Ej: cosh(0) -> 1",
        "tanh" => "tanh(x): Tangente hiperbólica. Ej: tanh(0) -> 0",
        "asinh" => "asinh(x): Arcoseno hiperbólico.",
        "acosh" => "acosh(x): Arcocoseno hiperbólico.",
        "atanh" => "atanh(x): Arcotangente hiperbólica.",
        "deg2rad" => "deg2rad(x): Convierte grados a radianes. Ej: deg2rad(180) -> 3.1415",
        "rad2deg" => "rad2deg(x): Convierte radianes a grados. Ej: rad2deg(pi) -> 180",
        "cm2in" => "cm2in(x): Centímetros a pulgadas. Ej: cm2in(2.54) -> 1",
        "in2cm" => "in2cm(x): Pulgadas a centímetros. Ej: in2cm(1) -> 2.54",
        "m2ft" => "m2ft(x): Metros a pies. Ej: m2ft(1) -> 3.28",
        "ft2m" => "ft2m(x): Pies a metros. Ej: ft2m(3.28) -> 1",
        "fact" => "fact(n): Factorial de n. Ej: fact(5) -> 120",
        "log10" => "log10(x): Logaritmo base 10. Ej: log10(100) -> 2",
        "log2" => "log2(x): Logaritmo base 2. Ej: log2(8) -> 3",
        "isprime" => "isprime(n): 1 si es primo, 0 si no. Ej: isprime(7) -> 1",
        "nextprime" => "nextprime(n): Encuentra el siguiente número primo. Ej: nextprime(8) -> 11",

        // --- Funciones (2 args) ---
        "atan2" => "atan2(y, x): Arcotangente de dos variables.",
        "hypot" => "hypot(x, y): Hipotenusa (sqrt(x²+y²)). Ej: hypot(3, 4) -> 5",
        "root" => "root(n, x): Raíz n-ésima de x. Ej: root(3, 8) -> 2",
        "log" => "log(base, n): Logaritmo en base específica. Ej: log(3, 9) -> 2",
        "mcd" => "mcd(a, b): Máximo Común Divisor. Ej: mcd(12, 18) -> 6",
        "mcm" => "mcm(a, b): Mínimo Común Múltiplo. Ej: mcm(4, 6) -> 12",
        "comb" | "nCr" => "comb(n, k): Combinaciones de n en k. Ej: comb(5, 2) -> 10",
        "perm" | "nPr" => "perm(n, k): Permutaciones de n en k. Ej: perm(5, 2) -> 20",
        "pow" => "pow(base, exp): Potencia. Ej: pow(2, 10) -> 1024",
        "min" => "min(a, b): El menor de dos valores. Ej: min(5, 3) -> 3",
        "max" => "max(a, b): El mayor de dos valores. Ej: max(5, 3) -> 5",
        "mod" => "mod(a, b): Residuo de la división (módulo). Ej: mod(10, 3) -> 1",
        "rand" => "rand(min, max): Número aleatorio entre min y max.",
        "pct" => "pct(parte, total): Porcentaje que representa la parte. Ej: pct(10, 50) -> 20",
        "applypct" => "applypct(%, val): Aplica un porcentaje a un valor. Ej: applypct(20, 100) -> 20",

        // --- Funciones (3 args) ---
        "r3d" => "r3d(a, b, c): Regla de tres directa (c*b/a).",
        "r3i" => "r3i(a, b, c): Regla de tres inversa (a*b/c).",

        // --- Complejos ---
        "arg" => "arg(z): Argumento (ángulo) de un número complejo.",
        "conj" => "conj(z): Conjugado de un complejo (a+bi -> a-bi).",
        "re" => "re(z): Parte real de un complejo.",
        "im" => "im(z): Parte imaginaria de un complejo.",

        // --- Bases ---
        "bin" => "bin(n): Muestra n en binario (0b...). Ej: bin(10) -> 0b1010",
        "oct" => "oct(n): Muestra n en octal (0o...). Ej: oct(10) -> 0o12",
        "hex" => "hex(n): Muestra n en hexadecimal (0x...). Ej: hex(255) -> 0xff",

        _ => "Ayuda no disponible para este término. Usa 'help' para la lista general.",
    };
    println!("{}", help_text);
}
