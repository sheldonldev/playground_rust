mod common_programming_concepts;
mod guessing_game;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    chapter: String,
}
fn main() {
    let args: Cli = Cli::parse();
    let chapter: String = args.chapter;

    if chapter == "1" {
        guessing_game::guess();
    } else if chapter == "2" {
        common_programming_concepts::variables_and_mutability();
    }
}
