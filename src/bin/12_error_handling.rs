use std::fs;

// rust has internally written enum for try/catch. its called Result enum, and its like this.

// enum Result<T, E> {
//   Ok(T),
//   Err(E),
// }

//we dont need to write it, as error prone code already returns this.

//EXAMPLE 1: NORMAL
// fn main() {
//   let file = fs::read_to_string("example.txt");

//   match file {
//     Ok(content) => {
//       print!("content, {}", content)
//     }
//     Err(err) => {
//       print!("error, {}", err)
//     }
//   }
// }

//EXAMPLE 2: RETURN CUSTOM ERROR
fn main() {
  let file = read_file_unsafe("example.txt".to_string());

  match file {
    Ok(content) => {
      print!("content, {}", content)
    }
    Err(err) => {
      print!("error, {}", err)
    }
  }
}

fn read_file_unsafe(path: String) -> Result<String, String> {
  let file = fs::read_to_string(path);
  match file {
    Ok(content) => Ok(content),
    Err(_) => Err("error reading file".to_string()),
  }
}
