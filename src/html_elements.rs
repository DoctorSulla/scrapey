/// Comprehensive enum containing all standard HTML elements plus an Unknown variant
/// for handling non-standard or unrecognized elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HtmlElement {
    // Document structure
    Html,
    Head,
    Body,
    Title,
    Meta,
    Link,
    Style,
    Script,
    Base,

    // Sectioning elements
    Article,
    Section,
    Nav,
    Aside,
    Header,
    Footer,
    Main,
    Hgroup,

    // Headings
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,

    // Text content
    P,
    Hr,
    Pre,
    Blockquote,
    Ol,
    Ul,
    Li,
    Dl,
    Dt,
    Dd,
    Figure,
    Figcaption,
    Div,

    // Inline text semantics
    A,
    Em,
    Strong,
    Small,
    S,
    Cite,
    Q,
    Dfn,
    Abbr,
    Ruby,
    Rt,
    Rp,
    Data,
    Time,
    Code,
    Var,
    Samp,
    Kbd,
    Sub,
    Sup,
    I,
    B,
    U,
    Mark,
    Bdi,
    Bdo,
    Span,
    Br,
    Wbr,

    // Image and multimedia
    Picture,
    Source,
    Img,
    Svg,
    Math,
    Audio,
    Video,
    Track,
    Map,
    Area,

    // Embedded content
    Iframe,
    Embed,
    Object,
    Param,

    // Scripting
    Canvas,
    Noscript,

    // Demarcating edits
    Del,
    Ins,

    // Table content
    Table,
    Caption,
    Colgroup,
    Col,
    Tbody,
    Thead,
    Tfoot,
    Tr,
    Td,
    Th,

    // Forms
    Form,
    Label,
    Input,
    Button,
    Select,
    Datalist,
    Optgroup,
    Option,
    Textarea,
    Output,
    Progress,
    Meter,
    Fieldset,
    Legend,

    // Interactive elements
    Details,
    Summary,
    Dialog,

    // Web Components
    Template,
    Slot,

    // Obsolete elements (HTML5 deprecated but still may appear)
    Acronym,
    Applet,
    Basefont,
    Big,
    Center,
    Dir,
    Font,
    Frame,
    Frameset,
    Noframes,
    Strike,
    Tt,

    // Custom or unrecognized elements
    Unknown(String),
}

