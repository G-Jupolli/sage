use chain::Chain;

mod chain;
pub mod helper_functions;

fn main() {
    println!("Hello, world!");

    let mut chain = Chain::create(5.0, 5.0);

    println!("Init {chain}");

    chain.head.move_chain();
    chain.head.move_chain();
    chain.head.move_chain();
    chain.head.move_chain();
    chain.head.move_chain();

    // println!("Move 1 {chain}");

    chain.head.move_chain();

    println!("Moved {chain}");
}
