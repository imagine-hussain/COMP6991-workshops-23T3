use termgame::{
    run_game, CharChunkMap, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent,
};

use std::error::Error;
use std::time::Duration;
/// This is a single "buffer".
struct Buffer {
    text: String,
}

impl Buffer {
    /// This creates a new Buffer, to use it you should run:
    /// ```rust
    /// Buffer::new()
    /// ```
    fn new() -> Buffer {
        Buffer {
            text: String::new(),
        }
    }

    /// A [`CharChunkMap`] is how termgame stores characters.
    /// This converts a buffer into something which can be shown on screen.
    /// You will likely not need to change this function.
    fn chunkmap_from_textarea(&mut self, map: &mut CharChunkMap) {
        let (mut line, mut col) = (0, 0);
        for c in self.text.chars() {
            map.insert(col, line, c.into());
            col += 1;
            if c == '\n' {
                line += 1;
                col = 0;
            }
        }
    }

    /// Adds a char to the end of the buffer.
    fn push_char(&mut self, c: char) {
        self.text.push(c);
    }

    /// Removes the last char in the buffer.
    fn pop_char(&mut self) {
        self.text.pop();
    }

    // /// This is an example of a function that takes the Buffer as owned,
    // /// as well as another text area; and returns a new Buffer.
    // /// You would either need to return a `Buffer`, or be sure that
    // /// the user will not want the `Buffer` anymore.
    // fn example_owned(self, another_arg: Buffer) -> Buffer {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// mutable reference.
    // fn example_ref_mut(&mut self, another_arg: i32) {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// reference.
    // fn example_ref(&self) -> i32 {
    //     todo!()
    // }
}

/// This struct implements all the
/// logic for how the editor should work. It
/// implements "Controller", which defines how
/// something should interact with the terminal.
struct BufferEditor {
    buffer: Buffer,
}

impl Controller for BufferEditor {
    /// This gets run once, you can probably ignore it.
    fn on_start(&mut self, _game: &mut Game) {}

    /// Any time there's a keypress, you'll get this
    /// function called.
    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(c)) => self.buffer.push_char(c),
            SimpleEvent::Just(KeyCode::Enter) => self.buffer.push_char('\n'),
            SimpleEvent::Just(KeyCode::Backspace) => self.buffer.pop_char(),
            SimpleEvent::Just(KeyCode::Esc) => {
                game.end_game();
            }
            SimpleEvent::Just(KeyCode::Up) => {
                let mut viewport = game.get_viewport();
                if viewport.y > 0 {
                    viewport.y -= 1;
                }
                game.set_viewport(viewport)
            }
            SimpleEvent::Just(KeyCode::Down) => {
                let mut viewport = game.get_viewport();
                viewport.y += 1;
                game.set_viewport(viewport)
            }
            _ => {}
        }
        let mut chunkmap = CharChunkMap::new();
        self.buffer.chunkmap_from_textarea(&mut chunkmap);
        game.swap_chunkmap(&mut chunkmap);
    }

    /// This function gets called regularly, so you can use it
    /// for logic that's independent of key-presses like
    /// implementing a "mouse".
    fn on_tick(&mut self, _game: &mut Game) {}
}

fn run_command(cmd: &str) -> Result<(), Box<dyn Error>> {
    let mut editor = BufferEditor {
        buffer: Buffer::new(),
    };
    if cmd.starts_with("open") {
        run_game(
            &mut editor,
            GameSettings::new().tick_duration(Duration::from_millis(25)),
        )?;
    } else {
        println!("Command not recognised!");
    }

    Ok(())
}

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to BuffeRS. ");

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                run_command(&line)?;
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

struct Point {
    x: i32,
    y: i32,
}

// getter and setter
//

// Copy / Clone
//  ^      ^
fn foo() {
    // values vs binding
    // "ownership"
    // Exists somewhere
    // Value "String" -> Struct { len, capacity, ptr }
    // bind that string into `s` -> "Owning bind"
    // let mut s = String::from("hi");
    // Borrow that value temporarily
    // `&` shared borrow       //
    // `&mut` exclusive borrow //
    // let s2 = &mut s;
    // let s3 = &mut s;
    // hello world
    // hello there
    // s3.push('a');
    // s2.push('a');
    // let s3 = &s;

    // s2
    // drop(s2); // free(s2)
}

// strings are immutable
// s = "asdf" // s pointer to "asdf"
// s2 = s     // pointer to "asdf".push('a') -> "asdfa"
// s3 = s     // pointer to "asdf"
// "mark and sweep"
//
// 'throughput' vs 'latency'
// nursery , __
// infant mortality ->
// 'generational heap' ->
// overhead (depends:tm:) ->
// automatic -> easy!
// no leak! (ehhhh) -> cycles! can propose leaks
//    ^

// Looking at:
// [ ] Mutable and Exclusive Borrows
// [ ] Lifetimes !!!!
// [ ] Data analysis - avoding cloning
//
//
//

fn ex2(v: Vec<i32>) {
    // v.iter() -> &v
    for (i, x) in v.iter().enumerate() {
        // &
        let x2 = v.get(i);
        //
    }
}

