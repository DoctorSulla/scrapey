use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Node {
    parent_element: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    properties: HashMap<String, String>,
}

// TODO
