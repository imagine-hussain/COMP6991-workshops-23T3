use libc::{c_char, c_double, c_int, fclose, fgets, fopen, fscanf, FILE};

struct File {
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
                    false => *r as usize,
                };
                out.extend_from_slice(&buf[..bytes_read - 1]);
                if bytes_read < Self::BUF_SIZE {
                    break;
                }
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

unsafe trait Scary {}

struct Foo;

unsafe impl Scary for Foo {}

fn raw_ptr() -> *mut String {
    let mut s = String::from("scary");
    let boxed = Box::leak(Box::new(s));
    let p = boxed as *mut String;
    unsafe {
        drop(Box::from_raw(p));
    }
    return p;
}

/// # Safety
/// bro pls give me good ptr
unsafe fn free<T>(p: *mut T) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}

fn precedenc(f: *mut File) {
    unsafe { (*f).fp };
}
