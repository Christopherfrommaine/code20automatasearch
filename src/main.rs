mod bruteforce;
mod satsolver;
mod finitestatemachine;

fn main() {
    crate::satsolver::filesatsolver::test();
    crate::satsolver::filesatsolver::find_specific(7);
    crate::satsolver::filesatsolver::main();
}