import init, { run } from "../pkg/bucceolang.js";

// Print to terminal
function terminalPrint(text) {
  const out = document.getElementById("terminalOutput");
  // Split text into lines and add each as a div for proper styling
  text.split("\n").forEach((line) => {
    out.insertAdjacentHTML(
      "beforeend",
      `<div class="term-line">${escapeHtml(line)}</div>`,
    );
  });
  out.scrollTop = out.scrollHeight;
  document.getElementById("terminalWindow").style.display = "";
}

// Escape HTML for terminal output
function escapeHtml(str) {
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

document.getElementById("runMenu").addEventListener("click", (e) => {
  e.preventDefault();
  run_bl();
});

async function run_bl() {
  await init();
  document.getElementById("terminalOutput").innerHTML = "";
  document.getElementById("terminalWindow").style.display = "";
  const input = document.getElementById("editor").value;

  const output = run(input);
  terminalPrint(output);
}
