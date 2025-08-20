use std::{collections::HashMap, str::Chars};

use crate::html_elements::HtmlElement;

pub type TokenStream = Vec<Token>;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    OpeningTag,
    ClosingTag,
    VoidTag,
    Comment,
    Text,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub enum AttrState {
    CapturingKey,
    CapturingValue,
    CapturingWrapper,
    WhiteSpace,
}

#[derive(PartialEq, Debug)]
pub enum ParsingState {
    CapturingTag,
    CapturingText,
    DeterminingTokenType,
    CapturingRawText,
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    token_value: String,
    token_element: Option<HtmlElement>,
    properties: HashMap<String, String>,
}

#[derive(Debug)]
pub struct AppState {
    parsing_state: ParsingState,
    current_token: Token,
    token_stream: TokenStream,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            parsing_state: ParsingState::DeterminingTokenType,
            current_token: Token {
                token_type: TokenType::Unknown,
                token_value: String::new(),
                token_element: None,
                properties: HashMap::new(),
            },
            token_stream: vec![],
        }
    }

    fn capturing_tag_transition(&mut self, chars: &mut std::iter::Peekable<Chars>) {
        if chars.peek() == Some(&'!') {
            self.current_token.token_type = TokenType::Comment;
        } else if chars.peek() == Some(&'/') {
            self.current_token.token_type = TokenType::ClosingTag;
        } else {
            self.current_token.token_type = TokenType::OpeningTag;
        }
        self.parsing_state = ParsingState::CapturingTag;
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::new()
    }
}

impl Token {
    pub fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_html_element(&self) -> Option<HtmlElement> {
        self.token_element.clone()
    }

    pub fn get_text(&self) -> String {
        self.token_value.clone()
    }

    pub fn get_properties(&self) -> HashMap<String, String> {
        self.properties.clone()
    }

    fn set_tag_and_properties(&mut self) {
        let mut tag_content = self.token_value[1..&self.token_value.len() - 1].trim();
        if tag_content.ends_with('/') {
            tag_content = &tag_content[..tag_content.len() - 1];
        }

        if tag_content.starts_with('/') {
            tag_content = &tag_content[1..];
        }
        match tag_content.find(' ') {
            Some(index) => {
                let tag = &tag_content[0..index];
                let properties = &tag_content[index + 1..];
                self.token_element = Some(HtmlElement::from_tag_name(tag));
                self.properties = parse_properties(properties);
            }
            None => self.token_element = Some(HtmlElement::from_tag_name(tag_content)),
        }

        if self.token_element.as_ref().unwrap().is_void_element() {
            self.token_type = TokenType::VoidTag;
        }
    }
}

fn parse_properties(properties: &str) -> HashMap<String, String> {
    let mut properties_map = HashMap::new();
    let mut wrapper: Option<char> = None;
    let mut key_buffer = String::new();
    let mut value_buffer = String::new();
    let mut state = AttrState::CapturingKey;

    let mut chars = properties.chars().peekable();
    while let Some(char) = chars.next() {
        match state {
            AttrState::CapturingKey => {
                if char == '=' {
                    state = AttrState::CapturingWrapper;
                } else if char.is_whitespace() {
                    // Continue until we find a non-whitespace character
                    for c in chars.by_ref() {
                        if !c.is_whitespace() {
                            if c == '=' {
                                state = AttrState::CapturingWrapper;
                            } else {
                                properties_map.insert(key_buffer.clone(), "".to_string());
                                key_buffer.clear();
                                value_buffer.clear();
                                state = AttrState::WhiteSpace;
                            }
                            break;
                        }
                    }
                } else {
                    key_buffer.push(char);
                }
            }
            AttrState::CapturingWrapper => {
                if char == ' ' {
                    continue;
                } else if char == '\'' {
                    wrapper = Some('\'');
                } else if char == '"' {
                    wrapper = Some('"');
                } else {
                    wrapper = None;
                    value_buffer.push(char);
                }
                state = AttrState::CapturingValue
            }
            AttrState::CapturingValue => match wrapper {
                Some(v) => {
                    if char == v {
                        properties_map.insert(key_buffer.clone(), value_buffer.clone());
                        key_buffer.clear();
                        value_buffer.clear();
                        state = AttrState::WhiteSpace;
                    } else {
                        value_buffer.push(char);
                    }
                }
                None => {
                    if char == ' ' {
                        properties_map.insert(key_buffer.clone(), value_buffer.clone());
                        key_buffer.clear();
                        value_buffer.clear();
                        state = AttrState::WhiteSpace;
                    }
                }
            },
            AttrState::WhiteSpace => {
                if char != ' ' {
                    state = AttrState::CapturingKey;
                    key_buffer.push(char)
                }
            }
        }
    }
    if !key_buffer.is_empty() {
        if !value_buffer.is_empty() {
            // We were capturing a value
            properties_map.insert(key_buffer.clone(), value_buffer.clone());
        } else {
            // Boolean attribute
            properties_map.insert(key_buffer.clone(), "".to_string());
        }
    }

    properties_map
}

