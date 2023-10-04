use std::ops::Add;
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

// Add two directions
// trait -> interface
// traits are PascalCase
impl Add<Direction> for Direction {
    type Output = Direction;
    fn add(self, rhs: Self) -> Self::Output {
        Direction {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Self {
        Direction { x, y }
    }

    fn double(&mut self) {
        self.x *= 2;
        self.y *= 2;
    }
}

pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

struct Book<'a> {
    // titel needs to atleast as long as the Book
    title: &'a str,
}

//
const s: &'static str = "hello";

// fn calls_foo<'a>() -> Book<'a> {
//     let b1 = foo("alsf");
//     let s1 = String::from("hi");
//     let rs = &s1;
//     let b2 = foo(rs);
//     return b2;
// }
//
// fn foo<'a, 'b>(s: &'a str, s2: &'b str) -> Book<'a> {
//     // Book and s need to be linked and live the same
//     // i dont care abt s2, just have it live for this func
//     println!("{}", s2);
//     // Book<'L>
//     Book { title: s }
// }
fn less_contrived_lifetimes() {
    let source: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let mut dest: Vec<&i32> = Vec::new();

    for x in &source {
        let r = &mut dest;
        add_to_dest(r, x);
    }
    // --- 'a
    // &mut dest 'r
}

// Exercise: why are these annotations not okay?
// fn add_to_dest<'a>(dest: &'a mut Vec<&'a i32>, x: &'a i32) {
//
// Actual need:
// fn add_to_dest<'a>(dest: &mut Vec<&'a i32>, x: &'a i32) {
// a lives for SOME amoutn time
// b lives for ATLEAST a
fn add_to_dest<'a, 'b, 'c: 'b>(dest: &'a mut Vec<&'b i32>, x: &'c i32) {
    //                       ^        ^         ^
    //                       a        b         b
    // k
    dest.push(x);
}
