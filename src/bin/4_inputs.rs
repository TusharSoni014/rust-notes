use std::io;
fn main() {
  let mut name = String::new();

  println!("Enter your name:");
  io::stdin()
    .read_line(&mut name)
    .expect("Failed to read line");

  let input = name.trim();
  print!("hello, {input}")
}


/*
📌 Rust Input & Error Handling Notes (Simplified)

✅ input.parse::<i32>() gives a Result<T, E>
You have two ways to handle it:

1️⃣ Using match (manual handling)
let result = input.parse::<i32>();

match result {
    Ok(num) => println!("Parsed number: {}", num),    // success case
    Err(e) => println!("Oops! Couldn't parse: {}", e), // error case
}
→ Match gives full control on what to do for Ok and Err

2️⃣ Using unwrap or expect (auto panic)
let num = input.parse::<i32>().unwrap(); // panics if input is invalid
let num = input.parse::<i32>().expect("Invalid number"); // panics with message

🔸 unwrap/expect are just shortcuts for match when you don’t need custom error handling

🔍 Summary:
- use match when you want to do something on error
- use unwrap/expect when you just want to crash if error occurs
- unwrap = panic with default msg
- expect = panic with your msgF

*/