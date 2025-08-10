struct User {
    name : String,
    email : String,
    age : i8,
}

fn main() {

    let example = User {
        name : String::from("Samblackspy"),
        email : String::from("example@gmail.com"),
        age : 21,
    };
    println!{"name of the user: {}", example.name};
    println!{"email of the user: {}", example.email};
    println!{"age of the user: {}", example.age};
}