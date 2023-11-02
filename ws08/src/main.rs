use std::{
    num::NonZeroUsize,
    slice::Chunks,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

pub struct ParallelIterator {
    iter: Vec<i32>,
}

impl From<Vec<i32>> for ParallelIterator {
    fn from(iter: Vec<i32>) -> Self {
        Self { iter }
    }
}

fn find_in_chunk(chunk: &[i32], target: i32) -> Option<usize> {
    for (i, item) in chunk.iter().enumerate() {
        if *item == target {
            return Some(i);
        }
    }
    None
}

// Some(7)
// [0, 1, 2, 3, 4, 5 | 6, 7, 8, 9, 10]
//                     0, 1
// 1 * 6 + 2
// chunK_size * chunk_number + (index_inside_chunk)

impl ParallelIterator {
    const NUM_THREADS: usize = 512;

    fn find(&self, search_for: i32) -> Option<usize> {
        let chunk_size = (self.iter.len() as f64 / Self::NUM_THREADS as f64).ceil() as usize;
        let chunks = self.iter.chunks(chunk_size);

        let offset_to_idx = |(ch_num, offset)| (ch_num * chunk_size) + offset;
        // Server!
        // parralleism
        // 1. load balancer -> split accros real diff machines
        // 2. AWS lambda -> each requreust is on a diff machine
        // cold start -> park it some core
        // "real OS" -> "hypervisor"

        thread::scope(|s| {
            let handles: Vec<_> = chunks
                .map(|chunk| s.spawn(move || find_in_chunk(chunk, search_for)))
                .collect();
            handles
                .into_iter()
                .enumerate()
                .find_map(|(chunk, h)| h.join().unwrap().map(|offset| (chunk, offset)))
                .map(offset_to_idx)
        })
    }

    fn find_all(&self, search_for: i32) -> Vec<usize> {
        // TODO
        vec![]
    }

    pub fn map<T, F>(&self, f: &F) -> Vec<T>
    where
        T: Send,
        F: Fn(i32) -> T + Sync,
    {
        let chunks = self.to_chunks();

        thread::scope(|s| {
            let handles: Vec<_> = chunks
                .map(move |chunk| {
                    s.spawn(move || chunk.into_iter().map(move |&i| f(i)).collect::<Vec<_>>())
                })
                .collect();

            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .flatten()
                .collect()
        })
    }

    fn calculate_chunk_size(&self) -> usize {
        let num_threads = std::thread::available_parallelism().unwrap().get();
        (self.iter.len() as f64 / num_threads as f64).ceil() as usize
    }

    fn to_chunks(&self) -> Chunks<i32> {
        let chunk_size = self.calculate_chunk_size();
        self.iter.chunks(chunk_size)
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
    let test_length = 10000000;
    let find_index = 9000000;

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
        iter.find(2);
        assert_eq!(iter.find(2), Some(find_index));
    });

    let double = &|x| x * 2;
    let linear: Vec<i32> = (0..1_000_000).collect();
    let parallel = ParallelIterator::from(linear.clone());

    time("map_normal", linear, |v| {
        let x: Vec<_> = v.into_iter().map(double).collect();
        if x.len() < rand::random::<usize>() / usize::MAX + 100 {
            panic!("out of luck");
        }
    });

    time("map_parall", parallel, |v| {
        let x = v.map(double);
        if x.len() < rand::random::<usize>() / usize::MAX + 100 {
            panic!("out of luck");
        }
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
//
// data races!
// race condition
//
//
// incrementing and decremeing is a problem?
// clone
// pointer +1, -1
// Rc
//
//
// Send, Sync
// Send -> Can send acrross threads
// Sync -> Can share references across threads
//
