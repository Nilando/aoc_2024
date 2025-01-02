use std::collections::{HashMap, HashSet};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::{
    attributes::*,
    cmd::{CommandArg, Format},
    exec, parse,
    printer::{DotPrinter, PrinterContext},
};

#[derive(Debug)]
enum GateKind {
    And,
    Or,
    Xor
}

#[derive(Debug)]
struct Gate {
    w1: String,
    w2: String,
    kind: GateKind,
    dest: String,
}

fn main() {
    let input = std::include_str!("../input.txt");
    let mut wires = HashMap::<String, bool>::new();
    let mut gates = Vec::<Gate>::new();

    let mut parse_flag = true;
    for line in input.lines() {
        if line.trim().is_empty() {
            parse_flag = false;
            continue;
        }

        if parse_flag {
            let mut iter = line.split(':');
            let wire_name = iter.next().unwrap();
            let value = iter.next().unwrap().trim().parse::<usize>().unwrap() == 1;

            wires.insert(String::from(wire_name), value);
        } else {
            let mut iter = line.split(' ');
            let w1 = iter.next().unwrap();
            let kind = match iter.next().unwrap() {
                "AND" => GateKind::And,
                "XOR" => GateKind::Xor,
                "OR" => GateKind::Or,
                _ => panic!("bad gate kind"),
            };
            let w2 = iter.next().unwrap();
            let _ = iter.next().unwrap();
            let dest = iter.next().unwrap();

            gates.push(
                Gate {
                    w1: String::from(w1),
                    w2: String::from(w2),
                    kind,
                    dest: String::from(dest),
                }
            );
        }
    }

    println!("total gates: {}", gates.len());
    println!("total wires: {}", wires.len());

    let part1 = add_with_gates(&gates, &mut wires);
    println!("{}", part1);

    let mut g = graph!(id!("id"));
    let mut gate_id = 0;
    for Gate { w1, w2, kind, dest } in gates.iter() {
        gate_id += 1;
        g.add_stmt(stmt!(node!(w1)));
        g.add_stmt(stmt!(node!(w2)));
        g.add_stmt(stmt!(node!(dest)));

        let shape = 
        match kind {
            GateKind::And => {
                NodeAttributes::shape(shape::invhouse)
            }
            GateKind::Or => {
                NodeAttributes::shape(shape::circle)
            }
            GateKind::Xor => {
                NodeAttributes::shape(shape::doublecircle)
            }
        };

        g.add_stmt(stmt!(node!(gate_id; shape)));
        g.add_stmt(stmt!(edge!(node_id!(w1) => node_id!(gate_id))));
        g.add_stmt(stmt!(edge!(node_id!(w2) => node_id!(gate_id))));
        g.add_stmt(stmt!(edge!(node_id!(gate_id) => node_id!(dest))));
    }

    let mut bad_wires = vec!["frn", "z05", "z39", "wtt","gmq","z21","wnf", "vtj"];

    bad_wires.sort();

    for wire in bad_wires.iter() {
        print!("{wire},");
    }
        println!("");


    /*
    let mut g = graph!(id!("id");
         node!("nod"),
         subgraph!("sb";
             edge!(node_id!("a") => subgraph!(;
                node!("n"; NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::triangle))
            ))
        ),
        edge!(node_id!("a1") => node_id!(esc "a2"))
    );
    */


    let svg_name = String::from("part2.svg");
    exec(g.clone(), &mut PrinterContext::default(), vec![
        CommandArg::Format(Format::Svg),
        CommandArg::Output(svg_name)
    ]).unwrap();
}

fn init_wires(x: usize, y: usize) -> HashMap<String, bool> {
    let mut wires = HashMap::<String, bool>::new();

    for i in 0..44 {
        wires.insert(format!("x{i:0>2}"), ((1 << i) & x) != 0);
        wires.insert(format!("y{i:0>2}"), ((1 << i) & y) != 0);
    }

    wires
}

fn add_with_gates(gates: &Vec<Gate>, wires: &mut HashMap<String, bool>) -> usize {
    let mut break_flag = false;

    while !break_flag {
        break_flag = true;

        for Gate { w1, w2, kind, dest } in gates.iter() {
            if wires.get(dest).is_some() || wires.get(w1).is_none() || wires.get(w2).is_none() {
                continue;
            }

            let v1 = wires.get(w1).unwrap();
            let v2 = wires.get(w2).unwrap();

            let result: bool = 
            match kind {
                GateKind::And => *v1 && *v2,
                GateKind::Or => *v1 || *v2,
                GateKind::Xor => *v1 ^ *v2,
            };

            wires.insert(dest.to_string(), result);
            break_flag = false;
        }
    }

    let mut z_wires = vec![];
    for (wire, _) in wires.iter() {
        if wire.chars().next().unwrap() == 'z' {
            z_wires.push(wire.to_string());
        }
    }

    z_wires.sort();
    z_wires.reverse();

    let mut result = 0;
    for z_wire in z_wires.iter() {
        result <<= 1;

        if *wires.get(z_wire.as_str()).unwrap() {
            result += 1;
        }
    }

    result
}
