mod bruteforce;
mod satsolver;
mod finitestatemachine;

fn main() {
    crate::satsolver::filesatsolver::test();
    // crate::satsolver::filesatsolver::general_run_symmetric(5, 1);
    // crate::satsolver::filesatsolver::find_specific(7);
    // crate::satsolver::filesatsolver::main_symmetric();
}