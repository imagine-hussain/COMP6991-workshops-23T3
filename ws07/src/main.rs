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
    can_run: Box<dyn FnMut(&HashSet<Prerequisites>) -> bool>,
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
            let task = &mut self.tasks[random_index];
            if (task.can_run)(&self.prerequisites)
            // .is_subset(&self.prerequisites)
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
        can_run: HashSet::new(),
        task: Box::new(move || {
            // this keeps a ref to time_since_run
            // &String
            println!("last_run, {}", time_since_run_2);
            println!("Cleaning cargo.");
            run_command(&["cargo", "clean"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CleanedCargo]))
        }),
    });
    // sched, close
    // sched, close, prereqs
    // add_task!(sched, || {
    // ...
    // })
    //
    // add_task!(sched, || {
    // ...
    // },
    // [Prerequisites::CleanedCargo]
    // )
    dbg!(time_since_run);

    // Build the print_randoms binary
    scheduler.add_task(Task {
        can_run: HashSet::from([Prerequisites::CleanedCargo]),
        task: Box::new(|| {
            println!("Building print_randoms.");
            run_command(&["cargo", "build", "--bin", "print_randoms"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CargoBuiltPrintRandoms]))
        }),
    });

    // Build the program_runner binary
    scheduler.add_task(Task {
        can_run: HashSet::from([Prerequisites::CleanedCargo]),
        task: Box::new(|| {
            println!("Building program_runner");
            run_command(&["cargo", "build", "--bin", "program_runner"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CargoBuiltProgramRunner]))
        }),
    });

    // Run the program_runner binary
    scheduler.add_task(Task {
        can_run: HashSet::from([
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
        can_run: HashSet::new(),
        task: Box::new(|| {
            println!("Removing randoms.");
            run_command(&["rm", "-f", "randoms"]);
            TaskResult::Finished(HashSet::from([Prerequisites::CleanedTestFile]))
        }),
    });
    vec![1, 2, 3];

    scheduler
}

fn foo() {
    let v = vec![1, 2, 3];
    let mut sum = String::from("");
    let v2: Vec<_> = v
        .into_iter()
        .map(|x| {
            sum += x.to_string().as_str();
            // drop(sum);
            x * 2
        })
        .collect();
}

fn double(x: i32) -> i32 {
    x * 2
}

fn uesr() {
    takes_once(Box::new(|x| x * 2));
}

fn takes_once(f: Box<dyn Fn(i32) -> i32>) {
    f(1);
    // f(1);
}

// fn foo2<'a>() -> Vec<&'a String> {
//     let s = String::from("hi");
//     let v = vec![&s];
//     v
// }
//
fn cosure_life() -> impl FnMut() {
    let s = String::from("hi");
    // let s = s.clone();
    let z = String::from("hi");
    let x = {
        let s = s.clone();
        move || {
            println!("{}", s.clone());
        }
    };
    drop(z);
    dbg!(s);
    x
}

fn foo3() {
    let vec = vec![1, 2, 3];
    let mut sum = String::from("hif");
    vec.into_iter()
        .map(|x| {
            sum += x.to_string().as_str();
            x * 2
        })
        .collect::<Vec<_>>();
}

// fn double(x: i32) -> i32 {
//     x * 2
// }
//

// tradeoffs
//
// lib authro, client
//
// Fn
// FnMut
// FnOnce

fn liver() -> impl FnMut(i32) -> i32 {
    let s = String::from("hi");
    let c = {
        let s = s.clone();
        move |x| {
            println!("{}", s);
            x * 2
        }
    };
    dbg!(s);

    return c;
}

// fn liver_v<'a>() -> Vec<&'a String> {
//     let s = String::from("hi");
//     vec![&s]
// }

// fn do_task<'a>() -> Task<'a> {
//     let s = String::from("hi");
//     return Task {
//         prerequisites: Default::default(),
//         task: Box::new(|| {
//             println!("{}", s);
//             TaskResult::Finished(HashSet::new())
//         }),
//     };
// }
