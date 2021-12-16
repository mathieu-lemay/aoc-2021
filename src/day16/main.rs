use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_2021::get_input_as_string;

#[derive(Debug, PartialEq, Eq)]
enum PacketData {
    Litteral(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

impl Packet {
    fn version_sum(&self) -> usize {
        let mut sum = self.version as usize;

        if let PacketData::Operator(packets) = &self.data {
            for p in packets {
                sum += p.version_sum()
            }
        }

        sum
    }

    fn value(&self) -> usize {
        match &self.data {
            PacketData::Litteral(v) => *v as usize,
            PacketData::Operator(packets) => match self.type_id {
                0 => packets.iter().map(|p| p.value()).sum(),
                1 => packets.iter().map(|p| p.value()).product(),
                2 => packets.iter().map(|p| p.value()).min().unwrap(),
                3 => packets.iter().map(|p| p.value()).max().unwrap(),
                5 => {
                    if packets[0].value() > packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].value() < packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].value() == packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("Invalid type id: {}", self.type_id),
            },
        }
    }
}

fn bits2int(bits: &mut VecDeque<u8>, size: u8) -> u64 {
    let mut val = 0u64;

    for _ in 0..size {
        val <<= 1;
        val |= bits.pop_front().unwrap() as u64;
    }

    val
}

fn decode_litteral(bits: &mut VecDeque<u8>) -> u64 {
    let mut value = 0u64;

    loop {
        let byte = bits2int(bits, 5);
        value <<= 4;
        value |= byte & 0xF;

        if byte & 0x10 == 0 {
            break;
        }
    }

    value
}

fn decode_operator(bits: &mut VecDeque<u8>) -> Vec<Packet> {
    let len_type = bits.pop_front().unwrap();
    let sub_packets = match len_type {
        0 => {
            let len = bits2int(bits, 15) as usize;
            let mut sub_bits = bits.drain(..len).collect::<VecDeque<_>>();

            let mut packets = Vec::new();

            while let Some(p) = decode_next_packet(&mut sub_bits) {
                packets.push(p);
            }

            packets
        }
        1 => {
            let nb = bits2int(bits, 11) as usize;

            (0..nb).map(|_| decode_next_packet(bits).unwrap()).collect()
        }
        _ => panic!("Unsupported len type: {}", len_type),
    };

    sub_packets
}

fn decode_next_packet(bits: &mut VecDeque<u8>) -> Option<Packet> {
    if !bits.iter().any(|&b| b == 1) {
        return None;
    }

    let version = bits2int(bits, 3) as u8;
    let type_id = bits2int(bits, 3) as u8;

    let value = match type_id {
        4 => PacketData::Litteral(decode_litteral(bits)),
        _ => PacketData::Operator(decode_operator(bits)),
    };

    Some(Packet {
        version,
        type_id,
        data: value,
    })
}

fn parse(input: &str) -> Vec<Packet> {
    let mut bits = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| {
            let d = c.to_digit(16).unwrap() as u8;
            vec![d >> 3 & 1, d >> 2 & 1, d >> 1 & 1, d & 1]
        })
        .collect::<VecDeque<u8>>();

    let mut packets = Vec::new();

    while let Some(p) = decode_next_packet(&mut bits) {
        packets.push(p);
    }

    packets
}

fn part_1(packets: &[Packet]) -> usize {
    packets.iter().map(|p| p.version_sum()).sum()
}

fn part_2(packets: &[Packet]) -> usize {
    packets.iter().map(|p| p.value()).sum()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let packets = parse(input);

    let p1 = part_1(&packets);
    let p2 = part_2(&packets);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day16.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2, Packet, PacketData};

    #[test]
    fn test_parse_litteral_packet() {
        let input = "D2FE28";

        let expected = vec![Packet {
            version: 6,
            type_id: 4,
            data: PacketData::Litteral(2021),
        }];

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_operator_packet_with_len_of_sub_packets() {
        let input = "38006F45291200";

        let expected = vec![Packet {
            version: 1,
            type_id: 6,
            data: PacketData::Operator(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    data: PacketData::Litteral(10),
                },
                Packet {
                    version: 2,
                    type_id: 4,
                    data: PacketData::Litteral(20),
                },
            ]),
        }];

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse_operator_packet_with_number_of_sub_packets() {
        let input = "EE00D40C823060";

        let expected = vec![Packet {
            version: 7,
            type_id: 3,
            data: PacketData::Operator(vec![
                Packet {
                    version: 2,
                    type_id: 4,
                    data: PacketData::Litteral(1),
                },
                Packet {
                    version: 4,
                    type_id: 4,
                    data: PacketData::Litteral(2),
                },
                Packet {
                    version: 1,
                    type_id: 4,
                    data: PacketData::Litteral(3),
                },
            ]),
        }];

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse("8A004A801A8002F478")), 16);
        assert_eq!(part_1(&parse("620080001611562C8802118E34")), 12);
        assert_eq!(part_1(&parse("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part_1(&parse("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse("C200B40A82")), 3);
        assert_eq!(part_2(&parse("04005AC33890")), 54);
        assert_eq!(part_2(&parse("880086C3E88112")), 7);
        assert_eq!(part_2(&parse("CE00C43D881120")), 9);
        assert_eq!(part_2(&parse("D8005AC2A8F0")), 1);
        assert_eq!(part_2(&parse("F600BC2D8F")), 0);
        assert_eq!(part_2(&parse("9C005AC2F8F0")), 0);
        assert_eq!(part_2(&parse("9C0141080250320F1802104A08")), 1);
    }
}
