use serde::Serialize;

#[derive(Debug, Serialize)]
struct Element {
    element_type: String,
    content: String,
    sub_elements: Vec<Element>,
}
impl Element {
    fn new(element_type: &str, content: &str) -> Self {
        Element {
            element_type: element_type.to_string(),
            content: content.to_string(),
            sub_elements: Vec::new(),
        }
    }
    fn new_without_content(element_type: &str, sub_elements: Vec<Element>) -> Self {
        Element {
            element_type: element_type.to_string(),
            content: "".to_string(),
            sub_elements,
        }
    }
    fn md_to_html(md_symbol: &str) -> &str {
        match md_symbol {
            "*" => "em",
            "**" => "strong",
            "~~" => "del",
            _ => "span",
        }
    }
}

fn find_outer_symbols(markdown: &str) -> Vec<(String, usize, usize)> {
    let symbols = vec!["**", "*", "~~"];
    let mut stack: Vec<(String, usize)> = Vec::new();
    let mut pairs: Vec<(String, usize, usize)> = Vec::new();

    let mut i = 0;
    while i < markdown.len() {
        for &sym in &symbols {
            if markdown[i..].starts_with(sym) {
                if let Some((last_sym, start)) = stack.last().cloned() {
                    if last_sym == sym {
                        stack.pop(); // closing
                        pairs.push((sym.to_string(), start, i));
                        i += sym.len() - 1;
                        break;
                    }
                }

                stack.push((sym.to_string(), i)); // push new symbol
                i += sym.len() - 1;
                break;
            }
        }
        i += 1;
    }

    // filter outermost symbols
    pairs
        .clone()
        .into_iter()
        .filter(|(_sym, open, close)| !pairs.iter().any(|(_, o, c)| *open > *o && *close < *c))
        .collect()
}

fn parse_in_element(text: &str) -> Vec<Element> {
    // parses within an element for things like bold, italics, etc.
    let mut elements = Vec::new();

    let outer_md_elements = find_outer_symbols(&text);

    if outer_md_elements.is_empty() {
        elements.push(Element::new("span", &text));
        return elements;
    }

    let first_md_elem_start = &outer_md_elements[0].1;
    let pre_text = &text[0..*first_md_elem_start];
    if !pre_text.is_empty() {
        elements.push(Element::new("span", pre_text));
    }

    for (i, md_element) in outer_md_elements.iter().enumerate() {
        elements.push(Element::new_without_content(
            Element::md_to_html(&md_element.0),
            parse_in_element(&text[md_element.1 + md_element.0.len()..md_element.2]), // after the element open to before the element close
        ));

        // if this isn't the last element
        if i == outer_md_elements.len() - 1 {
            continue;
        }
        // if there is no gap
        let next_element = &outer_md_elements[i + 1];
        if md_element.2 + 1 == next_element.1 {
            continue;
        }
        elements.push(Element::new("span", &text[md_element.2..next_element.1]));
    }

    let last_md_elem_end = &outer_md_elements[outer_md_elements.len() - 1].2
        + outer_md_elements[outer_md_elements.len() - 1].0.len(); // after the last element close to the end
    let post_text = &text[last_md_elem_end..text.len()];
    if !post_text.trim().is_empty() {
        elements.push(Element::new("span", post_text));
    }

    elements
}

fn parse_line(line: &str) -> Vec<Element> {
    let mut elements = Vec::new();

    if line.starts_with("# ") {
        elements.push(Element::new_without_content(
            "h1",
            parse_in_element(&line[2..]),
        ));
    } else if line.starts_with("## ") {
        elements.push(Element::new_without_content(
            "h2",
            parse_in_element(&line[3..]),
        ));
    } else if line.starts_with("### ") {
        elements.push(Element::new_without_content(
            "h3",
            parse_in_element(&line[4..]),
        ));
    } else if line.starts_with("#### ") {
        elements.push(Element::new_without_content(
            "h4",
            parse_in_element(&line[5..]),
        ));
    } else if line.starts_with("##### ") {
        elements.push(Element::new_without_content(
            "h5",
            parse_in_element(&line[6..]),
        ));
    } else if line.starts_with("###### ") {
        elements.push(Element::new_without_content(
            "h6",
            parse_in_element(&line[7..]),
        ));
    } else if line.starts_with("> ") {
        elements.push(Element::new_without_content(
            "blockquote",
            parse_in_element(&line[2..]),
        ));
    } else {
        // default
        elements.push(Element::new_without_content("span", parse_in_element(line)));
    }

    if line.ends_with("  ") {
        elements.push(Element::new_without_content("br", Vec::new()));
    }

    elements
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn parse_md(content: String) -> Vec<Element> {
    let mut elements = Vec::new();

    for line in content.lines() {
        let clean_line = line.trim_start(); // remove any whitespace at the beginning
        elements.push(Element::new_without_content("span", parse_line(clean_line)));
    }

    // println!("{:?}", elements);
    elements
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![parse_md])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
