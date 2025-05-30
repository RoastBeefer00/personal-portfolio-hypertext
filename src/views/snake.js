import init from "/wasm/bevy_snake.js";
init().catch((error) => {
  if (
    !error.message.startsWith(
      "Using exceptions for control flow, don't mind me. This isn't actually an error!",
    )
  ) {
    throw error;
  }
});
const elementToRemove = document.getElementById("bevy-loading");
elementToRemove.remove();
