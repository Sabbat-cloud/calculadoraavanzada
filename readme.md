# 📚 Documentación de la Calculadora Avanzada en Rust

## Índice
- [Descripción General](#descripción-general)
- [Instalación y Uso](#instalación-y-uso)
- [Comandos del REPL](#comandos-del-repl)
- [Operaciones y Funciones](#operaciones-y-funciones)
- [Sistema de Variables](#sistema-de-variables)
- [Gestión de Pila](#gestión-de-pila)
- [Historial](#historial)
- [Graficación](#graficación)
- [Constantes Predefinidas](#constantes-predefinidas)
- [API del Módulo](#api-del-módulo)
- [Ejemplos de Uso](#ejemplos-de-uso)
- [Solución de Problemas](#solución-de-problemas)

---

## Descripción General

Calculadora científica avanzada escrita en Rust con soporte para más de 50 funciones matemáticas, sistema de variables, pila de memoria, graficación ASCII y REPL interactivo con autocompletado.

**Características principales:**
- Evaluación de expresiones matemáticas complejas
- Más de 50 funciones integradas
- Sistema de variables personalizadas
- Pila de memoria con operaciones (push, pop, dup, swap)
- Historial persistente con reutilización
- Graficación ASCII de funciones
- Autocompletado inteligente
- Modo grados/radianes intercambiable

---

## Instalación y Uso

### Requisitos
- Rust 1.60+ y Cargo

### Compilación
```bash
git clone <repo>
cd calculadora-avanzada
cargo build --release
cargo run
```

### Ejecución
```bash
./target/release/calculadora
```

La calculadora iniciará en modo REPL interactivo si no se usan argumentos:
Acepta: ./calculadora2026 2+2 o ./calculadora2026 "mod(12,14)" por ejemplo.
```
Calculadora Avanzada en Rust. Escribe 'help'. (Ctrl+D para salir)
[RAD] >> 
```

---

## Comandos del REPL

### Comandos Básicos
| Comando | Descripción |
|---------|-------------|
| `help` | Muestra ayuda completa |
| `exit` | Sale de la calculadora |
| `new` | Reinicia el sistema (borra variables, mantiene historial) |
| `mode` | Alterna entre modo RAD (radianes) y DEG (grados) |
| `vars` | Muestra todas las variables definidas |
| `ayuda`| Muestra la ayuda de una funcion (ej: `ayuda cos`)
### Comandos de Pila
| Comando | Descripción |
|---------|-------------|
| `mem` | Muestra el contenido actual de la pila |
| `push <expr>` | Evalúa expresión y la coloca en la pila |
| `push` | Coloca el último resultado (`last`) en la pila |
| `pop` | Saca el último valor de la pila |
| `dup` | Duplica el valor superior de la pila |
| `swap` | Intercambia los dos valores superiores |
| `clearstack` | Vacía toda la pila |

### Comandos Estadísticos
Operaciones sobre todos los valores en la pila:
- `sum` - Suma de todos los valores
- `avg` - Promedio de todos los valores
- `min` - Valor mínimo
- `max` - Valor máximo
- `std` - Desviación estándar

### Comandos de Historial
| Comando | Descripción |
|---------|-------------|
| `hist` | Muestra el historial completo |
| `clear` | Borra el historial del archivo |
| `!!` | Repite la última expresión del historial |
| `!N` | Repite la línea N del historial (1-indexado) |

### Graficación
```bash
plot <expresiones> [xmin xmax] [ymin ymax] [ancho alto]
```
- **expresiones**: Una o más funciones separadas por ; (ej: sin(x);cos(x)).

- **rangos opcionales**:
	- xmin xmax: Rango del eje horizontal (por defecto -10 10).
	- ymin ymax: Rango del eje vertical (por defecto autoescala).

- **dimensiones opcionales**:
	- ancho alto: Tamaño de la rejilla ASCII en caracteres (por defecto 80 24).

- **Nuevas características visuales*:

	- Marco informativo: El gráfico ahora incluye bordes (┌ ┐, └ ┘) para delimitar el área.
	- Etiquetas de ejes: Se muestran los valores numéricos de los límites en las esquinas y el valor medio en el eje Y para mejor referencia.
	- Ejes dinámicos: Los ejes | y - se cruzan con un símbolo + en el origen $(0,0)$.

**Ejemplos:**
```
# Gráfico básico (80x24)
plot sin(x)

# Definir solo el rango X
plot x^2 -5 5

# Definir rango X, Y y dimensiones de la rejilla (100 columnas por 30 líneas)
plot sin(x);cos(x) -6.28 6.28 -1.5 1.5 100 30

# Gráfico pequeño "miniatura"
plot sqrt(x) 0 20 0 5 40 10
```

---

## Operaciones y Funciones

### Operadores Aritméticos
| Símbolo | Operación | Ejemplo |
|---------|-----------|---------|
| `+` | Suma | `5 + 3` |
| `-` | Resta | `10 - 4` |
| `*` | Multiplicación | `6 * 7` |
| `/` | División | `15 / 3` |
| `^` | Potencia | `2 ^ 8` |
| `%` | Módulo | `17 % 5` |

### Funciones Trigonométricas (1 argumento)
Todas aceptan grados o radianes según el modo actual.

| Función | Descripción | Dominio | Ejemplo |
|---------|-------------|---------|---------|
| `sin(x)` | Seno | ℝ | `sin(30)` |
| `cos(x)` | Coseno | ℝ | `cos(pi)` |
| `tan(x)` | Tangente | x ≠ 90°+k·180° | `tan(45)` |
| `asin(x)` | Arcoseno | [-1, 1] | `asin(0.5)` |
| `acos(x)` | Arcocoseno | [-1, 1] | `acos(0)` |
| `atan(x)` | Arcotangente | ℝ | `atan(1)` |

### Funciones Hiperbólicas (1 argumento)
| Función | Descripción | Dominio |
|---------|-------------|---------|
| `sinh(x)` | Seno hiperbólico | ℝ |
| `cosh(x)` | Coseno hiperbólico | ℝ |
| `tanh(x)` | Tangente hiperbólica | ℝ |
| `asinh(x)` | Inversa sinh | ℝ |
| `acosh(x)` | Inversa cosh | x ≥ 1 |
| `atanh(x)` | Inversa tanh | \|x\| < 1 |

### Funciones Exponenciales y Logarítmicas
| Función | Descripción | Dominio |
|---------|-------------|---------|
| `exp(x)` | eˣ | ℝ |
| `ln(x)` | Logaritmo natural | x > 0 |
| `log10(x)` | Logaritmo base 10 | x > 0 |
| `log2(x)` | Logaritmo base 2 | x > 0 |
| `log(base, x)` | Logaritmo base b | base>0, base≠1, x>0 |

### Funciones de Raíces y Potencias
| Función | Sintaxis | Descripción |
|---------|----------|-------------|
| `sqrt(x)` | `sqrt(16)` | Raíz cuadrada |
| `cbrt(x)` | `cbrt(27)` | Raíz cúbica |
| `root(n, x)` | `root(3, 8)` | Raíz n-ésima |
| `pow(base, exp)` | `pow(2, 10)` | Potencia |

### Funciones de Redondeo y Signo
| Función | Descripción | Ejemplo | Resultado |
|---------|-------------|---------|-----------|
| `floor(x)` | Piso | `floor(3.7)` | 3.0 |
| `ceil(x)` | Techo | `ceil(3.2)` | 4.0 |
| `round(x)` | Redondeo | `round(3.5)` | 4.0 |
| `trunc(x)` | Truncar | `trunc(-3.7)` | -3.0 |
| `abs(x)` | Valor absoluto | `abs(-5)` | 5.0 |
| `sign(x)` | Signo | `sign(-10)` | -1.0 |

### Conversiones de Unidades
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `deg2rad(x)` | Grados → Radianes | `deg2rad(180)` |
| `rad2deg(x)` | Radianes → Grados | `rad2deg(pi)` |
| `cm2in(x)` | Centímetros → Pulgadas | `cm2in(2.54)` |
| `in2cm(x)` | Pulgadas → Centímetros | `in2cm(1)` |
| `m2ft(x)` | Metros → Pies | `m2ft(1)` |
| `ft2m(x)` | Pies → Metros | `ft2m(3.28084)` |

### Conversión de Bases
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `bin(x)` | Muestra valor en binario | `bin(10)` -> 0b1010 |
| `oct(x)` | Muestra valor en octal | `oct(10)` -> 0o12 |
| `hex(x)` | Muestra valor en hexadecimal | `hex(255)` -> 0xff |

### Funciones de Combinatoria
| Función | Sintaxis | Descripción |
|---------|----------|-------------|
| `fact(n)` | `fact(5)` | Factorial (n ≥ 0) |
| `comb(n, k)` | `comb(5, 2)` | Combinaciones |
| `nCr(n, k)` | `nCr(10, 3)` | Combinaciones (alias) |
| `perm(n, k)` | `perm(5, 2)` | Permutaciones |
| `nPr(n, k)` | `nPr(10, 3)` | Permutaciones (alias) |

### Funciones de Números Primos
| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `isprime(n)` | Verifica si n es primo | `isprime(17)` → 1.0 |
| `nextprime(n)` | Siguiente primo ≥ n | `nextprime(10)` → 11.0 |

### Funciones de MCD y MCM
| Función | Sintaxis | Descripción |
|---------|----------|-------------|
| `mcd(a, b)` | `mcd(12, 18)` | Máximo común divisor |
| `mcm(a, b)` | `mcm(4, 6)` | Mínimo común múltiplo |

### Funciones Estadísticas y Aleatorias
| Función | Sintaxis | Descripción |
|---------|----------|-------------|
| `min(a, b)` | `min(5, 3)` | Mínimo de dos valores |
| `max(a, b)` | `max(5, 3)` | Máximo de dos valores |
| `rand(min, max)` | `rand(0, 1)` | Número aleatorio |
| `pct(parte, total)` | `pct(15, 60)` | Porcentaje |
| `applypct(%, valor)` | `applypct(20, 100)` | Aplica porcentaje |

### Funciones de 3 Argumentos
| Función | Sintaxis | Descripción |
|---------|----------|-------------|
| `r3d(a, b, c)` | `r3d(2, 3, 6)` | Regla de tres directa: (c×b)/a |
| `r3i(a, b, c)` | `r3i(2, 3, 6)` | Regla de tres inversa: (a×b)/c |

---

## Sistema de Variables

### Variables Predefinidas
| Variable | Valor | Descripción |
|----------|-------|-------------|
| `pi` | 3.141592653589793 | π |
| `e` | 2.718281828459045 | Número de Euler |
| `tau` | 6.283185307179586 | 2π |
| `phi` | 1.618033988749895 | Número áureo |
| `golden` | 1.618033988749895 | Áureo (alias) |
| `c` | 299792458 | Velocidad de la luz (m/s) |
| `last` | último resultado | Variable especial |
| `ans` | último resultado | Alias de `last` |

### Definición de Variables
```bash
# Sintaxis: <nombre> = <expresión>
radio = 5
area = pi * radio^2
perimetro = 2 * pi * radio

# Uso de variables
area + perimetro
```

### Variables Especiales
- `x`: Usada en graficación, se puede sobreescribir
- `last`/`ans`: Siempre contiene el último resultado calculado

---

## Gestión de Pila

La pila permite almacenar múltiples valores para operaciones posteriores.

### Ejemplo de Flujo de Pila:
```bash
[RAD] >> 5 + 3
= 8

[RAD] >> push
PUSH(last) -> 8 (size=1)

[RAD] >> 2 * 7
= 14

[RAD] >> push
PUSH(last) -> 14 (size=2)

[RAD] >> mem
Pila: [8, 14]

[RAD] >> sum
= 22

[RAD] >> avg
= 11

[RAD] >> dup
DUP -> 11 (size=3)

[RAD] >> swap
SWAP -> top=14 (size=3)

[RAD] >> pop
POP -> 8 (size=2)
```

---

## Historial

### Archivo de Historial
- Ubicación: `historial.txt` (en directorio de ejecución)
- Formato: `<expresión> = <resultado>`
- Persiste entre ejecuciones

### Comandos de Reutilización
```bash
# Historial actual:
# 1: 5 + 3 = 8
# 2: sin(30) = 0.5
# 3: 2^10 = 1024

!!      # Repite: 2^10
!2      # Repite: sin(30)
!1      # Repite: 5 + 3
```

---

## Graficación

### Sintaxis Completa
```bash
plot <funciones> [xmin xmax] [ymin ymax]
```

### Ejemplos Detallados:
```bash
# Gráfico básico con autoescala
plot sin(x)

# Dos funciones con rango X personalizado
plot sin(x);cos(x) -10 10

# Tres funciones con rangos X e Y fijos
plot x^2;sqrt(x);log(x+1) 0 10 0 5

# Función paramétrica usando múltiples plots
plot t*cos(t);t*sin(t) 0 6.28 -10 10
```

### Símbolos de Gráfico:
- `•` - Primera función
- `x` - Segunda función
- `*` - Tercera función
- `+` - Cuarta función
- `o` - Quinta función
- `#` - Sexta función
- `@` - Séptima función

---

## API del Módulo

### Estructura Principal
```rust
pub struct Calculator {
    pub memory_stack: Vec<f64>,
    pub vars: HashMap<String, f64>,
    pub history_file: String,
    pub is_radians: bool,
    pub last_result: f64,
}
```

### Métodos Principales
```rust
impl Calculator {
    pub fn new() -> Self;
    pub fn reset(&mut self);
    pub fn evaluate(&mut self, expr: &str) -> Result<f64, String>;
    pub fn plot(&mut self, input: &str);
}
```

### Módulos del Crate
| Módulo | Responsabilidad |
|--------|----------------|
| `calc` | Estructura principal y constantes |
| `eval` | Evaluación de expresiones y funciones |
| `lexer` | Tokenización y parsing |
| `repl` | Interfaz interactiva y comandos |
| `help` | Documentación de ayuda |
| `history` | Gestión de historial |
| `math_ext` | Funciones matemáticas extendidas |
| `plot` | Graficación ASCII |
| `token` | Definición de tokens |

---

## Ejemplos de Uso

### Ejemplo 1: Cálculos Científicos
```bash
[RAD] >> sqrt(16) + log10(100)
= 6

[RAD] >> sin(pi/2) * cos(0)
= 1

[RAD] >> fact(7) / (fact(3) * fact(4))
= 35
```

### Ejemplo 2: Trabajo con Variables
```bash
[DEG] >> radio = 5
radio = 5

[DEG] >> area = pi * radio^2
area = 78.53981633974483

[DEG] >> perimetro = 2 * pi * radio
perimetro = 31.41592653589793

[DEG] >> area + perimetro
= 109.95574287564276
```

### Ejemplo 3: Uso de Pila para Cálculos Complejos
```bash
# Calcular estadísticas sobre un conjunto de datos
[RAD] >> push 10 20 30 40 50
PUSH -> 10 (size=1)
PUSH -> 20 (size=2)
PUSH -> 30 (size=3)
PUSH -> 40 (size=4)
PUSH -> 50 (size=5)

[RAD] >> sum
= 150

[RAD] >> avg
= 30

[RAD] >> std
= 14.142135623730951
```

### Ejemplo 4: Graficación Avanzada
```bash
# Gráfico de funciones trigonométricas
[RAD] >> plot sin(x);cos(x);tan(x) -6.28 6.28 -2 2

# Gráfico de función cuadrática y raíz
[RAD] >> plot x^2;sqrt(x) 0 10 0 10

# Múltiples funciones con diferentes símbolos
[RAD] >> plot exp(-0.1*x)*sin(x);cos(x) -20 20 -2 2
```

### Ejemplo 5: Script Interactivo
```bash
# Resolver ecuación cuadrática: x² - 5x + 6 = 0
[RAD] >> a = 1
a = 1

[RAD] >> b = -5
b = -5

[RAD] >> c = 6
c = 6

[RAD] >> discriminante = b^2 - 4*a*c
discriminante = 1

[RAD] >> x1 = (-b + sqrt(discriminante)) / (2*a)
x1 = 3

[RAD] >> x2 = (-b - sqrt(discriminante)) / (2*a)
x2 = 2
```

---

## Solución de Problemas

### Errores Comunes y Soluciones

| Error | Causa Probable | Solución |
|-------|----------------|----------|
| `Error: Función/operador X no implementado` | Nombre de función incorrecto | Verificar ortografía o usar `help` |
| `Error: Variable 'X' no existe` | Variable no definida | Definir variable primero |
| `Error: División por cero` | Denominador cero | Verificar expresión |
| `Error: sqrt(x) requiere x >= 0` | Argumento negativo | Usar valor absoluto o cbrt |
| `Error: ln(x) requiere x > 0` | Logaritmo de no positivo | Verificar dominio |
| `Error: Resultado no finito` | Overflow/Underflow | Simplificar expresión |
| `Error: mcd/mcm fuera de rango` | Números muy grandes | Reducir valores |

### Atajos de Teclado
| Combinación | Acción |
|-------------|--------|
| `Ctrl + C` | Cancela entrada actual |
| `Ctrl + D` | Sale de la calculadora |
| `↑` / `↓` | Navega por historial de comandos |
| `Tab` | Autocompleta comandos/variables |

### Modos de Ángulo
- **DEG**: Funciones trigonométricas usan grados (0-360)
- **RAD**: Funciones trigonométricas usan radianes (0-2π)
- **Cambio**: Comando `mode` alterna entre ambos

### Formatos Numéricos Soportados
```
Entero:       123
Decimal:      123.456
Científico:   1.23e4, 5.6E-3
Notación E:   2.5e6
Prefijo .:    .75 (0.75)
Negativo:     -45, -3.14
```

---

## Contribución y Extensión

### Añadir Nuevas Funciones
Para añadir una nueva función matemática:

1. **Añadir a la lista de funciones** en `repl.rs`:
```rust
const FUNCS: &[&str] = &[
    // ...
    "nueva_funcion",
];
```

2. **Implementar en `eval.rs`**:
```rust
match func {
    // ...
    "nueva_funcion" => {
        let arg = vals.pop().ok_or("Falta argumento")?;
        // Implementación
        push_checked(vals, resultado)?;
    }
}
```

3. **Actualizar ayuda** en `help.rs`

### Personalización
- **Archivo de historial**: Modificar `history_file` en `Calculator::new()`
- **Precisión**: Modificar `format!` en salidas
- **Tamaño de gráfico**: Cambiar `width` y `height` en `plot.rs`

---

## Licencia y Créditos

Esta calculadora es un proyecto educativo que demuestra:
- Parsing de expresiones matemáticas en Rust
- Diseño de REPL interactivo
- Arquitectura modular
- Manejo de errores robusto

**Funcionalidades inspiradas en:**
- Calculadoras HP con notación RPN
- Python con módulos math y numpy
- Herramientas Unix como bc y dc

---

*(c) sabbat.cloud . Calculadora v1*
