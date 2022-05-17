#![allow(dead_code)]
// 784 IS TO LOW

use std::io::Cursor;
use bitstream_io::{BigEndian, BitReader, BitRead};

// const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = "D2FE28"; // test 1: literal:2021
// const INPUT: &str = "38006F45291200"; // test 1: ?
// const INPUT: &str = "EE00D40C823060"; // test 1: ?
// const INPUT: &str = "8A004A801A8002F478"; // test 1: 16
const INPUT: &str = "620080001611562C8802118E34"; // test 1: 12
// const INPUT: &str = "C0015000016115A2E0802F182340"; // test 1: 23
// const INPUT: &str = "A0016C880162017C3686B18A3D4780"; // test 1: 31

fn packet_stream() -> BitReader<Cursor<Vec<u8>>, BigEndian> {
    let raw: Vec<u8> = hex::decode(INPUT.trim()).unwrap();
    println!("RAW LENGTH: {}", raw.len() * 8);
    let cursor = Cursor::new(raw);
    BitReader::new(cursor)
}

#[derive(Debug)]
enum Packet {
    Operator {
        version: u64,
        type_id: u8,
        packets: Vec<Packet>,
        length: u64,
    },
    Literal {
        version: u64,
        type_id: u8,
        value: u64,
        length: u64,
    },
}

// type id 4 == literal
// any other == operator
impl Packet {
    fn parse(stream: &mut impl BitRead) -> Option<Self> {
        let ver: u64 = stream.read(3).unwrap_or(999);
        let typ: u8 = stream.read(3).unwrap_or(255);
        if ver == 999 || typ == 255 { println!("WAT"); return None }

        match typ {
            4 => Packet::parse_literal(stream, ver, typ),
            _ => Packet::parse_operator(stream, ver, typ),
        }
    }

    fn parse_literal(stream: &mut impl BitRead, version: u64, type_id: u8) -> Option<Self> {
        let mut value: u64 = 0;
        let mut length: u64 = 6;
        loop {
            let go = stream.read_bit().unwrap_or(false);
            let group: u64 = stream.read(4).unwrap_or(0);
            if !go && group == 0 { break }
            // println!("({}:{}) VAL:{}, GO:{} GRP:{}", version, type_id, value, go, group);
            length += 5;
            value <<= 4;
            value |= group;
            if !go { break }
        }
        println!("READ LITERAL({} {} {} {})", version, type_id, value, length);
        Some(Packet::Literal{ version, type_id, value, length })
    }

    fn parse_operator(stream: &mut impl BitRead, version: u64, type_id: u8) -> Option<Self> {
        let ltype = stream.read_bit().unwrap_or(false);
        let ltype_length: u32 = if ltype { 11 } else { 15 };
        // println!("VER:{} T:{} STREAM:{}", version, type_id, stream);
        let sub_length: u64 = stream.read(ltype_length).unwrap_or(0);

        let packets = match ltype {
            true => Packet::parse_nth_packets(stream, sub_length),
            false => Packet::parse_length_packets(stream, sub_length),
        };

        let length = 7 + ltype_length as u64 + packets.iter().fold(0, |acc, p| acc + p.len());
        println!("^= OPERATOR:{:?} SUBLEN:{} PACKET:{}", ltype, sub_length, length);
        Some(Packet::Operator { version, type_id, packets, length })
    }

    fn parse_nth_packets(stream: &mut impl BitRead, count: u64) -> Vec<Packet> {
        // println!("READING(N:{}) PACKETS", count);
        let res: Vec<Packet> = (0..count)
            .map(|_| Packet::parse(stream))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        println!("NTH PACKET: NTH:{} PACKETS:{}", count, res.len());
        res
    }

    fn parse_length_packets(stream: &mut impl BitRead, length: u64) -> Vec<Packet> {
        let mut res: Vec<Packet> = Vec::new();
        let mut len = 0;

        loop {
            if let Some(pkt) = Packet::parse(stream) {
                len += pkt.len();
                res.push(pkt);

                // println!("LENGTH READ END MAX:{} LEN:{}", length, len);
                if len >= length {
                    println!("LENGTH PACKET: LEN:{} READ:{}", length, len);
                    break
                }
            }
            println!("LENGTH PACKET: LEN:{} PACKETS:{}", length, res.len());
            break
        }

        res
    }

    fn len(&self) -> u64 {
        match self {
            Packet::Literal{length, ..} => { *length },
            Packet::Operator{length, ..} => { *length },
        }
    }

    fn stage_one_version_total(&self) -> u64 {
        let x = match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator { version, packets, .. } => {
                version + packets.iter().fold(0, |acc, p| acc + p.stage_one_version_total())
            },
        };
        // println!("VTOTAL: {}", x);
        x
    }
}

fn stage_one() -> u64 {
    let mut stream = packet_stream();
    let packet = Packet::parse(&mut stream).unwrap();

    println!("FINAL: {:?}", packet);

    packet.stage_one_version_total() as u64
}

fn stage_two() -> i64 {
    0
}

fn main() {
    // Stage 1:
    println!("Stage 1: {}", stage_one());
    // Stage 2:
    println!("Stage 2: {}", stage_two());
}
