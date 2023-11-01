fn main() {
    const SOMETHING_IMPORTANT: &str = "LUL";
    let mut x = 69;
    println!("The value of x is: {x}");
    x = 96;
    println!("The value of x is: {x}");
    println!("Dead code here: {SOMETHING_IMPORTANT}");
    let unsigned_integer: u8 = 255;
    let float_value: f32 = 15.0;
    let boolean: bool = true;
    let character: char = '🦀';
    let string_message: &str = "rust is cool";
    let fixed_tuple: (i8,f64,u16) = (127, 69.69, 65535);
    println!("{} {} {}",fixed_tuple.0,fixed_tuple.1,fixed_tuple.2);
    let (x,y,z) = fixed_tuple;
    println!("
        {unsigned_integer}
        {float_value}
        {boolean}
        {character},
        {string_message},
        {x} {y} {z}");
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let b = [69; 69];
    println!("{:?} {:?}",a,b);
    println!("good number {}",b[0]);
    another_function();
    function_that_takes_arguments(69);
    rust();
}

fn another_function() {
    println!("Another function.");
}
fn function_that_takes_arguments(x: u8){
    println!("Another function, but with arguments: {x}");
}
fn rust(){
    let logo: char = '🦀';
    println!("logo: {logo}");
}