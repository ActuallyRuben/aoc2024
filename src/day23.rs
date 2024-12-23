use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let a_entry = connections.entry(a).or_default();
        a_entry.insert(b);
        let b_entry = connections.entry(b).or_default();
        b_entry.insert(a);
    }
    connections
}

pub fn part1(input: &str) -> usize {
    let connections = parse_input(input);
    let mut groups: HashSet<[&str; 3]> = HashSet::new();
    for (node, neighbors) in &connections {
        if !node.starts_with('t') {
            continue;
        }
        for neighbor in neighbors {
            for neighbors_neighbor in &connections[neighbor] {
                if neighbors_neighbor == node {
                    continue;
                }
                if neighbors.contains(neighbors_neighbor) {
                    let mut group = [*node, *neighbor, *neighbors_neighbor];
                    group.sort_unstable();
                    groups.insert(group);
                }
            }
        }
    }
    groups.len()
}

fn get_pair_mut<T>(slice: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    assert_ne!(a, b, "cannot get two mutable references to index {a}");
    if a < b {
        let (p1, p2) = slice.split_at_mut(b);
        (&mut p1[a], &mut p2[0])
    } else {
        let (p1, p2) = slice.split_at_mut(a);
        (&mut p1[b], &mut p2[0])
    }
}

fn merge_groups<'a>(
    gid_a: usize,
    gid_b: usize,
    groups: &mut Vec<HashSet<&'a str>>,
    nodes: &mut HashMap<&'a str, usize>,
) {
    if gid_a == gid_b {
        return;
    }
    let (group_a, group_b) = get_pair_mut(groups, gid_a, gid_b);
    println!("joining {:?} and {:?}", group_a, group_b);
    for node in group_b.drain() {
        nodes.insert(node, gid_a);
        group_a.insert(node);
    }
}

fn connect_nodes<'a>(
    a: &'a str,
    b: &'a str,
    groups: &mut Vec<HashSet<&'a str>>,
    nodes: &mut HashMap<&'a str, usize>,
) {
    match (nodes.contains_key(a), nodes.contains_key(b)) {
        (false, false) => {
            let gid = groups.len();
            groups.push(HashSet::from([a, b]));
            nodes.insert(a, gid);
            nodes.insert(b, gid);
        }
        (true, false) => {
            let gid = nodes[a];
            groups[gid].insert(b);
            nodes.insert(b, gid);
        }
        (false, true) => {
            let gid = nodes[b];
            groups[gid].insert(a);
            nodes.insert(a, gid);
        }
        (true, true) => {
            merge_groups(nodes[a], nodes[b], groups, nodes);
        }
    }
}

pub fn part2(input: &str) -> String {
    let mut groups: Vec<HashSet<&str>> = Vec::new();
    let mut nodes: HashMap<&str, usize> = HashMap::new();
    for (a, b) in input.lines().map(|x| x.split_once('-').unwrap()) {
        connect_nodes(a, b, &mut groups, &mut nodes);
    }
    let mut max_group = groups
        .into_iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    max_group.sort_unstable();
    max_group.join(",")
}
