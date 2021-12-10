use crate::prelude::*;

day!(10, parse => pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delimiter {
    /* ( */ ParensOpen,
    /* ) */ ParensClose,
    /* [ */ BracketsOpen,
    /* ] */ BracketsClose,
    /* { */ BracesOpen,
    /* } */ BracesClose,
    /* < */ AngleOpen,
    /* > */ AngleClose,
}

impl Delimiter {
    fn is_open(self) -> bool {
        use Delimiter::*;
        matches!(self, ParensOpen | BracketsOpen | BracesOpen | AngleOpen)
    }

    fn get_opposite(self) -> Self {
        use Delimiter::*;
        match self {
            ParensOpen => ParensClose,
            ParensClose => ParensOpen,
            BracketsOpen => BracketsClose,
            BracketsClose => BracketsOpen,
            BracesOpen => BracesClose,
            BracesClose => BracesOpen,
            AngleOpen => AngleClose,
            AngleClose => AngleOpen,
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    delimiters: Vec<Delimiter>,
}

fn pt1(input: &[Line]) -> u32 {
    let mut stack = Vec::new();
    input
        .iter()
        .filter_map(|line| {
            stack.clear();
            get_corrupted_and_remaining(line, &mut stack)
        })
        .map(|delim| match delim {
            Delimiter::ParensClose => 3,
            Delimiter::BracketsClose => 57,
            Delimiter::BracesClose => 1197,
            Delimiter::AngleClose => 25137,
            _ => unreachable!(),
        })
        .sum()
}

fn get_corrupted_and_remaining(line: &Line, stack: &mut Vec<Delimiter>) -> Option<Delimiter> {
    debug_assert!(stack.is_empty());
    for &delim in &line.delimiters {
        if delim.is_open() {
            stack.push(delim.get_opposite());
            continue;
        }
        if Some(delim) != stack.pop() {
            return Some(delim);
        }
    }

    None
}

fn pt2(input: &[Line]) -> u64 {
    let mut stack = Vec::new();
    let mut scores = input
        .iter()
        .filter_map(|line| {
            stack.clear();
            if let Some(_) = get_corrupted_and_remaining(line, &mut stack) {
                return None;
            }
            if stack.is_empty() {
                return None;
            }
            let mut score = 0;
            for &delim in stack.iter().rev() {
                score = score * 5
                    + match delim {
                        Delimiter::ParensClose => 1,
                        Delimiter::BracketsClose => 2,
                        Delimiter::BracesClose => 3,
                        Delimiter::AngleClose => 4,
                        _ => unreachable!(),
                    };
            }
            Some(score)
        })
        .collect::<Vec<u64>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse(input: &str) -> ParseResult<Vec<Line>> {
    use parsers::*;
    let delimiter = #[rustfmt::skip] {
            token(('(', Delimiter::ParensOpen))
        .or(token((')', Delimiter::ParensClose)))
        .or(token(('[', Delimiter::BracketsOpen)))
        .or(token((']', Delimiter::BracketsClose)))
        .or(token(('{', Delimiter::BracesOpen)))
        .or(token(('}', Delimiter::BracesClose)))
        .or(token(('<', Delimiter::AngleOpen)))
        .or(token(('>', Delimiter::AngleClose)))
    };
    let line = delimiter.fold_mut(Vec::new(), |line, delim| line.push(delim));
    let line = line.map(|delimiters| Line { delimiters });
    line.sep_by(token('\n')).parse(input)
}

tests! {
    const EXAMPLE: &'static str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    simple_tests!(parse, pt1, pt1_tests, EXAMPLE => 26397);
    simple_tests!(parse, pt2, pt2_tests, EXAMPLE => 288957);
}
