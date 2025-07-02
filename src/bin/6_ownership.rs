// https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-13

// fn main() {
//   let x = 1; // crated on stack
//   new scope
//   {
//     let y = 3; // created on stack
//   }

//   println!("{}", y); // throws error
// }

// anything that will store on heap will have a owner, and every variable in the heap will have only one owner at a time, and if owner (store in stack), goes out of scope then heap variable dies and it deallocates from memory.

// fn main() {
//   let s1 = String::from("hello");
//   let s2 = s1;
//   println!(s1) // throws error, cuz owner is changed
// }

// EXAMPLE 1: STRING OWNERSHIP TAKEN
// fn main() {
//   let string: String = String::from("hello");
//   takes_ownership(string);
//   println!("{}", string);
// }

// fn takes_ownership(string: String) {
//   println!("{}", string);
// }


// EXAMPLE 2: STRING OWNERSHIP TAKEN AND RETURNED
fn main() {
  let string: String = String::from("hello");
  let string2 = takes_and_gives_back(string);
  println!("{}", string2);
}

fn takes_and_gives_back(string: String) -> String {
  println!("{}", string);
  return string;
}
