use chain::Chain;

mod chain;
pub mod helper_functions;

fn main() {
    println!("Hello, world!");

    let mut chain = Chain::create(
        64.0,
        64.0,
        4.0,
        vec![64, 64, 68, 68, 66, 64, 62, 60, 60, 56],
        1000,
        1000,
    );

    println!("Init {chain}");

    chain.travel();
    chain.travel();
    chain.travel();
    chain.travel();
    chain.travel();

    chain.travel();
    chain.travel();
    chain.travel();
    chain.travel();
    chain.travel();

    // println!("Move 1 {chain}");

    chain.travel();

    println!("Moved {chain}");
}
