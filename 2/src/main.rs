// src/main.rs
//
// Módulos em Rust são ficheiros separados declarados aqui com "mod".
// "use" importa nomes para o scope local — equivalente a "using" em C++ ou import em Java.

mod grade;
mod gradebook;
mod student;

use gradebook::GradeBook;
use student::Student;

fn main() {
    let mut book = GradeBook::new("Algoritmos e Estruturas de Dados");

    // Cada Student é criado e o ownership é passado para enroll().
    // Depois desta linha, a variável local "s" já não existe — o GradeBook é o dono.
    let mut s = Student::new(1, "Ana Costa");
    s.add_grade(18.5);
    s.add_grade(17.0);
    s.add_grade(19.0);
    book.enroll(s);

    let mut s = Student::new(2, "Bruno Melo");
    s.add_grade(13.0);
    s.add_grade(11.5);
    s.add_grade(14.0);
    book.enroll(s);

    let mut s = Student::new(3, "Carla Dias");
    s.add_grade(8.0);
    s.add_grade(7.5);
    s.add_grade(9.0);
    book.enroll(s);

    let mut s = Student::new(4, "David Nunes");
    s.add_grade(15.5);
    s.add_grade(16.0);
    s.add_grade(14.5);
    book.enroll(s);

    let mut s = Student::new(5, "Eva Ferreira");
    s.add_grade(6.0);
    s.add_grade(8.5);
    book.enroll(s);

    let mut s = Student::new(6, "Filipe Ramos");
    s.add_grade(12.0);
    s.add_grade(12.5);
    s.add_grade(11.0);
    s.add_grade(13.0);
    book.enroll(s);

    // Aluno sem notas — propositalmente para o Option ter impacto real
    let s = Student::new(7, "Gabriela Sousa");
    book.enroll(s);

    // Imprime o sumário geral
    book.summary();

    // Demonstra find() — devolve Option<&Student>
    println!("\nProcura directa:");
    match book.find(3) {
        Some(student) => println!("  Encontrado: {}", student),
        None => println!("  Aluno não encontrado."),
    }
    match book.find(99) {
        Some(student) => println!("  Encontrado: {}", student),
        None => println!("  ID 99 não existe."),
    }

    // find_mut devolve uma referência mutável — podemos adicionar uma nota ao aluno
    println!("\nAdicionando nota de melhoria a Carla Dias:");
    if let Some(carla) = book.find_mut(3) {
        match carla.add_grade(15.0) {
            Some(g) => println!("  Nota {:.1} adicionada. Novas stats: {}", g, carla),
            None    => println!("  Nota inválida, não foi adicionada."),
        }
    }

    println!("\nAlunos reprovados:");
    let failing = book.failing_students();
    if failing.is_empty() {
        println!("  Nenhum.");
    } else {
        for s in failing {
            println!("  {}", s);
        }
    }
}
