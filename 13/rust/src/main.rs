use std::cmp::Ordering;
use std::io;

enum Node {
    Value(i32),
    List(Vec<Node>),
}
type Packet = Vec<Node>;

// impl Ord for Packet {}

fn split_enclosed(input: &str) -> (&str, &str) {
    // Classic paren match with counter
    assert!(input.starts_with("["));
    let mut count = 0;
    let mut split_index = 0;
    for (index, char) in input.char_indices() {
        if char == '[' {
            count += 1;
        }
        if char == ']' {
            count -= 1;
            if count == 0 {
                split_index = index + ']'.len_utf8();
                break;
            }
        }
    }
    input.split_at(split_index)
}

fn parse_packet(input: &str) -> Packet {
    let mut packet = Packet::new();
    // Must work, or okay to panic
    let mut input = input.strip_prefix("[").unwrap().strip_suffix("]").unwrap(); // Hopefully not O(input.len())...
    while input.len() != 0 {
        if input.starts_with("[") {
            let (enclosed, remainder) = split_enclosed(input);
            packet.push(Node::List(parse_packet(enclosed)));
            input = remainder;
        } else {
            if let Some((value, remainder)) = input.split_once(",") {
                if value.len() != 0 {
                    packet.push(Node::Value(value.parse::<i32>().unwrap()));
                }
                input = remainder;
            } else {
                packet.push(Node::Value(input.parse::<i32>().unwrap()));
                break;
            }
        };
    }
    packet
}

fn compare_packets(lhs: &Packet, rhs: &Packet) -> Ordering {
    // There's likely a better way to do that comparison by checking lengths
    // first, maybe. It looks like complex boiler-plate, at least it works
    // exactly as described.
    let mut lhi = lhs.iter();
    let mut rhi = rhs.iter();
    let mut ordering = Ordering::Equal;
    loop {
        if let Some(left) = lhi.next() {
            if let Some(right) = rhi.next() {
                match left {
                    Node::Value(left_value) => match right {
                        Node::Value(right_value) => {
                            if left_value < right_value {
                                ordering = Ordering::Less;
                            } else if left_value > right_value {
                                ordering = Ordering::Greater;
                            }
                        }
                        Node::List(right_list) => {
                            ordering = compare_packets(&vec![Node::Value(*left_value)], right_list);
                        }
                    },
                    Node::List(left_list) => match right {
                        Node::Value(right_value) => {
                            ordering = compare_packets(left_list, &vec![Node::Value(*right_value)]);
                        }
                        Node::List(right_list) => {
                            ordering = compare_packets(left_list, right_list);
                        }
                    },
                }
            } else {
                ordering = Ordering::Greater;
            }
        } else {
            if let Some(_) = rhi.next() {
                ordering = Ordering::Less
            } else {
                // Only condition that forces equality to be returned
                return Ordering::Equal;
            }
        }
        if ordering != Ordering::Equal {
            break;
        }
    }
    ordering
}

fn main() {
    // Where I'm finally realising the power of the functional style...
    let mut inputs: Vec<Packet> = io::stdin()
        .lines()
        .map(|line| String::from(line.unwrap().trim()))
        .filter(|line| line != &"")
        .map(|line| parse_packet(line.as_str()))
        .collect();

    let mut sum = 0;

    println!("Part1 answer:");
    for (index, (left, right)) in inputs.chunks(2).map(|x| (&x[0], &x[1])).enumerate() {
        if compare_packets(&left, &right) != Ordering::Greater {
            // println!("  Pair {0} is in the right order", index + 1);
            sum += index + 1; // due to chunking and start a 1.
        }
    }
    println!("\tSum of indices in the right order: {sum}");

    println!("Part2 answer:");
    inputs.sort_by(|a, b| compare_packets(a, b));
    let first_divider = parse_packet(&"[[2]]");
    let second_divider = parse_packet(&"[[6]]");
    let decoder_key = (inputs
        .partition_point(|x| compare_packets(x, &first_divider) == Ordering::Less)
        + 1)
        * (inputs.partition_point(|x| compare_packets(x, &second_divider) == Ordering::Less) + 2);
    println!("\tDecoder key = {}", decoder_key);
}
