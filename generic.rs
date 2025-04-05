pub fn main() {
  let p = Point{x: 1.0, y:2.0};
  println!("point: {:?}", p);
  let p2 = PolyPoint{x: 3.0, y: String::from("hihi")};
  println!("poly point: {:?}", p2);

  // rustc 1.85.1 (4eb161250 2025-03-15) cannot infer my_choice, hence annotation
  let my_choice: Choice<String, f64> = Choice::ChoiceA(String::from("what"));
  match my_choice {
    Choice::ChoiceA(x) => println!("choice a: {}", x),
    Choice::ChoiceB(x) => println!("choice b: {}", x)
  }

  let my_other_choice: Choice<String, i64> = Choice::ChoiceA(String::from("what"));
  let re = swap(String::from("iii"), 32.3);
}

#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

#[derive(Debug)]
struct PolyPoint<T, E> {
  x: T,
  y: E,
}

enum Choice<T, E> {
  ChoiceA(T),
  ChoiceB(E),
}

fn swap<T, E>(a: T, b: E) -> (E, T) {
  return (b, a);
}