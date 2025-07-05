fn find_first_a(s: String) -> Option<usize> {
  for (index, character) in s.chars().enumerate() {
    if character == 'a' {
      return Some(index);
    }
  }
  return None;
}

fn main() {
  let my_str = String::from("this is a cat");
  let result = find_first_a(my_str);

  match result {
    Some(index) => print!("this letter a is found at index: {}", index),
    None => print!("this letter a is not found in the provided string"),
  }
}
