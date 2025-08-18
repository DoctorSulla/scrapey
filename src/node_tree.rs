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

impl From<&Node> for String {
    fn from(node: &Node) -> Self {
        match &node.node_type {
            NodeType::Text(text) => text.clone(),
            NodeType::Element(element) => {
                if node.properties.is_empty() {
                    return format!("<{}>", element.tag_name());
                }
                let properties = node
                    .properties
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect::<Vec<String>>()
                    .join(" ");
                let string_repr = format!("<{} {}>", element.tag_name(), properties);
                string_repr
            }
            NodeType::Document => "Document".to_string(),
            NodeType::Comment(comment) => comment.to_string(),
        }
    }
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
        }

        for child in &self.children {
            elements.extend(child.borrow().get_elements_by_class(class));
        }
        elements
    }

    pub fn get_string_repr(&self) -> String {
        let mut html = String::new();
        html.push_str(String::from(self).as_str());
        for child in &self.children {
            html.push_str(child.borrow().get_string_repr().as_str());
        }
        html
    }

    pub fn outer_html(&self) -> Option<String> {
        let html = self.get_string_repr();
        if let NodeType::Element(element) = &self.node_type {
            if element.is_void_element() {
                return Some(html);
            }
            return Some(format!("{}</{}>", html, element.tag_name()));
        }
        None
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
                    open_tags.push(new_node);
                }
                TokenType::ClosingTag => {
                    open_tags.pop();
                }
                TokenType::VoidTag => {
                    let new_node = Rc::new(RefCell::new(Node {
                        node_type: NodeType::Element(token.get_html_element().unwrap()),
                        parent_element: Some(Rc::downgrade(parent_element)),
                        children: vec![],
                        properties: token.get_properties(),
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
    const TEST: &str = r##"<html><head><title id="hmm">Test</title><br /></head><body><p id="some-paragraph">Hello, world!</p><div id='classy' class='bg-red p-10 primary'>This is a div with a few classes</div>
    <div class='bg-red'>This is another div with the same class</div><footer>No props footer</footer><hr class="thicc"/></body></html>"##;

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

    #[test]
    fn check_get_elements_by_class() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let elements = document.borrow().get_elements_by_class("bg-red");

        assert_eq!(elements.len(), 2);
    }

    #[test]
    fn check_get_elements_by_tag() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let elements = document.borrow().get_elements_by_tag(&HtmlElement::Div);

        assert_eq!(elements.len(), 2);
    }

    #[test]
    fn check_outer_html() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let element = document.borrow().get_element_by_id("hmm").unwrap();

        assert_eq!(
            element.outer_html().unwrap(),
            *"<title id=\"hmm\">Test</title>"
        );
    }

    #[test]
    fn check_no_props_outer_html() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let footers = document.borrow().get_elements_by_tag(&HtmlElement::Footer);

        assert_eq!(
            footers[0].outer_html().unwrap(),
            *"<footer>No props footer</footer>"
        );
    }

    #[test]
    fn check_void_tag_outer_html() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let brs = document.borrow().get_elements_by_tag(&HtmlElement::Br);

        assert_eq!(brs[0].outer_html().unwrap(), *"<br>");
    }

    #[test]
    fn check_void_tag_with_props_outer_html() {
        let tokens = get_tokens(TEST);
        let document = Node::from_token_stream(tokens);
        let hrs = document.borrow().get_elements_by_tag(&HtmlElement::Hr);

        assert_eq!(hrs[0].outer_html().unwrap(), *"<hr class=\"thicc\">");
    }
}
