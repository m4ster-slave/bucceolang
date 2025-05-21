import init, { greet } from "../pkg/bucceolang.js";

let waitingForInput = false;
let inputResolve = null;

// Print to terminal
function terminalPrint(text) {
  const out = document.getElementById("terminalOutput");
  out.textContent += text + "\n";
  out.scrollTop = out.scrollHeight;
  document.getElementById("terminalWindow").style.display = "";
}

// Handle read() from interpreter
function terminalRead(prompt) {
  terminalPrint(prompt || "Input:");
  const input = document.getElementById("terminalInput");
  input.style.display = "";
  input.focus();
  waitingForInput = true;
  return new Promise((resolve) => {
    inputResolve = resolve;
  });
}

// Listen for user input in terminal
document.getElementById("terminalInput").addEventListener("keydown", (e) => {
  if (e.key === "Enter" && waitingForInput) {
    waitingForInput = false;
    const val = e.target.value;
    e.target.value = "";
    e.target.style.display = "none";
    terminalPrint("> " + val);
    if (inputResolve) inputResolve(val);
  }
});

// Run code using WASM interpreter
async function run() {
  await init();
  document.getElementById("terminalOutput").textContent = "";
  document.getElementById("terminalWindow").style.display = "";
  const input = document.getElementById("editor").value;

  // Wrap JS callbacks for WASM
  const print_callback = (msg) => terminalPrint(msg);
  const input_callback = async (prompt) => await terminalRead(prompt);

  // Call the exposed WASM function (adjust name as needed)
  await window.bucceolang.run_code_with_io(
    input,
    print_callback,
    input_callback,
  );

  // Example usage of the suggested code
  const printCallback = (output) => {
    console.log(output); // Or update UI
  };

  const inputCallback = () => {
    return prompt("Input:"); // Or get input from UI
  };

  run_code_with_io(input, printCallback, inputCallback);
}

window.setExample = function (text) {
  document.getElementById("editor").value = text;
};

document.getElementById("runMenu").addEventListener("click", (e) => {
  e.preventDefault();
  run();
});

function updateClock() {
  const now = new Date();
  const h = now.getHours().toString().padStart(2, "0");
  const m = now.getMinutes().toString().padStart(2, "0");
  document.getElementById("taskbarClock").textContent = `${h}:${m}`;
}
setInterval(updateClock, 1000);
updateClock();
