const { invoke } = window.__TAURI__.core;

async function calculateNode(content) {
  let elements = await invoke("calculate_node", { content: content })
}

window.addEventListener("DOMContentLoaded", () => {

});