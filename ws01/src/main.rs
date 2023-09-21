use bmp;
use std::{
    fs,
    io::{stdout, Write},
};

// sum prod
struct Error {
    netwqork_err: Option<i32>,
    db_err: Option<i32>,
}

fn draw_pixel(path: &str) {
    fs::read(path).unwrap();
    let mut image = match bmp::open(path) {
        Ok(i) => {
            i.get_width();
        }
        Err(_) => {}
    };
    // Result::Ok
    // Result::Err
    // if let Ok(i) = bmp::open(path) { i.get_width();
    //     i.set_pixel(50, 50, bmp::Pixel::from(bmp::consts::RED));
    // };
    // i.save(path).expect("This should save correctly.");
    //
    // ...
}

fn main() {
    let path = dbg!(std::env::args().nth(1).expect("You must provide a path."));

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    // stdout().flush().unwrap();
    // let mut op = String::new();
    // std::io::stdin().read_line(&mut op).unwrap();
    //
    // match op.as_str() {
    //     "pixel\n" => draw_pixel(path.as_str()),
    //     //     ^ If you are on windows, try \r\n
    //     _ => {
    //         eprintln!("The operation {op} was not recognised!");
    //     }
    // }
}

// unwrap_or
// unwrap
// expect
// pattern match
// if let / let else
