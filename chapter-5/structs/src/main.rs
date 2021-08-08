struct User {
    username: String,
    email: String,
    sign_in_account: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("youssef@email.com"),
        username: String::from("youssef"),
        sign_in_account: 100,
        active: false,
    };
    let color = Color(0, 0, 0);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_account: 1,
        active: true,
    }
}
