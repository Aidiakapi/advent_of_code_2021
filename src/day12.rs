use crate::prelude::*;
use jagged_array::{Jagged2, Jagged2Builder};
use std::{cell::RefCell, mem::swap, ops::DerefMut};

day!(12, parse => pt1, pt2);

type Node = u8;
const NODE_IS_LARGE: Node = 1 << 7;
const START_NODE: Node = 0;
const END_NODE: Node = 1;

type VisitedNodes = u32;
const MAX_NODE_COUNT: usize = 32;

#[derive(Debug)]
struct Input {
    edges: Jagged2<Node>,
    _names: Vec<String>,
}

fn get_row(edges: &Jagged2<Node>, node: Node) -> &[Node] {
    edges.get_row((node & !NODE_IS_LARGE) as usize).unwrap()
}

fn pt1(input: &Input) -> usize {
    fn visit(current_node: Node, input: &Input, visited: VisitedNodes, total_paths: &mut usize) {
        for &next_node in get_row(&input.edges, current_node) {
            if next_node == END_NODE {
                *total_paths += 1;
                continue;
            }
            if next_node & NODE_IS_LARGE == NODE_IS_LARGE {
                visit(next_node, input, visited, total_paths);
            } else if visited & (1 << next_node) == 0 {
                visit(next_node, input, visited | (1 << next_node), total_paths);
            }
        }
    }
    let mut total_paths = 0;
    visit(START_NODE, input, 1 << START_NODE, &mut total_paths);
    total_paths
}

fn pt2(input: &Input) -> usize {
    fn visit(
        current_node: Node,
        input: &Input,
        visited: VisitedNodes,
        has_visited_twice: bool,
        total_paths: &mut usize,
    ) {
        for &next_node in get_row(&input.edges, current_node) {
            if next_node == END_NODE {
                *total_paths += 1;
                continue;
            }
            if next_node & NODE_IS_LARGE == NODE_IS_LARGE {
                visit(next_node, input, visited, has_visited_twice, total_paths);
            } else if visited & (1 << next_node) == 0 {
                visit(
                    next_node,
                    input,
                    visited | (1 << next_node),
                    has_visited_twice,
                    total_paths,
                );
            } else if !has_visited_twice && next_node != START_NODE {
                visit(next_node, input, visited, true, total_paths)
            }
        }
    }
    let mut total_paths = 0;
    visit(START_NODE, input, 1 << START_NODE, false, &mut total_paths);
    total_paths
}

fn get_edges<'s>(edges: &'s Vec<(Node, Node)>, node: Node) -> impl Iterator<Item = Node> + 's {
    edges.iter().filter_map(move |&(a, b)| {
        if a & !NODE_IS_LARGE == node {
            Some(b)
        } else if b & !NODE_IS_LARGE == node {
            Some(a)
        } else {
            None
        }
    })
}

fn parse(input: &str) -> ParseResult<Input> {
    use parsers::*;
    let names = RefCell::new(vec![
        (START_NODE, "start".to_owned()),
        (END_NODE, "end".to_owned()),
    ]);
    let node = take_while(|c| c.is_ascii_alphabetic()).map(|str| {
        let mut names = names.borrow_mut();
        match names.iter().find(|(_, x)| *x == str) {
            Some((idx, _)) => *idx,
            None => {
                let mut idx = names.len() as Node;
                assert!((idx & NODE_IS_LARGE) == 0);
                if str.chars().all(|c| c.is_ascii_uppercase()) {
                    idx |= NODE_IS_LARGE;
                }
                names.push((idx, str.to_owned()));
                idx
            }
        }
    });
    let edges = node
        .clone()
        .and(token('-').then(node))
        .sep_by::<_, Vec<(Node, Node)>>(token('\n'));
    edges
        .map(|edges: Vec<_>| {
            let names = {
                let mut temp = Vec::new();
                swap(&mut temp, names.borrow_mut().deref_mut());
                temp
            };
            assert!(names.len() <= MAX_NODE_COUNT);

            let mut builder = Jagged2Builder::new();
            for i in 0..names.len() {
                builder.extend(get_edges(&edges, i as u8));
            }

            Input {
                edges: builder.into(),
                _names: names.into_iter().map(|(_, name)| name).collect(),
            }
        })
        .parse(input)
}

tests! {
    const EXAMPLE1: &'static str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    const EXAMPLE2: &'static str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    const EXAMPLE3: &'static str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    simple_tests!(parse, pt1, pt1_tests,
        EXAMPLE1 => 10,
        EXAMPLE2 => 19,
        EXAMPLE3 => 226,
    );
    simple_tests!(parse, pt2, pt2_tests,
        EXAMPLE1 => 36,
        EXAMPLE2 => 103,
        EXAMPLE3 => 3509,
    );
}
