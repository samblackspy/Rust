fn main(){


    let string = String::from("hello world!");
    let length = get_string_length(string);
    println!("length of the string is {}", length)

}


fn get_string_length(str: String)-> usize {
    str.chars().count()

}