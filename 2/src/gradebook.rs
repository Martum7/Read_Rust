// src/gradebook.rs
//
// HashMap<K, V> — equivalente a std::unordered_map em C++ ou HashMap em Java.
// Aqui usamos u32 como chave (student id) e Student como valor.

use crate::grade::LetterGrade;
use crate::student::Student;
use std::collections::HashMap;

pub struct GradeBook {
    course: String,
    students: HashMap<u32, Student>,
}

impl GradeBook {
    pub fn new(course: &str) -> Self {
        GradeBook {
            course: course.to_string(),
            students: HashMap::new(),
        }
    }

    // Toma ownership do Student — depois de chamar enroll, o chamador já não tem o Student.
    // Isto é diferente de C++ (onde seria uma cópia ou move explícito) e de Java (onde é sempre referência).
    pub fn enroll(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    fn remove(&mut self, id: u32) -> Option<Student> {
        self.students.remove(&id)
    }

    // Devolve Option<&Student> — referência imutável. Pode ser None se o id não existir.
    // O lifetime implícito aqui garante que a referência não vive mais do que o GradeBook.
    pub fn find(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Student> {
        self.students.get_mut(&id)
    }

    pub fn class_average(&self) -> Option<f64> {
        let averages: Vec<f64> = self
            .students
            .values() // itera os valores do HashMap (sem keys)
            .filter_map(|s| s.average()) // filter_map: ignora None, desempacota Some
            .collect(); // materializa o iterator num Vec

        if averages.is_empty() {
            return None;
        }
        Some(averages.iter().sum::<f64>() / averages.len() as f64)
    }

    // Devolve referências para os N melhores alunos, ordenados por média descendente.
    // Vec<&Student> — vector de referências. Nenhum dado é copiado.
    pub fn top_students(&self, n: usize) -> Vec<&Student> {
        let mut ranked: Vec<&Student> = self
            .students
            .values()
            .filter(|s| s.average().is_some())
            .collect();

        // sort_by é como std::sort com comparador. partial_cmp porque f64 não implementa Ord total.
        ranked.sort_by(|a, b| {
            b.average()
                .unwrap()
                .partial_cmp(&a.average().unwrap())
                .unwrap()
        });

        ranked.into_iter().take(n).collect()
    }

    pub fn failing_students(&self) -> Vec<&Student> {
        self.students.values().filter(|s| !s.passed()).collect()
    }

    // Agrupa alunos por letra de nota usando um HashMap de saída.
    pub fn by_letter_grade(&self) -> HashMap<LetterGrade, Vec<&Student>> {
        let mut groups: HashMap<LetterGrade, Vec<&Student>> = HashMap::new();

        for student in self.students.values() {
            if let Some(grade) = student.letter_grade() {
                // entry().or_insert_with() — insere Vec vazio se a chave não existir, depois faz push
                groups.entry(grade).or_insert_with(Vec::new).push(student);
            }
        }
        groups
    }

    pub fn summary(&self) {
        println!("═══════════════════════════════════════");
        println!("  Cadeira: {}", self.course);
        println!("  Alunos inscritos: {}", self.students.len());

        match self.class_average() {
            Some(avg) => println!("  Média da turma: {:.2}", avg),
            None => println!("  Média da turma: sem dados"),
        }

        let passing = self.students.values().filter(|s| s.passed()).count();
        let failing = self.students.len() - passing;
        println!("  Aprovados: {}  |  Reprovados: {}", passing, failing);
        println!("═══════════════════════════════════════");

        println!("\nTop 3 alunos:");
        for (i, s) in self.top_students(3).iter().enumerate() {
            println!("  {}. {}", i + 1, s);
        }

        println!("\nDistribuição por nota:");
        let mut groups: Vec<_> = self.by_letter_grade().into_iter().collect();
        groups.sort_by_key(|(grade, _)| std::cmp::Reverse(grade.clone()));
        for (grade, students) in &groups {
            let names: Vec<&str> = students.iter().map(|s| s.name.as_str()).collect();
            println!(
                "  {}: {} aluno(s) — {}",
                grade,
                students.len(),
                names.join(", ")
            );
        }
    }
}
