extern crate memmap;

use std::env::args;
use std::fs::OpenOptions;
use std::io;
use std::mem::transmute;
use std::path::Path;

use memmap::{MmapOptions, Mmap};

type BitVecSlice = u64;
const BIT_VEC_SLICE_SIZE: u8 = 64;


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
    fn get_range(&self, loc: usize) -> BitVecSlice {
        let l = loc % (self.size - 64);

        let byte_idx_st = (l >> 3) as usize;
        let byte_idx_en = ((l + 63) >> 3) as usize;
        let new_size: u8 = 64 as u8;

        let ptr: *const u8 = self.mmap.as_ptr();

        // read the last byte first
        let end = unsafe {
            *ptr.offset(byte_idx_en as isize)
        };
        // align the end of the data with the end of the u64/u128
        let mut v = BitVecSlice::from(end);
        v >>= 7 - ((l + 63) & 7);

        if l < self.size - BIT_VEC_SLICE_SIZE as usize {
            // really nasty/unsafe, but we're just reading a u64/u128 out instead of doing it
            // byte-wise --- also does not work with legacy mode!!!
            unsafe {
                let lg_ptr: *const BitVecSlice = transmute(ptr.offset(byte_idx_st as isize));
                v |= (*lg_ptr).to_be() << (l & 7) >> (BIT_VEC_SLICE_SIZE - new_size);
            }
        } else {
            // special case if we can't get a whole u64 out without running outside the buffer
            let bit_offset = new_size + (l & 7) as u8;
            for (new_idx, old_idx) in (byte_idx_st..byte_idx_en).enumerate() {
                unsafe {
                    v |= BitVecSlice::from(*ptr.offset(old_idx as isize)) <<
                        (bit_offset - 8u8 * (new_idx as u8 + 1));
                }
            }
        }

        // mask out the high bits in case we copied extra
        v & (BitVecSlice::max_value() >> (BIT_VEC_SLICE_SIZE - new_size))
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
        r += test.get_range(i);
        i = next_random(i);
    }
    println!("{}", r);
}
