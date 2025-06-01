window.setExample = function (example) {
  let text = "";
  switch (example) {
    case 1:
      text = 'print "Hello, world!";';
      break;
    case 2:
      text = `
fn greet(name) {
  print "Hello from " + name + "!";
}

greet("Bucceolang");
`.trim();
      break;
    case 3:
      text = `
for (var i = 1; i <= 5; i = i + 1) {
  print i * i;
}
`.trim();
      break;
    case 4:
      text = `
class Person {
  fn init(name) {
    this.name = name;
  }

  fn greet() {
    print "Hi, I'm " + this.name + ".";
  }
}

var alice = Person("Tobias");
alice.greet();
`.trim();
      break;

    case 5:
      text = `
fn makeCounter() {
  var i = 0;
  fn count() {
    i = i + 1;
    print i;
  }
  return count;
}
var counter = makeCounter();
counter(); 
counter();
`.trim();
      break;
    case 6:
      text = `
var a = "global";
{
  fn showA() {
    print a;
  }

  showA();
  var a = "block";
  showA();
}
`.trim();
      break;
    case 7:
      text = `
fn fib(n) {
    if (n <= 1) return n;
    return fib(n - 2) + fib(n - 1);
}

for (var i = 0; i < 10; i = i + 1) {
    print fib(i);
}
`.trim();
      break;
    case 8:
      text = `
class Node {
    fn init(value) {
        this.value = value;
        this.next = nil;
    }
}

class LinkedList {
    fn init() {
        this.head = nil;
    }

    fn add(value) {
        var node = Node(value);
        if (this.head == nil) {
            this.head = node;
        } else {
            var current = this.head;
            while (current.next != nil) {
                current = current.next;
            }
            current.next = node;
        }
    }

    fn print_list() {
        var current = this.head;
        while (current != nil) {
            print current.value;
            current = current.next;
        }
    }
}

var list = LinkedList();
list.add(1);
list.add(2);
list.add(3);
list.print_list();

`.trim();
      break;
    default:
      text = 'print "Invalid example selected.";';
  }
  document.getElementById("editor").value = text;
};

function updateClock() {
  const now = new Date();
  const h = now.getHours().toString().padStart(2, "0");
  const m = now.getMinutes().toString().padStart(2, "0");
  const d = now.getDate().toString().padStart(2, "0");
  const mo = (now.getMonth() + 1).toString().padStart(2, "0"); // Months are zero-based
  const y = now.getFullYear();

  const time = `${h}:${m}`;
  const date = `${d}/${mo}/${y}`;

  const clockElement = document.getElementById("taskbarClock");
  clockElement.textContent = `${time}\n${date}`;
  clockElement.style.whiteSpace = "pre"; // Ensures that \n is rendered as a line break
}
setInterval(updateClock, 1000);
updateClock();

document.getElementById("clearInput").addEventListener("click", (e) => {
  e.preventDefault();

  document.getElementById("editor").value = "";
});
