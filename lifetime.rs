pub fn main() {
  let s1 = "what is this";
  let s2 = "hello world...........";
  println!("longer str: {}", longer_str(s1, s2));
  println!("s1: {}", s1);
  println!("s2: {}", s2);

  println!("longer str 2: {}", longer_str2(s1, s2));
  println!("s1: {}", s1);
  println!("s2: {}", s2);

  // s3 has a different lifetime than s1, s2
  let s3 = String::from("another life");
  println!("longer str: {}", longer_str(s1, &s3));
  println!("longer str 2: {}", longer_str2(s1, &s3));
}

// same life time for s1 s2
fn longer_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  if s1.len() > s2.len() {
    return s1;
  }

  return s2;
}

// better performance
fn longer_str2<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str where 'a: 'out, 'b: 'out {
  if s1.len() > s2.len() {
    return s1;
  }

  return s2;
}