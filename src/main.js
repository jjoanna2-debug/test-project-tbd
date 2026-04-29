const button = document.querySelector("#practice-button");
const statusMessage = document.querySelector("#status-message");

function showPracticeMessage() {
  const timestamp = new Date().toLocaleTimeString([], {
    hour: "2-digit",
    minute: "2-digit",
  });

  statusMessage.textContent = `Tiny check passed at ${timestamp}. GitHub survived another commit.`;
}

button?.addEventListener("click", showPracticeMessage);
