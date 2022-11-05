struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusernam123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("adifferentemail@example.com");

    // we moved user1 data to user2, no longer able to call user1
    let user2 = User {
        email: String::from("differentemail@example.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}