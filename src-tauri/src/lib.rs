struct Element {
	element_type: String,
	content: String,
	children: Vec<Element>
}
impl Element {
	fn new(element_type: &str, content: &str, sub_elements: children: Vec<Element>) -> Self {
		Element {
			element_type: element_type.to_string(),
			content: content.to_string(),
			children: children
		}
	}
	fn new(element_type: &str, sub_elements: children: Vec<Element>) -> Self {
		Element {
			element_type: element_type.to_string(),
			content: "".to_string(),
			children: children
		}
	}
}

fn parse_inline(text: &str) -> Vec<Element> { // parses within a line for things like bold, italics, etc.
  let mut elements = Vec::new();

  for character in text.chars() {

  }

  elements
}

fn parse_line(line: &str) -> Vec<Element> {
  let mut elements = Vec::new();

  if trimmed_line.starts_with("# ") {
    elements.push(Element::new("h1", parse_inline(&trimmed_line[2..])));
  } else if trimmed_line.starts_with("## ") {
    elements.push(Element::new("h2", parse_inline(&trimmed_line[3..])));
  } else if trimmed_line.starts_with("### ") {
    elements.push(Element::new("h3", parse_inline(&trimmed_line[4..])));
  } else if trimmed_line.starts_with("#### ") {
    elements.push(Element::new("h4", parse_inline(&trimmed_line[5..])));
  } else if trimmed_line.starts_with("##### ") {
    elements.push(Element::new("h5", parse_inline(&trimmed_line[6..])));
  } else if trimmed_line.starts_with("###### ") {
    elements.push(Element::new("h6", parse_inline(&trimmed_line[7..])));
  } else if trimmed_line.starts_with("> ") {
    elements.push(Element::new("blockquote", parse_inline(&trimmed_line[2..])));
  } else { // default
    elements.push(Element::new("p", &parse_inline(trimmed_line)));
  }

  elements
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn parse_md(content: String) -> Vec<Vec<Element>> {
  let mut elements = Vec::new();

  for line in content.lines() {
    let clean_line = line.trim_start(); // remove any whitespace at the beginning
  }
  // if character is #*x then do header tag
  // if character is * then italic
  // if character is ** then bold
  // if character is > then blockquote
  // etc. for ol, ul, code, horizontal rules, links, strikethrough, and sub/superscript. might add more later

  // based on the markdown cheat sheet https://www.markdownguide.org/cheat-sheet/

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
