use std::num::NonZeroU16;

use hashbrown::HashMap;

#[derive(Clone, Copy, Debug)]
struct Node {
    is_term: bool,
    next: [Option<NonZeroU16>; 5],
}

struct Tree {
    nodes: Vec<Node>,
}

const fn chr_to_idx(chr: u8) -> usize {
    match chr {
        b'b' => 0,
        b'g' => 1,
        b'r' => 2,
        b'u' => 3,
        b'w' => 4,
        _ => panic!("invalid chr"),
    }
}

impl Tree {
    fn new() -> Tree {
        Tree {
            nodes: vec![Node {
                is_term: false,
                next: [None; 5],
            }],
        }
    }

    fn add(&mut self, mut elt: &[u8]) {
        let mut idx = 0;

        while !elt.is_empty() {
            let chr = elt[0];
            let k = chr_to_idx(chr);
            idx = if let Some(node) = self.nodes[idx].next[k] {
                node.get() as usize
            } else {
                let new_node = Node {
                    is_term: false,
                    next: [None; 5],
                };
                let new_idx = self.nodes.len();
                self.nodes.push(new_node);
                self.nodes[idx].next[k] = NonZeroU16::new(new_idx.try_into().unwrap());
                new_idx
            };
            elt = &elt[1..];
        }

        self.nodes[idx].is_term = true;
    }

    fn search(&self, elt: &[u8], cache: &mut HashMap<usize, usize>) -> usize {
        fn recurse(tree: &Tree, elt: &[u8], i: usize, cache: &mut HashMap<usize, usize>) -> usize {
            if let Some(res) = cache.get(&i) {
                return *res;
            }

            let mut j = i;
            let mut node = &tree.nodes[0];
            let mut res = 0;

            while j < elt.len() {
                let chr = elt[j];
                let Some(idx) = node.next[chr_to_idx(chr)] else {
                    break;
                };
                j += 1;

                node = &tree.nodes[idx.get() as usize];

                if node.is_term {
                    res += recurse(tree, elt, j, cache);
                }
            }

            if j == elt.len() && node.is_term {
                res += 1;
            }

            cache.insert(i, res);
            res
        }

        cache.clear();
        recurse(self, elt, 0, cache)
    }
}

fn main() {
    let input = aoc::input_str(19);

    let (available, desired) = input.split_once("\n\n").unwrap();
    let mut tree = Tree::new();
    for towel in available.split(", ") {
        tree.add(towel.as_bytes());
    }
    let tree = tree;

    let mut p1 = 0;
    let mut p2 = 0;

    let mut cache = HashMap::new();
    for pattern in desired.split_ascii_whitespace() {
        let tmp = tree.search(pattern.as_bytes(), &mut cache);
        if tmp != 0 {
            p1 += 1;
            p2 += tmp;
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}