use crate::prelude::*;
use std::{iter::Cycle, ops::Range};

day!(21, parse => pt1, pt2);

struct Game {
    dice: Cycle<Range<u32>>,
    rolls: u32,
    p1: Player,
    p2: Player,
}

struct Player {
    pawn_position_sub_1: u32,
    score: u32,
}

impl Game {
    fn new(input: &(u8, u8)) -> Game {
        Game {
            dice: (1..101).cycle(),
            rolls: 0,
            p1: Player::new(input.0),
            p2: Player::new(input.1),
        }
    }

    fn roll(&mut self) -> u32 {
        self.rolls += 1;
        self.dice.next().unwrap()
    }

    fn roll_3_sum(&mut self) -> u32 {
        self.roll() + self.roll() + self.roll()
    }

    fn play(&mut self) {
        loop {
            let roll = self.roll_3_sum();
            self.p1.advance_by(roll);
            if self.p1.score >= 1000 {
                break;
            }
            let roll = self.roll_3_sum();
            self.p2.advance_by(roll);
            if self.p2.score >= 1000 {
                break;
            }
        }
    }
}

impl Player {
    fn new(position: u8) -> Player {
        Player {
            pawn_position_sub_1: position as u32 - 1,
            score: 0,
        }
    }

    fn advance_by(&mut self, steps: u32) {
        self.pawn_position_sub_1 += steps;
        self.pawn_position_sub_1 %= 10;
        self.score += self.pawn_position_sub_1 + 1;
    }
}

fn pt1(input: &(u8, u8)) -> MulSubmission<u32> {
    let mut game = Game::new(input);
    game.play();
    MulSubmission(game.p1.score.min(game.p2.score), game.rolls)
}

const ROLL_SUM_AND_MULTIPLIER: [(u8, u8); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn pt2(input: &(u8, u8)) -> u64 {
    // Positions are 1 less than the "rules" say
    fn play_and_count_victories(
        active_pos: u8,
        passive_pos: u8,
        active_score: u8,
        passive_score: u8,
    ) -> (u64, u64) {
        let mut active_wins = 0;
        let mut passive_wins = 0;
        for &(roll_sum, multiplier) in &ROLL_SUM_AND_MULTIPLIER {
            let active_pos = (active_pos + roll_sum) % 10;
            let active_score = active_score + active_pos + 1;
            if active_score >= 21 {
                active_wins += multiplier as u64;
                continue;
            }

            let (pw, aw) =
                play_and_count_victories(passive_pos, active_pos, passive_score, active_score);
            active_wins += aw * multiplier as u64;
            passive_wins += pw * multiplier as u64;
        }
        (active_wins, passive_wins)
    }
    let (p1_wins, p2_wins) = play_and_count_victories(input.0 - 1, input.1 - 1, 0, 0);
    p1_wins.max(p2_wins)
}

fn parse(input: &str) -> ParseResult<(u8, u8)> {
    use parsers::*;
    token("Player 1 starting position: ")
        .then(number_u8)
        .trailed(token("\nPlayer 2 starting position: "))
        .and(number_u8)
        .parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => MulSubmission(745, 993));
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 444356092776315);
}