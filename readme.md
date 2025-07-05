# Rust Learning Journey - 4 Hour Bootcamp

This repository contains a comprehensive collection of Rust programming examples and exercises covering fundamental to intermediate concepts. Each file in the `src/bin/` directory represents a standalone program that can be run individually.

## Repository Structure

```
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── rustfmt.toml        # Code formatting configuration
├── .vscode/            # VSCode settings
└── src/bin/            # Executable programs
    ├── 1_numbers.rs
    ├── 2_bool.rs
    ├── 3_loops.rs
    ├── 3_1_password_validator.rs
    ├── 4_inputs.rs
    ├── 4_1_quiz.rs
    ├── 5_stack_heap.rs
    ├── 6_ownership.rs
    ├── 7_borrowing_references.rs
    ├── 8_struct.rs
    ├── 9_impl.rs
    ├── 10_enums.rs
    ├── 11_pattern_matching.rs
    ├── 12_error_handling.rs
    ├── 13_option_enum.rs
    └── 14_cargo.rs
```

## Topics Covered

### 1. **Numbers and Data Types** ([1_numbers.rs](src/bin/1_numbers.rs))
- Signed vs Unsigned integers (i8, i16, i32, i64, i128 vs u8, u16, u32, u64, u128)
- Floating-point numbers (f32, f64)
- Type annotations and inference
- Number ranges and bit representation

### 2. **Boolean Logic and Options** ([2_bool.rs](src/bin/2_bool.rs))
- Boolean data type
- Conditional statements (if/else if/else)
- Introduction to `Option<T>` enum
- Pattern matching with `match`
- `unwrap()` vs proper error handling

### 3. **Loops and String Processing** ([3_loops.rs](src/bin/3_loops.rs))
- For loops with ranges
- String manipulation methods
- Character iteration
- Function definitions with return types
- String vs &str differences

### 4. **Password Validator Project** ([3_1_password_validator.rs](src/bin/3_1_password_validator.rs))
- Practical application combining multiple concepts
- Arrays vs Vectors comparison
- String validation logic
- Struct definitions with `#[derive(Debug)]`
- Method chaining and character analysis

### 5. **User Input Handling** ([4_inputs.rs](src/bin/4_inputs.rs))
- Reading from stdin using `std::io`
- String parsing and type conversion
- Error handling with `Result<T, E>`
- `unwrap()` vs `expect()` vs `match`
- Input validation techniques

### 6. **Quiz Application** ([4_1_quiz.rs](src/bin/4_1_quiz.rs))
- Interactive console application
- Enum definitions for multiple choice
- Struct usage for complex data
- System commands (clearing screen)
- User interaction flow

### 7. **Memory Management - Stack vs Heap** ([5_stack_heap.rs](src/bin/5_stack_heap.rs))
- Stack memory characteristics (LIFO, fixed-size)
- Heap memory allocation (dynamic, flexible)
- Memory pointer tracking
- String capacity and reallocation
- Performance implications

### 8. **Ownership System** ([6_ownership.rs](src/bin/6_ownership.rs))
- Rust's unique ownership model
- Move semantics
- Ownership transfer between functions
- Returning ownership from functions
- Memory safety without garbage collection

### 9. **Borrowing and References** ([7_borrowing_references.rs](src/bin/7_borrowing_references.rs))
- Immutable references (`&T`)
- Mutable references (`&mut T`)
- Borrowing rules and restrictions
- Multiple borrowing scenarios
- Race condition prevention

### 10. **Structs and Data Structures** ([8_struct.rs](src/bin/8_struct.rs))
- Classic structs with named fields
- Tuple structs
- Unit structs
- Struct instantiation and field access
- Debug trait implementation

### 11. **Implementation Blocks** ([9_impl.rs](src/bin/9_impl.rs))
- Adding methods to structs
- `&self` parameter usage
- Associated functions vs methods
- Multiple impl blocks
- Return value syntax (no semicolon)

### 12. **Enums and Variants** ([10_enums.rs](src/bin/10_enums.rs))
- Basic enum definitions
- Enums with associated data
- Variant instantiation
- Enum usage patterns

### 13. **Pattern Matching** ([11_pattern_matching.rs](src/bin/11_pattern_matching.rs))
- Exhaustive pattern matching with `match`
- Enum destructuring
- Extracting values from enum variants
- Compile-time completeness checking

### 14. **Error Handling** ([12_error_handling.rs](src/bin/12_error_handling.rs))
- `Result<T, E>` enum for error handling
- File operations and error propagation
- Custom error types
- Error handling patterns

### 15. **Option Enum** ([13_option_enum.rs](src/bin/13_option_enum.rs))
- `Option<T>` for nullable values
- `Some(T)` and `None` variants
- Safe null handling
- String searching example

### 16. **External Crates and Cargo** ([14_cargo.rs](src/bin/14_cargo.rs))
- Adding dependencies with `cargo add`
- Using external crates (`chrono`, `rand`)
- Date/time handling
- Random number generation

## How to Run

Each file can be run individually using Cargo:

```bash
# Run a specific program
cargo run --bin 1_numbers
cargo run --bin password_validator
cargo run --bin quiz

# Or compile and run manually
rustc src/bin/1_numbers.rs
./1_numbers
```

## Key Learning Points

- **Memory Safety**: Understanding stack vs heap allocation
- **Ownership**: Rust's unique approach to memory management
- **Borrowing**: Safe reference handling without garbage collection
- **Pattern Matching**: Exhaustive and safe enum handling
- **Error Handling**: Explicit error types with `Result` and `Option`
- **Type System**: Strong static typing with inference
- **Performance**: Zero-cost abstractions and predictable performance

## Prerequisites

- Rust toolchain installed (rustc, cargo)
- Basic programming concepts understanding
- Text editor or IDE (VSCode configuration included)

This repository serves as a comprehensive introduction to Rust programming, covering essential concepts through practical examples and projects.