impl HtmlElement {
    /// Creates an HtmlElement from a string tag name
    pub fn from_tag_name(tag: &str) -> Self {
        match tag.to_lowercase().as_str() {
            // Document structure
            "html" => Self::Html,
            "head" => Self::Head,
            "body" => Self::Body,
            "title" => Self::Title,
            "meta" => Self::Meta,
            "link" => Self::Link,
            "style" => Self::Style,
            "script" => Self::Script,
            "base" => Self::Base,

            // Sectioning elements
            "article" => Self::Article,
            "section" => Self::Section,
            "nav" => Self::Nav,
            "aside" => Self::Aside,
            "header" => Self::Header,
            "footer" => Self::Footer,
            "main" => Self::Main,
            "hgroup" => Self::Hgroup,

            // Headings
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            "h4" => Self::H4,
            "h5" => Self::H5,
            "h6" => Self::H6,

            // Text content
            "p" => Self::P,
            "hr" => Self::Hr,
            "pre" => Self::Pre,
            "blockquote" => Self::Blockquote,
            "ol" => Self::Ol,
            "ul" => Self::Ul,
            "li" => Self::Li,
            "dl" => Self::Dl,
            "dt" => Self::Dt,
            "dd" => Self::Dd,
            "figure" => Self::Figure,
            "figcaption" => Self::Figcaption,
            "div" => Self::Div,

            // Inline text semantics
            "a" => Self::A,
            "em" => Self::Em,
            "strong" => Self::Strong,
            "small" => Self::Small,
            "s" => Self::S,
            "cite" => Self::Cite,
            "q" => Self::Q,
            "dfn" => Self::Dfn,
            "abbr" => Self::Abbr,
            "ruby" => Self::Ruby,
            "rt" => Self::Rt,
            "rp" => Self::Rp,
            "data" => Self::Data,
            "time" => Self::Time,
            "code" => Self::Code,
            "var" => Self::Var,
            "samp" => Self::Samp,
            "kbd" => Self::Kbd,
            "sub" => Self::Sub,
            "sup" => Self::Sup,
            "i" => Self::I,
            "b" => Self::B,
            "u" => Self::U,
            "mark" => Self::Mark,
            "bdi" => Self::Bdi,
            "bdo" => Self::Bdo,
            "span" => Self::Span,
            "br" => Self::Br,
            "wbr" => Self::Wbr,

            // Image and multimedia
            "picture" => Self::Picture,
            "source" => Self::Source,
            "img" => Self::Img,
            "svg" => Self::Svg,
            "math" => Self::Math,
            "audio" => Self::Audio,
            "video" => Self::Video,
            "track" => Self::Track,
            "map" => Self::Map,
            "area" => Self::Area,

            // Embedded content
            "iframe" => Self::Iframe,
            "embed" => Self::Embed,
            "object" => Self::Object,
            "param" => Self::Param,

            // Scripting
            "canvas" => Self::Canvas,
            "noscript" => Self::Noscript,

            // Demarcating edits
            "del" => Self::Del,
            "ins" => Self::Ins,

            // Table content
            "table" => Self::Table,
            "caption" => Self::Caption,
            "colgroup" => Self::Colgroup,
            "col" => Self::Col,
            "tbody" => Self::Tbody,
            "thead" => Self::Thead,
            "tfoot" => Self::Tfoot,
            "tr" => Self::Tr,
            "td" => Self::Td,
            "th" => Self::Th,

            // Forms
            "form" => Self::Form,
            "label" => Self::Label,
            "input" => Self::Input,
            "button" => Self::Button,
            "select" => Self::Select,
            "datalist" => Self::Datalist,
            "optgroup" => Self::Optgroup,
            "option" => Self::Option,
            "textarea" => Self::Textarea,
            "output" => Self::Output,
            "progress" => Self::Progress,
            "meter" => Self::Meter,
            "fieldset" => Self::Fieldset,
            "legend" => Self::Legend,

            // Interactive elements
            "details" => Self::Details,
            "summary" => Self::Summary,
            "dialog" => Self::Dialog,

            // Web Components
            "template" => Self::Template,
            "slot" => Self::Slot,

            // Obsolete elements
            "acronym" => Self::Acronym,
            "applet" => Self::Applet,
            "basefont" => Self::Basefont,
            "big" => Self::Big,
            "center" => Self::Center,
            "dir" => Self::Dir,
            "font" => Self::Font,
            "frame" => Self::Frame,
            "frameset" => Self::Frameset,
            "noframes" => Self::Noframes,
            "strike" => Self::Strike,
            "tt" => Self::Tt,

            // Unknown element
            _ => Self::Unknown(tag.to_string()),
        }
    }

