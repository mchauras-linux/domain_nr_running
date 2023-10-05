mod helpers;
mod domain;

fn main() {
    let schedstat = helpers::read_file_string("/proc/schedstat").expect("Error Reading file /proc/schedstat");
    println!("schedstat: {schedstat}");
}
