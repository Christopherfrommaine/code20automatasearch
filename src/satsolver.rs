use splr::*;
use cgrustplot::plots::array_plot;
use rayon::prelude::*;
use crate::customuint::U256;

const DEBUG: bool = false;
const DEBUG_CHECKS: bool = false;
const DEBUG_TAB: bool = false;

type CNF = Vec<Vec<i32>>;
fn step_to_cnf(inp: i32, nums: Vec<i32>) -> CNF {
    if DEBUG {println!("inp: {inp}, nums: {nums:?}")};

    assert!(nums.len() == 5);

    vec![vec![-inp,-nums[0],-nums[1],-nums[2],-nums[3],-nums[4]], vec![-inp,-nums[0],-nums[1],-nums[2],nums[3],nums[4]], vec![-inp,-nums[0],-nums[1],nums[2],-nums[3],nums[4]], vec![-inp,-nums[0],-nums[1],nums[2],nums[3],-nums[4]], vec![-inp,-nums[0],nums[1],-nums[2],-nums[3],nums[4]], vec![-inp,-nums[0],nums[1],-nums[2],nums[3],-nums[4]], vec![-inp,-nums[0],nums[1],nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],-nums[1],-nums[2],-nums[3],nums[4]], vec![-inp,nums[0],-nums[1],-nums[2],nums[3],-nums[4]], vec![-inp,nums[0],-nums[1],nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],nums[1],-nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],nums[1],nums[2],nums[3]], vec![-inp,nums[0],nums[1],nums[2],nums[4]], vec![-inp,nums[0],nums[1],nums[3],nums[4]], vec![-inp,nums[0],nums[2],nums[3],nums[4]], vec![-inp,nums[1],nums[2],nums[3],nums[4]], vec![inp,-nums[0],-nums[1],-nums[2],-nums[3],nums[4]], vec![inp,-nums[0],-nums[1],-nums[2],nums[3],-nums[4]], vec![inp,-nums[0],-nums[1],nums[2],-nums[3],-nums[4]], vec![inp,-nums[0],-nums[1],nums[2],nums[3],nums[4]], vec![inp,-nums[0],nums[1],-nums[2],-nums[3],-nums[4]], vec![inp,-nums[0],nums[1],-nums[2],nums[3],nums[4]], vec![inp,-nums[0],nums[1],nums[2],-nums[3],nums[4]], vec![inp,-nums[0],nums[1],nums[2],nums[3],-nums[4]], vec![inp,nums[0],-nums[1],-nums[2],-nums[3],-nums[4]], vec![inp,nums[0],-nums[1],-nums[2],nums[3],nums[4]], vec![inp,nums[0],-nums[1],nums[2],-nums[3],nums[4]], vec![inp,nums[0],-nums[1],nums[2],nums[3],-nums[4]], vec![inp,nums[0],nums[1],-nums[2],-nums[3],nums[4]], vec![inp,nums[0],nums[1],-nums[2],nums[3],-nums[4]], vec![inp,nums[0],nums[1],nums[2],-nums[3],-nums[4]]]
}

fn determine_cnf(width: i32, period: i32) -> CNF {

    let mut ind = 1;
    let reserved = ind; ind += 1;

    let mut o = vec![vec![-reserved]];

    let mut table: Vec<Vec<i32>> = vec![vec![0; width as usize]; period as usize];

    if DEBUG{println!("tab: \n {table:?}");}

    let index_table_else_reserved = |(t, r, c): (&Vec<Vec<i32>>, i32, i32)| if 0 <= r && r < period && 0 <= c && c < width {t[r as usize][c as usize]} else {reserved};

    // Fill table
    for row in 0..period {
        for col in 0..width {
            table[row as usize][col as usize] = ind; ind += 1;
        }
    }

    if DEBUG || DEBUG_TAB {println!("filled tab: \n {table:?}");}

    // Set contraints
    for row in 0..period {
        // extra padding to include contraints that the bordering cells must also stay at zero
        for col in -2..(width + 2) {
            // next row
            let mut rown = row - 1;
            if row == 0 {rown = period - 1;}

            let nums: Vec<i32> = ((col - 2)..=(col + 2)).map(|c| (&table, rown, c)).map(index_table_else_reserved).collect();

            // Code 20 constraints
            o.extend_from_slice(&step_to_cnf(index_table_else_reserved((&table, row, col)), nums));
            
            // o.extend_from_slice(&vec![nums]);  // debugging indices
        }
    }

    // Add unequal row constraints
    // See note for derivation
    for row in 0..(period - 1) {
        let mut zs = Vec::new();
        for col in 0..width {
            let z = ind; ind += 1;
            let a = table[row as usize][col as usize];
            let b = table[period as usize - 1][col as usize];

            zs.push(z);

            o.extend_from_slice(&[vec![-z, -a, -b], vec![z, -a, b], vec![z, a, -b], vec![-z, a, b]]);
        }
        o.push(zs);
    }
    

    // Not all zero
    // println!("first row: \n {:?}", table[0]);

    o.push(table.into_iter().flatten().collect());

    o
}

