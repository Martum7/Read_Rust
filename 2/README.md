# Lição 2 — Sistema de notas (`gradebook`)

## Correr
```bash
cargo run
```

---

## Estrutura do projecto

```
src/
├── main.rs        — ponto de entrada, monta tudo
├── student.rs     — struct Student com métodos
├── grade.rs       — enum LetterGrade
└── gradebook.rs   — struct GradeBook, lógica agregada
```

Este é código multi-ficheiro real. Cada ficheiro é um **módulo** declarado em `main.rs` com `mod`. O `pub` controla o que é visível fora do módulo — se não tiver `pub`, é privado (como `private` em C++/Java).

---

## Conceitos centrais

### Ownership & move
Quando chamas `book.enroll(s)`, o valor `s` é **movido** para dentro do `GradeBook`. Depois dessa linha, `s` deixa de existir no scope actual — o compilador rejeita qualquer uso. Não há cópia implícita como em Java (onde seria sempre uma referência), nem tens de fazer `std::move` explicitamente como em C++11.

### `&self` vs `&mut self`
```rust
fn average(&self) -> Option<f64>      // só lê
fn add_grade(&mut self, grade: f64)   // modifica
```
O compilador garante em tempo de compilação: **nunca podes ter uma referência mutável e uma imutável ao mesmo tempo**. Não há data races, não há use-after-free.

### `Option<T>` em vez de null
`average()` devolve `Option<f64>`. Não há `null`. O tipo força-te a lidar com a ausência de valor. As formas idiomáticas de desempacotar:
```rust
match opt { Some(v) => ..., None => ... }
if let Some(v) = opt { ... }
opt.map(|v| v * 2)
opt.unwrap_or(0.0)
opt.unwrap_or_else(|| compute_default())
```

### `enum` com `match` exaustivo
`match` em Rust obriga-te a cobrir **todos** os casos. Se adicionares uma variante ao enum e te esqueceres de actualizar um `match` algures, o compilador dá erro. Isso não acontece em C++ `switch` (onde podes esquecer um `case` silenciosamente).

### Iterators
Em vez de loops manuais, Rust usa **iterators encadeados**:
```rust
self.students.values()
    .filter_map(|s| s.average())
    .collect::<Vec<_>>()
```
`filter_map` é como um `filter` + `map` ao mesmo tempo: ignora `None`, desempacota `Some`. `collect()` materializa o iterator numa colecção.

### `derive`
```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
```
Gera implementações automáticas de traits (interfaces). `Hash` é necessário para usar o tipo como chave de `HashMap`. `Ord` permite ordenar. Se não derivares, o compilador diz-te exactamente o que falta.

---

## Desafios 🛠️

Lê o código com calma antes de alterar. Os desafios aumentam em dificuldade.

**1.** Há métodos em `student.rs` que existem mas nunca são chamados (`highest`, `lowest`, `grade_count`). Faz com que o sumário em `gradebook.rs` mostre também a nota mais alta e mais baixa de cada aluno no top 3.

**2.** O `GradeBook` não permite remover um aluno. Adiciona um método `remove(&mut self, id: u32) -> Option<Student>` — repara no tipo de retorno: o que significa devolver `Option<Student>` (owned) em vez de `Option<&Student>`?

**3.** Actualmente `add_grade` em `student.rs` aceita silenciosamente notas inválidas (fora de 0–20) sem dizer nada. Muda o comportamento: o método deve devolver algo que indique ao chamador se a nota foi aceite ou rejeitada. Decide tu o tipo de retorno mais adequado. Propaga essa mudança para `main.rs`.

**4.** O `match` em `LetterGrade::from_score` usa guards (`if s >= ...`). Como alternativa, Rust suporta range patterns noutros contextos. Pesquisa o que é um **range pattern** em Rust e tenta reescrever o `from_score` sem guards — vais encontrar uma limitação com `f64` que te vai obrigar a pensar numa solução.

**5.** O `top_students` usa `.unwrap()` duas vezes no closure de `sort_by`. Isso é seguro aqui porque o `filter` antes garante que todos os elementos têm `average().is_some()` — mas é frágil. Reescreve o sort de forma a não precisar de nenhum `unwrap`.

**6.** Adiciona ao `GradeBook` um método `worst_student(&self) -> Option<&Student>` que devolva o aluno com a média mais baixa (ignorando alunos sem notas). Implementa-o **sem copiar** a lógica de `top_students` — pensa em como reutilizar o que já existe.

---

## Próxima lição

- **Traits** como interfaces (mas mais poderosas)
- **Generics** — escrever código que funciona para vários tipos
- **Result\<T, E\>** — gestão de erros sem excepções
- **Closures** em profundidade — captura de variáveis, `move` closures
