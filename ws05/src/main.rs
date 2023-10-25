use std::{any::Any, fmt::Display};

fn main() {}

// poly morphism
//  ^    ^

// only 1 value at a time!
//
// exhaustiveness

enum Planet {
    Earth,
    Mars,
    Jupiter,
}

fn foo1() {
    let v = BadVec::<i32>::default();
    let v2 = BadVec::<f64>::default();
}

#[derive(Default)]
struct BadVec<T>
where
    T: Default,
{
    data: [T; 10],
}

fn print_vec(v: Vec<Box<dyn Any>>) {
    for e in v {
        let x: Box<i32> = e.downcast().unwrap();
    }
}

fn does_print() {
    let v: Vec<Box<dyn Any>> = vec![Box::new(1), Box::new(2), Box::new("hi")];
    print_vec(v);
}

// // dyn dispatch?
// // // ^?//
// //
// struct DisplayLut {
//     data: T;
//     fmt: *const fn(&T, ...) -> String,
//     clone: *const...
//     check_eq: *const fn(&T, &T) -> bool,
//     drop: *const fn(T),
// }
//
// Box<DisplayLut>
//
// // folow the ptr to the LUT
// // follow the ptr to the function
