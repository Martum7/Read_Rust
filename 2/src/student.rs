// src/student.rs

// Em Rust não há classes — há structs com blocos "impl" separados.
// É parecido com C++ structs mas os métodos ficam fora da definição do tipo.

use crate::grade::LetterGrade;

// derive(Debug) gera automaticamente código para imprimir com {:?}
// derive(Clone) gera código para copiar o valor — em Rust copiar não é implícito
#[derive(Debug, Clone)]
pub struct Student {
    pub name: String, // String = heap-allocated, owned. Diferente de &str que é uma referência.
    pub id: u32,
    grades: Vec<f64>, // privado — só acessível através dos métodos abaixo
}

impl Student {
    // Construtor por convenção chama-se "new". Não há construtores especiais como em C++/Java.
    // &str aqui porque não precisamos de tomar ownership do nome — só queremos ler.
    pub fn new(id: u32, name: &str) -> Self {
        Student {
            id,
            name: name.to_string(), // &str -> String: converte a referência para owned String
            grades: Vec::new(),
        }
    }

    // &mut self — método que modifica o estado interno. Como this* em C++ mas explícito.
    // Devolve Some(grade) se aceite, None se inválida — o chamador sabe o que aconteceu.
    pub fn add_grade(&mut self, grade: f64) -> Option<f64> {
        if grade >= 0.0 && grade <= 20.0 {
            self.grades.push(grade);
            Some(grade)
        } else {
            None
        }
    }

    // &self — só lê, não modifica. O compilador garante que ninguém altera enquanto isto corre.
    // Retorna Option<f64> em vez de f64 porque pode não haver notas.
    // Não há null em Rust — Option<T> é o substituto seguro.
    pub fn average(&self) -> Option<f64> {
        if self.grades.is_empty() {
            return None; // ausência de valor, sem null pointer
        }
        let sum: f64 = self.grades.iter().sum();
        Some(sum / self.grades.len() as f64)
    }

    pub fn highest(&self) -> Option<f64> {
        // Iterator::copied() converte &f64 para f64; f64::NAN não suporta Ord então usamos partial
        self.grades.iter().copied().reduce(f64::max)
    }

    pub fn lowest(&self) -> Option<f64> {
        self.grades.iter().copied().reduce(f64::min)
    }

    pub fn letter_grade(&self) -> Option<LetterGrade> {
        // Option::map transforma Some(x) em Some(f(x)), e None permanece None
        self.average().map(LetterGrade::from_score)
    }

    pub fn passed(&self) -> bool {
        // if let — forma idiomática de "se houver valor dentro do Option, usa-o"
        if let Some(avg) = self.average() {
            avg >= 9.5
        } else {
            false
        }
    }

    pub fn grade_count(&self) -> usize {
        self.grades.len()
    }
}

// Implementar Display permite usar {} em println! com Student
impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self
            .average()
            .map(|v| format!("{:.1}", v))
            .unwrap_or_else(|| "sem notas".to_string()); // unwrap_or_else: valor default se None

        let grade = self
            .letter_grade()
            .map(|g| g.to_string())
            .unwrap_or_else(|| "-".to_string());

        let high = self
            .highest()
            .map(|h| format!("{:.1}", h))
            .unwrap_or_else(|| "sem notas".to_string());

        let low = self
            .lowest()
            .map(|l| format!("{:.1}", l))
            .unwrap_or_else(|| "sem notas".to_string());

        let count = format!("{}", self.grade_count());
        write!(
            f,
            "[{}] {} — média: {} ({}) — highest grade: {} — lowest grade: {} — count: {}",
            self.id, self.name, avg, grade, high, low, count
        )
    }
}
