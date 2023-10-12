use std::fmt::Display;

use simulator_lib::directions::{coordinate::Coordinate, direction::Direction};
use simulator_lib::{start_server, Asteroid, ObjectType, Planet};
fn main() {
    let mut objects = vec![
        ObjectType::Planet(Planet {
            coordinate: Coordinate::new(500, 500),
            weight: 50,
        }),
        ObjectType::Asteroid(Asteroid {
            coordinate: Coordinate::new(250, 250),
            velocity: Direction { x: 30, y: -40 },
        }),
        ObjectType::Asteroid(Asteroid {
            coordinate: Coordinate::new(750, 750),
            velocity: Direction { x: -30, y: 40 },
        }),
    ];

    println!("Starting server. Open phys_simulation.html to see the simulation.");
    start_server("localhost:16991", objects, 70);
}

// // monomorphisation
// // dynamic dispatch
// //
// //
// // poly - morphism????
// // poly - gon, nomial
// // morph - form / shape
// //
// //
// // enums?
// //
// // options and result
// //
// // Some | None
// // Pros
// // type safe?
// // exhaustiveness
// // make illegal states unrep!
// //
// // Cons:
// // exhaustiveness
// //  - can put common functionality *inside* the enum
// //  - offloads some responsiblity from the user
// // open-closed?
// // HAVE TO KNOW all your variants upfront
// // cannot extend
// // SUM type
// //
// enum Planet {
//     Mercury,
//     Jupiter,
//     Mars,
//     Earth,
//     Venus,
//     Saturn,
// }
//
// /// some other file, some other module
// impl Planet {
//     fn get_size(&self) -> u32 {
//         match self {
//             Planet::Mercury => 1,
//             Planet::Jupiter => todo!(),
//             Planet::Mars => todo!(),
//             Planet::Earth => todo!(),
//             Planet::Venus => todo!(),
//             _ => unreachable!("done"),
//         }
//     }
// }
//
// trait Physics {
//     fn mass(&self) -> f32;
//     fn accel(&self) -> f32;
// }
//
// struct Earth;
//
// impl Physics for Earth {
//     fn mass(&self) -> f32 {
//         1.0
//     }
//
//     fn accel(&self) -> f32 {
//         1.0
//     }
// }
//
// impl Physics for Mars {
//     fn mass(&self) -> f32 {
//         1.0
//     }
//
//     fn accel(&self) -> f32 {
//         // lhjlk
//         1.0
//     }
// }
//
// struct Mars;
//
// // what if I dont know all the things that can be passed in?
// //
// // pros?
// //
// //
// fn calculate_force<T: Physics>(obj: T) {
//     //            ^T
//     // f = m * a
//     //
//     return obj.mass() * obj.accel();
// }
//
// struct PhysicsLut<T> {
//     mass: *const fn(T) -> f32,
//     accel: *const fn(T) -> f32,
//     t: *const T,
// }
//
// fn foo1() {
//     let earth = Earth;
//     let mars = Mars;
//
//     calculate_force(earth);
//     calculate_force(mars);
// }
//
// // Mars::mass
// // Earth::mass
// // mono - morphisation
// // like C++ templating!
//
// // fn calculate_force::<Earth>
// // fn calculate_force::<Mars>
// // pros?
// // - BLAZINGLY FAST!
// //  - no runtime lookup
// // - inlining!
// // - monomorphisation can be SLOW?
// // - CACHE!!!!!!!!!
// //
// // cons?
// // - binary sizes - lazy?
// // - zero cost abstraction
// // - Vec::<i32> -> remove, push
// // - Vec::<f32> -> remove, push, get
// //
// //
// //
// impl Mars {
//     #[inline(always)]
//     fn get_mass(&self) -> f32 {
//         1.0
//     }
// }
//
// fn foo() {
//     let m = Mars;
//     // jump
//     let mass = m.get_mass();
// }
//
// struct Moon {
//     mass: f32,
//     accel: f32,
// }
//
// struct Sun {
//     mass: f32,
// }
//
// fn genercis_not_work() {
//     // let t: Option<i32> = None;
//     // let t3: Option<i32> = Some(3);
//     // 0 size, 8 byutes
//     // many size? turn it into a single size?
//     let v: Vec<Box<dyn Physics>> = vec![Box::new(Mars), Box::new(Earth)];
//     // type of v?
//     // Vec<T????>
//     debug_vec(v);
//     // how are tings in a vector stored?
// }
// // vec own - not a ref!
// // not a pointer
// fn debug_vec(v: Vec<Box<dyn Physics>>) {
//     for item in v {
//         println!("mass: {}", item.mass());
//     }
// }
//
// trait DiplayablePhysics: Physics + std::fmt::Display {
//     //                    ^ BOUDN by
//     fn get_force() {}
// }
//
// // automatically implement this for ALL types that
// // implement physics and display!
// impl<T: Physics + std::fmt::Display> DiplayablePhysics for T {}
//
// // Error
// impl DiplayablePhysics for Earth {}
//
//
// // BOTH!
// fn () {
//
// }
//
// // T
// // LUT  - Physics
// // LUYT - Display
// //
// // *T
// // *LUT DispalyablePhysics
// //
// //
// // 1.
//
// //  Physics -> mass, accel
// //  Display -> mass
// //  100 other traits?
// //  2. Perfromance
//
// // "multipl traits on a trait object?"
// // ONE REALLY BIG LUT
// // ALWYS""  gets constructed
// fn multiple_traits<T: DiplayablePhysics>(t: T) {}
//
//
//
trait Foo {
    fn foo(&self);
}

trait Foo2 {
    fn foo(&self);
}

struct Example;

impl Foo for Example {
    fn foo(&self) {
        println!("i32");
    }
}

impl Foo2 for Example {
    fn foo(&self) {
        todo!()
    }
}

fn uses_both() {
    let ex = Example;
    // UFCS

    // ex.foo();
    Foo::foo(&ex);
    Foo2::foo(&ex);
    ex.thing();
    // eqiv
    // Example::thing(&self);
}

// Super
// Kid1, Kid2 inherit Super and override a function
// Grandchild inherits Kid1 AND Kid2
impl Example {
    fn thing(&self) {}
}

// qns?
// workshop
// Generics is *exactly* one thing
// Vec<T: Physics> -> REpalce the T with the type
// vec![Earth] -> Vec<Earth>
// vec![Mars] -> Vec<Mars>
// vec![Mars, Earth] -> Vec<Mars... CANNOT have Earth>
//
