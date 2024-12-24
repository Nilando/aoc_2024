use std::collections::{HashMap, HashSet};

fn main() {
    part1();
    part2();
}

fn part2() {
    let input = std::include_str!("../input.txt");
    let mut connections = HashMap::<&str, Vec<&str>>::new();
    let mut largest_networks = Vec::<HashSet<&str>>::new();
    
    for line in input.lines() {
        let mut iter = line.split('-');
        let comp_a = iter.next().unwrap();
        let comp_b = iter.next().unwrap();

        add_connection(&mut connections, comp_a, comp_b);
        add_connection(&mut connections, comp_b, comp_a);

        largest_networks.push(HashSet::from([comp_a, comp_b]));
    }

    let mut largest_network_size = 0;
    let mut new_largest_network = HashSet::<&str>::new();

    for mut network in largest_networks.into_iter() {
        for (comp, _) in connections.iter() {
            if network.contains(comp) {
                continue;
            }

            let mut in_network_flag = true;
            for c in network.iter() {
                if !are_connected(&connections, comp, c) {
                    in_network_flag = false;
                    break;
                }
            }

            if in_network_flag {
                network.insert(comp);
            }
        }

        if network.len() >= largest_network_size {
            largest_network_size = network.len();
            new_largest_network = network;
        }
    }

    let mut password = new_largest_network.iter().collect::<Vec<&&str>>();
    password.sort();

    for c in password.iter() {
        print!("{},", c);
    }
    println!("");
}

fn part1() {
    //let input = std::include_str!("../test.txt");
    let input = std::include_str!("../input.txt");
    let mut connections = HashMap::<&str, Vec<&str>>::new();
    let mut found_sets = HashSet::<[&str; 3]>::new();
    
    for line in input.lines() {
        let mut iter = line.split('-');
        let comp_a = iter.next().unwrap();
        let comp_b = iter.next().unwrap();

        add_connection(&mut connections, comp_a, comp_b);
        add_connection(&mut connections, comp_b, comp_a);
    }

    for (comp, conns) in connections.iter() {
        for a in 0..conns.len() {
            for b in a+1..conns.len() {
                let comp_a = conns[a];
                let comp_b = conns[b];

                if comp.chars().next().unwrap() != 't' && comp_a.chars().next().unwrap() != 't' && comp_b.chars().next().unwrap() != 't' {
                    continue;
                }

                let mut found_set = [comp, comp_a, comp_b];
                found_set.sort();

                if found_sets.get(&found_set).is_some() {
                    continue;
                }

                if are_connected(&connections, comp_a, comp_b) {
                    found_sets.insert(found_set);
                }
            }
        }
    }

    println!("part1: {}", found_sets.len());
}

fn are_connected(connections: &HashMap<&str, Vec<&str>>, a: &str, b: &str) -> bool {
    if let Some(conns) = connections.get(a) {
        if conns.contains(&b) {
            return true
        }
    }

    false
}

fn add_connection<'a>(connections: &mut HashMap<&'a str, Vec<&'a str>>, a: &'a str, b: &'a str) {
    if let Some(conn_list) = connections.get_mut(a) {
        conn_list.push(b);
    } else {
        connections.insert(a, vec![b]);
    }
}
