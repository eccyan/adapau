const worker = new Worker("worker.js");
const urlInput = document.getElementById("ogg-url");
const playButton = document.getElementById("play-btn");

playButton.addEventListener("click", () => {
  const url = urlInput.value;
  worker.postMessage({
    command: "execute",
    audioFiles: [url],
  });
});

worker.onmessage = (event) => {
  const { data } = event;
  if (data.type === "done") {
    console.log("done");
  }
};
