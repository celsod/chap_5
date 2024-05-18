fn main() {
    let user1 = User{ // can be mutable but the entire instance becomes mutable, not just a value
        active: true,
        username: String::from("someusername123"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };

    println!("The user's email is: {}", user1.email); //use dot notation to access struct

    let user2 = User{ //long form new user using user1's attributes
        active: user1.active,
        username: user1.username,
        sign_in_count: user1.sign_in_count,
        email: String::from("another@example.com"),
    };

    let user3 = User{ //struct update syntax; uses values from user1 that will not be changed
        email: String::from("third@example.com"), //email for user3 is only difference between 1 and 3
        ..user1 //must come last to specify remaining fields come from user1 struct
        //we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2
        //if both email and username were unique per user, then we would not have a problem due to data types
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

struct User { //data structure
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User { //long annoying way to define a function for a structure
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User { //field init shorthand
    User {
        active: true,
        username, //parameter and field are the same name; only need to write once
        email, //same thing on this line
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32); //tuple structures; each tuple is its own type even if the fields are the same types
struct Point(i32, i32, i32);

struct AlwaysEqual; //unit-like structure