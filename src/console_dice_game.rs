use crate::console_utilities::prompt_and_read;
use crate::dice_game_core;

pub struct ConsoleDiceGame {
  game: dice_game_core::DiceGame,
}

impl ConsoleDiceGame {
  pub fn new(num_players: u32) -> ConsoleDiceGame {
      return ConsoleDiceGame{ game: (dice_game_core::DiceGame{ players: (num_players) })  }
  }

  pub fn run(&self) {
      println!("{}", dice_game_core::DiceGame::rules());

      // TODO: instead of delegate, replace with player pattern
      self.game.run(self)
  }
}

struct ConsolePlayer {

}

impl dice_game_core::Player for ConsolePlayer {
    fn get_initial_roll(&self) -> u8 {

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

    fn make_initial_bet(&self) -> u16 {
      prompt_and_read(&String::from("Player's initial bet"))
    }

    fn react_to_bet(&self, current_bet: u16) -> dice_game_core::PlayerAction {
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