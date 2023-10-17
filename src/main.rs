extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use]
extern crate serde_derive;

mod domain;
mod helpers;
mod node;

fn main() {
    let schedstat =
        // helpers::read_file_string("/proc/schedstat").expect("Error Reading file /proc/schedstat");
        helpers::read_file_string("./schedstat.txt").expect("Error Reading file /proc/schedstat");
    let mut cpu_node;

    for line in schedstat.lines() {
        let line_vec: Vec<&str> = line.split(" ").collect();

        // Create cpu base domain
        if line_vec[0].contains("cpu") {
            cpu_node = helpers::get_cpu_node(line_vec[0]);
        // Create Domain heirarchy
        } else if line_vec[0].contains("domain") {
            helpers::get_domain_node(line_vec[0], line_vec[1]);
        }
    }

    node::write_sched_domain();
}
