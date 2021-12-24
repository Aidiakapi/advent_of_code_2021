use crate::prelude::*;
use framework::astar::astar_no_path;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    mem::transmute,
};

day!(23, parse => pt1, pt2);

type Cost = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl From<usize> for Amphipod {
    fn from(n: usize) -> Self {
        debug_assert!(n < 4);
        unsafe { transmute(n as u8) }
    }
}

const COSTS: [Cost; 4] = [1, 10, 100, 1000];

// #############
// #01.2.3.4.56# <-- indices into hallway
// ###0#2#4#6### <-- indices into rooms
//   #1#3#5#7#   ^^^
//   #########
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Positions<const DEPTH: usize>
where
    [(); DEPTH * 4]:,
{
    missing_counts: [u8; 4],
    hallway: [Option<Amphipod>; 7],
    rooms: [Option<Amphipod>; DEPTH * 4],
}

const MOVE_RIGHT_BIT: u8 = 0x80;
const HALLWAY_MOVEMENT: [[u8; 7]; 4] = {
    const B: u8 = MOVE_RIGHT_BIT;
    #[rustfmt::skip] [
        [B|2, B|1,   1,   3,   5,   7,   8],
        [B|4, B|3, B|1,   1,   3,   5,   6],
        [B|6, B|5, B|3, B|1,   1,   3,   4],
        [B|8, B|7, B|5, B|3, B|1,   1,   2],
    ]
};

impl<const DEPTH: usize> Positions<DEPTH>
where
    [(); DEPTH * 4]:,
{
    fn next_positions(&self, positions: &mut Vec<(Positions<DEPTH>, Cost)>) {
        // Drop down from hallway
        for i in 0..7 {
            let amphipod = match self.hallway[i] {
                Some(a) => a,
                None => continue,
            };

            let movement = HALLWAY_MOVEMENT[amphipod as usize][i];

            // Can only drop if there are no items in between current position,
            // and the position 1 to the left or right of the target room.
            let check_range = if movement & MOVE_RIGHT_BIT == MOVE_RIGHT_BIT {
                i + 1..amphipod as usize + 2
            } else {
                amphipod as usize + 2..i
            };
            if self.hallway[check_range].iter().any(|spot| spot.is_some()) {
                continue;
            }

            // Can only drop if target chute only has empty spots (that haven't)
            // been locked in.
            let base = (amphipod as usize) * DEPTH;
            if self.rooms[base..base + self.missing_counts[amphipod as usize] as usize]
                .iter()
                .any(|spot| spot.is_some())
            {
                continue;
            }
            let mut new = *self;
            let missing_count = &mut new.missing_counts[amphipod as usize];
            let base_cost = (movement & !MOVE_RIGHT_BIT) as Cost + *missing_count as Cost;
            let new_y = (*missing_count - 1) as usize;
            *missing_count -= 1;
            new.rooms[base + new_y] = Some(amphipod);
            new.hallway[i] = None;
            positions.push((new, base_cost * COSTS[amphipod as usize]));
        }

        // Raise up to hallway
        for i in 0..4 {
            let base = i * DEPTH;
            let missing_count = self.missing_counts[i];
            let (y, amphipod) = match (0..missing_count)
                .map(|y| y as usize)
                .find_map(|y| self.rooms[base + y].map(|a| (y, a)))
            {
                Some(x) => x,
                None => continue,
            };

            let left_count = self.hallway[0..i + 2]
                .iter()
                .rev()
                .take_while(|x| x.is_none())
                .count();
            let right_count = self.hallway[i + 2..7]
                .iter()
                .take_while(|x| x.is_none())
                .count();
            for j in i + 2 - left_count..i + 2 + right_count {
                let base_cost = (HALLWAY_MOVEMENT[i][j] & !MOVE_RIGHT_BIT) as Cost + y as Cost + 1;
                let mut new = *self;
                new.rooms[base + y] = None;
                new.hallway[j] = Some(amphipod);
                positions.push((new, base_cost * COSTS[amphipod as usize]));
            }
        }
    }

    fn heuristic(&self) -> Cost {
        let mut total_cost = 0;
        for (amphipod, &missing_count) in self.missing_counts.iter().enumerate() {
            for position in 0..missing_count as usize {
                let other_amphipod = match self.rooms[amphipod + position] {
                    Some(a) => a as usize,
                    None => continue,
                };
                // NOTE: We do not count the Y-moves to fall into the other slot, because of a
                // trick used down below, that adds all the correct number of drops for all the
                // missing items.
                let y_cost = position as Cost;
                let x_cost = (amphipod as Cost).abs_diff(other_amphipod as Cost) * 2;
                let cost = (x_cost + y_cost) * COSTS[other_amphipod];
                total_cost += cost;
            }
            // This is the total moves "down" into the rooms, in order to full it up
            total_cost += ((missing_count * (missing_count + 1)) / 2) as Cost * COSTS[amphipod];
        }
        total_cost += self
            .hallway
            .iter()
            .enumerate()
            .filter_map(|(i, a)| a.map(|a| (i, a)))
            .map(|(i, a): (usize, Amphipod)| {
                (HALLWAY_MOVEMENT[a as usize][i] & !MOVE_RIGHT_BIT) as Cost * COSTS[a as usize]
            })
            .sum::<Cost>();
        total_cost
    }

    fn is_solution(&self) -> bool {
        self.missing_counts == [0, 0, 0, 0]
    }

    fn initialize_missing_counts(&mut self) {
        for amphipod in 0..4 {
            let base = amphipod * DEPTH;
            self.missing_counts[amphipod] = DEPTH as u8
                - (0..DEPTH)
                    .rev()
                    .take_while(|&i| self.rooms[base + i] == Some(amphipod.into()))
                    .count() as u8;
        }
    }
}

