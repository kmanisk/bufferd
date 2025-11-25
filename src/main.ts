// import { invoke } from "@tauri-apps/api/core";
// // src/main.ts
// // const msg = document.getElementById('msg')!;
// // const btn = document.getElementById('ping')!;
// //
// // msg.textContent = 'Frontend ready';
// //
// // btn.addEventListener('click', async () => {
// //   // For now this just changes the DOM.
// //   // After Tauri init we can call Rust commands using @tauri-apps/api
// //   msg.textContent = 'Button clicked — ready to call backend';
// // import { invoke } from "@tauri-apps/api/tauri";
//
// let clipboardHistory: string[] = [];
// const list = document.getElementById("clipboard-list");
//
// async function updateClipboard() {
//   const text: string = await invoke("get_clipboard");
//   if (text && clipboardHistory[0] !== text) {
//     clipboardHistory.unshift(text);
//     if (clipboardHistory.length > 20) clipboardHistory.pop();
//     renderList();
//   }
// }
//
// function renderList() {
//   if (!list) return;
//   list.innerHTML = "";
//   clipboardHistory.forEach((entry) => {
//     const div = document.createElement("div");
//     div.className = "entry";
//     div.textContent = entry;
//     div.onclick = async () => {
//       await invoke("write_clipboard", { text: entry });
//       alert("Copied back to clipboard!");
//     };
//     list.appendChild(div);
//   });
// }
//
// // Poll clipboard every 500ms
// setInterval(updateClipboard, 500);
//
//
import { invoke } from "@tauri-apps/api/core";
// src/main.ts
// const msg = document.getElementById('msg')!;
// const btn = document.getElementById('ping')!;
//
// msg.textContent = 'Frontend ready';
//
// btn.addEventListener('click', async () => {
//   // For now this just changes the DOM.
//   // After Tauri init we can call Rust commands using @tauri-apps/api
//   msg.textContent = 'Button clicked — ready to call backend';
// import { invoke } from "@tauri-apps/api/tauri";

let clipboardHistory: string[] = [];
const list = document.getElementById("clipboard-list");

async function updateClipboard() {
  const text: string = await invoke("get_clipboard");
  if (text && clipboardHistory[0] !== text) {
    clipboardHistory.unshift(text);
    if (clipboardHistory.length > 20) clipboardHistory.pop();
    console.log(text);
    renderList();
  }
}

function renderList() {
  if (!list) return;
  list.innerHTML = "";
  clipboardHistory.forEach((entry) => {
    const div = document.createElement("div");
    div.className = "entry";
    div.textContent = entry;
    div.onclick = async () => {
      await invoke("write_clipboard", { text: entry });
      alert("Copied back to clipboard!");
    };
    list.appendChild(div);
  });
}

// Poll clipboard every 500ms
setInterval(updateClipboard, 500);