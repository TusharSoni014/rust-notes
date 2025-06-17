fn main() {
  let x: i32 = -5;
  let y: u32 = 5;
  let z: f32 = 5.12;
  println!("Hello, world!");
  print!("x:{x}, y:{y}, z:{z}")
}

/*

rustc filename.rs

= to convert rust to exe

*/

/*

# Rust Integer Notes - Signed vs Unsigned

## Unsigned Integers (u8, u16, u32, u64, u128)
- Only store positive numbers (including 0)
- Range: `0` to `2^n - 1`
- No negative values
- All bits used for value

Examples:
- u8  → 0 to 255         (2^8 = 256 values)
- u16 → 0 to 65,535      (2^16 = 65,536 values)
- u32 → 0 to 4,294,967,295

## Signed Integers (i8, i16, i32, i64, i128)
- Store both positive and negative numbers
- Range: `-2^(n-1)` to `2^(n-1) - 1`
- 1 bit reserved for sign (two's complement)

Examples:
- i8  → -128 to 127       (2^7 = 128 + 128 = 256 values)
- i16 → -32,768 to 32,767 (2^15 = 32,768 + 32,768 = 65,536 values)
- i32 → -2,147,483,648 to 2,147,483,647

## Quick Reference

| Type | Bits | Unsigned Range       | Signed Range                    |
|------|------|----------------------|---------------------------------|
| u8   | 8    | 0 to 255             | —                               |
| i8   | 8    | —                    | -128 to 127                     |
| u16  | 16   | 0 to 65535           | —                               |
| i16  | 16   | —                    | -32,768 to 32,767               |
| u32  | 32   | 0 to 4,294,967,295   | —                               |
| i32  | 32   | —                    | -2,147,483,648 to 2,147,483,647 |

## Tips
- Use `u*` when number will **never be negative** (e.g., index, length).
- Use `i*` when negative numbers are **possible** (e.g., temperatures, balances).

*/
