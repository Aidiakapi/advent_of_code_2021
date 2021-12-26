// 4x6 for each character
pub const WIDTH: usize = 4;
pub const HEIGHT: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Char(u32);

impl Char {
    pub fn from_contains<F: FnMut(usize, usize) -> bool>(mut f: F) -> Char {
        let mut char = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                char = char << 1 | if f(x, y) { 1 } else { 0 }
            }
        }
        Char(char)
    }

    pub fn recognize(self) -> Option<char> {
        for &(n, c) in &ALPHABET {
            if self.0 == n {
                return Some(c);
            }
        }
        None
    }
}

pub fn recognize_from_contains<F: FnMut(usize, usize) -> bool>(mut f: F) -> Option<String> {
    let mut s = String::new();
    for ox in (0..).step_by(5) {
        let c = Char::from_contains(|x, y| f(ox + x, y));
        if c.0 == 0 {
            break;
        }
        let c = c.recognize()?;
        s.push(c);
    }
    Some(s)
}

const ALPHABET: [(u32, char); 14] = #[rustfmt::skip] [
    (
        0b_0110 << 20 |
        0b_1001 << 16 |
        0b_1001 << 12 |
        0b_1111 <<  8 |
        0b_1001 <<  4 |
        0b_1001 <<  0,
        'A'
    ),
    (
        0b_1110 << 20 |
        0b_1001 << 16 |
        0b_1110 << 12 |
        0b_1001 <<  8 |
        0b_1001 <<  4 |
        0b_1110 <<  0,
        'B'
    ),
    (
        0b_0110 << 20 |
        0b_1001 << 16 |
        0b_1000 << 12 |
        0b_1000 <<  8 |
        0b_1001 <<  4 |
        0b_0110 <<  0,
        'C'
    ),
    (
        0b_1111 << 20 |
        0b_1000 << 16 |
        0b_1110 << 12 |
        0b_1000 <<  8 |
        0b_1000 <<  4 |
        0b_1111 <<  0,
        'E'
    ),
    (
        0b_1111 << 20 |
        0b_1000 << 16 |
        0b_1110 << 12 |
        0b_1000 <<  8 |
        0b_1000 <<  4 |
        0b_1000 <<  0,
        'F'
    ),
    (
        0b_0110 << 20 |
        0b_1001 << 16 |
        0b_1000 << 12 |
        0b_1011 <<  8 |
        0b_1001 <<  4 |
        0b_0111 <<  0,
        'G'
    ),
    (
        0b_1001 << 20 |
        0b_1001 << 16 |
        0b_1111 << 12 |
        0b_1001 <<  8 |
        0b_1001 <<  4 |
        0b_1001 <<  0,
        'H'
    ),
    (
        0b_0011 << 20 |
        0b_0001 << 16 |
        0b_0001 << 12 |
        0b_0001 <<  8 |
        0b_1001 <<  4 |
        0b_0110 <<  0,
        'J'
    ),
    (
        0b_1001 << 20 |
        0b_1010 << 16 |
        0b_1100 << 12 |
        0b_1010 <<  8 |
        0b_1010 <<  4 |
        0b_1001 <<  0,
        'K'
    ),
    (
        0b_1000 << 20 |
        0b_1000 << 16 |
        0b_1000 << 12 |
        0b_1000 <<  8 |
        0b_1000 <<  4 |
        0b_1111 <<  0,
        'L'
    ),
    (
        0b_1110 << 20 |
        0b_1001 << 16 |
        0b_1001 << 12 |
        0b_1110 <<  8 |
        0b_1000 <<  4 |
        0b_1000 <<  0,
        'P'
    ),
    (
        0b_1110 << 20 |
        0b_1001 << 16 |
        0b_1001 << 12 |
        0b_1110 <<  8 |
        0b_1010 <<  4 |
        0b_1001 <<  0,
        'R'
    ),
    (
        0b_1001 << 20 |
        0b_1001 << 16 |
        0b_1001 << 12 |
        0b_1001 <<  8 |
        0b_1001 <<  4 |
        0b_0110 <<  0,
        'U'
    ),
    (
        0b_1111 << 20 |
        0b_0001 << 16 |
        0b_0010 << 12 |
        0b_0100 <<  8 |
        0b_1000 <<  4 |
        0b_1111 <<  0,
        'Z'
    ),
];
