extern crate memmap;

use std::env::args;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

use memmap::{MmapOptions, Mmap};


struct Test {
  mmap: Mmap,
  size: usize,
}

impl Test {
    pub fn new<P>(filename: P) -> io::Result<Self> where P: AsRef<Path> {
        let file = OpenOptions::new().read(true).write(false).open(filename)?;
        let size = file.metadata()?.len();
        let mmap = unsafe {
            MmapOptions::new().map(&file)?
        };
        Ok(Test {
            mmap: mmap,
            size: size as usize,
        })
    }

    #[inline(never)]
    pub fn get(&self, l: usize) -> u64 {
        let loc = l % self.size;

        let ptr: *const u8 = self.mmap.as_ptr();
        let end_byte = unsafe {
            u64::from(*ptr.offset(loc as isize))
        };
        let mut v = u64::from(end);
        v >>= 7 - ((loc - 1) & 7);
        v
    }
}

// we could use an RNG, but I want to make sure everything is
// as comparable as possible
fn next_random(n: usize) -> usize {
    // https://en.wikipedia.org/wiki/Xorshift
    let mut x = n as u32;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x as usize
}

fn main() {
    let filename = args().nth(1).expect("need [filename] [n_samples]");
    let n_samples = args().nth(2).expect("need [n_samples]").parse::<usize>().expect("n_samples must be an integer");

    let test = Test::new(filename).unwrap();
    let mut r = 0;
    let mut i = 1;
    for _ in 0..n_samples {
        r += test.get(i);
        i = next_random(i);
    }
    println!("{}", r);
}
