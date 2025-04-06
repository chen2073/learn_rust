pub fn main() {
  // fn immutable ref in higher scope
  let s1 = "111".to_string();
  let s2 = "222".to_string();

  let fn_func = |s| {
    println!("{}", s1);
    println!("{}", s);
  };

  fn_func("aa".to_string());
  fn_func("bb".to_string());
  fn_func("cc".to_string());
  println!("{}", s1);
  println!("{}", s2);

  // fnMut, mutable ref in higher scope
  let mut s3 = "333".to_string();
  let mut fn_mut_func = |s| {
    println!("{}", s3);
    println!("{}", s);
    s3 = "nonono".to_string();
  };

  fn_mut_func("ddd".to_string());
  println!("{}", s3);

  // transfer ownership
  let s4 = "444".to_string();
  let fn_once_func = || {
    println!("{}", s4);
    std::mem::drop(s4);
  };
  fn_once_func();
  // println!("{}", s4); error out

  let s5 = "555".to_string();
  let move_func = move || {
    println!("{}", s5);
  };
  move_func();
  move_func();
  // println!("{}", s5); error out

  let a = 4;
  let f = |x ,y| x + y + a;
  let c = apply(f, 10, 20);
  println!("{c}");

  let mut aa = 100;
  twice_apply(|| {
    println!("deduct 20");
    aa -= 20;
  });
  println!("{aa}");

  let bb = "kk".to_string();
  fn_once(|| println!("{bb}"));
  println!("{bb}");
  fn_once(|| println!("{bb}"));
  println!("{bb}");
  fn_once(move || println!("{bb}"));
  // println!("{bb}"); bb is freed in prev func call
}

fn apply<T, F>(f: F, x: T, y: T) -> T where F: Fn(T, T) -> T {
  return f(x, y);
}

fn twice_apply<F>(mut f: F) -> () where F: FnMut() -> () {
  f();
  f();
}

fn fn_once<F>(f: F) -> () where F: FnOnce() -> () {
  f();
}