fn cnf_example() -> CNF {
    vec![vec![1, 2], vec![-1, 3], vec![1, -3], vec![-1, 2]]
}

fn format_table(table: &CNF) -> String {
    "[".to_string() + &table.iter()
        .map(|row| row.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "))
        .map(|s| "[".to_string() + &s + "]")
        .collect::<Vec<_>>()
        .join("\n") + "]"
}

pub fn main() {
    vec![1, 2, 3, 4, 6, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30].into_par_iter().for_each(|i| {
        let mut w = 5;
        while !run_thing(2 << w, i) && (2 << w) < 100 {
            w += 1;
        }
    });

    // (1..100).into_par_iter().for_each(|i| {
    //     run_thing(100, i);
    // });

    // vec![50, 100, 200, 500].into_par_iter().for_each(|w| {run_thing(w, 12);});
}

pub fn run_thing(width: i32, period: i32) -> bool {

    let v: CNF = determine_cnf(width, period);

    // println!("{}", format_table(&v));

    // Removes all statements with -1 (tautology) and removes 1 from any statements (contradiction) and simplifies tautological form [p, -p, ...] -> true.
    let non_taut: Vec<Vec<i32>> = v.clone().into_iter().filter(|r| !r.contains(&-1)).map(|r| r.into_iter().filter(|c| *c != 1).collect()).collect();
    let non_taut: Vec<Vec<i32>> = non_taut.into_iter().filter(|r| !(1..=(r.iter().map(|c| c.abs()).max().unwrap_or(0))).any(|x| r.contains(&x) && r.contains(&-x))).map(|r| {let mut o = Vec::new(); r.into_iter().for_each(|i| if !o.contains(&i) {o.push(i)}); o}).collect();
    let non_taut: Vec<Vec<i32>> = {let mut o = Vec::new(); non_taut.into_iter().for_each(|r| if !o.contains(&r) {o.push(r);}); o};
    
    if DEBUG {println!("Simplified: \n{}", format_table(&non_taut));}

    match Certificate::try_from(non_taut) {
        Ok(Certificate::SAT(ans)) => {handle_sol(ans, width, period); return true;},
        Ok(Certificate::UNSAT) => println!("s UNSATISFIABLE for period {period} and width {width}"),
        Err(e) => panic!("s UNKNOWN; {}", e),
    }

    false
}

fn step64(init: u64) -> u64 {
    // Bitshift to have the neighbors of each bit be (a, b, c, d, e)
    let a = init.rotate_right(2);
    let b = init.rotate_right(1);
    let c = init;
    let d = init.rotate_left(1);
    let e = init.rotate_left(2);

    // Bitwise definition of code 20
    (a | b | c | d | e) ^ (a ^ b ^ c ^ d ^ e)
}

fn step256(init: U256) -> U256 {
    // Bitshift to have the neighbors of each bit be (a, b, c, d, e)
    let a = init.rotate_right(2);
    let b = init.rotate_right(1);
    let c = init;
    let d = init.rotate_left(1);
    let e = init.rotate_left(2);

    // Bitwise definition of code 20
    (a | b | c | d | e) ^ (a ^ b ^ c ^ d ^ e)
}

fn u64tobits(n: u64) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::with_capacity(64);
    for i in (0..64).rev() {
        bits.push(((n >> i) & 1) as u8);
    }
    bits
}

fn u256tobits(n: U256) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::with_capacity(256);
    for i in (0..256).rev() {
        bits.push(((n >> i) & U256::from(1)).as_u64() as u8);
    }
    bits
}

fn vec_bool_to_bytes(bits: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity((bits.len() + 7) / 8);
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for &bit in chunk {
            byte = (byte << 1) | (bit as u8);
        }
        // Shift left to align MSB if chunk is less than 8 bits
        byte <<= 8 - chunk.len();
        bytes.push(byte);
    }
    bytes
}

fn handle_sol(ans: Vec<i32>, width: i32, period: i32) {
    println!("Solution Found for period {period}!");

    let first_row = &ans[1..(width as usize + 1)];
    let binary_row: Vec<bool> = first_row.iter().map(|d| d > &0).collect();
    let row_bytes: Vec<u8> = vec_bool_to_bytes(&binary_row);
    if row_bytes.len() < 4 * 8 {
        let state = U256::from_big_endian(&row_bytes);



        println!("State: {state}");

        let mut s = state;
        
        array_plot::array_plot(&(0..(5 * period)).map(|_| {let temp = s; s = step256(s); temp}).map(|x| {let mut o = u256tobits(x); o.reverse(); o}).collect()).set_axes(false).print();

    } else {
        println!("State (as list): {binary_row:?}");
    }
    
}