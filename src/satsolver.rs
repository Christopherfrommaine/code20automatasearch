use splr::*;

type CNF = Vec<Vec<i32>>;

fn step_to_cnf(nums: Vec<i32>) -> CNF {
    assert!(nums.len() == 6);

    vec![vec![-nums[0], -nums[1], -nums[2], -nums[3], -nums[4], -nums[5]], vec![ -nums[0], -nums[1], -nums[2], -nums[3], nums[4], nums[5]], vec![ -nums[0], -nums[1], -nums[2], nums[3], -nums[4], nums[5]], vec![ -nums[0], -nums[1], -nums[2], nums[3], nums[4], -nums[5]], vec![ -nums[0], -nums[1], nums[2], -nums[3], -nums[4], nums[5]], vec![ -nums[0], -nums[1], nums[2], -nums[3], nums[4], -nums[5]], vec![ -nums[0], -nums[1], nums[2], nums[3], -nums[4], -nums[5]], vec![ -nums[0], nums[1], -nums[2], -nums[3], -nums[4], nums[5]], vec![ -nums[0], nums[1], -nums[2], -nums[3], nums[4], -nums[5]], vec![ -nums[0], nums[1], -nums[2], nums[3], -nums[4], -nums[5]], vec![ -nums[0], nums[1], nums[2], -nums[3], -nums[4], -nums[5]], vec![ -nums[0], nums[1], nums[2], nums[3], nums[4]], vec![ -nums[0], nums[1], nums[2], nums[3], nums[5]], vec![ -nums[0], nums[1], nums[2], nums[4], nums[5]], vec![ -nums[0], nums[1], nums[3], nums[4], nums[5]], vec![ -nums[0], nums[2], nums[3], nums[4], nums[5]], vec![ nums[0], -nums[1], -nums[2], -nums[3], -nums[4], nums[5]], vec![ nums[0], -nums[1], -nums[2], -nums[3], nums[4], -nums[5]], vec![ nums[0], -nums[1], -nums[2], nums[3], -nums[4], -nums[5]], vec![ nums[0], -nums[1], -nums[2], nums[3], nums[4], nums[5]], vec![ nums[0], -nums[1], nums[2], -nums[3], -nums[4], -nums[5]], vec![ nums[0], -nums[1], nums[2], -nums[3], nums[4], nums[5]], vec![ nums[0], -nums[1], nums[2], nums[3], -nums[4], nums[5]], vec![ nums[0], -nums[1], nums[2], nums[3], nums[4], -nums[5]], vec![ nums[0], nums[1], -nums[2], -nums[3], -nums[4], -nums[5]], vec![ nums[0], nums[1], -nums[2], -nums[3], nums[4], nums[5]], vec![ nums[0], nums[1], -nums[2], nums[3], -nums[4], nums[5]], vec![ nums[0], nums[1], -nums[2], nums[3], nums[4], -nums[5]], vec![ nums[0], nums[1], nums[2], -nums[3], -nums[4], nums[5]], vec![ nums[0], nums[1], nums[2], -nums[3], nums[4], -nums[5]], vec![ nums[0], nums[1], nums[2], nums[3], -nums[4], -nums[5]]]
}

fn determine_cnf(width: i32, period: i32) -> CNF {
    let reserved = (period + 1) * (width + 2);


    let mut o = vec![vec![-reserved]];  // 0 is always false

    for row in 1..period {

        let in_range = |i: i32| if i < width * row || i >= width * (row + 1) {reserved} else {i};

        for col in 0..=width {
            let pos = (1 + row) * width + col;
            let mut nums = vec![pos];
            nums.extend(((pos - 2)..=(pos + 2)).map(in_range));

            println!("nums: {nums:?}");

            o.extend_from_slice(&step_to_cnf(nums));
        }
    }
    
    let row = period;
    let in_range = |i: i32| if i < width * row || i >= width * (row + 1) {reserved} else {i};
    for col in 0..=width {
        let pos = width + col;
        let mut nums = vec![pos];
        nums.extend(((pos - 2)..=(pos + 2)).map(in_range));
        println!("nums: {nums:?}");
        o.append(&mut step_to_cnf(nums));
    }

    o.push((width..(width * (period + 1))).collect());

    println!("o: \n{o:?}");

    o
}

fn cnf_example() -> CNF {
    vec![vec![1, 2], vec![-1, 3], vec![1, -3], vec![-1, 2]]
}

pub fn main() {
    let v: CNF = determine_cnf(5, 2);

    match Certificate::try_from(v) {
        Ok(Certificate::SAT(ans)) => println!("s SATISFIABLE: {:?}", ans),
        Ok(Certificate::UNSAT) => println!("s UNSATISFIABLE"),
        Err(e) => panic!("s UNKNOWN; {}", e),
    }
}