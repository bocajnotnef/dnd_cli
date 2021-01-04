use std::io::{Write, stdin, stdout};
use rand::Rng;

struct DiceGame {
    // TODO: set betting range
    players: u32,
}

fn prompt_and_read<T: std::str::FromStr>(prompt: &str) -> T {
    let mut input = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();

    loop {
        stdin()
               .read_line(&mut input)
               .expect("Failed to read line");

        let _: T = match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue,
        };
    }
}

trait DiceGameDelegate { // so I'm a swift alum, sue me
    fn get_player_roll(&self) -> u8;
}

impl DiceGame {
    fn get_constrained_player_roll(delegate: &dyn DiceGameDelegate) -> u8 {
        // get player roll
        let mut the_roll = delegate.get_player_roll();
        loop {
            if (1..7).contains(&the_roll) {
                break;
            }
            the_roll = delegate.get_player_roll();
        }

        the_roll
    }

    pub fn run(&self, delegate: &dyn DiceGameDelegate) {

        // PHASE ONE: Get all the rolls

        let player_roll = DiceGame::get_constrained_player_roll(delegate);
        let mut rng = rand::thread_rng();
        let ai_rolls: Vec<u8> = (1..self.players+1).map(|_| rng.gen_range(1,7)).collect();

        println!("The rolls are:");
        println!("Player: {}", player_roll);
        println!("AIs: {:?}", ai_rolls);

        // PHASE TWO: Place all the bets
        // TODO: for now, full rando this. eventually, make players think

        let mut betting_order: Vec<(u8, bool)> = Vec::new();
        for ai_bet in ai_rolls.iter() {
            betting_order.push((*ai_bet, false));
        }
        betting_order.push((player_roll, true));
        betting_order.sort_by(|a, b| a.0.cmp(&b.0));

        println!("The betting order is: (bet, is_player): {:?}", betting_order);
    }

    fn rules() -> &'static str {
        return "The Rules:
\t* Roll 4d6--keep them hidden!
\t* Choose one of the d6 to reveal to the other players.
\t* Your goal is to have the highest score, using only your remaining 3d6.
\t* Place bets, in order of descending revealed dice. (i.e., highest first)
\t* After all bets are settled (e.g. everyone has called/raised/folded), reveal your dice."
    }
}

struct ConsoleDiceGame {
    game: DiceGame,
}

impl ConsoleDiceGame {
    fn new(num_players: u32) -> ConsoleDiceGame {
        return ConsoleDiceGame{ game: (DiceGame{ players: (num_players) })  }
    }

    fn run(&self) {
        println!("{}", DiceGame::rules());

        self.game.run(self)
    }
}

impl DiceGameDelegate for ConsoleDiceGame {
    fn get_player_roll(&self) -> u8 {
        return prompt_and_read("Player's chosen roll");
    }
}

fn main() {
    println!("Greetings. Shall we play a game of dice?");
    let num_players = prompt_and_read("Number of AI players");

    let game = ConsoleDiceGame::new(num_players);
    game.run();
}
