// ============================================================
//  LIÇÃO 1 — Ler e entender código Rust básico
//  Corre com: cargo run
// ============================================================

fn main() {
    // ---------- VARIÁVEIS ----------
    // "let" declara uma variável. Por defeito é IMUTÁVEL (não podes mudar o valor).
    let nome = "Vasco";
    let last_name: &str = "Azevedo";
    let idade: u32 = 25; // u32 = número inteiro sem sinal (0 ou positivo)

    println!("Olá, {} {}! Tens {} anos.", nome, last_name, idade);

    // "mut" torna a variável mutável (podes alterar o valor).
    let mut pontos= 0;
    pontos = pontos + 10;
    println!("Pontos actuais: {}", pontos);

    // ---------- TIPOS BÁSICOS ----------
    let inteiro: i32 = -42;       // i32 = inteiro com sinal (pode ser negativo)
    let decimal: f64 = 3.14;      // f64 = número com vírgula (decimal)
    let verdade: bool = true;     // bool = verdadeiro ou falso
    let letra: char = 'R';        // char = um único carácter (usa plicas)
    let texto: &str = "Rust!";    // &str = texto fixo (fatia de string)

    println!("inteiro={}, decimal={}, bool={}, char={}, texto={}", inteiro, decimal, verdade, letra, texto);

    // ---------- FUNÇÕES ----------
    // Chama funções definidas mais abaixo neste ficheiro.
    let soma = somar(3, 7);
    println!("3 + 7 = {}", soma);

    let saudacao = cumprimentar("mundo");
    println!("{}", saudacao);

    // ---------- IF / ELSE ----------
    let temperatura = 22;

    if temperatura > 30 {
        println!("Está calor.");
    } else if temperatura > 15 {
        println!("Está agradável.");
    } else {
        println!("Está frio.");
    }

    // Em Rust, if também é uma expressão — pode retornar um valor.
    let descricao = if verdade { "verdadeiro" } else { "falso" };
    println!("verdade é: {}", descricao);

    //-------TESTE estranho-------
    let strange = if temperatura > 30 {
        println!("Está calor.");
        1
    } else if temperatura > 15 {
        println!("Está agradável.");
        2
    } else {
        println!("Está frio.");
        3
    };
    println!("strange test works{}", strange);


    // ---------- LOOPS ----------

    // "loop" corre para sempre até encontrar "break".
    let mut contador = 0;
    loop {
        contador += 1;
        if contador == 3 {
            break; // sai do loop
        }
    }
    println!("loop parou no contador: {}", contador);

    // "while" corre enquanto a condição for verdadeira.
    let mut n = 5;
    while n > 0 {
        print!("{} ", n); // print! (sem ln) não salta linha
        n -= 1;
    }
    println!(); // linha em branco para separar output

    // "for" itera sobre um intervalo ou colecção.
    for i in 1..=5 {
        // 1..=5 significa de 1 até 5 inclusive
        print!("{} ", i);
    }
    println!();

    // ---------- ARRAYS ----------
    // Array de tamanho fixo. Todos os elementos têm o mesmo tipo.
    let frutas = ["maçã", "banana", "laranja", "uvas"];
    println!("Frutas: {:?}", frutas); // {:?} imprime o array inteiro

    for fruta in frutas {
        println!("  - {}", fruta);
    }
}

// ============================================================
//  DEFINIÇÃO DE FUNÇÕES
// ============================================================

// "fn" define uma função.
// Os tipos dos parâmetros são obrigatórios.
// "-> i32" indica o tipo de retorno.
fn somar(a: i32, b: i32) -> i32 {
    // A última expressão SEM ponto e vírgula é o valor de retorno.
    a + b
}

// Esta função devolve uma String (texto dinâmico, alocado em memória).
fn cumprimentar(quem: &str) -> String {
    // format! funciona como println! mas devolve uma String em vez de imprimir.
    format!("Olá, {}!", quem)
}

