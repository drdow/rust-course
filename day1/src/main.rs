fn main() {
    println!("Hello, Rust!"); // macro

    let x: i32 = 10;
    let y: u8 = 255; // max of u8 size
    let z: u16 = 1_000;
    let z1: u16 = 1_u16;
    let symbol: char = 'a';
    let string_var = String::from("this is string variable");
    let true_or_false_value: bool = false;

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("z1 = {}", z1);
    println!("symbol = '{}'", symbol);
    println!("string_var = '{}'", string_var);
    println!("true_or_false_value = {}", true_or_false_value);
    println!("sum of z + z1 = {}",sum(z as i32, z1 as i32)); // cast + function call
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}