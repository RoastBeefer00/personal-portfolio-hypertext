import init from "/wasm/bevy_snake.js";

// Initialize the WASM module
init()
  .then(() => {
    console.log("Bevy game loaded successfully");
    const loading = document.getElementById("bevy-loading");
    if (loading) {
      loading.remove();
    }
  })
  .catch((error) => {
    console.error("Failed to load Bevy game:", error);
    const loading = document.getElementById("bevy-loading");
    if (loading) {
      loading.textContent = "Failed to load game: " + error.message;
    }
  });
