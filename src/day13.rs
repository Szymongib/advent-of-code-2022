#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Literal(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if less(self, other) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    Literal(u32),
    ListStart,
    ListEnd,
}

fn tokenize_packet(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut num = c.to_digit(10).unwrap();
                while let Some(c) = chars.peek() {
                    if c.is_digit(10) {
                        num = num * 10 + c.to_digit(10).unwrap();
                        chars.next(); // consume peeked char
                    } else {
                        tokens.push(Token::Literal(num));
                        break;
                    }
                }
            }
            ',' => {}
            '[' => tokens.push(Token::ListStart),
            ']' => tokens.push(Token::ListEnd),
            _ => panic!("invalid char: {}", c),
        }
    }

    tokens
}

fn parse_list<T: Iterator<Item = Token>>(tokens: &mut T) -> Packet {
    let mut list = Vec::new();

    while let Some(t) = tokens.next() {
        match t {
            Token::Literal(num) => {
                list.push(Packet::Literal(num));
            }
            Token::ListStart => {
                list.push(parse_list(tokens));
            }
            Token::ListEnd => {
                break;
            }
        }
    }

    Packet::List(list)
}

fn parse_packet(line: &str) -> Packet {
    let tokens = tokenize_packet(line);

    match tokens[0] {
        Token::ListStart => parse_list(&mut tokens[1..].to_vec().into_iter()),
        Token::Literal(num) => Packet::Literal(num),
        _ => panic!("invalid packet"),
    }
}

fn less(a: &Packet, b: &Packet) -> bool {
    match (a, b) {
        (Packet::Literal(a), Packet::Literal(b)) => a < b,
        (Packet::List(a), Packet::List(b)) => {
            let mut a = a.iter();
            let mut b = b.iter();

            loop {
                match (a.next(), b.next()) {
                    (Some(a), Some(b)) => {
                        if less(a, b) {
                            return true;
                        } else if less(b, a) {
                            return false;
                        }
                    }
                    (Some(_), None) => return false,
                    (None, Some(_)) => return true,
                    (None, None) => return false,
                }
            }
        }
        (p1 @ Packet::Literal(_), p2 @ Packet::List(_)) => {
            less(&Packet::List(vec![p1.clone()]), p2)
        }
        (p1 @ Packet::List(_), p2 @ Packet::Literal(_)) => {
            less(p1, &Packet::List(vec![p2.clone()]))
        }
    }
}

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let packet_pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|packets_pair| {
            let pp: Vec<Packet> = packets_pair
                .lines()
                .map(|packet| parse_packet(packet))
                .collect();
            assert_eq!(pp.len(), 2);
            (pp[0].clone(), pp[1].clone())
        })
        .collect();

    let mut right_order_idx_sum: usize = 0;

    for (idx, (a, b)) in packet_pairs.iter().enumerate() {
        if less(a, b) {
            right_order_idx_sum += idx + 1;
        }
    }

    Ok(right_order_idx_sum)
}

pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let packet_pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|packets_pair| {
            let pp: Vec<Packet> = packets_pair
                .lines()
                .map(|packet| parse_packet(packet))
                .collect();
            assert_eq!(pp.len(), 2);
            (pp[0].clone(), pp[1].clone())
        })
        .collect();

    let div_packet1 = Packet::List(vec![Packet::List(vec![Packet::Literal(2)])]);
    let div_packet2 = Packet::List(vec![Packet::List(vec![Packet::Literal(6)])]);

    let mut all_packets = vec![div_packet1.clone(), div_packet2.clone()];

    for (p1, p2) in packet_pairs {
        all_packets.push(p1);
        all_packets.push(p2);
    }

    all_packets.sort_unstable();

    let decoder_key = all_packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p == div_packet1 || *p == div_packet2)
        .map(|(idx, _)| idx + 1)
        .product();

    Ok(decoder_key)
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 140);
    }
}
