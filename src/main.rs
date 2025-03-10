mod bruteforce;
mod customuint;
mod splrsatsolver;
mod satcreator;
mod filesatsolver;

fn main() {
    crate::filesatsolver::test();
    crate::filesatsolver::main();
}