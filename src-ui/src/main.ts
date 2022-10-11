import {invoke} from "@tauri-apps/api";

const $ = document.querySelector.bind(document)

window.addEventListener("DOMContentLoaded", async () => {
	const searchBtnEl = $("#search-btn");
	const generateBtnEl = $("#generate-btn");
	const setupBtnEl = $("#setup-btn");
	const expandBtnEl = $("#expand-btn");
	const collapseBtnEl = $("#collapse-btn");
	const copyClipBtnEl = $("#send-to-clipboard");
	const randomizeBtnEl = $("#randomize-message");
	const clearBtnEl = $("#clear-message-editor");
	
	async function search() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		let searchInput = $('#message-search-input') as HTMLInputElement
		let messageList = $('#message-list')
		const messages = await invoke("search", {search_str: searchInput.value});
	}
});


