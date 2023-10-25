use rand::Rng;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;
#[allow(dead_code)]
enum TaskResult {
    Finished(HashSet<Prerequisites>),
    RunMeAgain,
}

/// This is a particular task that needs to be run.
///
/// A task has "prerequisites" -- it can't run until
/// they have happened.
struct Task<'a> {
    prerequisites: HashSet<Prerequisites>,
    task: Box<dyn FnMut() -> TaskResult + 'a>,
}

/// This contains all the tasks, and also all the prerequisites
/// that have already happened.
struct Scheduler<'a> {
    tasks: Vec<Task<'a>>,
    prerequisites: HashSet<Prerequisites>,
}

impl<'a> Scheduler<'a> {
    fn start(mut self) {
        let mut rng = rand::thread_rng();
        loop {
            if self.tasks.is_empty() {
                break;
            }
            let random_index = rng.gen_range(0..self.tasks.len());
            if self.tasks[random_index]
                .prerequisites
                .is_subset(&self.prerequisites)
            {
                let mut task = self.tasks.swap_remove(random_index);
                let task_result = (task.task)();
                match task_result {
                    TaskResult::Finished(new_prerequisites) => {
                        self.prerequisites.extend(new_prerequisites.into_iter());
                    }
                    TaskResult::RunMeAgain => {
                        self.tasks.push(task);
                    }
                }
            }
        }
    }

    fn add_task(&mut self, task: Task<'a>) {
        self.tasks.push(task);
    }

    fn new() -> Self {
        Self {
            tasks: vec![],
            prerequisites: HashSet::new(),
        }
    }
}

/// This is a list of every "event" that can happen in our
/// scheduler system.
#[derive(PartialEq, Eq, Hash)]
enum Prerequisites {
    CleanedTestFile,
    CleanedCargo,
    CargoBuiltPrintRandoms,
    CargoBuiltProgramRunner,
    WrittenRandoms,
}

/// Tasks should happen in this order:
///
/// Clean Cargo  -> Wait until cargo is clean -> Build PrintRandoms -> Wait until it's built.  -> Run PrintRandoms with ProgramRunner
///                                          \                                                 |
///                                           -> Build ProgramRunner -> Wait until it's built -|
///                                                                                            |
/// Clean Test File --------------------------------------------------------------------------/
use std::process::{Command, Stdio};
use std::thread;

fn run_command(command: &[&str]) {
    Command::new(command[0])
        .args(&command[1..])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        // If only we could use try_wait() here.
        .wait()
        .unwrap();
}

fn main() {
    //

    //
    let scheduler = init_scheduler();
    scheduler.start();
}

// Functoin, Closures, ~lifetimes
// Scheduler!
// - Tasks
// - run those taaask
// - order
// - cargo run?
// - tokio <- ...
// - cache
// - yes!!!!! javascrip event loop
// - "event" -> PQ
// - async?
// //
// await foo();
// block();
// // processes -> context switching, threads!
//
//
// Game Engines!
// - ECS - Entity Component System
// - uses a scheduler to run functions
// - 2511 - 2022T2/3

// what is a closure? what is a fucntioon?
//             ^
fn foo1() {
    let v1 = vec![1, 2, 3];
    let mut sum = String::new();
    let v2: Vec<_> = v1.into_iter().map(double).collect();
    // running sum?
    // captures its environment
    // ownership, lifetimes, borrowing,
    // borrow!
    // i32?
    // closure, identifgied by the args and return
    // (i32) -> i32
    let double = |x: i32| x * 2;
    let v3: Vec<_> = v2
        .into_iter()
        .map(|x| {
            sum += x.to_string().as_str();
            // drop takes ownership of sum
            // drop(sum);
            x * 2
        })
        .collect();
}

// what is the big limitation here?
fn double(x: i32) -> i32 {
    x * 2
}

//
// Closure?
// - its a unique type that cannot be written down
// -
//
// - Fn
// - FnMut
// - FnOnce
//  - can butcher / own things
//  - only called at most once
//
//
fn takes_ocne(f: impl FnOnce() -> i32) {
    f();
    // f(); //
}