    /// Returns the tag name as a string
    pub fn tag_name(&self) -> &str {
        match self {
            // Document structure
            Self::Html => "html",
            Self::Head => "head",
            Self::Body => "body",
            Self::Title => "title",
            Self::Meta => "meta",
            Self::Link => "link",
            Self::Style => "style",
            Self::Script => "script",
            Self::Base => "base",

            // Sectioning elements
            Self::Article => "article",
            Self::Section => "section",
            Self::Nav => "nav",
            Self::Aside => "aside",
            Self::Header => "header",
            Self::Footer => "footer",
            Self::Main => "main",
            Self::Hgroup => "hgroup",

            // Headings
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",

            // Text content
            Self::P => "p",
            Self::Hr => "hr",
            Self::Pre => "pre",
            Self::Blockquote => "blockquote",
            Self::Ol => "ol",
            Self::Ul => "ul",
            Self::Li => "li",
            Self::Dl => "dl",
            Self::Dt => "dt",
            Self::Dd => "dd",
            Self::Figure => "figure",
            Self::Figcaption => "figcaption",
            Self::Div => "div",

            // Inline text semantics
            Self::A => "a",
            Self::Em => "em",
            Self::Strong => "strong",
            Self::Small => "small",
            Self::S => "s",
            Self::Cite => "cite",
            Self::Q => "q",
            Self::Dfn => "dfn",
            Self::Abbr => "abbr",
            Self::Ruby => "ruby",
            Self::Rt => "rt",
            Self::Rp => "rp",
            Self::Data => "data",
            Self::Time => "time",
            Self::Code => "code",
            Self::Var => "var",
            Self::Samp => "samp",
            Self::Kbd => "kbd",
            Self::Sub => "sub",
            Self::Sup => "sup",
            Self::I => "i",
            Self::B => "b",
            Self::U => "u",
            Self::Mark => "mark",
            Self::Bdi => "bdi",
            Self::Bdo => "bdo",
            Self::Span => "span",
            Self::Br => "br",
            Self::Wbr => "wbr",

            // Image and multimedia
            Self::Picture => "picture",
            Self::Source => "source",
            Self::Img => "img",
            Self::Svg => "svg",
            Self::Math => "math",
            Self::Audio => "audio",
            Self::Video => "video",
            Self::Track => "track",
            Self::Map => "map",
            Self::Area => "area",

            // Embedded content
            Self::Iframe => "iframe",
            Self::Embed => "embed",
            Self::Object => "object",
            Self::Param => "param",

            // Scripting
            Self::Canvas => "canvas",
            Self::Noscript => "noscript",

            // Demarcating edits
            Self::Del => "del",
            Self::Ins => "ins",

            // Table content
            Self::Table => "table",
            Self::Caption => "caption",
            Self::Colgroup => "colgroup",
            Self::Col => "col",
            Self::Tbody => "tbody",
            Self::Thead => "thead",
            Self::Tfoot => "tfoot",
            Self::Tr => "tr",
            Self::Td => "td",
            Self::Th => "th",

            // Forms
            Self::Form => "form",
            Self::Label => "label",
            Self::Input => "input",
            Self::Button => "button",
            Self::Select => "select",
            Self::Datalist => "datalist",
            Self::Optgroup => "optgroup",
            Self::Option => "option",
            Self::Textarea => "textarea",
            Self::Output => "output",
            Self::Progress => "progress",
            Self::Meter => "meter",
            Self::Fieldset => "fieldset",
            Self::Legend => "legend",

            // Interactive elements
            Self::Details => "details",
            Self::Summary => "summary",
            Self::Dialog => "dialog",

            // Web Components
            Self::Template => "template",
            Self::Slot => "slot",

            // Obsolete elements
            Self::Acronym => "acronym",
            Self::Applet => "applet",
            Self::Basefont => "basefont",
            Self::Big => "big",
            Self::Center => "center",
            Self::Dir => "dir",
            Self::Font => "font",
            Self::Frame => "frame",
            Self::Frameset => "frameset",
            Self::Noframes => "noframes",
            Self::Strike => "strike",
            Self::Tt => "tt",

            // Unknown element
            Self::Unknown(tag) => tag,
        }
    }

    /// Checks if the element is a void element (self-closing)
    pub fn is_void_element(&self) -> bool {
        matches!(
            self,
            Self::Area
                | Self::Base
                | Self::Br
                | Self::Col
                | Self::Embed
                | Self::Hr
                | Self::Img
                | Self::Input
                | Self::Link
                | Self::Meta
                | Self::Param
                | Self::Source
                | Self::Track
                | Self::Wbr
        )
    }

    /// Checks if the element is obsolete/deprecated
    pub fn is_obsolete(&self) -> bool {
        matches!(
            self,
            Self::Acronym
                | Self::Applet
                | Self::Basefont
                | Self::Big
                | Self::Center
                | Self::Dir
                | Self::Font
                | Self::Frame
                | Self::Frameset
                | Self::Noframes
                | Self::Strike
                | Self::Tt
        )
    }

