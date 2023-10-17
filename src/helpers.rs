use std::fs;

use crate::node::{Node, self};

pub(crate) fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

pub(crate) fn get_cpu_node(cpu_nu: &str) -> Node {
    let nr: i16 = cpu_nu[3..].parse().unwrap();
    let mut span = Vec::new();
    let cpu_domain;
    let len = nr / 32;
    for _ in 0..len {
        span.push(0);
    }
    span.push(1 << (nr % 32));
    cpu_domain = Node::new(-1, span, nr);
    println!("cpu: {}", nr);
    node::insert_node(cpu_domain.clone());
    cpu_domain
}

pub(crate) fn get_domain_node(domain_nu: &str, cpu_mask_str: &str) -> Node {
    let level: i8 = domain_nu[6..].parse().unwrap();
    let mut cpu_span: Vec<u32> = Vec::new();
    for mask in cpu_mask_str.rsplit(",") {
        cpu_span.push(u32::from_str_radix(mask, 16).unwrap());
    }

    let domain = Node::new(level, cpu_span, -1);
    node::insert_node(domain.clone());
    // println!("Level: {level}\n{:#10X?}\n", cpu_span);
    domain
}
