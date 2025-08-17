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

#[derive(Clone, Debug, PartialEq)]
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

    pub fn get_elements_by_class(&self, class: &str) -> Vec<Node> {
        let mut elements = Vec::new();
        if let NodeType::Element(_element) = &self.node_type {
            if self.get_class_list().contains(&class.to_string()) {
                elements.push(self.clone());
            }
            for child in &self.children {
                elements.extend(child.borrow().get_elements_by_class(class));
            }
        }
        elements
    }

    pub fn get_elements_by_tag(&self, tag: &HtmlElement) -> Vec<Node> {
        let mut elements = Vec::new();
        if let NodeType::Element(element) = &self.node_type {
            if element == tag {
                elements.push(self.clone());
            }
        }
        for child in &self.children {
            elements.extend(child.borrow().get_elements_by_tag(tag));
        }

        elements
    }

    pub fn get_class_list(&self) -> Vec<String> {
        let mut classes = vec![];
        if let Some(class_list) = self.properties.get("class") {
            classes = class_list
                .split_whitespace()
                .map(|v| v.to_string())
                .collect();
        }
        classes
    }

    pub fn get_element_by_id(&self, id: &str) -> Option<Node> {
        if let NodeType::Element(_element) = &self.node_type
            && self.properties.get("id") == Some(&id.to_string())
        {
            return Some(self.clone());
        }

        for child in &self.children {
            if let Some(node) = child.borrow().get_element_by_id(id) {
                return Some(node);
            }
        }

        None
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
                        parent_element: Some(Rc::downgrade(parent_element)),
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
                        parent_element: Some(Rc::downgrade(parent_element)),
                        children: vec![],
                        properties: HashMap::new(),
                    }));
                    parent_element.borrow_mut().children.push(new_node.clone());
                }
                TokenType::Text => {
                    let new_node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Text(token.get_text()),
                        parent_element: Some(Rc::downgrade(parent_element)),
                        children: vec![],
                        properties: HashMap::new(),
                    }));
                    parent_element.borrow_mut().children.push(new_node.clone());
                }
                TokenType::Comment => {
                    let new_node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Comment(token.get_text()),
                        parent_element: Some(Rc::downgrade(parent_element)),
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
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use crate::tokeniser::get_tokens;

    #[cfg(test)]
    const TEST: &str = r##"<html><head><title>Test</title></head><body><p id="some-paragraph">Hello, world!</p><div id='classy' class='bg-red p-10 primary'>This is a div with a few classes</div></body></html>"##;

    #[test]
    fn try_walk_tree() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        document.borrow().walk_tree();
    }

    #[test]
    fn check_element_by_id() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let element = document
            .borrow()
            .get_element_by_id("some-paragraph")
            .unwrap();

        assert_eq!(element.node_type, NodeType::Element(HtmlElement::P))
    }

    #[test]
    fn check_get_classes() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let element = document
            .borrow()
            .get_element_by_id("classy")
            .unwrap()
            .get_class_list();

        assert_eq!(
            element,
            vec![
                "bg-red".to_string(),
                "p-10".to_string(),
                "primary".to_string()
            ]
        );
    }
}
