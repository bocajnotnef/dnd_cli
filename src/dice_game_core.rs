use rand::Rng;

pub enum PlayerAction {
    Fold,
    Call,
    Raise(u16)
}

pub trait Player {
    fn get_initial_roll(&self) -> u8;
    fn make_initial_bet(&self) -> u16;
    fn react_to_bet(&self, current_bet: u16) -> PlayerAction;
}

struct RandomPlayer {

}

impl Player for RandomPlayer {
    fn get_initial_roll(&self) -> u8 {
        ${0:todo!()}
    }

    fn make_initial_bet(&self) -> u16 {
        todo!()
    }

    fn react_to_bet(&self, current_bet: u16) -> PlayerAction {
        todo!()
    }
}

pub trait Delegate { // so I'm a swift alum, sue me
    fn get_player_roll(&self) -> u8;
    fn get_player_bet(&self, min: u16) -> u16;

    // TODO: refactor from 'delegate' to 'player'
}
pub struct DiceGame {
// TODO: set betting range
    pub players: u32,
}

impl DiceGame {
    fn get_constrained_player_roll(delegate: &dyn Delegate) -> u8 {
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

    pub fn run(&self, delegate: &dyn Delegate) {

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

        let mut the_pot: u16 = 0;
        let mut max_bet: u16 = 0;

        for to_bet in betting_order.iter() {
            if to_bet.1 {
                // is player
                if max_bet > 0 {
                    // player must "call" at least the max bet

                }
            } else {

            }
        }
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