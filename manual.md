---
````md
# Calculadora Avanzada en Rust — Manual de uso

Calculadora interactiva en consola con:
- Evaluación de expresiones matemáticas
- Variables y constantes predefinidas
- Funciones científicas (trig, logs, raíces, combinatoria, etc.)
- Modo **DEG/RAD**
- Pila de memoria con operaciones y estadísticas
- Historial de expresiones (`historial.txt`)
- Historial de comandos con flechas (si usas `rustyline`, opcional)
- Gráficas ASCII (`plot`) con soporte multi-función

---

## Índice

- [1. Inicio rápido](#1-inicio-rápido)
- [2. Sintaxis de expresiones](#2-sintaxis-de-expresiones)
- [3. Constantes y variables](#3-constantes-y-variables)
- [4. Funciones disponibles](#4-funciones-disponibles)
- [5. Comandos de la REPL](#5-comandos-de-la-repl)
  - [5.1 Generales](#51-generales)
  - [5.2 Pila (memoria)](#52-pila-memoria)
  - [5.3 Historial](#53-historial)
  - [5.4 Gráficas ASCII](#54-gráficas-ascii)
- [6. Ejemplos por nivel](#6-ejemplos-por-nivel)
- [7. Archivos generados](#7-archivos-generados)
- [8. Resolución de errores comunes](#8-resolución-de-errores-comunes)
- [9. Subir a GitHub](#9-subir-a-github)

---

## 1. Inicio rápido

Ejecuta el programa:

```bash
cargo run
````

Verás un prompt con el modo actual:

```
[DEG] >>
```

Escribe una expresión:

```
[DEG] >> 2+2
= 4
```

Ayuda:

```
[DEG] >> help
```

Salir:

```
[DEG] >> exit
```

---

## 2. Sintaxis de expresiones

### Operadores soportados

* Suma: `+`
* Resta: `-`
* Multiplicación: `*`
* División: `/`
* Potencia: `^`
* Módulo: `%` (y también `mod(a,b)`)

### Precedencia (aprox.)

1. Paréntesis `( )`
2. Potencias `^` (asociativa por la derecha)
3. `* / %`
4. `+ -`

### Números soportados

* Enteros: `123`
* Decimales: `123.45`, `.5`
* Notación científica: `1e6`, `2.5E-3`

Ejemplos:

```txt
1e6
2.5e-3
(2+3)*4
2^3^2
```

---

## 3. Constantes y variables

### Constantes predefinidas

* `pi`  → π
* `e`   → e
* `tau` → 2π
* `phi` / `golden` → número áureo
* `c`   → velocidad de la luz (299792458)

Ejemplo:

```txt
pi
2*pi
rad2deg(pi)
```

### Variables de usuario

Asignación con `=`:

```txt
a = 10
b = sqrt(81)
```

Luego:

```txt
a + b
```

### `last` / `ans` (último resultado)

Puedes reutilizar el último resultado como `last` o `ans`:

```txt
2+3
ans*10
```

---

## 4. Funciones disponibles

> Nota: la mayoría son `func(x)` (1 arg). Otras son `func(a,b)` o `func(a,b,c)`.

### Trigonometría

* 1 arg: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
* 2 args: `atan2(y,x)`, `hypot(x,y)`

### Raíces y logs

* `sqrt(x)`, `cbrt(x)`, `root(n,x)`
* `ln(x)`, `log(base,x)`, `log10(x)`, `log2(x)`

### Exponenciales y potencias

* `exp(x)`, `pow(a,b)` (equivalente a `a^b`)

### Redondeo y signo

* `floor`, `ceil`, `round`, `trunc`, `abs`, `sign`

### Hiperbólicas

* `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`

### Conversión de unidades

* `deg2rad`, `rad2deg`
* `cm2in`, `in2cm`, `m2ft`, `ft2m`

### Combinatoria

* `fact(n)`, `comb(n,k)` / `nCr(n,k)`, `perm(n,k)` / `nPr(n,k)`

### Utilidades

* `min(a,b)`, `max(a,b)`
* `mod(a,b)` (módulo estilo Python)
* `mcd(a,b)`, `mcm(a,b)`
* `isprime(n)` → `1` si primo, `0` si no
* `nextprime(n)` → siguiente primo
* `rand(min,max)` → aleatorio pseudo-simple
* `pct(parte,total)` → porcentaje
* `applypct(porcentaje,valor)` → aplica %
* Regla de 3:

  * `r3d(a,b,c)` directa: (c*b)/a
  * `r3i(a,b,c)` inversa: (a*b)/c

---

## 5. Comandos de la REPL

### 5.1 Generales

#### `help`
* **Ejemplo**

  ```txt
  help
  ```

#### `exit`

* **Ejmplo***

  ```txt
  exit
  ```
#### `new` (reset)

Reinicia pila y variables (restaura constantes).

* **Uso**

  ```txt
  new
  ```

#### `mode` (DEG/RAD)


* **Ejemplo**

  ```txt
  mode
  ```

#### `vars`

Muestra variables (incluye constantes).

* **Ejemplo**

  ```txt
  vars
  ```
---

### 5.2 Pila (memoria)

#### `mem`

Muestra la pila actual.

* **Ejemplo**

  ```txt
  mem
  ```

#### `push` (sin args)

Empuja `last/ans` a la pila.

* **Ejemplo**

  ```txt
  2+3
  push
  mem
  ```

#### `push <expr> [expr2 ...]`

Evalúa y empuja 1 o varios elementos.

* **Ejemplo**

  ```txt
  push 10 20 30
  ```

#### `pop`

Extrae el último de la pila y lo deja en `last`.

* **Ejemplo**

  ```txt
  push 1 2 3
  pop
  mem
  ```

#### `dup`

Duplica el top de la pila.

* **Ejemplo**

  ```txt
  push 1 2 3 4 5
  dup
  sum
  ```

#### `swap`

Intercambia los dos últimos elementos.

* **Ejemplo**

  ```txt
  push 1 2 3
  swap
  mem
  ```
#### `clearstack`

Vacía la pila.

* **Ejemplo**

  ```txt
  push 1 2 3
  clearstack
  mem
  ```

#### Estadísticas: `sum`, `avg`, `min`, `max`, `std`

Operan sobre la pila.

* **Básico**

  ```txt
  push 1 2 3
  sum
  ```
* **Intermedio**

  ```txt
  push 10 20 30 40
  avg
  ```
* **Avanzado**: comparar dispersión

  ```txt
  clearstack
  push 1 1 1 1 1
  std
  clearstack
  push 1 2 3 4 5
  std
  ```

---

### 5.3 Historial

> Hay 2 historiales posibles:
>
> * `historial.txt` → historial de **expresiones evaluadas** (se guarda `expr = resultado`)
> * `historial_cmds.txt` → historial de **comandos** (↑↓) si usas `rustyline` (opcional)

#### `hist`

Muestra el archivo `historial.txt`.

* **Básico**

  ```txt
  hist
  ```
* **Intermedio**

  ```txt
  2+2
  3+3
  hist
  ```
* **Avanzado**: localizar expresiones para repetir con `!N`

  ```txt
  hist
  !3
  ```

#### `clear`

Vacía `historial.txt`.

* **Ejemplo**

  ```txt
  clear
  ```

#### `!!` (repite última expresión del historial)

* **Ejemplo**

  ```txt
  12*12
  !!
  ans+1
  ```

#### `!N` (repite la expresión N del historial, 1-index)

* **Ejemplo**

  ```txt
  hist
  !1
  ```
---

### 5.4 Gráficas ASCII

#### `plot <exprs> [xmin xmax] [ymin ymax]`

* Puedes graficar una o varias expresiones separadas por `;`
* Variable independiente: `x`
* Si no das `ymin/ymax`, hace auto-escala

**Ejemplos**

```txt
plot sin(x)
plot cos(x)
plot sin(x);cos(x) -10 10
plot sin(x);cos(x);tan(x) -6.28 6.28 -2 2
```

Consejos:

* `tan(x)` suele “romper” la escala si no limitas `ymin/ymax`.
* Si has definido `x` como variable fija, usa `new` o borra `x`.

---

## 6. Ejemplos por nivel

### Básicos

```txt
2+2
(2+3)*4
sqrt(81)
sin(90)      # en DEG
mode
sin(pi/2)    # en RAD
```

### Intermedios

```txt
a = 10
b = 2.5
a*b + 3

log(10, 1000)
root(3, 8)
comb(10, 3)

push 10 20 30 40
avg
```

### Avanzados

```txt
# trig inversa comparando modos
mode
asin(1)
mode
asin(1)

# usar historial y ans como “pipeline”
(12^2) + (5^3)
push
!!
ans/2
push
mem
std

# plot múltiple con escalado controlado
plot sin(x);cos(x);0.5*sin(2*x) -10 10 -2 2
```

---

## 7. Archivos generados

* `historial.txt`
  Guarda resultados evaluados como:

  ```
  <expr> = <resultado>
  ```

* (Opcional, si usas rustyline) `historial_cmds.txt`
  Guarda comandos para navegar con flechas ↑↓

---

## 8. Resolución de errores comunes

* **“División por cero”**

  * Revisa `/0`, `%0` o `mod(a,0)`.

* **“Resultado no finito (NaN/Inf)”**

  * Suele ocurrir con dominios inválidos: `sqrt(-1)`, `ln(0)`, `tan(90)` en DEG, etc.

* **Trigonometría “no cuadra”**

  * Comprueba el modo:

    ```txt
    mode
    ```

* **`!N` no existe**

  * Mira primero `hist` para saber cuántas líneas hay.

---

## Licencia

Licencia MIT

```

---
