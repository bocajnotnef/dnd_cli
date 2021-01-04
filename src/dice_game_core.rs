use core::panic;
use std::io::{stdin, stdout, Write};

#[derive(PartialEq, Copy, Clone)]
pub enum PlayerAction {
    Fold,
    Call,
    Raise(u16),
}

pub enum PlayerResult {
    Lost,
    Won(u16), // TODO: refactor w/ currency units
}

// TODO: put the random generator in like, 'GlobalGame' or 'MetaUniverse' or something
// currently VERY STUPID that rolling a dice modifies the playerstate
pub trait Player {
    fn get_initial_roll(&mut self) -> u8;
    // TODO: consider units for bets;
    fn make_initial_bet(&mut self) -> u16;
    fn react_to_bet(&mut self, current_bet: u16) -> PlayerAction;
    fn get_meaningful_rolls(&mut self) -> (u8, u8, u8); // TODO: better name
    fn inform_of_result(&self, result: PlayerResult);
}

pub struct DiceGame {
    players: Vec<Box<dyn Player>>,
}

impl DiceGame {
    pub fn new(players: Vec<Box<dyn Player>>) -> DiceGame {
        DiceGame { players: players }
    }

    pub fn run(&mut self) {
        // PHASE ONE: Get all the rolls

        // get initial rolls, then sort them to get our betting order
        #[derive(Debug)]
        struct PlayerRoll {
            index: usize,
            roll: u8,
        }
        let mut initial_rolls: Vec<PlayerRoll> = Vec::new();
        for (position, player_box) in self.players.iter_mut().enumerate() {
            initial_rolls.push(PlayerRoll {
                index: position,
                roll: player_box.get_initial_roll(),
            });
        }
        initial_rolls.sort_by(|a, b| a.roll.cmp(&b.roll));

        // PHASE TWO: Place all the bets
        println!(
            "The betting order is: (bet, is_player): {:?}",
            initial_rolls
        );

        let mut the_bet = self.players[initial_rolls[0].index].make_initial_bet();
        // NOTE: bet order/indexing should match initial_rolls ordering, but this is implicit
        let mut bets: Vec<PlayerAction> = Vec::new();
        bets.push(PlayerAction::Raise(the_bet));

        for _ in 1..initial_rolls.len() {
            bets.push(PlayerAction::Call);
        }

        // this isn't as readable as I want it to be
        // TLDR run this loop as long as there's a "raise" present
        while bets.iter().any(|x| {
            if let PlayerAction::Raise(_) = x {
                // TODO: handle what happens if everyone besides one guy folds
                true
            } else {
                false
            }
        }) {
            // let mut new_bets: Vec<PlayerAction> = Vec::new();

            for (index, action) in bets.iter_mut().enumerate() {
                match action {
                    PlayerAction::Raise(old_bet) => {
                        if *old_bet < the_bet {
                            *action = self.players[index].react_to_bet(the_bet);
                            // new_bets.push(self.players[index].react_to_bet(the_bet));
                        } else {
                            *action = PlayerAction::Call;
                            // new_bets.push(PlayerAction::Call); // this was a raise from last 'round'--resolve it
                            continue;
                        }
                    }
                    PlayerAction::Fold => (),// do nothing,
                    PlayerAction::Call => {
                        let reaction = self.players[index].react_to_bet(the_bet);
                        if let PlayerAction::Raise(new_bet) = reaction {
                            the_bet = new_bet;
                        }
                        *action = reaction;
                    }
                }
            }
        }

        // at this point everyone should be a 'call' or 'fold'
        assert!(
            !bets
                .iter()
                .any(|action| if let PlayerAction::Raise(_) = action {
                    true
                } else {
                    false
                }),
            "Raise where there shouldn't be one"
        );

        // PHASE 3: figure out who won
        // TODO: should get the player's other dice rolls @ startup, not here

        let mut final_rolls: Vec<u16> = Vec::new();

        for player in self.players.iter_mut() {
            let rolls = player.get_meaningful_rolls();
            final_rolls.push((rolls.0 + rolls.1 + rolls.2).into());
        }

        let the_pot = bets.iter().fold(0, |acc, bet| if let PlayerAction::Call = bet {acc + the_bet} else {acc});

        // https://stackoverflow.com/a/57815298
        let winning_player_idx = final_rolls.iter().enumerate().max_by(|(_, value0), (_, value1)| value0.cmp(value1)).map(|(idx, _)| idx).unwrap();
        for (index, player) in self.players.iter().enumerate() {
            if index == winning_player_idx {
                player.inform_of_result(PlayerResult::Won(the_pot));
            } else {
                player.inform_of_result(PlayerResult::Lost);
            }
        }
    }

    pub fn rules() -> &'static str {
        return "The Rules:
\t* Roll 4d6--keep them hidden!
\t* Choose one of the d6 to reveal to the other players.
\t* Your goal is to have the highest score, using only your remaining 3d6.
\t* Place bets, in order of descending revealed dice. (i.e., highest first)
\t* After all bets are settled (e.g. everyone has called/raised/folded), reveal your dice.";
    }
}
