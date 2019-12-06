use std::collections::HashSet;
use std::ops::Add;

struct Wire {
    nodes: Vec<Node>
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
struct Node {
    x: isize,
    y: isize,
}

impl Node {
    pub fn abs(self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[aoc_generator(day3)]
fn generate_wires(input: &str) -> Vec<Wire> {
    let split: Vec<&str> = input.lines().collect();
    let wires: Vec<Vec<&str>> = split.iter().map(|&wire| wire.split(',').collect()).collect();
    wires.iter().map(|path| map_to_wire(path.to_vec())).collect()
}

fn map_to_wire(path: Vec<&str>) -> Wire {
    let mut wire = Wire { nodes: vec![Node { x: 0, y: 0 }] };

    path.iter().for_each(|&string| {
        let (direction, range) = string.split_at(1);
        let add_node = match direction.chars().nth(0).unwrap() {
            'R' => {
                Node { x: 1, y: 0 }
            }
            'L' => {
                Node { x: -1, y: 0 }
            }
            'U' => {
                Node { x: 0, y: 1 }
            }
            'D' => {
                Node { x: 0, y: -1 }
            }
            _ => panic!("fuuuuuuuuuuuuuuuuuuuu")
        };
        for _ in 0..range.parse().unwrap() {
            wire.nodes.push(wire.nodes.last().unwrap().clone() + add_node);
        }
    });
    wire.nodes.sort_by(|a, b| a.abs().cmp(&b.abs()));
    wire
}

#[aoc(day3, part1)]
fn find_nearest_knot(wires: &[Wire]) -> isize {
    let mut knots: HashSet<Node> = HashSet::new();
    for node_a in wires[0].nodes.iter().skip(1) {
        for node_b in wires[1].nodes.iter().skip(1) {
            if node_a == node_b { knots.insert(*node_a); }
        };
    };
    knots.iter().map(|node| node.abs()).min().unwrap()
}

#[aoc(day3, part2)]
fn find_shortest_knot(wires: &[Wire]) -> isize {
    let mut nearest_knot: Option<isize> = None;
    let mut steps_a = 1;
    let mut steps_b = 1;
    for node_a in wires[0].nodes.iter().skip(1) {
        for node_b in wires[1].nodes.iter().skip(1) {
            if node_a == node_b {
                if nearest_knot.is_some() {
                    nearest_knot = Some(nearest_knot.unwrap().min(steps_a + steps_b));
                } else {
                    nearest_knot = Some(steps_a + steps_b);
                }
            }
            steps_b += 1;
        };
        steps_b = 0;
        steps_a += 1;
    };
    nearest_knot.unwrap()
}