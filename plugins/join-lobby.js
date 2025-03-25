var css = `
  .tenhout-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.85);
    z-index: 999;
    display: flex;
    justify-content: center;
    align-items: center;
  }
`;

function injectCSS() {
  const style = document.createElement("style");
  style.id = "tenhout-style";
  style.textContent = css;
  document.head.appendChild(style);
}

function uninjectCSS() {
  const style = document.getElementById("tenhout-style");
  if (style) {
    style.remove();
  }
}

function createBackdrop() {
  const backdrop = document.createElement("div");
  backdrop.className = "tenhout-backdrop";

  // Add text input and button to backdrop
  const input = document.createElement("input");
  input.id = "tenhout-lobby-number";
  input.type = "text";
  input.placeholder = "Enter lobby code";

  const joinButton = document.createElement("button");
  joinButton.className = "tenhout-button";
  joinButton.textContent = "Join";
  joinButton.onclick = handleInput;

  const cancelButton = document.createElement("button");
  cancelButton.className = "tenhout-button";
  cancelButton.textContent = "Cancel";
  cancelButton.onclick = teardown;

  backdrop.appendChild(input);
  backdrop.appendChild(joinButton);
  backdrop.appendChild(cancelButton);

  document.body.appendChild(backdrop);
}

function removeBackdrop() {
  const backdrop = document.querySelector(".tenhout-backdrop");
  if (backdrop) {
    backdrop.remove();
  }
}

function handleInput() {
  const input = document.getElementById("tenhout-lobby-number");
  if (input) {
    const lobbyNumber = input.value.trim().toUpperCase();
    console.log(lobbyNumber);
    if (lobbyNumber) {
      const currentUrl = window.location.href;
      const newUrl = `${currentUrl}/?${lobbyNumber}`;
      window.location.href = newUrl;
    }
  }
}

function setup() {
  injectCSS();
  createBackdrop();
}

function teardown() {
  removeBackdrop();
  uninjectCSS();
}

// Run setup
setup();
