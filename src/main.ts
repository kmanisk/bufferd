import { invoke } from "@tauri-apps/api/core";
// src/main.ts
const msg = document.getElementById('msg')!;
const btn = document.getElementById('ping')!;

msg.textContent = 'Frontend ready';

btn.addEventListener('click', async () => {
  // For now this just changes the DOM.
  // After Tauri init we can call Rust commands using @tauri-apps/api
  msg.textContent = 'Button clicked — ready to call backend';
});
