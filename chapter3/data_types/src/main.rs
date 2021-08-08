fn main() {
    // declare integer
    let integer: u32 = 10;

    println!("The value of integer is: {}", integer);

    // declare float number
    let f: f32 = 4.0;

    println!("The value of f is: {}", f);

    // declare boolean 
    let is_valid:bool = false;
    println!("The value of is_valid is: {}", is_valid);

    // declare character
    let char = 'o';

    println!("The value of char is {}", char);

    // declare tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let first_vaulue = tup.0;
    let second_value = tup.1;
    let one = tup.2;

    println!("The value of x is: {}", z);

    println!("The value of one is: {}", one);

    // declare array
    // Npte: use array if you have a fixed length of elements such as months or 
    // sepecific names in your programme
    let arr = [1, 2, 3];
}