// TRADEOFF between function and caller
// Prefer &     // great for caller, limits what func can do
// Then &mut    //
// Then move    // func can do everythign! caller loses the value

fn borrows_end() {
    let mut s = String::from("hi");
    borrows_s(&s); //
    s.push('a');
    // s.push('a');
    // let s2 = &mut s; // "SHOOULD" live till eend of func
    // borrows_s(s2);
    // // non-lexical lifetime
    // // cut of s2 early because its never used again
    // let s3 = &mut s; //
    // borrows_s(s3);
    // // borrows_s(s2); // problematic again!
}

fn borrows_s(_s: &String) {
    // _s.push('a');
}

// fn foo2() -> Vec<&String> {
//     let mut v = Vec::new(); //
//                             // on_vec(&mut v); // v
//                             // keep the vec
//
//     // "dangling pointer to freed memory"
//
//     // this okay?
//     v[0] = &String::from("hi");
//
//     // v ends here
//     // string ends here
//     // 'v escapes this function'
//     return v;
// }

// annotation
fn on_vec(v: &mut Vec<&String>) {
    //       ^        ^
    // v[0] = &String::from("hi");
    //                ^
    // 1 have some other owning strucure
    // lives for the whole program
    // mallcoc()
    // *
    // // i cannot free this pointer safely
    let s: &'static mut String = Box::leak(Box::new(String::from("hi")));
    v[0] = s;
    // string NEEDS TO LIVE as long as OR longer than the vector
    // mut v = binding is mut
    // v itself is mutable
    // &mut -> &
    // let s = String::from("dies early");
    // v[0] = &s;
    // what happens to s?
    // drop(s);
    // s ends here
    // v does not end here
    let og = Vec::<String>::new();
    //               ^
    // <key, String>
    // <&String>
    // //do some
    let temp: Vec<&String> = vec![];
    // return a Vec<String> instead
    // the caller does the extra work of extending the vec
    let v = vec![1, 2, 3];
}

struct Foo<'a> {
    s: &'a String,
}

// Hindley-Milner

// func_name<'a >
// &'a
// , 'a : 'b
//         ^ smaller one

fn named_block() {
    let most_outer = String::from("hi");
    let most_r = &most_outer;
    'outer: for i in 0..10 {
        let mut v: Vec<&String> = Vec::new();
        //     &'outer String
        // Vec with reference that live as long as OUTER!
        'inner: for j in 0..10 {
            v.push(most_r);
            let s1 = String::from("first");
            // let r = &s1;
            // THYPE of r aas being equiv to
            // &'inner String
            // innner < outer
            // inner is not a compatible TYPE for elemn of v
            // v.push(r);
        }
    }
}

// according the chat gpt this is the situtations we need lifetimes In Rust, lifetime annotations are often not required thanks to lifetime elision rules, which allow the compiler to infer lifetimes in many cases. However, there are situations where you need to annotate lifetimes manually:

// Multiple References in Function Signatures:
// When a function takes multiple references as parameters, and returns a reference, the compiler often cannot infer the relationships between the lifetimes of the references. In such cases, you need to annotate the lifetimes manually.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// Structs with References:
// When defining a struct that holds references, you must always annotate the lifetimes, because the compiler cannot infer the lifetimes in this context.

struct Book<'a> {
    title: &'a str,
}
// Implementing Methods on Structs with Lifetimes:
// When implementing methods for a struct with lifetime parameters, you need to declare the lifetimes.

// rust Copy code
impl<'a> Book<'a> {
    //
    fn new(title: &'a str) -> Self {
        Book { title }
    }
}
// Generic Types with Lifetimes:
// When defining generic types or functions with both generic type parameters and references, you might need to manually annotate lifetimes to specify the relationship between the lifetimes and the generic types.

fn book_foo() {
    let b = Book::new("hi");
}

// fn return_book<'a>() -> Book<'a> {
//     // BOOK needs a 'a on the title
//     // THAT 'a must exist as LONG as the book
//     // when we call new
//     // book returned with 'a
//     // 'a must live as long as the book
//     // BUT, this doesn't happen
//     // us doing the allocation
//     // return Book::new(&String::from("title"));
// }
//
// fn with_smart() -> &String {
//     let b = Box::new(String::from("title"));
//     // Box<String>
//     //
//     // T owned value drop(owned T)
//     // &T shared borrow -> drop the borrow
//     //
//     let b = Box::new(10);
//     return b.as_ref();
// }
//
// struct Wrapper<'a, T> {
//     value: &'a T,
// }

struct Smart<T> {
    value: T,
}

// levels to it
// .await fairly simply like its just js
//
// a.equals(Object b)
// a == b
//
// - implicty converted cahar to int
// - did pointer arithmetic
// - char *
// - then ADDEd stuff there
//
// (a + c  ...)
// string += 'c'
// .end()
//  ^^
//
// get()
//
// (int, int)
//
// a.0, a.1
//
// std::get<0>(a);
//

fn non_contrived() {
    let source: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let mut dest: Vec<&i32> = Vec::new();

    for x in &source {
        add_to_dest(&mut dest, x);
    }
}

// Exercise: why are these annotations not okay?
fn add_to_dest<'a>(v: &'a mut Vec<&'a i32>, x: &'a i32) {
    v.push(x);
}
