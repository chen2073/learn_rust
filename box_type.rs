struct Point {
  x: i32,
  y: i32, 
}

#[derive(Debug, Clone, Copy)]
struct Point2 {
  x: i32,
  y: i32, 
}

pub fn main() {
  // wrapping in box type moves value from stack to heap
  let box_point = Box::new(Point{x: 32, y:22});
  println!("x: {}, y: {}", box_point.x, box_point.y);

  let mut box_int = Box::new(16);
  println!("int: {}", *box_int);
  *box_int += 10;
  println!("int: {}", *box_int);

  // transfer ownership, box_type is unusable
  let y = box_int;
  // println!("x: {}, y: {}", y.x, y.y); will error out

  let p2 = Point2{ x: 11, y: 66};
  // copy instead of transferring ownership
  let z = p2;
  println!("x: {}, y: {}", z.x, z.y);
}