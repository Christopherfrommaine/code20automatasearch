use rayon::prelude::*;
use std::collections::HashSet;
use cgrustplot::plots::array_plot::array_plot;
use std::fs::OpenOptions;
use std::io::Write;

// Implements a code-20 step on a integer acting like a boolean list.
#[inline(always)]
fn step(init: u64) -> u64 {
    // Bitshift to have the neighbors of each bit be (a, b, c, d, e)
    let a = init.rotate_right(2);
    let b = init.rotate_right(1);
    let c = init;
    let d = init.rotate_left(1);
    let e = init.rotate_left(2);

    // Bitwise definition of code 20
    (a | b | c | d | e) ^ (a ^ b ^ c ^ d ^ e)
}

// Check if an automata will halt after max states
#[inline(always)]
fn lifetimeinfinite(init: u64, max: u32) -> bool {
    let mut o = init;
    for _ in 0..max {
        if o == 0 {return false;}

        o = step(o);
    }
    return true;
}

// Converts a u64 to a list of bits. Used for printing and debugging
fn u64tobits(n: u64) -> Vec<u8> {
    let mut bits = Vec::with_capacity(64);
    for i in (0..64).rev() {
        bits.push(((n >> i) & 1) as u8);
    }
    bits
}

// Creates a bitmask between two positions
fn set_bits_range(j: u64, jn: u64) -> u64 {
    if j <= jn {
        // Create a contiguous mask from j to jn
        ((1 << (jn - j + 1)) - 1) << j
    } else {
        // Wrap-around case: Set bits from j to 15 and from 0 to jn
        let high_mask = (!0u64) << j;
        let low_mask = (1 << (jn + 1)) - 1;
        high_mask | low_mask
    }
}

// Tests for multiple patterns in a single automata
fn test_duplicates(next_hundred: &[u64], h: &mut HashSet<u64>) -> bool {
    // The way this works is by splitting across empty columns and checking for uniqueness on each side of the split

    // Find split positions (i.e. empty columns)
    const MAX_SEP: u64 = 3;
    let mut split_pos: Vec<u64> = Vec::new();
    let mut num_split = 0;
    for g in 0..64 {
        if next_hundred.iter().all(|x| x & (1 << g) == 0) {
            num_split += 1;
            if num_split == MAX_SEP {
                split_pos.push(g);
            }
        } else {
            num_split = 0;
        }
    }

    // Check if each side of the split is unique
    let mut only = true;
    for j in 0..split_pos.len() {
        let jn = (j + 1) % split_pos.len();
        let bm = set_bits_range(split_pos[j], split_pos[jn]);
        // println!("{bm:016X}");
        let mn = bm & next_hundred[0];
        if h.contains(&(mn >> mn.trailing_zeros())) {
            only = false;
            break;
        }
    }

    only
}

fn main() {
    // Setup for the output file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Failed to create output file.");

    // Stores previously seen states
    let mut h: HashSet<u64> = HashSet::new();

    // Batching for paralellization
    let batch_size: u64 = 1_000_000;
    let mut bn: u64 = 0; // current batch number

    loop {
        // Generate list of possible initialization states
        let v: Vec<u64> = ((batch_size * bn + 1)..(batch_size * (bn + 1))).into_par_iter()
            
            .filter(|i| {
                // Filter for symmetries (where you can mirror or rotate to get the same state)
                let j = i.reverse_bits();
                if *i > j >> j.trailing_zeros() {return false;}

                // Filter for nonhalting (remove all automata that halt)
                lifetimeinfinite(i << 32, 500)
            })

            .filter(|i| {
                
                // Filter for uniqueness over 10 states
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
            // Check for uniqueness AGAIN but with more states (rare to get this far so it's okay that it repeats some calculations)
            let mut s = i << 32;
            for _ in 0..500 {s = step(s);}

            // Generate 100 states
            let mut next_hundred = [0u64; 100];
            for n in 0..100 {
                s = step(s);
                next_hundred[n] = s;
            }

            // Check if any have been seen
            let not_unique = next_hundred.iter().any(|x| h.contains(&(x >> x.trailing_zeros())));
            if not_unique {continue;}

            // Check for more than one duplicated patterns
            let mut only = test_duplicates(&next_hundred, &mut h);
            only = only && test_duplicates(&next_hundred[95..], &mut h);
            only = only && test_duplicates(&(next_hundred[90..].iter().map(|x| x >> x.trailing_zeros()).collect::<Vec<u64>>()), &mut h);
            only = only && test_duplicates(&(next_hundred[30..40].iter().map(|x| x >> x.trailing_zeros()).collect::<Vec<u64>>()), &mut h);

            // Update hash list with seen sttates
            for f in 0..64 {
                next_hundred.iter().for_each(|x| {
                    let xr = x.rotate_right(f);
                    h.insert(xr >> xr.trailing_zeros());
                    h.insert(xr.reverse_bits() >> xr.reverse_bits().trailing_zeros());
                });
            }

            // If the state is unique and only contains one pattern, write it to a file.
            if only {
                writeln!(file, "{},", i).expect("");
                
                println!("-----\nNew Found! {}:", i);
                array_plot(&next_hundred.iter().map(|x| u64tobits(*x)).collect()).set_axes(false).print();
                println!("-----")
            }
        }

        println!("Batch {bn} | Integers up to {} searched.", batch_size * bn);

        bn += 1;
    }
}
