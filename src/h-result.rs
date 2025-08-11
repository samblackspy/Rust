// result is used for error handling
// write a function that reads the contents of a file
// if the file doesn't exist, return an error
// if the file exists, return the contents of the file


use std::fs::read_to_string;


fn main(){
    let path:String = String::from("test.txt");
    functioncall(path);
}

fn functioncall(path:String){
    let result = read_to_string(path);
    match result {
        Ok(data)=> println!("read the file successfully here's the content inside: {}", data),
        Err(errormessage) => println!("found an error: {}", errormessage)
    }
}