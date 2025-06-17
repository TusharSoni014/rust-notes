fn main() {
  let first: i32 = 12;
  let string = String::from("something"); //Type: String
  let sliced = string.chars().nth(100); // Type Option<char>, optional kyuki compiler doesnt know ki value aaegi bhi ya nahi

  println!("x: {}", sliced.unwrap()); // either this, (not recommended)
  //or this

  match sliced {
    Some(ch) => print!("x:{}", ch),
    None => print!("no character at position"),
  }
  //this is recommended way
  
  let is_male: bool = true;
  let is_above_18: bool = true;

  if is_male {
    print!("you are male")
  } else if is_above_18 {
    print!("you are above 18")
  } else if is_male && is_above_18 {
    print!("you are 18 and male")
  }
}
