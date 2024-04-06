use std::io;

fn add(x: i8, y: i8) -> i8 {
    dbg!(x + y);
    return x + y;
}

fn mul(x: i32, y: i32) -> i32 {
    let result: i32 = x * y;
    dbg!(result);
    return result;
}

fn substraction(x: i8, y: i8) -> i8 {
    dbg!(x - y);
    x - y
}

fn quot(x: i8, y: i8) -> f64 {
    dbg!(x / y);
    casting(x) / casting(y)
    // (x / y) as f64
    // x as f64 / y as f64
}

fn casting(x: i8) -> f64 {
    x as f64
}

fn and_percent(x: i8, y: i8) -> i8 {
    dbg!(x % y);
    x % y
}

fn submit() -> i8 {
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("error");
    x.trim().parse().expect("non number")
}

fn init() -> (i8, i8) {
    let x: i8 = submit();
    let y: i8 = submit();
    (x, y)
}

fn main() {
    let numbers: (i8, i8) = init();
    let add_result: i8 = add(numbers.0, numbers.1);
    let subs: i8 = substraction(numbers.0, numbers.1);
    let mul_result: i32 = mul(numbers.0.into(), numbers.1.into());
    let quot: f64 = quot(numbers.0, numbers.1);
    let per: i8 = and_percent(numbers.0, numbers.1);

    println!(
        "add : {}\nsubs : {}\nmul : {}\nquot : {:.2}\nper : {}",
        add_result, subs, mul_result, quot, per
    );
}

// fn main() {
//     let mut x: i8 = 3;
//     println!("default : {x}");
//     let arr: [i32; 5] = [1; 5];
//     x = 2;
//     {
//         x += 1;
//         println!("scope : {x}");
//     }
//     println!("change : {x}");
//     println!("array : {:?}", arr);

//     tupple();

//     add(1, 5);
// }

// fn tupple() {
//     let tp: (i8, char, bool) = (5, 'a', true);
//     println!("{:?}", tp);
// }

// fn add(x: i8, y: i8) -> i8 {
//     dbg!(x + y);
//     x + y
// }
