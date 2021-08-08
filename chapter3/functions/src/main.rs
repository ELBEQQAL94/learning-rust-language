// main function
fn main() {
    // println!("Hello, world!");
    // another_function();
    // sum_a_b(1, 1);
    print!("The result is: {}", multiply_two_numbers(2, 2));
}

// other function
fn another_function() {
    println!("Hiiiiiiii");
}

// function took params
fn sum_a_b(a:i32, b:i32) {
    let sum = a + b;
    println!("Sum tow numbers is: {}", sum);
}

// function that return a value
fn multiply_two_numbers(a: i32, b:i32) -> i32{
    a * b
}

