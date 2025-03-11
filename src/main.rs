mod bruteforce;
mod satsolver;
mod finitestatemachine;

fn main() {
    // crate::satsolver::filesatsolver::main();
    // crate::satsolver::filesatsolver::general_run_symmetric(5, 1);
    // crate::satsolver::filesatsolver::find_specific(10);
    // crate::satsolver::filesatsolver::main_symmetric();

    for p in 1..100 {
        crate::satsolver::filesatsolver::find_specific(p);
    }
}