const { invoke } = window.__TAURI__.tauri;

let searchBtnEl;
let generateBtnEl;
let setupBtnEl;
let expandBtnEl;
let collapseBtnEl;
let copyClipBtnEl;
let randomizeBtnEl;
let clearBtnEl;

window.addEventListener("DOMContentLoaded", () => {
  searchBtnEl = document.querySelector("#message-search");
  generateBtnEl = document.querySelector("#generate-btn");
  setupBtnEl = document.querySelector("#setup-btn");
  expandBtnEl = document.querySelector("#expand-btn");
  collapseBtnEl = document.querySelector("#collapse-btn");
  copyClipBtnEl = document.querySelector("#send-to-clipboard");
  randomizeBtnEl = document.querySelector("#randomize-message");
  clearBtnEl = document.querySelector("#clear-message-editor");
});

async function search() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let searchInput = document.querySelector('#message-search-input')
  let messageList = document.querySelector('#message-list')
  const messages = await invoke("search", { search_str: searchInput.value });
}

window.search = search;
