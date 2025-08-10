// option enum

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main () {

    let thecharacter: String = String::from("and this is the string") ;
    let index: Option<i32> = find_first_a(thecharacter);

    match index {
        Some(value) => print!("i found you at: {}", value),
        None => print!(" i couldnt find you anywhere")
    }

}

fn find_first_a(s:String) -> Option<i32>{
    for (index, char) in  s.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}