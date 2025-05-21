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
