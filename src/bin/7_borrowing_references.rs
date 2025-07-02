// in rust, a variable can be borrowed by another variable unlimited times using references (&)

// borrowing references to another function
// fn main() {
//   let string: String = String::from("hello");
//   borrow_string(&string);
//   println!("{}", string);
// }

// fn borrow_string(string: &String) {
//   println!("{}", string);
// }

// unless we mutate the reference, we can borrow it as many times as we want
// fn main() {
//   let s1 = String::from("hello"); // ownership remaines to s1 even after borrowing
//   let s2 = &s1; // s2 borrows s1 by saving a reference to s1 (pointer to s1)
//   let s3 = &s1;
//   println!("{}", s1);
//   println!("{}", s2);
//   println!("{}", s3);
// }

// #### MUTABLE REFERENCES

// example 1:
// fn main() {
//   let mut s1 = String::from("hello");
//   change_string(&mut s1);
//   println!("{}", s1);
// }  

// fn change_string(string: &mut String) {
//   string.push_str(", world");
// }

// example 2:
// es example me, s2 and s3 use kiye h yani print hue h toh compilation error aayega
// IT DOES NOT COMPILE BECAUSE WE ARE USING THE REFERENCES TO PRINT THEM.
// fn main() {
//   let mut s1 = String::from("hello");
//   let s2 = &mut s1;
//   let s3 = &mut s1; // cannot borrow as mutable more than once at a time
//   let s4 = &s1; // cannot borrow as normal reference as well more than once at a time
//   println!("{}", s1);
//   println!("{}", s2);
//   println!("{}", s3);
//   println!("{}", s4);
// }


// example 3:
// es example me, s2 and s3 use NAHI kiye h yani print hue h toh compilation error NAHI aayega.
// IT COMPILES BECAUSE WE ARE NOT USING THE REFERENCES TO PRINT THEM.
fn main() {
  let mut s1 = String::from("hello");
  let s2 = &mut s1;
  let s3 = &mut s1; // cannot borrow as mutable more than once at a time
  let s4 = &s1; // cannot borrow as normal reference as well more than once at a time
  println!("{}", s1);
  println!("{}", s4);
}

// WHY WE CANNOT HAVE REFERENCES OF A VARIABLE MORE THAN ONCE WHEN ITS MUTABLE?
// ANS: BECAUSE IN THE ABOVE EX, S4 EXPECTS A IMMUTABLE REFERENCE TO S1, BUT S2 AND S3 ARE MUTABLE REFERENCES TO S1, SO S4 WILL EXPECT S1 NOT TO BE CHANGED, BUT S2 AND S3 ARE CHANGING IT, HENCE IT CAN PANIC THE COMPILER SO WE CANNOT HAVE REFERENCES OF A VARIABLE MORE THAN ONCE WHEN ITS MUTABLBLY REFERENCED TO A VARIABLE. SECOND REASON IS PREVENTING RACE CONDITIONS, IF TWO REFERENCED VARIABLES ARE MUTATING THE SAME VARIABLE AT THE SAME TIME, IT CAN LEAD TO RACE CONDITIONS.

// NOTE: THE ABOVE EXAMPLE IS COMPILED BECAUSE WE ARE NOT USING THE REFERENCES TO PRINT THEM. HENCE NOT MUTATING THE VARIABLE.
