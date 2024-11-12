struct Element {
	element_type: String,
	content: String,
}
impl Element {
	fn new(element_type: &str, content: &str) -> Self {
		Element {
			element_type: element_type.to_string(),
			content: content.to_string(),
		}
	}
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn calculate_node(content: String) -> Vec<String> {
  let mut elements = Vec::new();

  // for character in content
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
    .invoke_handler(tauri::generate_handler![calculate_node])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}