# Lox Interpreter in Rust

Ein Lox-Interpreter geschrieben in Rust, inspiriert vom Buch [_Crafting Interpreters_](https://craftinginterpreters.com/).  
**Status:** ⚠️ _Noch in Entwicklung!_

## ✅ Bisher implementierte Kernfunktionen

- [x] Scanner (Tokenisierung)
- [ ] Parser
- [ ] Statische Analyse
- [ ] Intermediate Representation (IR)
- [ ] (Optimierungen)
- [ ] Codegenerierung
- [ ] (Bytecode Virtual Machine)

## ▶️ Nutzung

Derzeit unterstützt das CLI-Tool nur das Scannen (Tokenisieren) von `.lox` Dateien:

```bash
$ ./your_program.sh tokenize test.lox
```

## 📚 Sprachspezifikation

Die Lox-Spezifikation findest du hier:  
➡️ [Offizielle Dokumentation](https://craftinginterpreters.com/the-lox-language.html)

## 📂 Projektdokumentation & Diagramme

Zur weiteren Orientierung und Projektplanung sind folgende Dokumente verfügbar:

### 📄 Dokumentation

- [📜 Pflichtenheft](./assets/007%20%20%20pflichtenheft.md)
- [🧱 Abstrakte Struktur](./assets/003%20%20%20abstract.md)
- [📊 Diagrammübersicht](./assets/004%20%20%20diagramms.md)
- [📅 Besprechungsprotokoll](./assets/002%20%20%20protokoll.md)

### 🧩 Diagramme

- [📌 Klassendiagramm](./assets/Klassendiagramm.png)
- [🎯 Use-Case-Diagramm](./assets/008%20%20%20Use_Case_Diagramm.png)

### 🖼️ Logos & Grafiken

- [🖼️ Logo PNG](./assets/001%20%20%20logo.png)
- [🖼️ Logo JPEG](./assets/006%20%20%20logo.jpeg)

---

## 📖 Verwendete Ressourcen

- 📘 [_Crafting Interpreters_ – Robert Nystrom](https://craftinginterpreters.com/)
- 💻 [CodeCrafters Interpreter-Kurs](https://app.codecrafters.io/courses/interpreter/introduction)
