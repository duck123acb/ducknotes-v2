const { invoke } = window.__TAURI__.core;

async function style(content, selectionStart, selectionEnd, styleType) {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  return await invoke("style", { content: content, selectionStart: selectionStart, selectionEnd: selectionEnd, styleType: styleType });
}

window.addEventListener("DOMContentLoaded", () => {

});