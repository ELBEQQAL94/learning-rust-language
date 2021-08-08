use std::io;

fn main() {
    println!("Hello ---- temperature converter----");
    println!("___________________________");
    println!("Please enter your temperature:");

    let user_input = get_user_input();
    let convert_user_input = convert_user_input(user_input);
    // ask user what convertion you want
    // give hime option and let him choose a number based on the option
    println!("Please choose a number: ");
    println!("1 for f");
    println!("2 for c");
    let symbol_input = get_user_input();

    if symbol_input == "1" {
        let convert_c_to_f = convert_c_to_f(convert_user_input);
        println!("Your temperature from f to c is: {}", convert_c_to_f);
    } else {
        let convert_f_to_c = convert_f_to_c(convert_user_input);
        println!("Your temperature from f to c is: {}", convert_f_to_c);
    }
}

fn get_user_input() -> String{
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed please try again"); 
    return String::from(user_input.trim());
}

// convert user input from string type to valid
// temperatue number
fn convert_user_input(user_input: String) -> f32{
    user_input.parse::<f32>()
        .expect("Please enter a number.")
}

fn convert_f_to_c(temperature: f32) -> f32 {
    (temperature - 32.0) * (5.0 / 9.0)
}

fn convert_c_to_f(temperature: f32) -> f32 {
    (temperature * (9.0 / 5.0)) + 32.0
}
