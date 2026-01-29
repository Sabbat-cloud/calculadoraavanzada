use colored::Colorize;

pub fn show_help() {
    println!("{}", "\n--- CALCULADORA AVANZADA EN RUST ---".yellow().bold());
    
    // --- Helpers de formateo para columnas ---
    // Ancho fijo para la columna del comando
    const CMD_WIDTH: usize = 20;
    const DESC_WIDTH: usize = 28;

    // Función auxiliar para imprimir 2 columnas: [Cmd1 : Desc1] | [Cmd2 : Desc2]
    let print_row = |c1: &str, d1: &str, c2: &str, d2: &str| {
        // Columna 1
        print!("  {}", c1.cyan());
        // Rellenar espacios manualmente para no romper alineación con caracteres ANSI
        for _ in 0..CMD_WIDTH.saturating_sub(c1.len()) { print!(" "); }
        print!(": ");
        
        // Descripción 1 (Truncada o rellenada si es necesario para mantener la rejilla)
        let d1_pad = if d1.len() > DESC_WIDTH { &d1[..DESC_WIDTH] } else { d1 };
        print!("{:<width$} ", d1_pad, width = DESC_WIDTH);

        // Separador central
        print!("{} ", "|".truecolor(100, 100, 100)); // Gris oscuro

        // Columna 2
        if !c2.is_empty() {
            print!("  {}", c2.cyan());
            for _ in 0..CMD_WIDTH.saturating_sub(c2.len()) { print!(" "); }
            println!(": {}", d2);
        } else {
            println!(); // Salto de línea si no hay segunda columna
        }
    };

    // --- SECCIÓN 1: Operadores y Constantes (Unificada) ---
    println!("{}", "\nOperaciones y Constantes:".green().bold());
    // Combinamos ambas líneas usando un separador visual
    println!("  Operadores : +, -, *, /, ^, %   |   Constantes : pi, e, phi, tau, c");

    // --- SECCIÓN 2: Comandos Generales (2 Columnas) ---
    println!("{}", "\nComandos Generales:".green().bold());
    print_row("help", "Muestra esta ayuda", "exit / quit", "Salir del programa");
    print_row("new", "Reinicia la calculadora", "clear", "Limpia la pantalla/hist");
    print_row("mode", "Alterna RAD / DEG", "fmt", "Alterna Decimal/Científico");
    print_row("var = <expr>", "Asignar variable", "vars", "Listar variables");

    // --- SECCIÓN 3: Pila de Memoria (2 Columnas) ---
    println!("{}", "\nGestión de Pila (Stack):".green().bold());
    print_row("push <expr>", "Añadir a la pila", "pop", "Sacar último valor");
    print_row("dup", "Duplicar último", "swap", "Intercambiar top 2");
    print_row("mem", "Ver pila completa", "clearstack", "Vaciar pila");
    print_row("sum / avg", "Suma/Promedio pila", "min / max", "Mín/Máx de la pila");

    // --- SECCIÓN 4: Herramientas Avanzadas (2 Columnas) ---
    println!("{}", "\nHerramientas Avanzadas:".green().bold());
    print_row("hist", "Ver historial", "!! / !N", "Repetir última / línea N");
    print_row("last / ans", "Último resultado", "plot <expr>", "Graficar (ASCII/Braille)");
    print_row("export <file>", "Exportar plot a SVG", "solve <eq> <x>", "Newton-Raphson (raíz)");
    print_row("integ <ex> <a> <b>", "Integral definida", "deriv <ex> <x>", "Derivada en un punto");
    print_row("ayuda <cmd>", "Ayuda detallada de func", "", "");

    // --- SECCIÓN 5: Funciones Matemáticas (Compacto / Grid) ---
    println!("{}", "\nFunciones Matemáticas Disponibles:".green().bold());
    
    let print_cat = |label: &str, funcs: &str| {
        println!("  {:<15} : {}", label.blue(), funcs);
    };

    print_cat("Trigonometría", "sin, cos, tan, asin, acos, atan, atan2, hypot");
    print_cat("Hiperbólicas", "sinh, cosh, tanh, asinh, acosh, atanh");
    print_cat("Exponenciales", "exp, ln, log10, log2, log(b,n), pow, sqrt, cbrt, root");
    print_cat("Redondeo/Num", "abs, sign, floor, ceil, round, trunc, fact, mcd, mcm");
    print_cat("Conversiones", "deg2rad, rad2deg, cm2in, in2cm, m2ft, ft2m");
    print_cat("Combinatoria", "comb (nCr), perm (nPr)");
    print_cat("Estadística/Var", "min, max, mod, rand, pct, applypct");
    print_cat("Regla de 3", "r3d(a,b,c), r3i(a,b,c)");
    print_cat("Complejos", "abs, arg, conj, re, im");
    print_cat("Bases", "bin(n), oct(n), hex(n)");
    
    println!();
}

