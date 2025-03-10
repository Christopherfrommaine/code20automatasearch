mod bruteforce;
mod satsolver;
mod finitestatemachine;

fn main() {
    crate::satsolver::filesatsolver::test();
    crate::satsolver::filesatsolver::main();
}