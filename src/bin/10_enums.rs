// EXAMPLE 1:
// enum Direction {
//   East,
//   West,
//   North,
//   South,
// }

// fn main() {
//   move_around(Direction::North)
// }

// fn move_around(direction: Direction) {}



// EXAMPLE 2: enums with values
enum Shapes {
  Circle(f32),
  Square(f32),
  Rectangle(f32, f32),
}

fn main() {
  let circle = Shapes::Circle(32.0);
  let square = Shapes::Square(12.0);
  let rectangle = Shapes::Rectangle(12.0, 32.0);
}
