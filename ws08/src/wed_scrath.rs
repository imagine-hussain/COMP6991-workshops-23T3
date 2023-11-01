use std::{
    thread::{self, ScopedJoinHandle},
    time::Duration,
};

// Handles!
fn main() {
    // heyt os, can i has a threa dpls
    // "parallel"
    // sharing?
    // [10000]
    // [250], [250], [250], [250]
    let s = String::from("hi");
    let s = &s;
    let r = thread::scope(|scope| {
        vec![
            scope.spawn(move || {
                dbg!("Start twitter");
                // sleeps the thread!
                thread::sleep(Duration::from_secs(10));
                dbg!("finish twitter");
                dbg!(s);
                panic!("twitter crashed");
            }),
            scope.spawn(move || {
                dbg!("start google");
                // sleeps the thread!
                thread::sleep(Duration::from_secs(2));
                dbg!(&s);
                dbg!("finish google");
            }),
            scope.spawn(move || {
                dbg!("start facebook");
                // sleeps the thread!
                thread::sleep(Duration::from_secs(1));
                dbg!(&s);
                dbg!("finish facebook");
            }),
        ];
        "result"
    });
    dbg!(r);
}

// Do not communicate by sharing memory; instead, share memory by communicating.
