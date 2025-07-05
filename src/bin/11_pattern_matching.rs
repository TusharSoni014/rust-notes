enum Shapes {
  Circle(f32),
  Square(f32),
  Rectangle(f32, f32),
}

//pattern matching reqiures all the cases, if we miss any one case, then it will fail.
fn calculate_area(shape: Shapes) -> f32 {
  let ans = match shape {
    Shapes::Circle(radius) => 3.14 * radius * radius,
    Shapes::Rectangle(width, height) => width * height,
    Shapes::Square(side) => side * side,
  };
  return ans;
}

fn main() {
  let circle = Shapes::Circle(32.0);
  let square = Shapes::Square(12.0);
  let rectangle = Shapes::Rectangle(12.0, 32.0);

  println!("Circle: {:?}", calculate_area(circle));
  println!("Square: {:?}", calculate_area(square));
  println!("Rectangle: {:?}", calculate_area(rectangle));
}
