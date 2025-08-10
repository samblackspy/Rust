// enums: use it instead of strings to make it typesafe 
//so that the function which calls the enum 
// can't pass any other string; 
//if you know something can only have 5 types, 2 types, etc., 
// use an enum to restrict whoever is calling a function 
//on that specific type to one of those types
 

enum Shapes {
    Rectangle(f64, f64),
    Circle(f64),
    
 }

fn main(){

    let shape1 : Shapes = Shapes::Rectangle(12.0, 4.0);
    let shape2 : Shapes = Shapes::Circle(6.0);
    println!("area of shape1 is : {}", get_area(shape1));
    println!("area of shape2 is : {}", get_area(shape2))

}

fn get_area(shape: Shapes) -> f64 {
    let area = match shape {
        Shapes::Rectangle(a, b)=> a *b ,
        Shapes::Circle(r) => r * 3.14 * r,
    };
    return area;
}