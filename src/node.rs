use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

use crate::domain::Domain;

lazy_static! {
    static ref CPUS: Mutex<Vec<Node>> = {
        let cpu = Vec::new();
        Mutex::new(cpu)
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Node {
    node: Arc<Mutex<Domain>>,
}

impl Node {
    pub(crate) fn new(level: i8, span: Vec<u32>, cpu: i16) -> Self {
        Self {
            node: Domain::new(level, span, cpu),
        }
    }

    pub(crate) fn get_parent(&self) -> Option<Node> {
        self.node.lock().unwrap().parent.clone()
    }

    pub(crate) fn set_parent(&self, parent: Option<Node>) {
        self.node.lock().unwrap().parent = parent;
    }

    pub(crate) fn get_level(&self) -> i8 {
        self.node.lock().unwrap().level
    }

    pub(crate) fn get_nr_running(&self) -> i64 {
        self.node.lock().unwrap().nr_running
    }

    fn set_nr_running(&self, nr_running: i64) {
        self.node.lock().unwrap().nr_running = nr_running;
    }

    pub(crate) fn get_id(&self) -> i16 {
        self.node.lock().unwrap().id
    }

    fn is_cpu_node(&self) -> bool {
        let domain = self.node.lock().unwrap();
        if domain.id != -1 {
            return true;
        }
        false
    }

    pub(crate) fn insert_node(&self) {
        // let mut top_parent;
        let mut cpus = CPUS.lock().unwrap();
        if self.is_cpu_node() {
            cpus.insert(usize::try_from(self.get_id()).unwrap(), self.clone());
        } else {
            for i in cpus.iter() {
                if self.is_your_cpu(i) {
                    i.attach_domain(self);
                }
            }
        }
    }

    fn is_your_cpu(&self, cpu: &Node) -> bool {
        let cpu_id = cpu.get_id();
        self.is_in_span(cpu_id)
    }

    fn is_in_span(&self, cpu_id: i16) -> bool {
        let span_len = self.get_span().len();
        let bit: u32 = 1 << (cpu_id % 32);
        let index = span_len - 1 - usize::try_from(cpu_id / 32).unwrap();
        match self.get_span().get(index) {
            Some(val) => {
                if (val & bit) != 0 {
                    return true;
                }
                false
            }
            None => false,
        }
    }

    fn get_span(&self) -> Vec<u32> {
        self.node.lock().unwrap().span.clone()
    }

    fn attach_domain(&self, domain: &Node) {
        let mut cpu: Node = self.clone();

        loop {
            // If the level is same ignore and return
            if cpu.get_level() == domain.get_level() {
                return;
            } else if ((cpu.get_level() + 1) == domain.get_level()) && cpu.get_parent().is_none() {
                cpu.set_parent(Some(domain.clone()));
                return;
            }
            // Loop Control
            match cpu.get_parent() {
                Some(parent) => cpu = parent,
                None => break,
            }
        }
    }

    pub(crate) fn update_nr_running_for_cpu(&self, nr_running: i64) {
        let change = nr_running - self.get_nr_running();
        let mut cpu: Node = self.clone();
        if !self.is_cpu_node() {
            panic!("Not a cpu Node");
        }

        loop {
            cpu.set_nr_running(cpu.get_nr_running() + change);
            // Loop Control
            match cpu.get_parent() {
                Some(parent) => cpu = parent,
                None => break,
            }
        }
    }
}

pub(crate) fn get_domain_node_for_span(cpu_span: &Vec<u32>) -> Option<Node> {
    let cpus = get_cpus_for_span(cpu_span);
    for cpu in cpus {
        let mut iter = cpu;
        'parent_iter: loop {
            if *cpu_span == iter.get_span() {
                return Some(iter.clone());
            }
            // Loop Control
            match iter.get_parent() {
                Some(parent) => iter = parent,
                None => break 'parent_iter,
            }
        }
    }
    None
}

fn get_cpus_for_span(cpu_span: &Vec<u32>) -> Vec<Node> {
    let mut cpus = Vec::new();
    let mut cpu_bit: u32;
    let mut cpu: usize = 0;
    for mask in cpu_span.iter().rev() {
        if *mask == 0 {
            continue;
        }
        cpu_bit = 1;
        for i in 0..32 {
            if (cpu_bit << i) & mask != 0 {
                if let Some(cpu_node) = fetch_cpu_with_index(cpu + usize::try_from(i).unwrap()) {
                    cpus.push(cpu_node);
                }
            }
        }
        cpu += 32;
    }
    cpus
}

pub(crate) fn fetch_cpu_with_index(index: usize) -> Option<Node> {
    match CPUS.lock().unwrap().get(index) {
        Some(node) => Some(node.clone()),
        None => None,
    }
}

pub(crate) fn print_data() {
    let file = File::create("sched_data.json").unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &CPUS.lock().unwrap().clone()).unwrap();
    // println!("{:#X?}", &CPUS.lock().unwrap());
}
