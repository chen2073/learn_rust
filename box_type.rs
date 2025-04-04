struct Point {
  x: i32,
  y: i32, 
}

pub fn main() {
  let box_point = Box::new(Point{x: 32, y:22});
  println!("x: {}, y: {}", box_point.x, box_point.y);

  let mut box_int = Box::new(16);
  println!("int: {}", *box_int);
  *box_int += 10;
  println!("int: {}", *box_int);
}