impl<const DEPTH: usize> Display for Positions<DEPTH>
where
    [(); DEPTH * 4]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn push_value(s: &mut String, value: Option<Amphipod>) {
            s.push(match value {
                None => '.',
                Some(Amphipod::Amber) => 'A',
                Some(Amphipod::Bronze) => 'B',
                Some(Amphipod::Copper) => 'C',
                Some(Amphipod::Desert) => 'D',
            });
        }

        let mut s = String::with_capacity((13 + 1) * (DEPTH + 3));
        s.push_str("#############\n#");
        push_value(&mut s, self.hallway[0]);
        push_value(&mut s, self.hallway[1]);
        s.push('.');
        push_value(&mut s, self.hallway[2]);
        s.push('.');
        push_value(&mut s, self.hallway[3]);
        s.push('.');
        push_value(&mut s, self.hallway[4]);
        s.push('.');
        push_value(&mut s, self.hallway[5]);
        push_value(&mut s, self.hallway[6]);
        s.push_str("#\n");

        for y in 0..DEPTH {
            s.push_str(if y == 0 { "###" } else { "  #" });
            for x in 0..4 {
                push_value(&mut s, self.rooms[x * DEPTH + y]);
                s.push('#');
            }
            if y == 0 {
                s.push_str("##");
            }
            s.push('\n');
        }
        s.push_str("  #########");
        Display::fmt(&s, f)
    }
}

fn run<const DEPTH: usize>(mut input: Positions<DEPTH>) -> Result<Cost>
where
    [(); DEPTH * 4]:,
{
    input.initialize_missing_counts();
    astar_no_path(
        input,
        Positions::next_positions,
        Positions::heuristic,
        Positions::is_solution,
    )
    .ok_or(anyhow!("no solution"))
}

fn pt1(input: &Positions<2>) -> Result<Cost> {
    run(*input)
}

fn pt2(input: &Positions<2>) -> Result<Cost> {
    let positions = Positions::<4> {
        hallway: Default::default(),
        missing_counts: Default::default(),
        rooms: [
            input.rooms[0],
            Some(Amphipod::Desert),
            Some(Amphipod::Desert),
            input.rooms[1],
            input.rooms[2],
            Some(Amphipod::Copper),
            Some(Amphipod::Bronze),
            input.rooms[3],
            input.rooms[4],
            Some(Amphipod::Bronze),
            Some(Amphipod::Amber),
            input.rooms[5],
            input.rooms[6],
            Some(Amphipod::Amber),
            Some(Amphipod::Copper),
            input.rooms[7],
        ],
    };
    run(positions)
}

fn parse(input: &[u8]) -> ParseResult<Positions<2>> {
    use parsers::*;
    let amphi = #[rustfmt::skip]
            token((b'A', Some(Amphipod::Amber)))
        .or(token((b'B', Some(Amphipod::Bronze))))
        .or(token((b'C', Some(Amphipod::Copper))))
        .or(token((b'D', Some(Amphipod::Desert))));
    let row = amphi
        .clone()
        .trailed(token(b'#'))
        .and(amphi.clone())
        .trailed(token(b'#'))
        .and(amphi.clone())
        .trailed(token(b'#'))
        .and(amphi)
        .map(|(((a, b), c), d)| [a, b, c, d]);
    let parser = token(
        b"\
#############
#...........#
###",
    )
    .then(row.clone())
    .trailed(token(
        b"\
###
  #",
    ))
    .and(row)
    .trailed(token(
        b"\
#
  #########",
    ))
    .map(|([a, c, e, g], [b, d, f, h])| Positions {
        missing_counts: Default::default(),
        hallway: Default::default(),
        rooms: [a, b, c, d, e, f, g, h],
    });
    parser.parse(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 12521);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 44169);
}
