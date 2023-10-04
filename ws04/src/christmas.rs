use serde::Deserialize;
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    // Box<Option>
    // Option<Box>
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

impl Light {
    fn new(value: i32) -> Self {
        Light {
            left: None,
            right: None,
            brightness: value,
        }
    }

    // thoughts?????
    fn left(&mut self) -> &mut Light {
        self.left.get_or_insert(Box::new(Light::new(0)))
    }

    fn right(&mut self) -> &mut Light {
        match self.right {
            Some(_) => self.right.as_mut().unwrap(),
            None => {
                let new_light = Box::new(Light::new(0));
                self.right = Some(new_light);
                self.right.as_mut().unwrap()
            }
        }
    }

    fn set(&mut self, brightness: i32) -> &mut Self {
        self.brightness = brightness;
        self
    }
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn main() {
    let instructions = get_instructions_from_stdin();
    // reset
    let mut root: Light = Light::new(0);
    // why dont we want curr to be a Box<Light>???
    //          // What happens to the box?
    //          // is box copy? what does that mean
    // let curr = root;
    // &mut Light
    let mut curr = &mut root;

    for instruction in &instructions {
        match instruction {
            Instruction::Set(brightness) => {
                curr.set(*brightness);
            }
            Instruction::Left => {
                curr = curr.left();
            }
            Instruction::Right => {
                curr = curr.right();
            }
            Instruction::Reset => {
                curr = &mut root;
            }
        }
    }

    println!("{:?}", root);
    // for instruction in &instructions {
    //     let mut tmp = match instruction {
    //         // Instruction::Set(brightness) => Box::new(curr.set(*brightness)),
    //         Instruction::Left => {
    //             let left = curr.left();
    //             left
    //         },
    //         Instruction::Right => {
    //             let right = curr.right();
    //             right
    //         },
    //         Instruction::Reset => {
    //             // This is wrong, need to be actual root
    //             // Box::new(Light::new(0))
    //             &mut root
    //         },
    //         _ => panic!("Unknown instruction"),
    //     };
    //     curr =  &mut tmp;
    // };
}

//
//
// pub type NodePtr = Box<Light>;
//
// impl Tree <'_> {
//
//     fn new() -> Self {
//         let mut root = &mut Box::new(Light::new());
//         // let curr = &mut root;
//         Tree {
//             root,
//             curr: root,
//         }
//     }
//
//     fn traverse_left(&mut self) {
//         if self.curr.left.is_none() {
//             let new = &self.curr.insert_left();
//             self.curr = &mut new.unwrap();
//         }
//     }
//     fn traverse_right(&mut self) {
//         if self.curr.left.is_none() {
//             let new = &self.curr.insert_right();
//             self.curr = &mut new.unwrap();
//         }
//     }
//
//     fn set_curr(&mut self, brightness: i32) {
//         self.curr.set(brightness)
//     }
//
//     fn reset_curr(&mut self) {
//         self.curr = self.root;
//     }
//
// }
//
// impl Light { fn new() -> Self {
//         Light {
//             left: None,
//             right: None,
//             brightness: 0,
//         }
//     }
//
//     fn insert_left(&mut self) -> Option<Box<Light>> {
//         self.left = Some(Box::new(Light::new()));
//         // self.left
//         None
//     }
//
//     fn insert_right(&mut self) -> Option<Box<Light>> {
//         self.right = Some(Box::new(Light::new()));
//         self.right
//     }
//
//     fn set(&mut self, brightness: i32) {
//         self.brightness = brightness;
//     }
// }
//
//  THEORY abt data structure
//
//  - Vec
//  - Hashmap
//     - more complex
//     - why is the hash slow?
//     - security
//     - entropy
//     - FxHashSet
//  - VecDequue
//  - LinkedList
//
//
//
//  Vector
//  O(n)
//  [1, 2, 3, _]
//  [2, 3, _, _]
//  ^
//  size
//
//  VecDeque
//  [1, 2, 3, _]
//  [_, 2, 3, _]
//
//
//  LinkedList is mcuh slower?
//  O(1)?
//  Indirection! can be expensive
//  - cpu branch prediction
//  - cache locality
//
//
//  [1, 2, 3, _] -> prediction on cache
//      ^  ^
//
//  Memory access a bit of a lie!
//  [ ] -> register
//  ram -> cpu
//
//  // 3GHz clocks -
//  4 inches 2 inches
//  4 inches per clock cycle
//  3   20    50  300
//  L1, L2, L3
//  ^   ^    ^
//  Caches! O
//
// Cache Lines!
// 64 bytes - 128 bytes
// [......X.....]
// // 1 grab of the cache
// [1, 2, 3, 4, 5, 6, 7, 8]
// [9, ...........]
//
// [    (...)(...)(...)                          ]
//                 ^
//                                                ^ where misses hapen
//
// strucxt LL {
// next *;
// }
// [1, 2, ONE, 4, 5, 6, 7, 8]
// [1, 2, 3, 4, 5, TWO, 7, 8]
// [1, 2, 3, 4, 5, THREE, 7, 8]
// [1, 2, 3, 4, 5, FOUR, 7, 8]
//
// spatial
// temporal locality
//
