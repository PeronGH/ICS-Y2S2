mod parent_to_child;
mod simplex;
mod two_way;

fn main() {
    simplex::main().unwrap();
    parent_to_child::main().unwrap();
    two_way::main().unwrap();
}
