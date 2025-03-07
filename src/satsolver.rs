use splr::*;

type CNF = Vec<Vec<i32>>;

fn step_to_cnf(inp: i32, nums: Vec<i32>) -> CNF {
    println!("inp: {inp}, nums: {nums:?}");

    assert!(nums.len() == 5);

    vec![vec![-inp,-nums[0],-nums[1],-nums[2],-nums[3],-nums[4]], vec![-inp,-nums[0],-nums[1],-nums[2],nums[3],nums[4]], vec![-inp,-nums[0],-nums[1],nums[2],-nums[3],nums[4]], vec![-inp,-nums[0],-nums[1],nums[2],nums[3],-nums[4]], vec![-inp,-nums[0],nums[1],-nums[2],-nums[3],nums[4]], vec![-inp,-nums[0],nums[1],-nums[2],nums[3],-nums[4]], vec![-inp,-nums[0],nums[1],nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],-nums[1],-nums[2],-nums[3],nums[4]], vec![-inp,nums[0],-nums[1],-nums[2],nums[3],-nums[4]], vec![-inp,nums[0],-nums[1],nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],nums[1],-nums[2],-nums[3],-nums[4]], vec![-inp,nums[0],nums[1],nums[2],nums[3]], vec![-inp,nums[0],nums[1],nums[2],nums[4]], vec![-inp,nums[0],nums[1],nums[3],nums[4]], vec![-inp,nums[0],nums[2],nums[3],nums[4]], vec![-inp,nums[1],nums[2],nums[3],nums[4]], vec![inp,-nums[0],-nums[1],-nums[2],-nums[3],nums[4]], vec![inp,-nums[0],-nums[1],-nums[2],nums[3],-nums[4]], vec![inp,-nums[0],-nums[1],nums[2],-nums[3],-nums[4]], vec![inp,-nums[0],-nums[1],nums[2],nums[3],nums[4]], vec![inp,-nums[0],nums[1],-nums[2],-nums[3],-nums[4]], vec![inp,-nums[0],nums[1],-nums[2],nums[3],nums[4]], vec![inp,-nums[0],nums[1],nums[2],-nums[3],nums[4]], vec![inp,-nums[0],nums[1],nums[2],nums[3],-nums[4]], vec![inp,nums[0],-nums[1],-nums[2],-nums[3],-nums[4]], vec![inp,nums[0],-nums[1],-nums[2],nums[3],nums[4]], vec![inp,nums[0],-nums[1],nums[2],-nums[3],nums[4]], vec![inp,nums[0],-nums[1],nums[2],nums[3],-nums[4]], vec![inp,nums[0],nums[1],-nums[2],-nums[3],nums[4]], vec![inp,nums[0],nums[1],-nums[2],nums[3],-nums[4]], vec![inp,nums[0],nums[1],nums[2],-nums[3],-nums[4]]]
}

fn determine_cnf(width: i32, period: i32) -> CNF {

    let mut ind = 1;
    let reserved = ind; ind += 1;

    let mut o = vec![vec![-reserved]];

    let mut table: Vec<Vec<i32>> = vec![vec![0; width as usize]; period as usize];

    println!("tab: \n {table:?}");

    let index_table_else_reserved = |(t, r, c): (&Vec<Vec<i32>>, i32, i32)| if 0 <= r && r < period && 0 <= c && c < width {t[r as usize][c as usize]} else {reserved};

    // Fill table
    for row in 0..period {
        for col in 0..width {
            table[row as usize][col as usize] = ind; ind += 1;
        }
    }

    println!("filled tab: \n {table:?}");

    // Set contraints
    for row in 0..period {

        // extra padding to include contraints that the bordering cells must also stay at zero
        for col in -2..(width + 2) {
            // next row
            let rown = (row - 1) % period;

            let nums: Vec<i32> = ((col - 2)..=(col + 2)).map(|c| (&table, rown, c)).map(index_table_else_reserved).collect();

            o.extend_from_slice(&step_to_cnf(index_table_else_reserved((&table, row, col)), nums));
            
            // o.extend_from_slice(&vec![nums]);  // debugging indices
        }
    }

    // Not all zero
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
    let v: CNF = determine_cnf(100, 1);

    println!("{}", format_table(&v));

    // Removes all statements with -1 (tautology) and removes 1 from any statements (contradiction) and simplifies tautological form [p, -p, ...] -> true.
    let non_taut: Vec<Vec<i32>> = v.clone().into_iter().filter(|r| !r.contains(&-1)).map(|r| r.into_iter().filter(|c| *c != 1).collect()).collect();
    let non_taut: Vec<Vec<i32>> = non_taut.into_iter().filter(|r| !(1..=(r.iter().map(|c| c.abs()).max().unwrap())).any(|x| r.contains(&x) && r.contains(&-x))).map(|r| {let mut o = Vec::new(); r.into_iter().for_each(|i| if !o.contains(&i) {o.push(i)}); o}).collect();
    println!("Simplified: \n{}", format_table(&non_taut));

    match Certificate::try_from(v) {
        Ok(Certificate::SAT(ans)) => println!("s SATISFIABLE: {:?}", ans),
        Ok(Certificate::UNSAT) => println!("s UNSATISFIABLE"),
        Err(e) => panic!("s UNKNOWN; {}", e),
    }
}