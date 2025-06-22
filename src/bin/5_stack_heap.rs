fn main() {
  stack_fn(); // Call the function that uses stack memory
  heap_fn(); // Call the function that uses heap memory
  update_string(); // Call the function that changes size of variable at runtime
}

fn stack_fn() {
  // Declare a few integers on the stack
  let a = 10;
  let b = 20;
  let c = a + b;
  println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
  // Create a string, which is allocated on the heap
  let s1 = String::from("Hello");
  let s2 = String::from("World");
  let combined = format!("{} {}", s1, s2);
  println!("Heap function: Combined string is '{}'", combined);
}

// if there is a variable that needs to be stored in the heap, then the stack stores, len, capacity, and pointer (location to the data in the heap of where data is stored), then the len, capacity and pointer changes when the data is being updated.

// pointer address only changes when the initial location in memory cannot ask for more space from the prev location and hence it allocates new pointer where the new updated data is stored in heap
fn update_string() {
  // Start with a base string on the heap
  let mut s = String::from("Initial string");
  println!("Before update: {}", s);
  println!(
    "Capacity: {}, length: {}, Pointer:{:p}",
    s.capacity(),
    s.len(),
    s.as_ptr()
  );

  // Append some text to the string
  s.push_str(" and some additional text");
  println!("After update: {}", s);
  println!(
    "Capacity: {}, length: {}, Pointer:{:p}",
    s.capacity(),
    s.len(),
    s.as_ptr()
  );

  for _ in 0..100 {
    s.push_str(" and some additional text");
    println!(
      "Capacity: {}, length: {}, Pointer:{:p}",
      s.capacity(),
      s.len(),
      s.as_ptr()
    );
  }
}

// 🧠 Stack vs Heap in Rust (Short Notes)

// 🗂 Stack:
// - Fast, LIFO memory (Last In First Out).
// - Stores fixed-size data (e.g., integers, function calls).
// - Automatically allocated and deallocated.
// - Example: let x = 5; // stored on the stack

// 📦 Heap:
// - Slower, flexible memory for dynamically sized data.
// - Must request memory (e.g., Box, String, Vec).
// - Manually managed via ownership and borrowing.
// - Example: let s = String::from("hello"); // string data is stored on heap

// 🔁 Ownership in Rust ensures safe heap usage without garbage collector.

// stack pushed a unique stack frame in memory for each specific function and inside that stack frame for unique function, it stores all the variables.
// if a function is calling another function that passes and uses the same variable from main function, then a new stack frame will be created in the memory.