pub fn get_tokens(html: &str) -> TokenStream {
    let mut app_state = AppState::new();
    let mut chars = html.chars().peekable();

    while let Some(char) = chars.next() {
        match app_state.parsing_state {
            ParsingState::DeterminingTokenType => {
                if char == '<' {
                    app_state.capturing_tag_transition(&mut chars);
                } else {
                    app_state.current_token.token_type = TokenType::Text;
                    app_state.parsing_state = ParsingState::CapturingText;
                }
                app_state.current_token.token_value.push(char);
            }
            ParsingState::CapturingTag => {
                if char == '>' {
                    app_state.current_token.token_value.push(char);
                    if app_state
                        .current_token
                        .token_value
                        .to_lowercase()
                        .starts_with("<script")
                        || app_state
                            .current_token
                            .token_value
                            .to_lowercase()
                            .starts_with("<style")
                    {
                        app_state.parsing_state = ParsingState::CapturingRawText;
                    } else {
                        app_state.parsing_state = ParsingState::DeterminingTokenType;
                        app_state.current_token.set_tag_and_properties();
                        app_state.token_stream.push(app_state.current_token.clone());
                    }
                    app_state.current_token.token_value.clear();
                    app_state.current_token.properties.clear();
                    app_state.current_token.token_type = TokenType::Unknown;
                } else {
                    app_state.current_token.token_value.push(char);
                }
            }
            ParsingState::CapturingText => {
                if char == '<' {
                    if !app_state.current_token.token_value.trim().is_empty() {
                        app_state.token_stream.push(app_state.current_token.clone());
                    }
                    app_state.current_token.token_value = String::from("<");
                    app_state.capturing_tag_transition(&mut chars);
                } else {
                    app_state.current_token.token_value.push(char);
                }
            }
            ParsingState::CapturingRawText => {
                // For now we will just throw away style or script tags
                if app_state
                    .current_token
                    .token_value
                    .to_lowercase()
                    .ends_with("</script")
                    || app_state
                        .current_token
                        .token_value
                        .to_lowercase()
                        .ends_with("</style")
                {
                    app_state.parsing_state = ParsingState::DeterminingTokenType;
                    app_state.current_token.token_value.clear();
                    app_state.current_token.token_type = TokenType::Unknown;
                } else {
                    app_state.current_token.token_value.push(char);
                }
            }
        }
    }
    if !app_state.current_token.token_value.is_empty() {
        app_state.token_stream.push(app_state.current_token.clone());
    }
    app_state.token_stream
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_HTML_DOCUMENT: &'static str = r##"<!DOCTYPE HTML>
    <html>
    <head>
    <title>A</title>
    <script type="text/javascript">
        console.log("Hello World!");
    </script>
    <style type="text/css">
        body {
            background-color: #f0f0f0;
        }
    </style>
    </head>
    <body>
    <img src="random.jpg" height="400" width="300" />
    <input type="text" name="random-input" required />
    <nav class="top-nav">
    <div class="nav-item"><a class="nav-link" href="https://random.com/home">Home</a></div>
    <div class="nav-item"><a class="nav-link" href="https://random.com/about">About</a></div>
    <div class="nav-item"><a class="nav-link" href="https://random.com/contact">Contact</a></div>
    </nav>
    <h1>This is a test title</h1>
    <p class="pt-10">This is a random text document to be used for testing an HTML parser and tokeniser. There is a random <img alt="A cool photo" src="/cool_photo.png"> inserted into the middle of a paragraph.</p>
    </body>
    </html>"##;

    #[test]
    fn check_first_token() {
        let response = get_tokens(BASIC_HTML_DOCUMENT);
        assert_eq!(response[0].token_value, *"<!DOCTYPE HTML>");
        assert_eq!(response[0].token_type, TokenType::Comment);
    }

    #[test]
    fn check_second_token() {
        let response = get_tokens(BASIC_HTML_DOCUMENT);
        assert_eq!(response[1].token_value, *"<html>");
        assert_eq!(response[1].token_type, TokenType::OpeningTag);
        assert_eq!(response[1].token_element, Some(HtmlElement::Html));
    }

    #[test]
    fn check_fourth_token() {
        let response = get_tokens(BASIC_HTML_DOCUMENT);
        assert_eq!(response[3].token_value, *"<title>");
        assert_eq!(response[3].token_type, TokenType::OpeningTag);
    }

    #[test]
    fn check_single_char_text() {
        let response = get_tokens(BASIC_HTML_DOCUMENT);
        assert_eq!(response[4].token_value, *"A");
        assert_eq!(response[4].token_type, TokenType::Text);
    }

    #[test]
    fn check_self_closing_and_properties() {
        let response = get_tokens(BASIC_HTML_DOCUMENT);
        assert_eq!(response[8].token_element, Some(HtmlElement::Img));
        assert_eq!(response[8].token_type, TokenType::VoidTag);
        assert_eq!(
            response[8].properties.get("src").unwrap(),
            &"random.jpg".to_string()
        );
        assert_eq!(
            response[8].properties.get("height").unwrap(),
            &"400".to_string()
        );

        assert_eq!(
            response[8].properties.get("width").unwrap(),
            &"300".to_string()
        );
    }

    #[test]
    fn check_bool_property() {
        let response = get_tokens(BASIC_HTML_DOCUMENT);
        assert!(response[9].properties.get("required").unwrap().is_empty());
    }
}
