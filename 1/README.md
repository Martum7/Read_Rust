# Lição 1 — Primeiros passos a ler Rust

## Como correr

```bash
cargo run
```

Precisa de ter o [Rust instalado](https://rustup.rs). Se não tiveres, instala com:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## O que está neste projecto

Tudo está em **`src/main.rs`**. Abre esse ficheiro e lê com calma enquanto lês isto.

---

## Conceitos explicados

### Variáveis — `let` e `mut`
```rust
let nome = "Vasco";     // imutável — não podes mudar depois
let mut pontos = 0;     // mutável — podes alterar
pontos = pontos + 10;   // isto funciona porque tem "mut"
```
Rust obriga-te a ser explícito sobre o que pode mudar. Se tentares alterar uma variável sem `mut`, o compilador dá erro — e isso é bom, protege-te de bugs.

---

### Tipos de dados básicos
| Tipo    | Exemplo         | O que é                          |
|---------|-----------------|----------------------------------|
| `i32`   | `-42`           | Inteiro (pode ser negativo)      |
| `u32`   | `25`            | Inteiro sem sinal (≥ 0)          |
| `f64`   | `3.14`          | Número decimal                   |
| `bool`  | `true` / `false`| Verdadeiro ou falso              |
| `char`  | `'R'`           | Um único carácter                |
| `&str`  | `"Rust!"`       | Texto fixo (referência)          |
| `String`| `format!(...)` | Texto dinâmico (alocado)         |

Na maioria dos casos podes deixar Rust **inferir** o tipo automaticamente:
```rust
let x = 10;    // Rust percebe que é i32
let y = 3.14;  // Rust percebe que é f64
```

---

### Funções — `fn`
```rust
fn somar(a: i32, b: i32) -> i32 {
    a + b   // sem ";" no fim = é o valor de retorno
}
```
- Os tipos dos parâmetros são **obrigatórios**.
- O tipo de retorno vem depois do `->`.
- A **última expressão sem ponto e vírgula** é automaticamente o retorno (não precisas de `return`, embora funcione).

---

### If / Else
```rust
if temperatura > 30 {
    println!("Calor");
} else {
    println!("Fresco");
}
```
Nota especial: em Rust, `if` é uma **expressão** — pode produzir um valor:
```rust
let descricao = if verdade { "sim" } else { "não" };
```

---

### Loops
```rust
loop { /* infinito, sai com break */ }
while condicao { /* enquanto for true */ }
for i in 1..=5 { /* de 1 a 5 inclusive */ }
for i in 0..5  { /* de 0 a 4 (5 exclusive) */ }
```

---

### `println!` e `format!`
O `!` **indica** que é uma **macro** (não uma função normal).
```rust
println!("Olá, {}!", nome);        // imprime e salta linha
print!("sem saltar linha");        // imprime sem \n
format!("devolve uma {}", "String") // não imprime, retorna texto
```
`{:?}` imprime tipos mais complexos como arrays: `println!("{:?}", array)`.

---

## Coisas para tentares mudar 🛠️

Experimenta cada uma das seguintes alterações em `src/main.rs`, grava, e corre `cargo run` para ver o resultado. Se o compilador der erro, lê a mensagem — o Rust tem mensagens de erro excelentes.

1. **Muda o nome e a idade** na linha `let nome = "Vasco"` e `let idade: u32 = 25`.

2. **Tenta remover o `mut`** da variável `pontos` e vê o erro que o compilador dá.

3. **Muda a temperatura** para `35` ou `10` e vê qual ramo do `if` é executado.

4. **Muda o intervalo do `for`** de `1..=5` para `1..=10` e vê o que imprime.

5. **Adiciona uma fruta** ao array `frutas`. Nota: o array tem tamanho fixo — todas as frutas ficam no mesmo bloco de código.

6. **Cria uma nova função** que recebe um número e devolve o seu dobro. Modelo:
   ```rust
   fn dobro(n: i32) -> i32 {
       n * 2
   }
   ```
   Depois chama-a no `main`: `println!("dobro de 4 = {}", dobro(4));`

7. **Experimenta um loop `while`** que imprime os números pares de 2 a 20.

---

## Próximos passos (Lição 2)

Na próxima pasta aprenderás sobre:
- **Ownership** (o conceito mais único de Rust)
- **Structs** (agrupar dados)
- **Enums** e **match**
- **Vec** (arrays de tamanho variável)
