trait Action {
  fn speak(&self);
  fn listen(&self);
  fn lose_name(self);
}

struct Player {
  name: String,
  age: i32,
}

impl Action for Player {
  fn speak(&self) {
    println!("name is {} whose age is {} is speaking", self.name, self.age);
  }

  fn listen(&self) {
    println!("name is {} whose age is {} is listening", self.name, self.age);
  }

  fn lose_name(self) {
    println!("name {} is gone from now", self.name)
  }
}

pub fn main() {
  let player = Player{name: String::from("john"), age: 32};
  player.speak();
  player.listen();

  // calling lose_name will result in name to be free
  player.lose_name();
  // println!("player name: {}", player.name);

  // immutable trait
  let obj = Obj{};
  get_obj_prop(&obj);
  println!("obj: {}", obj.get_prop());

  // ownership transfer
  let box_obj = Box::new(Obj{});
  println!("box obj before: {}", box_obj.get_prop());
  get_obj_prop_box(box_obj);
  // println!("box obj after: {}", box_obj.get_prop()); err out
  get_obj2_prop(Obj2{});
}

struct Obj {}
trait Property {
  fn get_prop(&self) -> String {
    return String::from("null");
  }
}

impl Property for Obj {
  fn get_prop(&self) -> String {
    return String::from("inertia");
  }
}

trait Property2 {
  fn get_prop2(&self) -> String;
}

struct Obj2 {}

impl Property for Obj2 {}
impl Property2 for Obj2 {
  fn get_prop2(&self) -> String {
    return String::from("mass");
  }
}

fn get_obj_prop(obj: &impl Property) {
  println!("obj property: {}", obj.get_prop());
}

fn get_obj_prop_box(obj: Box<dyn Property>) {
  println!("obj property: {}", obj.get_prop())
}

fn get_obj2_prop<T>(obj: T) where T: Property + Property2 {
  println!("property: {}", obj.get_prop());
  println!("property 2: {}", obj.get_prop2());
}