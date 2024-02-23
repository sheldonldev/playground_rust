mod chapter2_common_programming_concepts;
mod chapter1_guessing_game;
mod practice2_convert_temperatures;
mod practice2_generate_fibonacci;
mod practice2_the_12_days_of_christmas;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    chapter: String,
}
fn main() {
    let args: Cli = Cli::parse();
    let chapter: String = args.chapter;

    if chapter == "1" {
        chapter1_guessing_game::guess();
    } else if chapter == "2" {
        chapter2_common_programming_concepts::variables_and_mutability();
        chapter2_common_programming_concepts::scalar_data_types();
        chapter2_common_programming_concepts::functions_and_control_flow();
        practice2_convert_temperatures::test_convert_temperatures(); 
        practice2_generate_fibonacci::test_generate_fibonacci();
        practice2_the_12_days_of_christmas::print_lyrics();
    }
}
