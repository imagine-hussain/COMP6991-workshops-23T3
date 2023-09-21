use std::collections::HashMap;

fn main() {
    functional_sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
fn functional_sum(v: Vec<i32>) -> i32 {
    // |a, b| - this is the args
    // js -> `()` can be ambigious
    // "zero cost"
    let x = v
        .into_iter()
        .filter(|x| {
            println!("filter: {x}");
            x % 2 != 0
        })
        .map(|x| {
            println!("map: {x}");
            x * 2
        })
        .take(3);
    return x.fold(0, |acc, curr| acc + curr);
    //                           ^?
    //                                    ^    ^
}

#[derive(Hash, PartialEq, Eq)]
struct Foo;
// hashmaps
// pretend s is a string
fn count_chars(s: Vec<char>) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    for ch in s {
        *map.entry(ch).or_insert(0) += 1;
    }
    map
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_x(&self) -> i32 {
        self.x
    }
}

impl Copy for Point {}
// make it copy
// make it clone
fn uses_point(p: Point) {
    let x = 10;
    uses_int(x);
    uses_int(x); // no need for clone because this is a `Copy` type
    uses_point(p.clone());
    uses_point(p);
}

fn uses_int(_x: i32) {}
