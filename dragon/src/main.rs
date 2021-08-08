fn main() {
    println!("{}", dragon(7, 4));
}

fn dragon(bullets: u16, dragons: u16) -> bool {
    let result = bullets / dragons;
    if result >= 2 {
        return true;
    }
    return false;
}
