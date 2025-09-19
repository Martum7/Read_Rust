// src/grade.rs
//
// Enums em Rust são muito mais poderosos do que em C/C++/Java.
// Cada variante pode carregar dados diferentes — é uma "sum type" / tagged union.
// Aqui o enum é simples, mas na lição seguinte verás variantes com dados associados.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LetterGrade {
    F, // Reprovado       (0–9.4)
    D, // Suficiente      (9.5–11.9)
    C, // Bom             (12–13.9)
    B, // Muito Bom       (14–16.9)
    A, // Excelente       (17–20)
}

impl LetterGrade {
    pub fn from_score(score: f64) -> Self {
        // match é como switch mas exaustivo — o compilador obriga-te a cobrir todos os casos.
        // Usa ranges com "if" guard em vez de fall-through (não há fall-through em Rust).
        match score {
            17.0.. => LetterGrade::A,
            14.0..=16.99 => LetterGrade::B,
            12.0..=13.99 => LetterGrade::C,
            9.5..=11.99 => LetterGrade::D,
            _ => LetterGrade::F, // _ é wildcard: "qualquer outro valor"
        }
    }

    pub fn description(&self) -> &'static str {
        // &'static str — referência para string literal que vive durante todo o programa.
        // O lifetime 'static é o compilador a garantir que o dado não desaparece.
        match self {
            LetterGrade::A => "Excelente",
            LetterGrade::B => "Muito Bom",
            LetterGrade::C => "Bom",
            LetterGrade::D => "Suficiente",
            LetterGrade::F => "Reprovado",
        }
    }
}

impl std::fmt::Display for LetterGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // self aqui é &LetterGrade — uma referência imutável ao enum
        let letter = match self {
            LetterGrade::A => "A",
            LetterGrade::B => "B",
            LetterGrade::C => "C",
            LetterGrade::D => "D",
            LetterGrade::F => "F",
        };
        write!(f, "{}", letter)
    }
}
