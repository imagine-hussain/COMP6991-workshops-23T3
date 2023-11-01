use rand::Rng;

use std::time::Instant;

struct ParallelIterator {
    iter: Vec<i32>,
}

impl ParallelIterator {
    fn find(&self, search_for: i32) -> Option<usize> {
        // TODO
        None
    }

    fn find_all(&self, search_for: i32) -> Vec<usize> {
        // TODO
        vec![]
    }

    fn map(&self /*you will need to add the argument here*/) -> Vec<i32> {
        // TODO
        vec![]
    }
}

trait IntoParIter {
    fn into_par_iter(self) -> ParallelIterator;
}

impl IntoParIter for Vec<i32> {
    fn into_par_iter(self) -> ParallelIterator {
        ParallelIterator { iter: self }
    }
}

fn time<T>(test_name: &str, data: T, f: impl Fn(T) -> ()) {
    let before = Instant::now();
    f(data);
    println!("Test {test_name}: {:.2?}", before.elapsed());
}

fn main() {
    let test_length = 1000000;
    let find_index = 77722;

    let mut nums: Vec<i32> = vec![0, 1].repeat(test_length / 2);
    nums[find_index] = 2;
    nums[find_index + 100] = 2;
    nums[find_index + 1000] = 2;

    time("find_normal", nums.clone(), |nums| {
        let mut iter = nums.into_iter();
        assert_eq!(iter.position(|i| i == 2), Some(find_index));
    });

    time("find_parallel", nums.clone(), |nums| {
        let iter = nums.into_par_iter();
        assert_eq!(iter.find(2), Some(find_index));
    });
}

// Concurrency / async scheduling!!!!!!
//
// concurrency
//  multiple processes at the _same time_? ?
//
//  io-bound (input / output) - examples
//      network
//      device input
//      disk
//      database
//
//  3 seconds
//  1s get(google.com) // waiting
//  1s get(facebook.com) // waiting
//  1s get(twitter.com) // waiting
//
//  cpu-bound?
//  sum(vec1)
//  sum(vec2)
//  500 v1 -> 500 v2 -> 500 v1 -> 500 v2
//  1000 v1
//  1000 v2
//  0 -------------------------500---------------- 1000
//  xxxxxxxxxxxxxxxxxxxxxxxxxxx
//                              fffffffffffffffffffffff
//  0 ---------------------------------------------- 1
//  1s - io-bound
//  "cpu"
//
//  -- cpus
//
//  -- threads (os) ->
//  -- green (go) -> tokio async/await
//
//
//  google.com, facebook.com, twitter.com
//
//
//  io-bound vs cpu-bound
//
// paralleism
//
//
// thread
// data race
// mutex
// arc
// send, sync
// channel
//
//
// await ?
//
// always better?
// - high cost?
// - io bound -> dont need
//
