use html_parser::html_elements::HtmlElement;

fn main() {
    // Example 1: Creating elements from tag names
    println!("=== Creating elements from tag names ===");
    let div_element = HtmlElement::from_tag_name("div");
    let custom_element = HtmlElement::from_tag_name("my-custom-component");
    let img_element = HtmlElement::from_tag_name("img");

    println!("div element: {:?}", div_element);
    println!("custom element: {:?}", custom_element);
    println!("img element: {:?}", img_element);

    // Example 2: Getting tag names back
    println!("\n=== Getting tag names ===");
    println!("div tag name: {}", div_element.tag_name());
    println!("custom tag name: {}", custom_element.tag_name());
    println!("img tag name: {}", img_element.tag_name());

    // Example 3: Checking element properties
    println!("\n=== Element properties ===");
    let elements = vec![
        HtmlElement::Div,
        HtmlElement::Br,
        HtmlElement::Form,
        HtmlElement::H1,
        HtmlElement::Table,
        HtmlElement::Center,
        HtmlElement::Article,
        HtmlElement::Unknown("web-component".to_string()),
    ];

    for element in elements {
        println!("\nElement: {}", element.tag_name());
        println!("  Is void: {}", element.is_void_element());
        println!("  Is obsolete: {}", element.is_obsolete());
        println!("  Is sectioning: {}", element.is_sectioning());
        println!("  Is heading: {}", element.is_heading());
        println!("  Is form element: {}", element.is_form_element());
        println!("  Is table element: {}", element.is_table_element());
    }

    // Example 4: Working with unknown elements
    println!("\n=== Unknown elements ===");
    let unknown_tags = vec!["my-widget", "x-custom", "polymer-element"];

    for tag in unknown_tags {
        let element = HtmlElement::from_tag_name(tag);
        match element {
            HtmlElement::Unknown(ref name) => {
                println!("Found unknown element: {}", name);
            }
            _ => {
                println!("{} is a standard HTML element", tag);
            }
        }
    }

    // Example 5: Case insensitive parsing
    println!("\n=== Case insensitive parsing ===");
    let mixed_case_tags = vec!["DIV", "Span", "H1", "BUTTON"];

    for tag in mixed_case_tags {
        let element = HtmlElement::from_tag_name(tag);
        println!(
            "{} -> {:?} (tag name: {})",
            tag,
            element,
            element.tag_name()
        );
    }
}
