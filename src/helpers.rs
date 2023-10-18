use std::fs;

use crate::node::{self, Node};

pub(crate) fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

pub(crate) fn get_cpu_node(cpu_nu: &str) -> Node {
    let nr: i16 = cpu_nu[3..].parse().unwrap();
    let mut span = Vec::new();
    let len = nr / 32;
    for _ in 0..len {
        span.push(0);
    }
    span.push(1 << (nr % 32));
    Node::new(-1, span, nr)
}

pub(crate) fn get_domain_node(domain_nu: &str, cpu_mask_str: &str) -> Node {
    let level: i8 = domain_nu[6..].parse().unwrap();
    let mut cpu_span: Vec<u32> = Vec::new();
    for mask in cpu_mask_str.split(",") {
        cpu_span.push(u32::from_str_radix(mask, 16).unwrap());
    }

    let domain = node::get_domain_node_for_span(&cpu_span);
    match domain {
        Some(node) => node,
        None => Node::new(level, cpu_span, -1),
    }
}
