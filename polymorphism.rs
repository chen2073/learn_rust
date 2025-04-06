trait Chirp {
  fn chirp(&self) -> String;
}

trait ChirpSound {
  fn chirp_sound(&self) -> String;
}

trait Crow: ChirpSound {
  fn crow(&self) -> String;
}

struct Chicken {}

impl Chicken {
  fn sound(&self) -> String {
    return "guo".to_string();
  }
}

impl Chirp for Chicken {
  fn chirp(&self) -> String {
    return "chicken is chirping: ".to_string() + &self.chirp_sound();
  }
}

impl ChirpSound for Chicken {
  fn chirp_sound(&self) -> String {
    let sound = self.sound();
    let repeated = vec![sound; 3].join(" ");
    return repeated
  }
}

impl Crow for Chicken {
  fn crow(&self) -> String {
    return self.chirp_sound();
  }
}

fn make_it_chirp<T>(bird: &T) where T: Chirp {
  println!("{}", bird.chirp());
}

pub fn main() {
  let chicken = Chicken{};
  println!("crowing: {}", chicken.crow());
  make_it_chirp(&chicken);
}