use bmp::{self, open, BmpError, Image, Pixel};
use std::io::{stdout, Write};

enum MyError {}
type MyResult<T> = Result<T, MyError>;

// enum Result<T, E> {
//     Ok(Image),
//     Err(BmpError),
// }

fn draw_pixel(path: &str) {
    // getting value
    // no value => default withotu handing error
    let open_result: Result<Image, BmpError> = bmp::open(path);
    match bmp::open(path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Error: {}", err);
            Image::new(100, 100)
        }
    };
    //
    let mut image = open_result.unwrap_or(bmp::Image::new(100, 100));
    image.set_pixel(50, 50, bmp::Pixel::new(255, 255, 255));

    image.save(path).expect("This should save correctly.");
}

fn stripes(path: &str) {
    let mut image = bmp::open(path).unwrap_or(Image::new(100, 100));
    for x in 0..image.get_width() {
        for y in 0..image.get_height() {
            let color = match x % 2 == 0 {
                true => Pixel::new(255, 255, 255),
                false => Pixel::new(0, 0, 0),
            };
            image.set_pixel(x, y, color);
        }
    }
    image.save(path).expect("This should save correctly.");
}

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();

    match op.as_str() {
        "pixel\n" => draw_pixel(path.as_str()),
        //   ^ \r\n "carrigare returns" on windows
        "stripes\n" => stripes(path.as_str()),
        _ => {
            eprintln!("The operation {op} was not recognised!");
        }
    }
    println!("Done withotu error");
}

// null NULL
// SEGFAULT
// unwrapping -
// match
// if-let

enum Option<T> {
    Some(T),
    None,
}

impl Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(value) => value,
            Option::None => panic!("Tried to unwrap None!"),
        }
    }
    fn unwrap(self, message: String) -> T {
        match self {
            Option::Some(value) => value,
            Option::None => panic!("Tried to unwrap None!"),
        }
    }
}

// enum Result<T, E> {
//     Ok(T),
//     /// GOOD
//     Err(E), // Error happened
// }

fn nest_op(v: Option<Result<i32, String>>) {
    //
    match v {
        Option::Some(Ok(value)) => {}
        _ => {}
    };
}

fn unwrap(op: Option<i32>) -> i32 {
    let res = std::fs::read("./main.rs");
    if let Err(io_err) = res {}
    // Option::Some(10);
    let x = match op {
        //Option::Some(10)
        // "binds 10 to value"
        Option::Some(value) => {
            dbg!(value);
            value
        }
        Option::None => todo!("AHHH!!!!"),
    };
    x
}

fn foo() {
    let x = Option::Some(10);
    unwrap(x);
    unwrap(Option::None);
}

struct User {
    pub login: String,
    pub pass: String,
}

fn mathc_user(user: User) {
    match user {
        User { login, pass } => todo!(),
    };
}

fn smallest(v: Vec<i32>) -> Option<i32> {
    let mut smallest: i32 = *v.first().expect("adsfj");

    match smallest {
        1 => 1,
        2 => 2,
        _ => todo!("sdlkfj"), // nothing?? ()
    };

    match smallest < 0 {
        true => {}
        false => {}
    }
    //                                ^
    // for elem in v {
    //     if elem < smallest {
    //         smallest = elem;
    //     }
    // }
    // return smallest;
    todo!()
}

/// out scope - const
/// const = can be evaluated at compile time
/// const declaration must only call const functions
/// and use const operations
///
/// - BIG restriction - NO heap allocation :(
/// => cant have const Vec, String, HashMap, etc,
/// workarounds: LazyStatic
///
/// cannot be mutable
///
/// MUST have an explicit type
// non const
// const CVEC: Vec<i32> = vec![1, 2, 3];
// const X: String = String::from("asdf");
// str -> String : requires heap

// legal cos no heap allocation
const ALLOWED_SLICE: &[i32] = &[1, 2, 3];

// mut -> value is immutable
// "REAL" immutability
// const -> prevents rebinding != immutability
//
//

const XINT: i32 = 10;
const Y: &str = "sdflkj";
// HEAP
// STACK
// DATA

fn foo3() -> i32 {
    // 1 + 1 //
    // gets evaluated at compile time
    // 1 + 1 = 2
    // "fold"
    // Literals
    2
}

// struct User {
//     pub login: String,
//     pub pass: String,
// }

// ref and own = wk3
fn foo2(u: &mut User) {
    u.login = String::from("dsf"); // illegal
}
