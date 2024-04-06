use std::io;

fn loop_dan(x: i32, y: i32) {
    for x in x..y + 1 {
        for t in 1..10 {
            println!("{x} * {t} = {}", x * t);
        }
    }
}

fn submit() -> i32 {
    loop {
        let mut x: String = String::new();
        io::stdin().read_line(&mut x).expect("error");
        let check: i32 = match x.trim().parse::<i32>() {
            Ok(x) => x,
            Err(_) => 0,
        };
        if check != 0 {
            println!("{} is a number", check);
            return check;
        } else {
            println!("Not number. Try again.");
            continue;
        }
    }
}

fn num_init() -> (i32, i32) {
    let x: i32 = submit();
    let y: i32 = submit();
    (x, y)
}

fn main() {
    let numbs: (i32, i32) = num_init();
    println!("{}, {}", numbs.0, numbs.1);
    loop_dan(numbs.0, numbs.1);
}
