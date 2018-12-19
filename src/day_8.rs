#[derive(Debug)]
struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(tokens: &mut impl Iterator<Item=u8>) -> Node {
        let child_count = tokens.next().unwrap();
        let meta_count = tokens.next().unwrap();

        let child_nodes = (0..child_count)
            .map(|_| Node::parse(tokens))
            .collect();

        let metadata = (0..meta_count)
            .map(|_| tokens.next().unwrap() as usize)
            .collect();

        Node {
            child_nodes,
            metadata,
        }
    }

    fn meta_sum(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self.child_nodes.iter().map(|child| child.meta_sum()).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.child_nodes.is_empty() {
            self.meta_sum()
        } else {
            self.metadata.iter()
                .filter_map(|meta| {
                    if *meta == 0 {
                        None
                    } else {
                        self.child_nodes.get(meta - 1)
                    }
                })
                .map(Node::value)
                .sum()
        }
    }
}

fn main() {
    let input = include_str!("day_8.txt");
    let tokens: Vec<u8> = input.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut token_stream = tokens.into_iter();

    let root = Node::parse(&mut token_stream);
    println!("total metadata sum: {}", root.meta_sum());
    println!("root node value: {}", root.value());
}