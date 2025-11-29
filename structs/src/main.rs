struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new(username: String, email: String) -> User{
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    fn print_username(&self) {
        println!("{}", self.username);
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someon@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.email);
    user2.print_username();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
