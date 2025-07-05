struct Rect {
  width: u32,
  height: u32,
}

// in rust, if the last line of the function dont have semi colon, then its a return statement, just like the below example.
impl Rect {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn perimeter(&self) -> u32 {
    2 * (self.width + self.height)
  }
}

//unit struct
struct Circle;

impl Circle {
  fn area(radius: f64) -> f64 {
    3.14159 * radius * radius
  }
}

fn main() {
  let rect = Rect {
    height: 12,
    width: 32,
  };
  println!("area of the rect is {}", rect.area());
  println!("perimeter of the rect is {}", rect.perimeter());

  println!("circle area: {}", Circle::area(5.0))
}
