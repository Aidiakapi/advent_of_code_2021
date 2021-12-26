use crate::prelude::*;
use std::fmt::Display;

day!(24, parse => pt1, pt2);

type Int = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum OpCode {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arg {
    Register(Register),
    Constant(Int),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    opcode: OpCode,
    a: Register,
    b: Arg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State([Int; 4]);
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[w = {}, x = {}, y = {}, z = {}]",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

#[cfg(test)]
impl Arg {
    fn eval(&self, state: &[Int; 4]) -> Int {
        match self {
            Arg::Register(r) => state[*r as usize],
            Arg::Constant(c) => *c,
            Arg::None => panic!("cannot evalute a missing argument"),
        }
    }
}

#[cfg(test)]
fn exec<F>(program: &[Instruction], mut f: F) -> State
where
    F: FnMut() -> Int,
{
    let mut state = [0; 4];
    for instruction in program.iter() {
        let value = match instruction.opcode {
            OpCode::Inp => f(),
            opcode => {
                let a = state[instruction.a as usize];
                let b = instruction.b.eval(&state);
                match opcode {
                    OpCode::Inp => unreachable!(),
                    OpCode::Add => a + b,
                    OpCode::Mul => a * b,
                    OpCode::Div => a / b,
                    OpCode::Mod => a % b,
                    OpCode::Eql => {
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        };
        state[instruction.a as usize] = value;
    }
    State(state)
}

fn advance_z_once(&c: &(bool, Int, Int), digit: Int, mut z: Int) -> Int {
    let x = z % 26 + c.1 != digit;
    if c.0 {
        z /= 26;
    }
    if x {
        z = z * 26 + digit + c.2;
    }
    z
}

type Constants = [(bool, Int, Int); 14];
fn extract_constants(program: &[Instruction]) -> Constants {
    let mut cs = Constants::default();
    for i in 0..14 {
        let c = (
            match program[i * 18 + 4].b {
                Arg::Constant(1) => false,
                Arg::Constant(26) => true,
                _ => unreachable!(),
            },
            match program[i * 18 + 5].b {
                Arg::Constant(c) => c,
                _ => unreachable!(),
            },
            match program[i * 18 + 15].b {
                Arg::Constant(c) => c,
                _ => unreachable!(),
            },
        );
        assert!(c.0 || c.1 >= 10);
        cs[i] = c;
    }
    cs
}

// Const generics at this point are not good enough yet to just do this with a
// number. So instead, use this very primitive "type num", one for each of the
// digits up till 14.
trait TypeNum {
    const VALUE: usize;
    type Next: TypeNum;
}

macro_rules! impl_typenum {
    ($(($v:literal, $n:ident, $m:ident),)+) => {
        $(
            struct $n;
            impl TypeNum for $n {
                const VALUE: usize = $v;
                type Next = $m;
            }
        )+
    };
}

impl_typenum!(
    (0, TypeNum0, TypeNum1),
    (1, TypeNum1, TypeNum2),
    (2, TypeNum2, TypeNum3),
    (3, TypeNum3, TypeNum4),
    (4, TypeNum4, TypeNum5),
    (5, TypeNum5, TypeNum6),
    (6, TypeNum6, TypeNum7),
    (7, TypeNum7, TypeNum8),
    (8, TypeNum8, TypeNum9),
    (9, TypeNum9, TypeNum10),
    (10, TypeNum10, TypeNum11),
    (11, TypeNum11, TypeNum12),
    (12, TypeNum12, TypeNum13),
    (13, TypeNum13, TypeNum14),
    (14, TypeNum14, TypeNum0),
);

fn find_first<T: TypeNum, const REV: bool>(cs: &Constants, z: Int) -> Option<Int> {
    if T::VALUE >= 14 {
        return if z == 0 { Some(0) } else { None };
    }

    let c = &cs[T::VALUE];
    if c.0 {
        // In these specific steps, we have the opportunity to make Z lower by
        // dividing by 26. However, this is offset by * 26, which we want to
        // avoid, in order to keep Z low.
        // This is only for a specific digit, identified by:
        let digit = z % 26 + c.1;
        if matches!(digit, 1..=9) {
            let z = advance_z_once(c, digit, z);
            if let Some(n) = find_first::<T::Next, REV>(cs, z) {
                return Some(n * 10 + digit);
            }
        }

        return None;
    }

    if REV {
        for digit in (1..10).rev() {
            let z = advance_z_once(c, digit, z);
            if let Some(n) = find_first::<T::Next, REV>(cs, z) {
                return Some(n * 10 + digit);
            }
        }
    } else {
        for digit in 1..10 {
            let z = advance_z_once(c, digit, z);
            if let Some(n) = find_first::<T::Next, REV>(cs, z) {
                return Some(n * 10 + digit);
            }
        }
    }
    None
}

fn reverse_digits(mut digits: Int) -> Int {
    let mut new = 0;
    let mut base = 10_000_000_000_000;
    for _ in 0..14 {
        new += digits % 10 * base;
        digits /= 10;
        base /= 10;
    }
    new
}

fn pts<const REV: bool>(program: &[Instruction]) -> Result<Int> {
    let cs = extract_constants(program);
    find_first::<TypeNum0, REV>(&cs, 0)
        .map(reverse_digits)
        .ok_or_else(|| anyhow!("no solution"))
}

fn pt1(program: &[Instruction]) -> Result<Int> {
    pts::<true>(program)
}

fn pt2(program: &[Instruction]) -> Result<Int> {
    pts::<false>(program)
}

fn parse(input: &[u8]) -> ParseResult<Vec<Instruction>> {
    use parsers::*;
    let register = any().map_res(|c| match c {
        b'w' => Ok(Register::W),
        b'x' => Ok(Register::X),
        b'y' => Ok(Register::Y),
        b'z' => Ok(Register::Z),
        _ => Err(ParseError::UnexpectedChar),
    });
    let opcode = take_while(|c| c != b' ').map_res(|s| match s {
        b"inp" => Ok(OpCode::Inp),
        b"add" => Ok(OpCode::Add),
        b"mul" => Ok(OpCode::Mul),
        b"div" => Ok(OpCode::Div),
        b"mod" => Ok(OpCode::Mod),
        b"eql" => Ok(OpCode::Eql),
        _ => Err(ParseError::TokenDoesNotMatch),
    });
    let arg = token(b' ')
        .then(
            register
                .clone()
                .map(|x| Arg::Register(x))
                .or(number::<Int>().map(|x| Arg::Constant(x))),
        )
        .opt()
        .map(|x| x.unwrap_or(Arg::None));
    let instruction = opcode
        .trailed(token(b' '))
        .and(register)
        .and(arg)
        .map(|((opcode, a), b)| Instruction { opcode, a, b });
    instruction.sep_by(token(b'\n')).parse(input)
}

tests! {
    fn exec_with_inputs<const N: usize>(inputs: [Int; N]) -> impl Fn(&[Instruction]) -> State {
        move |input: &[Instruction]| -> State {
            let mut iter = inputs.into_iter();
            exec(input, || iter.next().unwrap())
        }
    }

    simple_tests!(parse, exec_with_inputs([5, 15]), eval_tests, b"\
inp x
mul x -1" => State([0, -5, 0, 0]), b"\
inp z
inp x
mul z 3
eql z x" => State([0, 15, 0, 1]), b"\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2" => State([0, 1, 0, 1]));
}