    /// Checks if the element is a sectioning element
    pub fn is_sectioning(&self) -> bool {
        matches!(
            self,
            Self::Article | Self::Section | Self::Nav | Self::Aside
        )
    }

    /// Checks if the element is a heading element
    pub fn is_heading(&self) -> bool {
        matches!(
            self,
            Self::H1 | Self::H2 | Self::H3 | Self::H4 | Self::H5 | Self::H6
        )
    }

    /// Checks if the element is a form-related element
    pub fn is_form_element(&self) -> bool {
        matches!(
            self,
            Self::Form
                | Self::Input
                | Self::Button
                | Self::Select
                | Self::Textarea
                | Self::Label
                | Self::Fieldset
                | Self::Legend
                | Self::Optgroup
                | Self::Option
                | Self::Datalist
                | Self::Output
                | Self::Progress
                | Self::Meter
        )
    }

    /// Checks if the element is a table-related element
    pub fn is_table_element(&self) -> bool {
        matches!(
            self,
            Self::Table
                | Self::Caption
                | Self::Colgroup
                | Self::Col
                | Self::Tbody
                | Self::Thead
                | Self::Tfoot
                | Self::Tr
                | Self::Td
                | Self::Th
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tag_name_standard_elements() {
        assert_eq!(HtmlElement::from_tag_name("div"), HtmlElement::Div);
        assert_eq!(HtmlElement::from_tag_name("DIV"), HtmlElement::Div);
        assert_eq!(HtmlElement::from_tag_name("p"), HtmlElement::P);
        assert_eq!(HtmlElement::from_tag_name("html"), HtmlElement::Html);
    }

    #[test]
    fn test_from_tag_name_unknown_element() {
        match HtmlElement::from_tag_name("custom-element") {
            HtmlElement::Unknown(tag) => assert_eq!(tag, "custom-element"),
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_tag_name_method() {
        assert_eq!(HtmlElement::Div.tag_name(), "div");
        assert_eq!(HtmlElement::Html.tag_name(), "html");
        assert_eq!(
            HtmlElement::Unknown("custom".to_string()).tag_name(),
            "custom"
        );
    }

    #[test]
    fn test_void_elements() {
        assert!(HtmlElement::Br.is_void_element());
        assert!(HtmlElement::Hr.is_void_element());
        assert!(HtmlElement::Img.is_void_element());
        assert!(!HtmlElement::Div.is_void_element());
        assert!(!HtmlElement::P.is_void_element());
    }

    #[test]
    fn test_obsolete_elements() {
        assert!(HtmlElement::Center.is_obsolete());
        assert!(HtmlElement::Font.is_obsolete());
        assert!(!HtmlElement::Div.is_obsolete());
        assert!(!HtmlElement::Span.is_obsolete());
    }

    #[test]
    fn test_sectioning_elements() {
        assert!(HtmlElement::Article.is_sectioning());
        assert!(HtmlElement::Section.is_sectioning());
        assert!(HtmlElement::Nav.is_sectioning());
        assert!(!HtmlElement::Div.is_sectioning());
    }

    #[test]
    fn test_heading_elements() {
        assert!(HtmlElement::H1.is_heading());
        assert!(HtmlElement::H6.is_heading());
        assert!(!HtmlElement::P.is_heading());
    }

    #[test]
    fn test_form_elements() {
        assert!(HtmlElement::Form.is_form_element());
        assert!(HtmlElement::Input.is_form_element());
        assert!(HtmlElement::Button.is_form_element());
        assert!(!HtmlElement::Div.is_form_element());
    }

    #[test]
    fn test_table_elements() {
        assert!(HtmlElement::Table.is_table_element());
        assert!(HtmlElement::Tr.is_table_element());
        assert!(HtmlElement::Td.is_table_element());
        assert!(!HtmlElement::Div.is_table_element());
    }
}
