struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

// Tuple struct example
struct Color(u8, u8, u8);

// Unit struct example
struct AlwaysEqual;

fn main() {
  let user = User {
    active: true,
    email: String::from("tushar@email.com"),
    username: String::from("tushar"),
    sign_in_count: 1,
  };
  println!("User: {:?}", user.username);

  // Tuple struct usage
  let black = Color(0, 0, 0);
  println!("Color: ({}, {}, {})", black.0, black.1, black.2);

  // Unit struct usage - like classes in js, dont have value but they have functions associated with them we can use later. it is more useful when using with implementation (impl).
  let subject = AlwaysEqual;
  println!(
    "Unit struct created: {:?}",
    std::any::type_name::<AlwaysEqual>()
  );
}
