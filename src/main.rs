use rayon::prelude::*;
use std::collections::HashSet;

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

#[inline(always)]
fn lifetimeinfinite(init: u64, max: u32) -> bool {
    let mut o = init;
    for _ in 0..max {
        if o == 0 {return false;}

        o = step(o);
    }
    return true;
}

fn hasheduniquepatterns() {
    let mut h: HashSet<u64> = HashSet::new();

    let batch_size: u64 = 1_000_000;
    let mut bn: u64 = 0;

    loop {
        let v: Vec<u64> = ((batch_size * bn + 1)..(batch_size * (bn + 1)))
            .into_par_iter()
            .filter(|i| {
                let j = i.reverse_bits();
                if *i > j >> j.trailing_zeros() {return false;}

                lifetimeinfinite(i << 32, 500)
            })
            .collect();
        bn += 1;

        for i in v {
            let mut s = i << 32;
            for _ in 0..500 {s = step(s);}

            let mut next_ten = [0u64; 10];
            for n in 0..10 {
                s = step(s);
                next_ten[n] = s;
            }

            // Also check reverse case
            let not_unique = next_ten.iter().any(|x| h.contains(&(x >> x.trailing_zeros())));

            if not_unique {continue;}

            // More expensive check, but extremely rare to get this far
            let mut next_hundred = [0u64; 100];
            for n in 0..100 {
                s = step(s);
                next_hundred[n] = s;
            }

            let not_unique = next_hundred.iter().any(|x| h.contains(&(x >> x.trailing_zeros())));

            if not_unique {continue;}

            for f in 0..64 {
                next_hundred.iter().for_each(|x| {
                    h.insert(x.rotate_right(f));
                    h.insert(x.reverse_bits().rotate_right(f));
                });
            }

            println!("{},", i);
        }
    }
}

fn main() {
    hasheduniquepatterns();
}