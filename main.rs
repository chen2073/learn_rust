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
    
    // create a copy of "aa"
    let my_str_cp = my_str.clone();
    println!("my_str copy: {}", my_str_cp);
    
    // "aa" ownership: _transfer_my_str_ownership
    let _transfer_my_str_ownership = my_str;
    // println!("my_str is gone {}", my_str);

    // _transfer_my_str_ownership will be free after print_len
    print_len(_transfer_my_str_ownership);
    // println!("_transfer_my_str_ownership is gone {}", _transfer_my_str_ownership);

    let str_cp = retain_s_print_len(my_str_cp);
    println!("str_cp is gone {}", str_cp);

    str_slice_stack();
    let e = str_slice_vs_String();
    println!("{e}");

    print_str("abcdefg");
    print_str_borrow(&"what is this".to_owned());
}

fn print_len(s: String) -> () {
    // when s goes out of scope, memory for s is freed
    println!("string length is {}", s.len());
    println!("str is: {}", s);
}

fn retain_s_print_len(s: String) -> String {
    // when s goes out of scope, memory for s is freed
    println!("string length is {}", s.len());
    println!("str is: {}", s);
    return s.clone();
}

fn dangle_str() -> String {
    return "test".to_owned();
}

fn no_dangle_str() -> String {
    let a = "test".to_owned();
    // let b = a; cause ownership err
    return a;
}

// bad: created a static value "test" throughout the lifetime of program
fn dangle_str_static() -> &'static str {
    return "test"
}

fn str_slice_stack() {
    // This string literal is stored in the binary and has a 'static lifetime
    let static_str = "Hello, world!";

    // Here's a stack-allocated array of bytes
    let stack_array = [72, 101, 108, 108, 111]; // ASCII for "Hello"
    
    // Create a &str that points to our stack data
    // Safety: We need to ensure this is valid UTF-8
    let stack_str = unsafe { std::str::from_utf8_unchecked(&stack_array) };
    
    println!("Static &str: {}", static_str);
    println!("Stack &str: {}", stack_str);
    
    // Another example using byte literals
    let stack_bytes = *b"Hello"; // This creates a [u8; 5] array on the stack
    let another_stack_str = unsafe { std::str::from_utf8_unchecked(&stack_bytes) };
    
    println!("Another stack &str: {}", another_stack_str);
}

fn str_slice_vs_String() -> &'static str {
    // String are heap allocated
    let a = String::from("test");
    // convert &str to String
    let b = "test2".to_owned();
    // String::from("a") = "a".to_string() = "a".to_owned()
    let c = "abc";
    let d = "b\0";
    println!("d: {d}");
    let e: &'static str = "e";
    println!("e: {e}");
    return e;
}

fn lifetime_example() {
    struct Person<'a> {
        name: String,
        color: &'a str,
        age: i32,
    }

    let name = String::from("john");
    let color= "purple";
    let john = Person{name: name, color: color, age: 33};
}

// accepts &String, &str
fn print_str(s: &str) {
    println!("{s}");
}

// accepts &String only
fn print_str_borrow(s: &String) {
    println!("{s}");
}