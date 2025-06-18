// fn main() {
//   for mut i in 0..100 {
//     i += 1;
//     println!("{i}")
//   }
// }

fn main() {
  let sentence: String = String::from("something is happening");
  // eachCharacter(sentence);
  let word = first_word(sentence);
  print!("{word}")
}

fn count_words(sentence: &str) -> usize {
  let words: Vec<&str> = sentence.split_whitespace().collect();
  return words.len();
}

/*

(::) is called namespace operator or path seoerator

(String) type ek owned, heap-allocated, growable, UTF-8 encoded text type hai.

sentence.split_whitespace()	Splits into words

sentence.len()	Returns total bytes (not chars!)

*/

fn each_character(sentence: String) {
  if sentence.len() > 0 {
    sentence.chars().for_each(|f: char| println!("{f}"));
  }
}

// while defining functions, return type must be defined, it cannot be infered like typescript
fn first_word(sentence: String) -> String {
  let mut word: String = String::from("");
  for char in sentence.chars() {
    word.push_str(char.to_string().as_str());
    if char == ' ' {
      break;
    }
  }
  return word;
}
