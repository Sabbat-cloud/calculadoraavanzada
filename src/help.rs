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
Pila:
  push <expr> [expr2 ...]       : Evalúa y mete (varios separados por espacios) o sin arg mete 'last'
  pop                           : Saca el último (lo deja en last)
  dup / swap / clearstack       : Operaciones de pila
  mem                           : Ver pila
  sum / avg / min / max / std   : Estadística sobre la pila
Historial:
  hist      : Muestra historial.txt                        clear    : Vacía historial.txt
  !!        : Repite última línea del historial            !N       : Repite la línea N del historial (1-index)
  last/ans  : Último valor
Otros:
  plot <exprs> [xmin xmax] [ymin ymax] [w h]
        Ej 1: plot sin(x);cos(x) -10 10 -2 2        Ej 2: plot tan(x) -5 5 -5 5 120 40 (Gráfico grande)
  mode  : Alterna RAD/DEG       new  : Reset        help/"exit","quit","salir"  : Ayuda / salir
"#
    );
}
