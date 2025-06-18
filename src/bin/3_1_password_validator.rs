fn main() {
  let sentence = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";
  let password = String::from("WeakPass");
  let result = validate_password(password);
  println!("{:#?}", result);
  // let result = count_words(sentence);
  // print!("{}", result)
}

// =======================
// 📌 Arrays vs Vectors in Rust
// =======================

// ✅ Array: [T; N]
// - Fixed-size, known at compile time
// - Stored on stack
// - Cannot grow or shrink
// - No push/pop methods
// - Example:

// let arr: [i32; 3] = [1, 2, 3];

// ✅ Vector: Vec<T>
// - Dynamic size, growable at runtime
// - Stored on heap
// - Has push(), pop(), insert(), remove(), etc.
// - Most commonly used for lists
// - Example:

// let mut vec: Vec<i32> = vec![1, 2, 3];
// vec.push(4); // [1, 2, 3, 4]

// ✅ Key Differences:
// - Use array when size is fixed and known ahead
// - Use vector when size is dynamic or unknown

// ✅ Common methods:
// arr.len()       // ✔️ for both
// vec.push(x)     // ❌ Array, ✔️ Vector
// vec.pop()       // ❌ Array, ✔️ Vector
// vec.remove(i)   // ❌ Array, ✔️ Vector
// arr[i], vec[i]  // ✔️ Indexing works on both

#[derive(Debug)] // Add this line
struct ValidPassword {
  valid: bool,
  message: String,
  password: String,
}

//password validator
fn validate_password(password: String) -> ValidPassword {
  let mut messages: Vec<String> = vec![];
  let specials = password.chars().any(|item| !item.is_alphabetic());
  if !specials {
    messages.push(String::from(
      "Password doesn't contain any special character!",
    ));
  }
  let length: bool = password.len() > 8 && 128 > password.len();
  if !length {
    messages.push(String::from(
      "Password length must be between 8 to 128 characters",
    ));
  }
  let capital = password.chars().any(|c| c.is_uppercase());
  if !capital {
    messages.push(String::from("Password must contain any 1 capital letter"));
  }
  let numbers: bool = password.chars().any(|c| c.is_numeric());
  if !numbers {
    messages.push(String::from("Password must contain atleast 1 number"));
  }

  if specials && length && capital && numbers {
    return ValidPassword {
      message: String::from("password is valid"),
      valid: true,
      password,
    };
  } else {
    return ValidPassword {
      message: messages.join(", "),
      valid: false,
      password,
    };
  }
}
