use crate::prelude::*;
use bitvec::{prelude::*, view::BitView};
use smallvec::SmallVec;

day!(16, parse => pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

fn read_bit(remainder: &mut &BitSlice) -> bool {
    let value = remainder[0];
    *remainder = &remainder[1..];
    value
}

fn read_bits<T, const N: usize>(remainder: &mut &BitSlice) -> T
where
    T: Default + BitView + std::ops::ShrAssign<usize> + Sized,
{
    let mut value = T::default();
    value.view_bits_mut::<Msb0>()[std::mem::size_of::<T>() * 8 - N..]
        .clone_from_bitslice(&remainder[0..N]);
    *remainder = &remainder[N..];
    value
}

fn read_version(remainder: &mut &BitSlice) -> u8 {
    read_bits::<u8, 3>(remainder)
}
fn read_packet_type(remainder: &mut &BitSlice) -> PacketType {
    unsafe { std::mem::transmute(read_bits::<u8, 3>(remainder)) }
}
fn read_5bit_encoded_int(remainder: &mut &BitSlice) -> u64 {
    let mut value = 0u64;
    loop {
        let has_more = read_bit(remainder);
        value = (value << 4) | (read_bits::<u8, 4>(remainder) as u64);
        if !has_more {
            break;
        }
    }
    value
}

fn pt1(input: &BitSlice) -> u64 {
    fn read_packet(remainder: &mut &BitSlice, version_sum: &mut u64) {
        let version = read_version(remainder);
        *version_sum += version as u64;
        let packet_type = read_packet_type(remainder);

        if packet_type == PacketType::Literal {
            let _value = read_5bit_encoded_int(remainder);
            return;
        }

        if !read_bit(remainder) {
            // Operator packet with total length in bits
            let bit_length = read_bits::<u16, 15>(remainder) as usize;
            let expected_remainder = &remainder[bit_length..];
            while expected_remainder != *remainder {
                read_packet(remainder, version_sum);
            }
        } else {
            // Operator packet with sub-packet count
            let sub_packet_count = read_bits::<u16, 11>(remainder) as usize;
            for _ in 0..sub_packet_count {
                read_packet(remainder, version_sum);
            }
        }
    }

    let mut version_sum = 0;
    let mut remainder = input;
    read_packet(&mut remainder, &mut version_sum);
    assert!(remainder.count_ones() == 0);

    version_sum
}

fn pt2(input: &BitSlice) -> u64 {
    fn evaluate_packet(remainder: &mut &BitSlice) -> u64 {
        let _version = read_version(remainder);
        let packet_type = read_packet_type(remainder);

        if packet_type == PacketType::Literal {
            return read_5bit_encoded_int(remainder);
        }

        let mut sub_packets = SmallVec::<[u64; 8]>::new();

        if !read_bit(remainder) {
            // Operator packet with total length in bits
            let bit_length = read_bits::<u16, 15>(remainder) as usize;
            let expected_remainder = &remainder[bit_length..];
            while expected_remainder != *remainder {
                sub_packets.push(evaluate_packet(remainder));
            }
        } else {
            // Operator packet with sub-packet count
            let sub_packet_count = read_bits::<u16, 11>(remainder) as usize;
            for _ in 0..sub_packet_count {
                sub_packets.push(evaluate_packet(remainder));
            }
        }

        #[rustfmt::skip]
        match packet_type {
            PacketType::Sum     => sub_packets.iter().cloned().sum(),
            PacketType::Product => sub_packets.iter().cloned().product(),
            PacketType::Minimum => sub_packets.iter().cloned().min().unwrap(),
            PacketType::Maximum => sub_packets.iter().cloned().max().unwrap(),
            PacketType::Literal => unreachable!(),
            PacketType::GreaterThan => if sub_packets[0] >  sub_packets[1] { 1 } else { 0 },
            PacketType::LessThan    => if sub_packets[0] <  sub_packets[1] { 1 } else { 0 },
            PacketType::EqualTo     => if sub_packets[0] == sub_packets[1] { 1 } else { 0 },
        }
    }

    let mut remainder = input;
    let result = evaluate_packet(&mut remainder);
    assert!(remainder.count_ones() == 0);

    result
}

fn parse(input: &str) -> ParseResult<BitVec> {
    use parsers::*;
    any()
        .map_res(|c| {
            c.to_digit(16)
                .map(|c| c as u8)
                .ok_or(ParseError::TokenDoesNotMatch)
        })
        .fold_mut(BitVec::new(), |x, c| {
            x.extend_from_bitslice(&c.view_bits::<Msb0>()[4..8])
        })
        .parse(input)
}

tests! {
    simple_tests!(parse, pt1, pt1_tests,
        "D2FE28" => 6,
        "38006F45291200" => 1 + 6 + 2,
        "8A004A801A8002F478" => 16,
        "620080001611562C8802118E34" => 12,
        "C0015000016115A2E0802F182340" => 23,
        "A0016C880162017C3686B18A3D4780" => 31,
    );
    simple_tests!(parse, pt2, pt2_tests,
        "C200B40A82" => 3,
        "04005AC33890" => 54,
        "880086C3E88112" => 7,
        "CE00C43D881120" => 9,
        "D8005AC2A8F0" => 1,
        "F600BC2D8F" => 0,
        "9C005AC2F8F0" => 0,
        "9C0141080250320F1802104A08" => 1,
    );
}
