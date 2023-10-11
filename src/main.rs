extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

mod domain;
mod helpers;

fn main() {
    let schedstat =
        // helpers::read_file_string("/proc/schedstat").expect("Error Reading file /proc/schedstat");
        helpers::read_file_string("./schedstat.txt").expect("Error Reading file /proc/schedstat");

    for line in schedstat.lines() {
        let line_vec: Vec<&str> = line.split(" ").collect();

        // Create cpu base domain
        if line_vec[0].contains("cpu") {
            helpers::get_cpu_node(line_vec[0]);
        // Create Domain heirarchy
        } else if line_vec[0].contains("domain") {
            helpers::get_domain_node(line_vec[0], line_vec[1])
        }
    }

    domain::write_sched_domain();
}
