pub const DEBUG: bool        = false;
pub const DEBUG_CHECKS: bool = false;
pub const DEBUG_TAB: bool    = false;

pub type CNF = Vec<Vec<i32>>;

pub fn step_to_cnf(inp: i32, nums: Vec<i32>) -> CNF {
    if DEBUG {println!("inp: {inp}, nums: {nums:?}")};

    assert!(nums.len() == 5);

    vec![vec![-inp,-nums[0],-nums[1],-nums[2],-nums[3],-nums[4]], vec![-inp,-nums[0],-nums[1],-nums[2],nums[3],nums[4]], vec![-inp,-nums[0],-nums[1],nums[2],-nums[3],nums[4]], vec![-inp,-nums[0],-nums[1],nums[2],nums[3],-nums[4]], vec![-inp,-nums[0],nums[1],-nums[2],-nums[3],nums[4]], vec![-inp,-nums[0],nums[1],-nums[2],nums[3],-nums[4]], vec![-inp,-nums[0],nums[1],nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],-nums[1],-nums[2],-nums[3],nums[4]], vec![-inp,nums[0],-nums[1],-nums[2],nums[3],-nums[4]], vec![-inp,nums[0],-nums[1],nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],nums[1],-nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],nums[1],nums[2],nums[3]], vec![-inp,nums[0],nums[1],nums[2],nums[4]], vec![-inp,nums[0],nums[1],nums[3],nums[4]], vec![-inp,nums[0],nums[2],nums[3],nums[4]], vec![-inp,nums[1],nums[2],nums[3],nums[4]], vec![inp,-nums[0],-nums[1],-nums[2],-nums[3],nums[4]], vec![inp,-nums[0],-nums[1],-nums[2],nums[3],-nums[4]], vec![inp,-nums[0],-nums[1],nums[2],-nums[3],-nums[4]], vec![inp,-nums[0],-nums[1],nums[2],nums[3],nums[4]], vec![inp,-nums[0],nums[1],-nums[2],-nums[3],-nums[4]], vec![inp,-nums[0],nums[1],-nums[2],nums[3],nums[4]], vec![inp,-nums[0],nums[1],nums[2],-nums[3],nums[4]], vec![inp,-nums[0],nums[1],nums[2],nums[3],-nums[4]], vec![inp,nums[0],-nums[1],-nums[2],-nums[3],-nums[4]], vec![inp,nums[0],-nums[1],-nums[2],nums[3],nums[4]], vec![inp,nums[0],-nums[1],nums[2],-nums[3],nums[4]], vec![inp,nums[0],-nums[1],nums[2],nums[3],-nums[4]], vec![inp,nums[0],nums[1],-nums[2],-nums[3],nums[4]], vec![inp,nums[0],nums[1],-nums[2],nums[3],-nums[4]], vec![inp,nums[0],nums[1],nums[2],-nums[3],-nums[4]]]
}

pub fn determine_cnf(width: i32, period: i32) -> CNF {
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
    
    // Add maximum seperation constraint to remove compound solutions
    // Also forces the last bit to be 1, meaning mostly unique solutions
    // Only applies for the first row; might seperate later and conjoin again
    const MAX_SEP: i32 = 4;
    for col in 0..(width - MAX_SEP) {
        let mut constraint = vec![-index_table_else_reserved((&table, 0, col))];

        for next in (col + 1)..=(col + MAX_SEP) {
            constraint.push(index_table_else_reserved((&table, 0, next)));
        }
        
        o.push(constraint);
    }

    // Not all zero
    if DEBUG_CHECKS {println!("first row: \n {:?}", table[0]);}

    o.push(table.into_iter().flatten().collect());

    o
}

pub fn format_table(table: &CNF) -> String {
    "[".to_string() + &table.iter()
        .map(|row| row.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "))
        .map(|s| "[".to_string() + &s + "]")
        .collect::<Vec<_>>()
        .join("\n") + "]"
}

pub fn create_cnf(width: i32, period: i32) -> CNF {
    let v: CNF = determine_cnf(width, period);

    let non_taut: Vec<Vec<i32>> = v.clone().into_iter().filter(|r| !r.contains(&-1)).map(|r| r.into_iter().filter(|c| *c != 1).collect()).collect();
    let non_taut: Vec<Vec<i32>> = non_taut.into_iter().filter(|r| !(1..=(r.iter().map(|c| c.abs()).max().unwrap_or(0))).any(|x| r.contains(&x) && r.contains(&-x))).map(|r| {let mut o = Vec::new(); r.into_iter().for_each(|i| if !o.contains(&i) {o.push(i)}); o}).collect();
    let non_taut: Vec<Vec<i32>> = {let mut o = Vec::new(); non_taut.into_iter().for_each(|r| if !o.contains(&r) {o.push(r);}); o};

    if DEBUG {println!("Simplified: \n{}", format_table(&non_taut));}

    non_taut
}