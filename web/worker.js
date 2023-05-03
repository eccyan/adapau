// importScripts("https://unpkg.com/@rustwasm/wasm-bindgen");

(async () => {
  const { default: init, play_ogg_files } = await import("./pkg/adapau.js");

  await init();

  self.addEventListener("message", async (event) => {
    const { audioFiles } = event.data;
    await play_ogg_files(audioFiles);
    postMessage({ type: "play" });
  });
})();