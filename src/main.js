const { invoke } = window.__TAURI__.core;

let parent;
let isEditing = false;
let rawMarkdown = `# Duck Notes
This is a small note taking app for the web.  
For now, this use ~~editing~~ plain text documents quickly!  
It is a local saving system  
**Deploys *from* Netlify**
`;

function renderElement(element) {
  const { element_type, content, sub_elements } = element;

  const el = document.createElement(element_type);

  el.textContent = content;

  sub_elements.forEach((sub_element) => {
    const subEl = renderElement(sub_element);
    el.appendChild(subEl);
  });

  return el;
}

async function calculateNode(content) {
  let elements = await invoke("parse_md", { content: content });

  elements.forEach((element) => {
    const el = renderElement(element);

    parent.appendChild(el);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  parent = document.querySelector("#parent");

  calculateNode(rawMarkdown);

  parent.addEventListener("focus", () => {
    if (!isEditing) {
      isEditing = true;
      parent.textContent = rawMarkdown;
    }
  });

  parent.addEventListener("keypress", (e) => {
    if (isEditing && e.key == "Escape") {
      parent.blur(); // ik blur is deprecated but it works so ima keep it until it DOESNT
      isEditing = false;
      parent.textContent = "";
      calculateNode(rawMarkdown);
    }
  });
});
