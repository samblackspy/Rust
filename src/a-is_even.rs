// check if a given number is even or odd
fn main() {
    let num:i32 = 42;
    let answer = is_even(num);
    println!{ "{}", answer }
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
else {
    return false;
};
}