use crate::satsolver::satcreator::create_cnf;
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
    use std::os::unix::process::CommandExt; // For `before_exec`
    use nix::sys::prctl;

    let mut r = Command::new("sh")
        .arg("-c")
        .arg(format!("./cryptominisat5 {filename}.cnf > {filename}_output.txt"))
        .before_exec(|| {
            prctl::set_pdeathsig(nix::sys::signal::Signal::SIGSTOP).expect("Failed to set parent death signal");
            Ok(())
        })
        .spawn()
        .expect("Failed to create process");

    let r2 = r.wait();

    println!("Ran ({w}, {p}) with result: ({r:?}, {r2:?})");

    println!("parsing ({w}, {p})...");

    parse_file_output(&(filename + "_output.txt"), w, p);

}

fn parse_file_output(filename: &str, w: i32, p: i32) {

    let string: String = std::fs::read_to_string(filename).expect(&format!("Could not read file: {}.", filename));

    let mut o: Vec<String> = Vec::new();

    string.split('\n').for_each(|s|
        if s.len() >= 3 && s.chars().nth(0) == Some('v') {
            o.push((&s[2..]).to_string())
        });
    
    handle_result(o.iter().flat_map(|s| s.chars()).collect::<String>(), w, p);
}

fn handle_result(res: String, w: i32, p: i32) {
    let o: Vec<i32> = res.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    println!("o ({w}, {p}): {o:?}");

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("total-file-output.txt")
        .expect("Couldn't create output file");

    writeln!(file, "({w}, {p}): {o:?}").expect("Couldn't write to output file");

    if o.len() >= 5 {
        crate::satsolver::splrsatsolver::handle_sol(o, w, p);
    } else {
        println!("Couldn't handle due to small solution")
    }
    
}

pub fn general_run(width: i32, period: i32) {
    println!("creating ({width}, {period})...");
    let cnf = create_cnf(width, period);
    let filename = format!("filesolver/cnf_for_w{width}_p{period}");

    println!("exporting ({width}, {period})...");
    export_cnf(&cnf, &(filename.clone() + ".cnf"));

    println!("running ({width}, {period})...");
    run_cnf_command(filename.clone(), width, period);
}

pub fn main() {
    use rayon::prelude::*;

    vec![
        (010, 02),
        (100, 10),
        (150, 12),
        (200, 07),
        (200, 11),
        (200, 13),
        (200, 17),
    ].into_par_iter().for_each(|(w, p)| {general_run(w, p);});

    (1..100).into_par_iter().for_each(|p| {general_run(15 * p, p);})
}

pub fn test() {
    vec![
        (010, 01),
        (010, 02),
        (020, 04),
        (050, 03),
        (100, 06),
    ].into_iter().for_each(|(w, p)| {general_run(w, p);});
}