pub fn show_specific_help(cmd: &str) {
    let (syntax, desc) = match cmd {
        // --- Funciones (1 arg) ---
        "exp" => ("exp(x)", "Calcula e elevado a la potencia x. Ej: exp(1) -> 2.718"),
        "sqrt" => ("sqrt(x)", "Raíz cuadrada. Ej: sqrt(16) -> 4"),
        "cbrt" => ("cbrt(x)", "Raíz cúbica. Ej: cbrt(27) -> 3"),
        "ln" => ("ln(x)", "Logaritmo natural (base e). Ej: ln(e) -> 1"),
        "abs" => ("abs(x)", "Valor absoluto o módulo de un complejo. Ej: abs(-5) -> 5"),
        "floor" => ("floor(x)", "Redondea hacia abajo al entero más cercano. Ej: floor(3.9) -> 3"),
        "ceil" => ("ceil(x)", "Redondea hacia arriba al entero más cercano. Ej: ceil(3.1) -> 4"),
        "round" => ("round(x)", "Redondea al entero más próximo. Ej: round(3.5) -> 4"),
        "trunc" => ("trunc(x)", "Elimina la parte decimal. Ej: trunc(-3.7) -> -3"),
        "sign" => ("sign(x)", "Retorna el signo del número (1, -1 o 0). Ej: sign(-10) -> -1"),
        "sin" => ("sin(x)", "Seno (según modo RAD/DEG). Ej: sin(pi/2) -> 1"),
        "cos" => ("cos(x)", "Coseno (según modo RAD/DEG). Ej: cos(0) -> 1"),
        "tan" => ("tan(x)", "Tangente (según modo RAD/DEG). Ej: tan(pi/4) -> 1"),
        "asin" => ("asin(x)", "Arcoseno. Ej: asin(1) -> 1.57 (rad)"),
        "acos" => ("acos(x)", "Arcocoseno. Ej: acos(0) -> 1.57 (rad)"),
        "atan" => ("atan(x)", "Arcotangente. Ej: atan(1) -> 0.78 (rad)"),
        "sinh" => ("sinh(x)", "Seno hiperbólico. Ej: sinh(0) -> 0"),
        "cosh" => ("cosh(x)", "Coseno hiperbólico. Ej: cosh(0) -> 1"),
        "tanh" => ("tanh(x)", "Tangente hiperbólica. Ej: tanh(0) -> 0"),
        "asinh" => ("asinh(x)", "Arcoseno hiperbólico."),
        "acosh" => ("acosh(x)", "Arcocoseno hiperbólico."),
        "atanh" => ("atanh(x)", "Arcotangente hiperbólica."),
        "deg2rad" => ("deg2rad(x)", "Convierte grados a radianes. Ej: deg2rad(180) -> 3.1415"),
        "rad2deg" => ("rad2deg(x)", "Convierte radianes a grados. Ej: rad2deg(pi) -> 180"),
        "cm2in" => ("cm2in(x)", "Centímetros a pulgadas. Ej: cm2in(2.54) -> 1"),
        "in2cm" => ("in2cm(x)", "Pulgadas a centímetros. Ej: in2cm(1) -> 2.54"),
        "m2ft" => ("m2ft(x)", "Metros a pies. Ej: m2ft(1) -> 3.28"),
        "ft2m" => ("ft2m(x)", "Pies a metros. Ej: ft2m(3.28) -> 1"),
        "fact" => ("fact(n)", "Factorial de n. Ej: fact(5) -> 120"),
        "log10" => ("log10(x)", "Logaritmo base 10. Ej: log10(100) -> 2"),
        "log2" => ("log2(x)", "Logaritmo base 2. Ej: log2(8) -> 3"),
        "isprime" => ("isprime(n)", "1 si es primo, 0 si no. Ej: isprime(7) -> 1"),
        "nextprime" => ("nextprime(n)", "Encuentra el siguiente número primo. Ej: nextprime(8) -> 11"),

        // --- Funciones (2 args) ---
        "atan2" => ("atan2(y, x)", "Arcotangente de dos variables."),
        "hypot" => ("hypot(x, y)", "Hipotenusa (sqrt(x²+y²)). Ej: hypot(3, 4) -> 5"),
        "root" => ("root(n, x)", "Raíz n-ésima de x. Ej: root(3, 8) -> 2"),
        "log" => ("log(base, n)", "Logaritmo en base específica. Ej: log(3, 9) -> 2"),
        "mcd" => ("mcd(a, b)", "Máximo Común Divisor. Ej: mcd(12, 18) -> 6"),
        "mcm" => ("mcm(a, b)", "Mínimo Común Múltiplo. Ej: mcm(4, 6) -> 12"),
        "comb" | "nCr" => ("comb(n, k)", "Combinaciones de n en k. Ej: comb(5, 2) -> 10"),
        "perm" | "nPr" => ("perm(n, k)", "Permutaciones de n en k. Ej: perm(5, 2) -> 20"),
        "pow" => ("pow(base, exp)", "Potencia. Ej: pow(2, 10) -> 1024"),
        "min" => ("min(a, b)", "El menor de dos valores. Ej: min(5, 3) -> 3"),
        "max" => ("max(a, b)", "El mayor de dos valores. Ej: max(5, 3) -> 5"),
        "mod" => ("mod(a, b)", "Residuo de la división (módulo). Ej: mod(10, 3) -> 1"),
        "rand" => ("rand(min, max)", "Número aleatorio entre min y max."),
        "pct" => ("pct(parte, total)", "Porcentaje que representa la parte. Ej: pct(10, 50) -> 20"),
        "applypct" => ("applypct(%, val)", "Aplica un porcentaje a un valor. Ej: applypct(20, 100) -> 20"),

        // --- Funciones (3 args) ---
        "r3d" => ("r3d(a, b, c)", "Regla de tres directa (c*b/a)."),
        "r3i" => ("r3i(a, b, c)", "Regla de tres inversa (a*b/c)."),

        // --- Comandos REPL ---
        "integ" => ("integ <expr> <min> <max> [steps]", "Calcula la integral definida numérica (Regla del Trapecio).\nEj: integ x^2 0 1 1000 -> 0.333..."),
        "deriv" => ("deriv <expr> <x> [h]", "Calcula la derivada numérica (diferencia centrada) de una función en un punto.\nEj: deriv x^2 3 -> 6"),

        // --- Complejos ---
        "arg" => ("arg(z)", "Argumento (ángulo) de un número complejo."),
        "conj" => ("conj(z)", "Conjugado de un complejo (a+bi -> a-bi)."),
        "re" => ("re(z)", "Parte real de un complejo."),
        "im" => ("im(z)", "Parte imaginaria de un complejo."),

        // --- Bases ---
        "bin" => ("bin(n)", "Muestra n en binario (0b...). Ej: bin(10) -> 0b1010"),
        "oct" => ("oct(n)", "Muestra n en octal (0o...). Ej: oct(10) -> 0o12"),
        "hex" => ("hex(n)", "Muestra n en hexadecimal (0x...). Ej: hex(255) -> 0xff"),

        // --- Graficacion ---
        "plot" =>("plot <expr>", "Muestra la gráfica en formato ascii en alta resolucion: Ej:  plot sin(x);cos(x)"),
        "export" =>("export <nombre.svg>", "Exporta el último plot en formato svg."),

        _ => ("", "Ayuda no disponible para este término. Usa 'help' para la lista general."),
    };

    if syntax.is_empty() {
        println!("{}", desc.red());
    } else {
        println!("{} : {}", syntax.cyan().bold(), desc);
    }
}
