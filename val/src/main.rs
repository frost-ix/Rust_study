use std::io;

fn sum(x: i8, y: i8) -> i8 {
    x + y
}

fn subtraction(x: i8, y: i8) -> i8 {
    x - y
}

fn production(x: i8, y: i8) -> i8 {
    x * y
}

fn quot(x: i8, y: i8) -> i8 {
    x / y
}

fn sumbit() -> i8 {
    let mut input_stream: String = String::new();
    io::stdin().read_line(&mut input_stream).expect("error");
    input_stream.trim().parse().expect("error return")
}

fn main() {
    let x: i8 = sumbit();
    let y: i8 = sumbit();
    let add: i8 = sum(x, y);
    let subt: i8 = subtraction(x, y);
    let prod: i8 = production(x, y);
    let quot: i8 = quot(x, y);
    println!("additional : {add}\nsubtration : {subt}\nproduction : {prod}\nquot : {quot}");

    // io::stdin().read_line(&mut input_stream).expect("error");
    // let res: i8 = sum(x, y);
    // println!("non-block : {res}");
}
