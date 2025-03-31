static MY_X: i32 = 45;
static mut MY_Y: i32 = 66;

fn main() {
    let x = 55;
    println!("x: {x}");
    {
        // new scope
        let x = 22;
        println!("x: {x}");
    }
    // shadow first x
    let x = "33";
    println!("x: {x}");
    let x = "44";
    println!("x: {x}");

    println!("my_x: {MY_X}");
    unsafe {
        MY_Y = 18;
        // println!("my_y: {MY_Y}");
    }

    let my_tuple = (0, "hihi", 3.2);
    println!("my tuple: {} {} {}", my_tuple.0, my_tuple.1, my_tuple.2);

    // ownership
    // primitive type are copied by value
    let a = 2;
    let _b = a;
    println!("a: {}", a);

    let my_str = String::from("aa");
    let _transfer_my_str_ownership = my_str;
    // println!("my_str is gone {}", my_str);
}