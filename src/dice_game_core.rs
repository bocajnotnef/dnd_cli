
use std::io::{stdin, stdout, Write};

use mockall::{automock, mock, predicate::*};

#[derive(PartialEq, Copy, Clone)]
pub enum PlayerAction {
    Fold,
    Call,
    Raise(u16),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerResult {
    Lost,
    Won(u32), // TODO: refactor w/ currency units
}

// TODO: put the random generator in like, 'GlobalGame' or 'MetaUniverse' or something
// currently VERY STUPID that rolling a dice modifies the playerstate
#[cfg_attr(test, automock)]
pub trait Player {
    fn get_initial_roll(&self) -> u8;
    // TODO: consider units for bets;
    fn make_initial_bet(&self) -> u16;
    fn react_to_bet(&self, current_bet: u16) -> PlayerAction;
    fn get_hands(&self) -> (u8, u8, u8); // TODO: better name
    fn inform_of_result(&self, result: PlayerResult);
}

pub struct DiceGame {
    players: Vec<Box<dyn Player>>,
}

#[derive(Debug)]
struct PlayerRoll {
    index: usize,
    roll: u8,
}

struct RiverState {
    pot: u32,
    indicies_in_play: Vec::<usize>
}

impl DiceGame {
    pub fn rules() -> &'static str {
        return "The Rules:
\t* Roll 4d6--keep them hidden!
\t* Choose one of the d6 to reveal to the other players.
\t* Your goal is to have the highest score, using only your remaining 3d6.
\t* Place bets, in order of descending revealed dice. (i.e., highest first)
\t* After all bets are settled (e.g. everyone has called/raised/folded), reveal your dice.";
    }

    pub fn new(players: Vec<Box<dyn Player>>) -> DiceGame {
        DiceGame { players: players }
    }

    // TODO: once factor out randomness, make static
    fn get_and_sort_initial_rolls(&mut self) -> Vec::<PlayerRoll> {
        let mut initial_rolls: Vec<PlayerRoll> = Vec::new();
        for (position, player_box) in self.players.iter_mut().enumerate() {
            initial_rolls.push(PlayerRoll {
                index: position,
                roll: player_box.get_initial_roll(),
            });
        }
        initial_rolls.sort_by(|a, b| b.roll.cmp(&a.roll));
        return initial_rolls
    }

    // TODO: once factor out randomness, make static
    fn process_bets(&mut self, initial_rolls: &Vec<PlayerRoll>) -> RiverState {
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
            for (index, action) in bets.iter_mut().enumerate() {
                match action {
                    PlayerAction::Raise(old_bet) => {
                        if *old_bet < the_bet {
                            *action = self.players[index].react_to_bet(the_bet);
                        } else {
                            *action = PlayerAction::Call;
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

        let the_pot = bets.iter().fold(0, |acc, bet| if let PlayerAction::Call = bet {acc + the_bet} else {acc});
        let mut player_indicies_remaining: Vec<usize> = Vec::new();

        for (player_index, action) in bets.iter().enumerate() {
            if let PlayerAction::Fold = action {
                continue;
            }

            player_indicies_remaining.push(player_index);
        }

        RiverState { pot: the_pot as u32, indicies_in_play: player_indicies_remaining}
    }

    pub fn run(&mut self) {
        // Phase 1: get initial rolls
        let initial_rolls = self.get_and_sort_initial_rolls();

        // Phase 2: get players' hands
        let mut hands: Vec<u8> = Vec::new();
        for player in self.players.iter_mut() {
            let rolls = player.get_hands();
            hands.push(rolls.0 + rolls.1 + rolls.2);
        }
        let RiverState{pot: the_pot, indicies_in_play: playing_indices} = self.process_bets(&initial_rolls);

        // PHASE 3: figure out who won
        let mut remaining_hands: Vec<(usize, u8)> = Vec::new();
        // TODO: this could probably be a map
        for index in playing_indices {
            remaining_hands.push((index, hands[index]));
        }

        // sort descending
        remaining_hands.sort_by(|(_, a), (_, b)| b.cmp(a));
        let max_hand = remaining_hands[0].1;
        let winning_indicies: Vec<usize> = remaining_hands.iter().filter_map(|(index, hand)| if *hand == max_hand { Some(*index) } else {None} ).collect();


        for (index, player) in self.players.iter().enumerate() {
            if winning_indicies.contains(&index) {
                player.inform_of_result(PlayerResult::Won(the_pot/(winning_indicies.len() as u32)));
            } else {
                player.inform_of_result(PlayerResult::Lost);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

    #[test]
    fn test_when_all_AI_fold_expect_the_player_will_win() {
        let mut ai_1 = Box::new(MockPlayer::new());
        ai_1.expect_get_initial_roll().times(1).return_const(1);
        ai_1.expect_make_initial_bet().times(0);
        ai_1.expect_react_to_bet()
            .with(mockall::predicate::eq(3)).times(1).return_const(PlayerAction::Fold);
        ai_1.expect_inform_of_result().with(mockall::predicate::eq(PlayerResult::Lost));

        let mut ai_2 = Box::new(MockPlayer::new());
        ai_2.expect_get_initial_roll().times(1).return_const(1);
        ai_2.expect_make_initial_bet().times(0);
        ai_2.expect_react_to_bet()
            .with(mockall::predicate::eq(3)).times(1).return_const(PlayerAction::Fold);
        ai_2.expect_inform_of_result().with(mockall::predicate::eq(PlayerResult::Lost));

        let mut player = Box::new(MockPlayer::new());
        player.expect_get_initial_roll().times(1).return_const(6);
        player.expect_make_initial_bet().times(1).return_const(1u16);
        player.expect_react_to_bet().times(0);
        player.expect_inform_of_result().with(mockall::predicate::eq(PlayerResult::Won(1)));

        let mut the_game = DiceGame::new(vec!(ai_1, ai_2, player));
        the_game.run();
    }
}