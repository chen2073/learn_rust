pub fn main() {
  let mut s = String::from("hello");
  // immutable reference: allow multiple ref
  let s1 = &s;
  let s2 = &s;
  println!("s1: {}, s2: {}", s1, s2);

  // mutable reference: allow only one ref, revoke immutable ref
  let s3 = &mut s;
  println!("s3: {}", s3);
  // println!("s1: {}, s2: {}", s1, s2); not allowed
  let s4 = &mut s;
  println!("s4: {}", s4);

  // lifetime
  let mut re: &str;
  {
    // initialization allowed in nested scope
    re = "abc";
    // declaration not visible outside scope
    let s5 = &s;
    re = assign(s5);
  }
  println!("re: {}", re);
  // println!("s5: {}", s5); not allow
}

fn assign<'a>(s: &'a str) -> &'a str {
  return s;
}