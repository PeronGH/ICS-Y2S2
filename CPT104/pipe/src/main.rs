mod parent_to_child;
mod simplex;

fn main() {
    simplex::main().unwrap();
    parent_to_child::main().unwrap();
}
