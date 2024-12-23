use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

type Connections<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse_input(input: &str) -> Connections {
    let mut connections: Connections = HashMap::new();
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

fn find_max_group<'a>(connections: &Connections<'a>) -> Option<HashSet<&'a str>> {
    let empty_map = HashSet::new();
    let nodes: Vec<&str> = connections.keys().copied().collect();
    (0..nodes.len())
        .into_par_iter()
        .map(|i| (nodes[i], &nodes[i + 1..]))
        .filter_map(|(head, tail)| find_max_group_rec(tail, connections, &empty_map, head))
        .max_by_key(|x| x.len())
}

fn find_max_group_rec<'a>(
    nodes: &[&'a str],
    connections: &Connections<'a>,
    group: &HashSet<&'a str>,
    new_node: &'a str,
) -> Option<HashSet<&'a str>> {
    if !connections[new_node].is_superset(&group) {
        return None;
    }
    let mut group = group.clone();
    group.insert(new_node);
    (0..nodes.len())
        .filter(|i| connections[new_node].contains(&nodes[*i]))
        .map(|i| (nodes[i], &nodes[i + 1..]))
        .filter_map(|(nbor, tail)| find_max_group_rec(tail, connections, &group, nbor))
        .max_by_key(|group| group.len())
        .or(Some(group))
}

pub fn part2(input: &str) -> String {
    let connections = parse_input(input);

    let mut max_group: Vec<_> = find_max_group(&connections).unwrap().into_iter().collect();
    max_group.sort_unstable();

    max_group.join(",")
}
