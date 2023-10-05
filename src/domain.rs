use std::rc::Rc;

#[derive(Debug)]
struct Domain {
    nr_running: u64,
    level: u8,
    parent: Rc<Domain>,
    sibling: Rc<Domain>,
    child: Rc<Domain>,
}

impl Domain {
    
}
