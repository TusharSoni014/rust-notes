use std::io;
use std::process::Command;

enum OptionChar {
  A,
  B,
  C,
  D,
}
struct Question {
  question: String,
  options: [String; 4],
  correct: OptionChar,
}
fn main() {
  let mut name = String::new();
  let mut score: u8 = 0;
  let questions: [Question; 5] = [
    Question {
      question: "something".to_string(),
      options: [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
      ],
      correct: OptionChar::A,
    },
    Question {
      question: "something 2".to_string(),
      correct: OptionChar::B,
      options: [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
      ],
    },
    Question {
      question: "something 3".to_string(),
      correct: OptionChar::C,
      options: [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
      ],
    },
    Question {
      question: "something 4".to_string(),
      correct: OptionChar::C,
      options: [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
      ],
    },
    Question {
      question: "something 5".to_string(),
      correct: OptionChar::C,
      options: [
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
      ],
    },
  ];
  println!("Welcome to Rust Quiz! v0.1");

  println!("Enter your name:");

  let user_inputs: Vec<OptionChar> = vec![];

  io::stdin()
    .read_line(&mut name)
    .expect("Failed to register the name.");

  clear_screen();
  println!("")
}

fn clear_screen() {
  Command::new("cmd").args(["/C", "cls"]).status().unwrap();
}

fn get_question(question_number: i8) {
  print!("")
}
