// importScripts("https://unpkg.com/@rustwasm/wasm-bindgen");
const { default: init, play_ogg_files } = await import("./pkg/adapau.js");
await init();

const urlInput = document.getElementById("ogg-url");
const playButton = document.getElementById("play-btn");

playButton.addEventListener("click", () => {
  urlInput.setAttribute("disabled", "disabled");
  playButton.setAttribute("disabled", "disabled");
  const old = playButton.document;
  playButton.document = "Playing...";

  const audioFiles = [urlInput.value];
  play_ogg_files(audioFiles);

  playButton.setAttribute("disabled", "enabled");
  playButton.document = old;
});