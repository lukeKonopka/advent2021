use crate::{OperatorType, VersionedPacket};

type ParseResult<Value, Rest> = Result<(Value, Rest), &'static str>;

fn repeated<T: 'static>(
    parser: fn(&str) -> ParseResult<T, &str>,
) -> Box<dyn Fn(&str) -> ParseResult<Vec<T>, &str>> {
    Box::new(move |input: &str| {
        let mut items = vec![];
        let mut current_input = input;
        loop {
            match parser(current_input) {
                Ok((v, next_input)) => {
                    items.push(v);
                    current_input = next_input;
                }
                Err(_) => break,
            }
        }
        Ok((items, current_input))
    })
}

fn repeated_n_times<T: 'static>(
    parser: fn(&str) -> ParseResult<T, &str>,
    n: usize,
) -> Box<dyn Fn(&str) -> ParseResult<Vec<T>, &str>> {
    Box::new(move |input: &str| {
        let mut items = vec![];
        let mut current_input = input;
        loop {
            if items.len() == n {
                break;
            }
            match parser(current_input) {
                Ok((v, next_input)) => {
                    items.push(v);
                    current_input = next_input;
                }
                Err(_) => break,
            }
        }
        Ok((items, current_input))
    })
}

fn parse_packet_version(input: &str) -> ParseResult<usize, &str> {
    if input.len() >= 3 {
        let v_str = &input[0..3];
        usize::from_str_radix(v_str, 2)
            .map(|v| (v, &input[3..]))
            .map_err(|_| "Cannot parse packet version: Invalid binary number")
    } else {
        Err("Cannot parse packet version: Unexpected end of input")
    }
}

fn parse_type_id(input: &str) -> ParseResult<usize, &str> {
    if input.len() >= 3 {
        let v_str = &input[0..3];
        usize::from_str_radix(v_str, 2)
            .map(|v| (v, &input[3..]))
            .map_err(|_| "Cannot parse type id: Invalid binary number")
    } else {
        Err("Cannot parse type id: Unexpected end of input")
    }
}

fn parse_group(input: &str) -> ParseResult<(usize, bool), &str> {
    if input.len() >= 5 {
        let is_last = input.chars().next().unwrap() == '0';
        let g_str = &input[1..5];
        usize::from_str_radix(g_str, 2)
            .map(|v| ((v, is_last), &input[5..]))
            .map_err(|_| "Cannot parse group: Invalid binary number")
    } else {
        Err("Cannot parse group: Unexpected end of input")
    }
}

fn parse_groups(input: &str) -> ParseResult<usize, &str> {
    let mut current_input = input;
    let mut value = 0;
    loop {
        let ((group_value, is_last), rest) = parse_group(current_input)?;
        current_input = rest;
        value = (value << 4) | group_value;
        if is_last {
            break;
        }
    }

    Ok((value, current_input))
}

struct PacketHeader {
    version: usize,
    type_id: usize,
}

fn parse_packet_header(input: &str) -> ParseResult<PacketHeader, &str> {
    let (version, input) = parse_packet_version(input)?;
    let (type_id, rest) = parse_type_id(input)?;
    Ok((PacketHeader { version, type_id }, rest))
}

fn parse_literal_packet(input: &str) -> ParseResult<VersionedPacket, &str> {
    let (PacketHeader { version, type_id }, input) = parse_packet_header(input)?;
    if type_id != 4 {
        Err("Cannot parse literal packet: Packet id is not equal 4")
    } else {
        let (value, rest) = parse_groups(input)?;
        Ok((
            VersionedPacket {
                version,
                packet: crate::Packet::Literal { value },
            },
            rest,
        ))
    }
}

enum PacketListLength {
    TotalLength(usize),
    SubpacketCount(usize),
}

fn parse_packet_list_length(input: &str) -> ParseResult<PacketListLength, &str> {
    if input.len() >= 1 {
        match input.chars().next().unwrap() {
            '0' => {
                if input.len() < 16 {
                    Err("Cannot parse packet list length: length type ID 0 should be 15 bits long")
                } else {
                    let total_str = &input[1..16];
                    usize::from_str_radix(total_str, 2)
                        .map(|v| (PacketListLength::TotalLength(v), &input[16..]))
                        .map_err(|_| "Cannot parse packet list length: Invalid binary number")
                }
            }
            '1' => {
                if input.len() < 12 {
                    Err("Cannot parse packet list length: length type ID 0 should be 11 bits long")
                } else {
                    let total_str = &input[1..12];
                    usize::from_str_radix(total_str, 2)
                        .map(|v| (PacketListLength::SubpacketCount(v), &input[12..]))
                        .map_err(|_| "Cannot parse subpacket count: Invalid binary number")
                }
            }
            _ => unreachable!("input should be in binary"),
        }
    } else {
        Err("Cannot parse packet list length: no length type ID")
    }
}

fn parse_operator_packet(input: &str) -> ParseResult<VersionedPacket, &str> {
    let (PacketHeader { version, type_id }, input) = parse_packet_header(input)?;
    if type_id == 4 {
        Err("Cannot parse operator packet: Packet id is equal 4")
    } else {
        let operator_type = OperatorType::from_usize(type_id);
        let (packet_list_len, input) = parse_packet_list_length(input)?;
        match packet_list_len {
            PacketListLength::TotalLength(total) => {
                let subpackets_str = &input[0..total];
                let parse_repeated_packets = repeated(parse_packet);
                let (children, _rest) = parse_repeated_packets(subpackets_str)?;
                Ok((
                    VersionedPacket {
                        version,
                        packet: crate::Packet::Operator {
                            children,
                            operator_type,
                        },
                    },
                    &input[total..],
                ))
            }
            PacketListLength::SubpacketCount(count) => {
                let parse_repeated_packets = repeated_n_times(parse_packet, count);
                let (children, rest) = parse_repeated_packets(input)?;
                Ok((
                    VersionedPacket {
                        version,
                        packet: crate::Packet::Operator {
                            children,
                            operator_type,
                        },
                    },
                    rest,
                ))
            }
        }
    }
}

pub(crate) fn parse_packet(input: &str) -> ParseResult<VersionedPacket, &str> {
    parse_literal_packet(input).or(parse_operator_packet(input))
}
