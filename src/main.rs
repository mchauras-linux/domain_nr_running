use node::Node;

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

mod domain;
mod helpers;
mod node;

const INDEX_TIMESTAMP: usize = 3;
const INDEX_CPU: usize = 5;
const INDEX_NR_RUNNING: usize = 7;

fn print_line(cpu_node: Node, timestamp: &str) {
    let mut node = cpu_node.clone();
    let mut line = String::new();
    line += &(timestamp.to_owned() + &"\tcpu".to_string());
    line += &(cpu_node.get_id().to_string() + &"=".to_string());
    line += &(cpu_node.get_nr_running().to_string() + &"\t\t".to_string());

    //Skip cpu node
    if let Some(parent) = node.get_parent() {
        node = parent;
    }

    loop {
        line += &(("d".to_string() + &node.get_level().to_string()) + &"=".to_string());
        line += &(node.get_nr_running().to_string() + &"\t".to_string());
        // Loop Control
        match node.get_parent() {
            Some(parent) => node = parent,
            None => break,
        }
    }

    println!("{}", line);
}

fn main() {
    let schedstat =
        // helpers::read_file_string("/proc/schedstat").expect("Error Reading file /proc/schedstat");
        helpers::read_file_string("./schedstat.txt").expect("Error Reading file /proc/schedstat");
    let mut node;

    for line in schedstat.lines() {
        let line_vec: Vec<&str> = line.split(" ").collect();

        // Create cpu base domain
        if line_vec[0].contains("cpu") {
            node = helpers::get_cpu_node(line_vec[0]);
            node.insert_node();
        } else if line_vec[0].contains("domain") {
            node = helpers::get_domain_node(line_vec[0], line_vec[1]);
            node.insert_node();
        }
    }

    let trace = helpers::read_file_string("./tracefile.txt").expect("Error reading trace file");
    for line in trace.lines() {
        if line.starts_with("#") {
            continue;
        }
        if line.contains("sched_update_nr_running") {
            let mut line_vec: Vec<&str> = line.split(" ").collect();
            //Remove empty strings
            line_vec.retain(|&s| s != "");
            let cpu: i16 = line_vec.get(INDEX_CPU).unwrap()[4..].parse().unwrap();
            let nr_running: i64 = line_vec.get(INDEX_NR_RUNNING).unwrap()[11..]
                .parse()
                .unwrap();
            let timestamp = line_vec.get(INDEX_TIMESTAMP).unwrap();
            if let Some(cpu_node) = node::fetch_cpu_with_index(usize::try_from(cpu).unwrap()) {
                cpu_node.update_nr_running_for_cpu(nr_running);
                print_line(cpu_node, timestamp);
            }
        }
    }

    node::print_data();
}
