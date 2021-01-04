use core::panic;

use rand::Rng;
use rand::prelude::ThreadRng;

pub enum PlayerAction {
    Fold,
    Call,
    Raise(u16)
}

pub trait Player {
    fn get_initial_roll(&mut self) -> u8;
    // TODO: consider units for bets;
    fn make_initial_bet(&mut self) -> u16;
    fn react_to_bet(&mut self, current_bet: u16) -> PlayerAction;
}

pub struct RandomPlayer {
    rand: ThreadRng
}

impl RandomPlayer {
    const MIN_BET: u16 = 1;
    const MAX_BET: u16 = 3;

    pub fn new() -> RandomPlayer {
        RandomPlayer { rand: rand::thread_rng() }
    }
}

impl Player for RandomPlayer {
    fn get_initial_roll(&mut self) -> u8 {
        return self.rand.gen_range(1, 7);
    }

    fn make_initial_bet(&mut self) -> u16 {
        return self.rand.gen_range(RandomPlayer::MIN_BET, RandomPlayer::MAX_BET+1);
    }

    fn react_to_bet(&mut self, current_bet: u16) -> PlayerAction {
        match self.rand.gen_range(0, 3) {
            0 => return PlayerAction::Fold,
            1 => return PlayerAction::Call,
            2 => return PlayerAction::Raise(self.rand.gen_range(RandomPlayer::MIN_BET, RandomPlayer::MAX_BET+1)),
            _ => panic!("This shouldn't be possible, but I don't know how to tell the compiler that the rand range is constrained, sooo")
        }
    }
}

pub struct DiceGame {
    players: Vec<Box<dyn Player>>
}

impl DiceGame {
    pub fn new(players: Vec<Box<dyn Player>>) -> DiceGame {
        DiceGame { players: players }
    }

    pub fn run(&mut self) {
        // PHASE ONE: Get all the rolls

        // get initial rolls, then sort them to get our betting order
        #[derive(Debug)]
        struct PlayerRoll { index: usize, roll: u8}
        let mut initial_rolls: Vec<PlayerRoll> = Vec::new();
        for (position, player_box) in self.players.iter_mut().enumerate() {
            initial_rolls.push(PlayerRoll{index: position, roll: player_box.get_initial_roll()});
        }
        initial_rolls.sort_by(|a, b| a.roll.cmp(&b.roll));

        // PHASE TWO: Place all the bets
        println!("The betting order is: (bet, is_player): {:?}", initial_rolls);

        let mut first_bet = self.players[initial_rolls[0].index].make_initial_bet();
        // TODO: build vec of PlayerActions to iter over; first better would be a 'call', I think
        // TODO: run around until everything is a 'call' or a 'fold'--NOTE: don't allow people to re-raise
        // TODO: might be hard to prevent future raises, but don't worry about that for now
        todo!("impl betting")
    }

    pub fn rules() -> &'static str {
        return "The Rules:
\t* Roll 4d6--keep them hidden!
\t* Choose one of the d6 to reveal to the other players.
\t* Your goal is to have the highest score, using only your remaining 3d6.
\t* Place bets, in order of descending revealed dice. (i.e., highest first)
\t* After all bets are settled (e.g. everyone has called/raised/folded), reveal your dice."
    }
}