use crate::console_utilities::prompt_and_read;
use crate::dice_game_core;

pub struct ConsoleDiceGame {
  game: dice_game_core::DiceGame,
}

impl <'a>ConsoleDiceGame {
  pub fn new(num_ai_players: u32) -> ConsoleDiceGame {
    let mut players: Vec<Box<dyn dice_game_core::Player>> = Vec::new();

    for _ in 0..num_ai_players {
      players.push(Box::new(dice_game_core::RandomPlayer::new()));
    }

    players.push(Box::new(ConsolePlayer{}));

    return ConsoleDiceGame{ game: dice_game_core::DiceGame::new(players)  };
  }

  pub fn run(&self) {
      println!("{}", dice_game_core::DiceGame::rules());

  }
}

struct ConsolePlayer {}

impl dice_game_core::Player for ConsolePlayer {
    fn get_initial_roll(&mut self) -> u8 {

      // get player roll
      let mut the_roll = prompt_and_read(&String::from("Player's chosen roll"));
      loop {
          if (1..7).contains(&the_roll) {
              break;
          }
          the_roll = prompt_and_read(&String::from("Player's chosen roll"));
      }

      the_roll
    }

    fn make_initial_bet(&mut self) -> u16 {
      prompt_and_read(&String::from("Player's initial bet"))
    }

    fn react_to_bet(&mut self, current_bet: u16) -> dice_game_core::PlayerAction {
      let player_reaction = prompt_and_read(&String::from("Bet to player; 0 to fold"));

      if player_reaction == 0 {
        return dice_game_core::PlayerAction::Fold;
      } else {
        if player_reaction == current_bet {
          return dice_game_core::PlayerAction::Call;
        } else {
          return dice_game_core::PlayerAction::Raise(player_reaction);
        }
      }
    }
}