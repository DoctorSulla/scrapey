use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    html_elements::HtmlElement,
    tokeniser::{TokenStream, TokenType},
};

type NodeRef = Rc<RefCell<Node>>;

#[derive(Clone, Debug)]
pub enum NodeType {
    Document,
    Element(HtmlElement),
    Text(String),
    Comment(String),
}

#[derive(Clone, Debug)]
pub struct Node {
    node_type: NodeType,
    parent_element: Option<Weak<RefCell<Node>>>,
    children: Vec<NodeRef>,
    properties: HashMap<String, String>,
}

impl Node {
    pub fn walk_tree(&self) {
        match &self.node_type {
            NodeType::Document => println!("Document"),
            NodeType::Element(element) => println!("Element: {:?}", element),
            NodeType::Text(text) => println!("Text: {}", text),
            NodeType::Comment(comment) => println!("Comment: {}", comment),
        }

        for child in &self.children {
            child.borrow().walk_tree();
        }
    }

    pub fn from_token_stream(token_stream: TokenStream) -> NodeRef {
        let root = Rc::new(RefCell::new(Node {
            node_type: NodeType::Document,
            parent_element: None,
            children: vec![],
            properties: HashMap::new(),
        }));

        let mut open_tags: Vec<NodeRef> = vec![];

        for token in token_stream.into_iter() {
            let parent_element = open_tags.last().unwrap_or(&root);
            match token.get_token_type() {
                TokenType::OpeningTag => {
                    let new_node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Element(token.get_html_element().unwrap()),
                        parent_element: Some(Rc::downgrade(&parent_element)),
                        children: vec![],
                        properties: token.get_properties(),
                    }));
                    parent_element.borrow_mut().children.push(new_node.clone());
                }
                TokenType::ClosingTag => {
                    open_tags.pop();
                }
                TokenType::VoidTag => {
                    let new_node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Element(token.get_html_element().unwrap()),
                        parent_element: Some(Rc::downgrade(&parent_element)),
                        children: vec![],
                        properties: HashMap::new(),
                    }));
                    parent_element.borrow_mut().children.push(new_node.clone());
                }
                TokenType::Text => {
                    let new_node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Text(token.get_text()),
                        parent_element: Some(Rc::downgrade(&parent_element)),
                        children: vec![],
                        properties: HashMap::new(),
                    }));
                    parent_element.borrow_mut().children.push(new_node.clone());
                }
                TokenType::Comment => {
                    let new_node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Comment(token.get_text()),
                        parent_element: Some(Rc::downgrade(&parent_element)),
                        children: vec![],
                        properties: token.get_properties(),
                    }));
                    parent_element.borrow_mut().children.push(new_node.clone());
                }
                TokenType::Unknown => {} // TODO
            }
        }
        root
    }
}

mod tests {
    use super::*;
    use crate::tokeniser::get_tokens;

    const TEST: &str =
        "<html><head><title>Test</title></head><body><p>Hello, world!</p></body></html>";

    #[test]
    fn try_walk_tree() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        document.borrow().walk_tree();
    }
}
