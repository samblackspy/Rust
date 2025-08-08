//find the fibonacci series of a given number
// 
fn main(){
    let num:i64 = 12;
    let ans = fibonacci_series(num);

    println!{"{}", ans }

}

fn fibonacci_series(num: i64)->  i64{
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    } 
    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;


}