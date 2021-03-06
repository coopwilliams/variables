fn main() {
    let mut user1 = User {
        email: String::from("boink@doink.com"),
        username: String::from("boink"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("sloink@doink.com");

    let user2 = User {
        email: String::from("clunk@bunk.com"),
        username: String::from("clunk"),
        ..user1
    }

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn build_user(email: String, username: String) => User {
    User {
        email, // we can use field init shorthand by giving params the same names as fields
        username,
        active: true,
        sign_in_count: 1,
    }
}