// struct Double {
//     functioon: *;
//     sum: &String,
// }
// .call

fn takes_mut(mut f: impl FnMut() -> i32) {
    // how many times?
    // as many times as want?
    // how many mutable refs can we have at once?
    f(); //
    f(); //
}

fn takes_fn(f: impl Fn() -> i32) {
    // how many times?
    // in total: unlimited
    // at the same time: unlimited
    f();
    f();
    f();
    f();
    f();
}

// we are a lib autor
//
pub fn operatef(f: impl Fn() -> i32) {
    f();
    f();
    f();
    f();
    f();
}

// whty we would use any of the other closure
//
// - the writer of the clousre
// - the user of the closure
//
// FnOnce
//  - drop
//  - moves so can only call once
//  - map, filter
// FnMut
// - used most places?
// - reasonable compromise
// Fn
// - might want an Fn
// - concurrency!
//
//
//fn
//
fn main2() {
    let v = vec![1, 2, 3];
    let mut sum = String::new();

    // FnOnce
    // FnMut : FnOnce
    // Fn: FnMut
    let double = |x| x * 2;
    let v2 = my_map(v, double);
    let v2 = my_map_2(v2, double);
}

pub fn my_map(v: Vec<i32>, f: impl Fn(i32) -> i32) -> Vec<i32> {
    let mut result = Vec::new();

    for e in v {
        result.push(f(e));
    }

    result
}

pub fn my_map_2(v: Vec<i32>, mut f: impl FnMut(i32) -> i32) -> Vec<i32> {
    let mut result = Vec::new();

    for e in v {
        result.push(f(e));
    }

    result
}

fn init_scheduler<'a>() -> Scheduler<'a> {
    let mut scheduler = Scheduler::new();

    //
    // this dies at the end of this functoin
    let time_since_run = String::from("");

    //
    // LIFETIME
    let time_since_run_2 = time_since_run.clone();
    // closure  - Attr
    // &, &mut - Hindley Milner approach
    // Ocaml, Haskell, SML
    // 1. what is the type of everything KNOWN
    // 2. list of eqns (represent as a directed graph)
    // type inference
    scheduler.add_task(Task {
        prerequisites: HashSet::new(),
        task: Box::new(move || {
            // this keeps a ref to time_since_run
            // &String
            println!("last_run, {}", time_since_run_2);
            println!("Cleaning cargo.");
            run_command(&["cargo", "clean"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CleanedCargo]))
        }),
    });
    dbg!(time_since_run);

    // Build the print_randoms binary
    scheduler.add_task(Task {
        prerequisites: HashSet::from([Prerequisites::CleanedCargo]),
        task: Box::new(|| {
            println!("Building print_randoms.");
            run_command(&["cargo", "build", "--bin", "print_randoms"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CargoBuiltPrintRandoms]))
        }),
    });

    // Build the program_runner binary
    scheduler.add_task(Task {
        prerequisites: HashSet::from([Prerequisites::CleanedCargo]),
        task: Box::new(|| {
            println!("Building program_runner");
            run_command(&["cargo", "build", "--bin", "program_runner"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CargoBuiltProgramRunner]))
        }),
    });

    // Run the program_runner binary
    scheduler.add_task(Task {
        prerequisites: HashSet::from([
            Prerequisites::CleanedTestFile,
            Prerequisites::CargoBuiltPrintRandoms,
            Prerequisites::CargoBuiltProgramRunner,
        ]),
        task: Box::new(|| {
            println!("Running program_runner.");
            run_command(&[
                "cargo",
                "run",
                "--bin",
                "program_runner",
                "randoms",
                "cargo",
                "run",
                "--bin",
                "print_randoms",
            ]);
            TaskResult::Finished(HashSet::from([Prerequisites::WrittenRandoms]))
        }),
    });

    // We also want to make sure there isn't a file called
    // "randoms"
    scheduler.add_task(Task {
        prerequisites: HashSet::new(),
        task: Box::new(|| {
            println!("Removing randoms.");
            run_command(&["rm", "-f", "randoms"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CleanedTestFile]))
        }),
    });
    vec![1, 2, 3];

    scheduler
}
