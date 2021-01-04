use crate::console_utilities::prompt_and_read;
use crate::dice_game_core;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct RandomPlayer {
    rand: ThreadRng,
    description: String,
}

impl RandomPlayer {
    const MIN_BET: u16 = 1;
    const MAX_BET: u16 = 3;

    pub fn new() -> RandomPlayer {
        let mut the_rng = rand::thread_rng();

        RandomPlayer {
            description: String::from(format!("A random player ({})", the_rng.gen_range(1, 1000))),
            rand: the_rng,
        }
    }
}

impl dice_game_core::Player for RandomPlayer {
    fn get_initial_roll(&mut self) -> u8 {
        return self.rand.gen_range(1, 7); // d6
    }

    fn make_initial_bet(&mut self) -> u16 {
        return self
            .rand
            .gen_range(RandomPlayer::MIN_BET, RandomPlayer::MAX_BET + 1);
    }

    fn react_to_bet(&mut self, current_bet: u16) -> dice_game_core::PlayerAction {
        match self.rand.gen_range(0, 3) {
            0 => return dice_game_core::PlayerAction::Fold,
            1 => return dice_game_core::PlayerAction::Call,
            2 => return dice_game_core::PlayerAction::Raise(self.rand.gen_range(RandomPlayer::MIN_BET, RandomPlayer::MAX_BET+1)),
            _ => panic!("This shouldn't be possible, but I don't know how to tell the compiler that the rand range is constrained, sooo")
        }
    }

    fn get_meaningful_rolls(&mut self) -> (u8, u8, u8) {
        return (
            // 3d6
            self.rand.gen_range(1, 7),
            self.rand.gen_range(1, 7),
            self.rand.gen_range(1, 7),
        );
    }

    fn inform_of_result(&self, result: dice_game_core::PlayerResult) {
        if let dice_game_core::PlayerResult::Won(amount) = result {
            println!()
        }
    }
}

pub struct ConsoleDiceGame {
  game: dice_game_core::DiceGame,
}

impl <'a>ConsoleDiceGame {
  pub fn new(num_ai_players: u32) -> ConsoleDiceGame {
    let mut players: Vec<Box<dyn dice_game_core::Player>> = Vec::new();

    for _ in 0..num_ai_players {
      players.push(Box::new(RandomPlayer::new()));
    }

    players.push(Box::new(ConsolePlayer{}));

    return ConsoleDiceGame{ game: dice_game_core::DiceGame::new(players)  };
  }

  pub fn run(&mut self) {
    println!("{}", dice_game_core::DiceGame::rules());
    self.game.run();
  }
}

struct ConsolePlayer {}

impl ConsolePlayer {
    // TODO: make generic roll
    // TODO: support multiple rolls for a single 'prompt'
    fn roll_d6(prompt: &String) -> u8 {
        let mut the_roll = prompt_and_read(prompt);
        loop {
            if (1..7).contains(&the_roll) {
            break;
        }
        the_roll = prompt_and_read(prompt);
      }

      the_roll
    }
}

impl dice_game_core::Player for ConsolePlayer {
    fn get_initial_roll(&mut self) -> u8 {
        ConsolePlayer::roll_d6(&String::from("Player's chosen initial (revealed!) roll"))
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

    fn get_meaningful_rolls(&mut self) -> (u8, u8, u8) {
        (
            ConsolePlayer::roll_d6(&String::from("Player's first die")),
            ConsolePlayer::roll_d6(&String::from("Player's second die")),
            ConsolePlayer::roll_d6(&String::from("Player's third die")),
        )
    }

    fn inform_of_result(&self, result: dice_game_core::PlayerResult) {
        match result {
            dice_game_core::PlayerResult::Won(amount) => println!("You won {} $money_units!", amount),
            dice_game_core::PlayerResult::Lost => println!("You lost! Git gud!")
        }
    }
}