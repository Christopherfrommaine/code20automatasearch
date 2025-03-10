use crate::satcreator::create_cnf;
use std::fs::File;
use std::io::Write;

fn export_cnf(clauses: &Vec<Vec<i32>>, filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file.");

    let num_clauses = clauses.len();
    let num_vars = clauses.iter()
        .flat_map(|clause| clause.iter().map(|&lit| lit.abs()))
        .max()
        .unwrap_or(0);

    // DIMACS header
    let mut out = vec![format!("p cnf {} {}", num_vars, num_clauses)];

    // Write each clause: literals separated by spaces and ending with a 0.
    for clause in clauses {
        for &literal in clause {
            out.push(literal.to_string() + " ")
        }
        out.push("0\n".to_string());
    }

    write!(file, "{}", out.concat()).expect("Failed to write to file.");
}

fn run_cnf_command(filename: String, w: i32, p: i32) {
    use std::process::Command;

    let r = Command::new("sh")
        .arg("-c")
        .arg(format!("./cryptominisat5 {filename}.cnf > {filename}_output.txt"))
        .output();

    println!("Ran with result: {r:?}");

    // if let Ok(res) = r {
    //     handle_result(String::from_utf8_lossy(&res.stdout).to_string(), w, p);
    // }
}

fn handle_result(res: String, w: i32, p: i32) {
    let o: Vec<i32> = res.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    println!("o: {o:?}");

    crate::splrsatsolver::handle_sol(o, w, p);
}

pub fn general_run(width: i32, period: i32) {
    println!("creating...");
    let cnf = create_cnf(width, period);
    let filename = format!("filesolver/cnf_for_w{width}_p{period}");

    println!("exporting...");
    export_cnf(&cnf, &(filename.clone() + ".cnf"));

    println!("running...");
    run_cnf_command(filename, width, period);
}

pub fn main() {
    general_run(10, 2);

    use rayon::prelude::*;
    (1..100).into_par_iter().for_each(
        |p| {general_run(100, p);}
    )
}