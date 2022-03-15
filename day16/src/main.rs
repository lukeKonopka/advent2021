use std::fs::read_to_string;

use parser::parse_packet;

mod parser;

#[derive(PartialEq, Eq, Debug)]
struct VersionedPacket {
    version: usize,
    packet: Packet,
}

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    Literal {
        value: usize,
    },
    Operator {
        operator_type: OperatorType,
        children: Vec<VersionedPacket>,
    },
}

impl Packet {
    fn eval(&self) -> usize {
        match self {
            Packet::Literal { value } => *value,
            Packet::Operator {
                operator_type,
                children,
            } => {
                let child_values = children.iter().map(|c| c.packet.eval()).collect();
                operator_type.apply(child_values)
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl OperatorType {
    pub(crate) fn from_usize(v: usize) -> Self {
        match v {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::Equal,
            _ => panic!("Unknown operator type id {}", v),
        }
    }
    fn apply(&self, values: Vec<usize>) -> usize {
        match self {
            OperatorType::Sum => values.iter().sum(),
            OperatorType::Product => values.iter().product(),
            OperatorType::Minimum => *values.iter().min().unwrap(),
            OperatorType::Maximum => *values.iter().max().unwrap(),
            OperatorType::GreaterThan => {
                assert!(
                    values.len() == 2,
                    "OperatorType::GreaterThan should have exactly two children"
                );
                let a = values.get(0).unwrap();
                let b = values.get(1).unwrap();
                if a > b {
                    1
                } else {
                    0
                }
            }
            OperatorType::LessThan => {
                assert!(
                    values.len() == 2,
                    "OperatorType::LessThan should have exactly two children"
                );
                let a = values.get(0).unwrap();
                let b = values.get(1).unwrap();
                if a < b {
                    1
                } else {
                    0
                }
            }
            OperatorType::Equal => {
                assert!(
                    values.len() == 2,
                    "OperatorType::Equal should have exactly two children"
                );
                let a = values.get(0).unwrap();
                let b = values.get(1).unwrap();
                if a == b {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn walker<T>(root: &VersionedPacket, visit_fn: fn(packet: &VersionedPacket) -> T) -> Vec<T> {
    let v = visit_fn(root);
    match &root.packet {
        Packet::Literal { .. } => vec![v],
        Packet::Operator { children, .. } => std::iter::once(v)
            .chain(children.iter().map(|p| walker(p, visit_fn)).flatten())
            .collect(),
    }
}

fn main() {
    let input = read_to_string("./input").expect("Cannot read input file");
    // let input = String::from("9C0141080250320F1802104A08");
    let binary = input
        .chars()
        .map(|c| {
            format!(
                "{:04b}",
                usize::from_str_radix(&String::from(c), 16).unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let (parsed, _rest) = parse_packet(&binary).expect("cannot parse input");
    let sum: usize = walker(&parsed, |p| p.version).iter().sum();
    let eval = parsed.packet.eval();
    println!("sum: {:?}", sum);
    println!("value: {:?}", eval);
}
