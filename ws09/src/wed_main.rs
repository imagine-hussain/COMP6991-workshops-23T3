use libc::{c_char, c_double, c_int, fclose, fgets, fopen, fscanf, FILE};

struct File {
    // // ????????????????
    // todo!()
    fp: *mut FILE,
}

/// This function converts a string into a Vec<i8> which can
/// be used to represent a c-string.
///
/// To turn this into a *mut c_char, use Vec<i8>::as_mut_ptr().
fn to_c_string(string: &str) -> Vec<i8> {
    let bytes: Vec<u8> = String::from(string).into_bytes();
    let mut c_chars: Vec<i8> = bytes.iter().map(|c| *c as i8).collect();

    c_chars.push(0); // null terminator

    c_chars
}

impl File {
    const BUF_SIZE: usize = 1024;
    pub fn open(path: &str) -> Option<Self> {
        let c_path = to_c_string(path);
        let c_rdonly = to_c_string("r");
        // # Safety:
        // We check that the file pointer is not null
        // before use. On a null ptr, we return None
        // from the function
        let fp = unsafe {
            let fp = fopen(c_path.as_ptr(), c_rdonly.as_ptr());
            match fp.is_null() {
                true => return None,
                false => fp,
            }
        };
        Some(Self { fp })
    }

    fn read_string(&mut self) -> Option<String> {
        // Read a string from the file
        let mut out = Vec::<u8>::new();
        let mut buf: [u8; Self::BUF_SIZE] = [0; Self::BUF_SIZE];
        let buf_p = buf.as_mut_ptr() as *mut c_char;
        loop {
            // # Safety:
            // Only read up to `Self::BUF_SIZE` bytes in
            // appropriatly sized buffer
            // fgets returns null on finish/err - Check that null
            dbg!(out.len());
            unsafe {
                let r = fgets(buf_p, Self::c_buf_size(), self.fp);
                let bytes_read = match r.is_null() {
                    true => break,
                    false if *r == 0 => break,
                    false => *r as usize,
                };
                out.extend_from_slice(&buf[..bytes_read - 1]);
            }
        }
        // Convert the bytes into a string, checking for utf8
        String::from_utf8(out).ok()
    }

    #[inline(always)]
    pub const fn c_buf_size() -> c_int {
        Self::BUF_SIZE as c_int
    }

    fn read_i64(&mut self) -> Option<i64> {
        todo!()
    }

    fn read_f64(&mut self) -> Option<f64> {
        todo!()
    }

    fn read_char(&mut self) -> Option<char> {
        todo!()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // what do we on drop?
        println!("Dropping file.");
    }
}

fn main() {
    let mut file = File::open("data/test_file.txt").expect("Could not open file.");
    let s = dbg!(file.read_string().unwrap());
    let i = file.read_i64().unwrap();
    let f = file.read_f64().unwrap();
    let c = file.read_char().unwrap();

    println!("{s} {i} {f} {c}");
}

// fn dp(cache, x) {
//     if x == 0 {
//         return cache.lock().unwrap()[0]
//     }
//     return cache.lock().unwrap()[x] + dp(cache, x - 1);
//     //            ^ guard                 ^ recursive call
// }
//
// struct Foo {
//     lcok: ReEntrant<T>
// }
//
// impl Foo {
//     fn f1(&self) {
//         lock.f1();
//         let mut lock = self.lock.lock().unwrap();
//     }
//
// }
//
//
// struct Foo {
//     inner: RefCell<i32>,
// }
//
// unsafe trait MyTrait {}

// WHAT??????????/
//
// Unsound vs Unsafe??????
// - sound: proper functioning code - does what we want
//  - No UB
//- safe:
//  - subset of sound code
//  - code that the compiler can guarantee is sound
//- unsafe code:
//  - *can* be unsound
//  - *can* be sound
//  - the COMPILEr cant check soudnness
//  - the USER must check soundness
//
// - There is code that is SOUND but, is not SAFE
// - Keeps possible "unsound" code in specificically marked regions
//
// FIVE unsafe superpowers:
//  - Dereference a raw pointer
//  - Call an unsafe function or method
//  - Access or modify a mutable static variable
//  - Implement an unsafe trait
//  - Access fields of unions
//
// Safe abstractions over unsafe code

// does rust give u file pointers?
// -
//
// fn foo() {}
//
// unsafe functionso vs block
//
// why?
// fn pointer() {
//     let x = String::new();
//     let p_raw = &x as *const String;
//     let p_mut: *mut String = p_raw as *mut String;
//     drop(x);
//     // # Safety
//     // AHJJJJJJJJJJ
//     // I HVE CHECKED that this is not bad
//     unsafe {
//         (*p_mut).push('h');
//     };
// }
//
// fn uses_unsafe() {
//     pointer();
//     unsafe {
//         scaryyyyyyyyyyy(std::ptr::null_mut());
//     };
// }
//
// /// # Safety
// /// DONT do x, y, z
// /// ENSURE a, b, c
// /// hey user, pls make sure, f is not null
// unsafe fn scaryyyyyyyyyyy(f: *mut FILE) {
//     *f;
//     let x = String::new();
//     let p_raw = &x as *const String;
//     let p_mut: *mut String = p_raw as *mut String;
//     drop(x);
//     // depends?
//     unsafe {
//         (*p_mut).push('h');
//     }
// }
//
// FFI <->
// foregin functions interface
