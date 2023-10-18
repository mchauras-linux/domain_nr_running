use std::sync::{Arc, Mutex};

use crate::node::Node;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Domain {
    //Cpu data
    pub(crate) id: i16,

    // Domain data
    pub(crate) nr_running: u64,
    pub(crate) level: i8,
    pub(crate) span: Vec<u32>,
    pub(crate) parent: Option<Node>,
}

impl Domain {
    pub(crate) fn new(level: i8, span: Vec<u32>, cpu: i16) -> Arc<Mutex<Domain>> {
        Arc::new(Mutex::new(Domain {
            id: cpu,
            nr_running: 0,
            level,
            span,
            parent: None,
        }))
    }
}
