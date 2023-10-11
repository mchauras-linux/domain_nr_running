use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

const MAX_LEVELS: usize = 10;
pub(crate) static mut ROOT: Option<Node> = None;

lazy_static! {
    static ref TEMP_NODES: Vec<Option<Node>> = vec![None; MAX_LEVELS];

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Node {
    node: Arc<Mutex<Domain>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Domain {
    //Cpu data
    id: i16,

    // Domain data
    nr_running: u64,
    level: i8,
    span: Vec<u32>,
    parent: Option<Node>,
    sibling: Option<Node>,
    child: Option<Node>,
}

impl Domain {
    pub(crate) fn new(level: i8, span: Vec<u32>, cpu: i16) -> Arc<Mutex<Domain>> {
        Arc::new(Mutex::new(Domain {
            id: cpu,
            nr_running: 0,
            level,
            span,
            parent: None,
            sibling: None,
            child: None,
        }))
    }
}

impl Node {
    pub(crate) fn new(level: i8, span: Vec<u32>, cpu: i16) -> Self {
        Self {
            node: Domain::new(level, span, cpu),
        }
    }

    fn is_cpu_node(&self) -> bool {
        let domain = self.node.lock().unwrap();
        if domain.id == -1 {
            return true;
        }
        false
    }

    fn insert_node(&self, node: Node) {
        if node.is_cpu_node() {

        }
        println!("{:#X?}", node)
    }
}

pub(crate) fn insert_node(node: Node) {
    match unsafe { &ROOT } {
        Some(root) => root.insert_node(node),
        None => unsafe { ROOT = Some(node) },
    }
}

pub(crate) fn write_sched_domain() {
    let file = File::create("sched_data.json").unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, unsafe { &ROOT }).unwrap();
}
