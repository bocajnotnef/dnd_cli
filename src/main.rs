use std::io::{Write, stdin, stdout};

mod dice_game_core;
mod console_dice_game;
mod console_utilities;

fn main() {
    println!("Greetings. Shall we play a game of dice?");
    let num_players = console_utilities::prompt_and_read(&String::from("Number of AI players"));

    let mut game = console_dice_game::ConsoleDiceGame::new(num_players);
    game.run();
}
