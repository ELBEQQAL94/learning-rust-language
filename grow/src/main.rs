fn main() {
    //println!("The result is: {}", grow(vec![2, 2, 2, 2, 2, 2]));
    println!("{}", get_planet_name(9))
}

// fn grow (nums: Vec<i32>) -> i32 {
//     // let mut multiplication: i32 = 1;
//     // for number in nums {
//     //     multiplication *= number;
//     // }
//     // multiplication
//     //nums.iter().product()
//     nums.iter().fold(1, |acc, val| acc * val)
// }

fn get_planet_name(id: u32) -> String {
    match id {
        1 => "Mercury".to_string(),
        2 => "Venus".to_string(),
        3 => "Earth".to_string(),
        4 => "Mars".to_string(),
        5 => "Jupiter".to_string(),
        6 => "Saturn".to_string(),
        7 => "Uranus".to_string(),
        8 => "Neptune".to_string(),
        _ => unreachable!(),
    }
}

