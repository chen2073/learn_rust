pub fn main() {
    println!("Hello, what's your name");
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).expect("unable to read stdin");

    if buffer == "Herbert" {
        println!("hello Herbert")
    } else {
        println!("you are not authorized")
    }
}