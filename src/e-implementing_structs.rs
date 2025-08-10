struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn perimeter(&self) -> i32 {
       (2 * self.width)  + (2 * self.height)
    }
    fn debug()-> i32 {
        return 1;
    }
}

fn main(){
    let shape = Rectangle{
        width: 8,
        height: 4
    };
     println!("area of rectangle is {}", shape.area());
     println!("perimeter of rectangle is {}", shape.perimeter());
    println!("return debug without self: {}", Rectangle::debug());

    }