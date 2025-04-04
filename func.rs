pub fn main() {
  let mut x = 3;
  let y = mutable_param(x);
  println!("x in main: {}", y);

  let mut a: i32 = 9;
  println!("a before mutable ref: {}", a);
  mutable_reference(&mut a);
  println!("a after mutable ref: {}", a);

  let p = Point{x: 16, y: 17};
  immutable_reference(&p);
  println!("point is still alive, x: {}", p.x);

  let mut p2 = Point{x: 32, y:47};
  mutable_reference_point(&mut p2);
  println!("point is still alive, x: {}", p2.x);
}

// this will err out in compilation as x is immutable
// fn immutable_param(x: i32) -> i32 {
//   x += 1;
//   println!("x in immutable_param: {x}");
//   return x;
// }

// correct expression
fn mutable_param(mut x: i32) -> i32 {
  x += 1;
  println!("x in immutable_param: {x}");
  return x;
}

fn mutable_reference(x: &mut i32) -> () {
  *x += 1;
  println!("x in mutable_reference {}", *x);
  return;
}

// by default struct doesn't implement trait Copy and Clone
struct Point {
  x: i32,
  y: i32,
}

fn immutable_reference(p: &Point) -> () {
  println!("point x: {}", (*p).x);
}

fn mutable_reference_point(p: &mut Point) -> () {
  println!("point x: {}", (*p).x);
  (*p).x += 10;
}