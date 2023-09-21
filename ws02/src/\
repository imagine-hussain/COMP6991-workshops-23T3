fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let v = v.into_iter().filter(|x| *x > 1).map(|x| x * 2).take(2);
    // take calles next on map, mapo calls next on the filter
    // take calles next on map, mapo calls next on the filter
    dbg!(v.collect::<Vec<_>>());
}

fn return_is_expr(b: bool) -> i32 {
    let x = return 10;
    //  ^ note that this is legal and x has a type
    //  of `!` which indicates "never"
    //  This is NOT the same as the `unit` `()` type
    //  Unit is a zero-sized value. `!` indicates that
    //  itsd unreachable to get a val for this
    //
    //
    // This is0 fine
    let x = match b {
        true => 10,
        false => return 20,
        // ^ This branch is `!` type so we don't
        // need to check that its value is `i32`
    };
    return x;
}

// The actaul Fold Implementation in Iterator
fn fold<B, F>(mut self, init: B, mut f: F) -> B
where
    // ignore this stuff (for now)
    Self: Sized,
    F: FnMut(B, Self::Item) -> B,
{
    let mut accum = init;
    // ^ default values the accum
    while let Some(x) = self.next() {
        accum = f(accum, x);
        //      ^ calls the func w/ accuma and the x
    }
    accum
}

// struct Take {
//     n: usize,
//     iter: Iterator,
//     // ^ the underlying iterator
// }
//
// impl Iterator for Take {
//     // // ignore this
//     // type Item = Self::;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.n == 0 {
//             return None;
//         }
//
//         self.n -= 1;
//         self.iter.next()
//     }
// }

struct Foo {
    x: i32,
    y: String,
    z: bool,
}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

fn find_most_visited_func(v: Vec<(String, i32)>) -> Option<(String, i32)> {
    let (Some(stat), Some(amt)) =  v.into_iter().fold((None, None), |acc, (station, amount)| {
        if amount >= acc.1.unwrap_or(-1) {
            (Some(station), Some(amount))
        } else {
            acc
        }
    }) else {
        return None
    };
    return Some((stat, amt));
}

fn find_most_visited_imper(v: Vec<(String, i32)>) -> Option<(String, i32)> {
    let mut max = None;
    let mut max_station = None;
    for (station, amount) in v {
        if amount >= max.unwrap_or(-1) {
            max = Some(amount);
            max_station = Some(station);
        }
    }

    match (max_station, max) {
        (Some(station), Some(amount)) => Some((station, amount)),
        _ => None,
    }
}

// SUM:
// bool:
//  - true
//  - false

fn test_sum(op: MyOp) {
    match op {
        MyOp::Some(true) => println!("true"),
        MyOp::Some(false) => println!("false"),
        MyOp::None => println!("none"),
    }
}

// "variants":
enum MyRes {
    MyOk(bool),  //2
    MyErr(bool), //2
}

// SUM = OR
// PRODUCT = AND
struct Point {
    x: i32,
    y: i32,
}

// union
// 'untagged union'
// type SomeOrNone = bool | None
// 'vtable that contains the type'
// 'tagged union'
// Some(bool) | None
// //
// //

fn match_test(x: MyRes) {
    // "exhaustive match"
    // "OR"
    match x {
        MyRes::MyOk(t) => todo!(),
        MyRes::MyErr(b) => todo!(),
    }
    let x: i32 = 10;
    match x {
        1 => {}
        2 => {}
        3 => {}
        i32::MAX => {}
        -1 => {}
        i32::MIN => {}
        a => {}
    }
}

// hint: not 6
// 8
// Cartesian Product
// {bool} x {bool} x {bool}
struct Named {
    x: bool, // 2
    y: bool, // 2
    z: bool, // 2
}

// sum up all the diff |variant| for each variant
enum MyOp {
    Some(bool), // 2
    None,       // 1
}

fn matches2(op: MyOp) {
    match op {
        // ^
        //   ^ NOT a variant of MyOp
        _ => {}
    };
}
