mod bruteforce;
mod satsolver;
mod finitestatemachine;

use crate::satsolver::filesatsolver;

fn main() {
    // filesolver::main();
    // filesolver::general_run_symmetric(5, 1);
    // filesolver::find_specific(10);
    // filesolver::main_symmetric();

    // for p in [1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 18, 29] {
    //     filesolver::find_specific(p);
    // }

    // use rayon::prelude::*;
    // [1, 2, 3, 4, 5, 6, 10].into_par_iter().for_each(|p| {
    //     let mut w = 2;
    //     loop {
    //         if filesatsolver::general_run_all(w, p) {break;}
    //         w = (w as f64 * 1.5) as i32;
    //     }
    // });

    let mut args: Vec<i32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();

    if args.len() == 0 {
        args = (1..50).collect();
    }

    args.into_iter().for_each(|p| {
        (0..2 * p).for_each(|diag| {
            filesatsolver::general_run_all(250, p, diag);
        })
    });
}