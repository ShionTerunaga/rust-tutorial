struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user = build_user(String::from("abc@example.com"), String::from("abc"));

    println!("emailは{}でnameは{}", user.email, user.username);

    println!(
        "サインイン回数は{}回で現在のアクティブかどうかは{}です",
        user.sign_in_count, user.active
    );
}
