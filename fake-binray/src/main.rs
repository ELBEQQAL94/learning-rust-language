fn main() {
    println!("{}", fake_binary("hello"));
}

fn fake_binary(x: &str) -> &str {
    let some = "45385593107843568".chars();
    let mut result: Vec<char> = Vec::new();

    for x in some {
        if x.to_digit(10) < 5 {
            result.push('0');
        }
        result.push(x);
    }

    let s: String = result.into_iter().collect();

    println!("{:?}", s);

    "Hello world"
}
