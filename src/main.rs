use rayon::prelude::*;
use std::collections::HashSet;
use cgrustplot::plots::array_plot::array_plot;

// Implements a code-20 step on a integer acting like a boolean list.
#[inline(always)]
fn step(init: u64) -> u64 {
    // Bitshift to have the neighbors of each bit be (a,.b, c, d, e)
    let a = init.rotate_right(2);
    let b = init.rotate_right(1);
    let c = init;
    let d = init.rotate_left(1);
    let e = init.rotate_left(2);

    // Based on the definition of code 20
    (a | b | c | d | e) ^ (a ^ b ^ c ^ d ^ e)
}

// Will the initialization never halt?
#[inline(always)]
fn lifetimeinfinite(init: u64, max: u32) -> bool {
    let mut o = init;
    for _ in 0..max {
        if o == 0 {return false;}

        o = step(o);
    }
    return true;
}

// Converts a u64 to a list of bits. Used for debugging
fn u64tobits(n: u64) -> Vec<u8> {
    let mut bits = Vec::with_capacity(64);
    for i in (0..64).rev() {
        bits.push(((n >> i) & 1) as u8);
    }
    bits
}

fn main() {
    let mut h: HashSet<u64> = HashSet::new();  // Keeps track of seen states

    // Batching for paralellization
    let batch_size: u64 = 1_000_000;
    let mut bn: u64 = 0; // batch number

    loop {
        // Paralellize filtering of halting states and filtering of seen states
        let v: Vec<u64> = ((batch_size * bn + 1)..(batch_size * (bn + 1)))
            .into_par_iter()
            .filter(|i| {
                let j = i.reverse_bits();
                if *i > j >> j.trailing_zeros() {return false;}

                lifetimeinfinite(i << 32, 500)
            })
            .filter(|i| {
                let mut s = i << 32;
                for _ in 0..500 {s = step(s);}

                let mut next_ten = [0u64; 10];
                for n in 0..10 {
                    s = step(s);
                    next_ten[n] = s;
                }

                !next_ten.iter().any(|x| h.contains(&(x >> x.trailing_zeros())))
            }).collect();
        
        // For non-halting, non-seen states:
        for i in v {
            // Check for uniqueness AGAIN (repeated in case the hash set was updated previuosly)
            let mut s = i << 32;
            for _ in 0..500 {s = step(s);}

            // More expensive check, but rare to get this far
            let mut next_hundred = [0u64; 100];
            for n in 0..100 {
                s = step(s);
                next_hundred[n] = s;
            }

            let not_unique = next_hundred.iter().any(|x| h.contains(&(x >> x.trailing_zeros())));

            if not_unique {continue;}

            if i == 678175 {
                let d: Vec<Vec<u8>> = next_hundred.iter().map(|x| u64tobits(*x)).collect();
                array_plot(&d).set_axes(false).print();
            }

            // Check for duplicated patterns
            const SEP: u64 = 3; // minimum seperation
            const SEPB: u64 = (1u64 << SEP) - 1;  // seperation bitmask
            let mut only = true;
            for g in 1..64 {
                if (SEPB.rotate_left(g) & s) == 0 {
                    let low = s & ((1 << g) - 1);
                    let high = s & (((1 << (64 - g)) - 1) << g);

                    if low != 0 && high != 0 && h.contains(&(low >> low.trailing_zeros())) || h.contains(&(high >> high.trailing_zeros())) {
                        only = false;
                        break;
                    }
                }
            }

            // Update hash list
            for f in 0..64 {
                next_hundred.iter().for_each(|x| {
                    let xr = x.rotate_right(f);
                    h.insert(xr >> xr.trailing_zeros());
                    h.insert(xr.reverse_bits() >> xr.reverse_bits().trailing_zeros());
                });
            }

            if only {
                println!("{},", i);
            }
        }

        bn += 1;
    }
}
