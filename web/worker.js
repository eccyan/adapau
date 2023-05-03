// importScripts("https://unpkg.com/@rustwasm/wasm-bindgen");

(async () => {
  const { default: play_ogg_files } = await import("./pkg/adapau.js");

  self.addEventListener("message", async (event) => {
    const { audioFiles } = event.data;
    const audioDataArray = await Promise.all(
      audioFiles.map(async (url) => {
        const response = await fetch(url);
        const data = await response.arrayBuffer();
        return data;
      })
    );
    play_ogg_files(audioDataArray);
    postMessage({ type: "done", buffer: buffer });
  });
})();
