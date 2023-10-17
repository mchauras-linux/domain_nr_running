use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

use crate::domain::Domain;

const MAX_LEVELS: usize = 10;

static mut ROOT: Option<Node> = None;

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

    fn is_cpu_node(&self) -> bool {
        let domain = self.node.lock().unwrap();
        if domain.id == -1 {
            return true;
        }
        false
    }

    pub(crate) fn get_child(&self) -> Option<Node> {
        self.node.lock().unwrap().child.clone()
    }

    pub(crate) fn set_child(&self, child: Option<Node>) {
        self.node.lock().unwrap().child = child;
    }

    pub(crate) fn get_parent(&self) -> Option<Node> {
        self.node.lock().unwrap().parent.clone()
    }

    pub(crate) fn set_parent(&self, parent: Option<Node>) {
        self.node.lock().unwrap().parent = parent;
    }

    pub(crate) fn get_sibling(&self) -> Option<Node> {
        self.node.lock().unwrap().sibling.clone()
    }

    pub(crate) fn set_sibling(&self, sibling: Option<Node>) {
        self.node.lock().unwrap().sibling = sibling;
    }

    pub(crate) fn get_level(&self) -> i8 {
        self.node.lock().unwrap().level
    }

    pub(crate) fn set_level(&self, level: i8) {
        self.node.lock().unwrap().level = level;
    }

    fn get_sibling_node_for_span(&self, node: &Node) -> Option<Node> {
        todo!()
    }

    fn insert_node(&self, node: Node) {
        let mut child_iter = Some(self.clone());
        let mut prev_child = Some(self.clone());
        let mut sibling_iter = Some(self.clone());
        if node.is_cpu_node() {
            'main: while !child_iter.is_none() {
                prev_child = child_iter.clone();
                if let Some(child_node) = child_iter {
                    // Check if the level is same
                    child_iter = child_node.get_sibling_node_for_span(&node);
                    // While Condition increment
                    child_iter = child_node.get_child();
                }
            }
            if let Some(iter) = prev_child.clone() {
                if iter.get_child().is_none() {
                    iter.set_child(Some(node.clone()));
                }
            }
        }
        println!("{:#X?}", node);
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
