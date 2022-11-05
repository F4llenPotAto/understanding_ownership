struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "trenton@gmail.com",
        username: "trentonornelas",
        active: true,
        sign_in_count: 1,
    };
}