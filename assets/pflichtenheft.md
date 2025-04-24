# **Pflichtenheft: Entwicklung eines Interpreters in Rust**

## **1. Projektbeschreibung**

### **1.1 Ziel des Projekts**

Das Ziel dieses Projekts ist die Entwicklung eines robusten und erweiterbaren Interpreters für eine speziell definierte Programmiersprache. Der Interpreter soll in Rust implementiert werden, um von dessen Sicherheitsgarantien (z. B. Speichersicherheit ohne Garbage Collector) und hoher Performance zu profitieren.

Die zu interpretierende Sprache soll folgende Schlüsselmerkmale unterstützen:

- **Dynamische Typisierung**: Variablen können zur Laufzeit ihren Datentyp ändern.
- **Closures**: Funktionen sollen ihren Erstellungskontext (Lexical Scoping) behalten können.
- **Speicherverwaltung**: Effiziente Handhabung von Speicherallokation und -freigabe, ggf. mittels Referenzzählung (Reference Counting).

Der Interpreter soll als eigenständige Anwendung entwickelt werden und eine klare, dokumentierte Schnittstelle für Erweiterungen bieten.

### **1.2 Projektkontext**

Dieses Projekt entsteht im Kontext der Erforschung moderner Interpreter-Designs und der praktischen Anwendung der Programmiersprache Rust für Systemsoftware. Es dient als Grundlage für:

- **Lehre und Forschung**: Veranschaulichung von Interpreterkonzepten wie Parsing, Ausführung und Speicherverwaltung.
- **Prototyping**: Ermöglichung der schnellen Entwicklung und Evaluierung neuer Sprachfeatures.
- **Erweiterbarkeit**: Bereitstellung einer Basis für zukünftige Erweiterungen (z. B. JIT-Kompilierung, statische Typanalyse).

---

## **2. Leistungsbeschreibung**

### **2.1 Funktionale Anforderungen**

#### **2.1.1 Grundlegende Sprachfeatures**

- **Syntaxparsing**:
  - Der Interpreter soll eine klare, dokumentierte Grammatik unterstützen.
  - Fehlermeldungen müssen aussagekräftig und positionsgenau sein.
- **Dynamische Typisierung**:
  - Unterstützung grundlegender Datentypen (Integer, Float, Boolean, String, Nil).
  - Implizite Typkonvertierung bei Operationen (z. B. `"Text" + 3 → "Text3"`).
- **Variablen & Scoping**:
  - Lexikalische Gültigkeitsbereiche (Block-Scoping).
  - Globale und lokale Variablen.
- **Kontrollstrukturen**:
  - Bedingungen (`if-else`).
  - Schleifen (`while`, `for`).
  - `break`/`continue` in Schleifen.
- **Funktionen & Closures**:
  - Definition und Aufruf von Funktionen.
  - Unterstützung von First-Class-Functions.
  - Closures mit korrektem Lexical Scoping.
- **Speicherverwaltung**:
  - Automatische Speicherbereinigung (z. B. via `Rc<RefCell<T>>` oder Arena-Allocation).
  - Keine Memory Leaks bei zyklischen Referenzen (falls möglich).

#### **2.1.2 Standardbibliothek & Ein-/Ausgabe**

- Grundlegende I/O-Operationen (`print`, `read_line`).
- Mathematische Funktionen (`sin`, `sqrt`, `random`).
- Listen- und Map-Operationen (`push`, `get`, `len`).

#### **2.1.3 Erweiterbarkeit**

- Möglichkeit zur Integration nativer Rust-Funktionen (FFI).
- Klare API für Sprach- und Bibliothekserweiterungen.

### **2.2 Nicht-funktionale Anforderungen**

- **Performance**:
  - Effiziente Ausführung von Schleifen und rekursiven Funktionen.
  - Geringe Latenz beim Starten des Interpreters.
- **Robustheit**:
  - Keine Abstürze bei ungültigen Eingaben (graceful error handling).
  - Memory-Safety durch Rust garantiert.
- **Portabilität**:
  - Lauffähig auf Windows, Linux und macOS.
  - Keine plattformspezifischen Abhängigkeiten.
- **Dokumentation**:
  - Ausführliche Code-Dokumentation mit `rustdoc`.
  - Benutzerhandbuch für die Programmiersprache.

### **2.3 Optionale Features (wenn Zeit bleibt)**

- **REPL (Read-Eval-Print-Loop)** für interaktives Testen.
- **Debugging-Unterstützung** (Breakpoints, Variableninspektion).
- **Optimierungen** (Constant Folding, Basic JIT-Compilation).

---

## **3. Systemkontext und Abgrenzung**

### **3.1 Systemkontext**

- **Eingabequellen**:
  - Skriptdateien (`.script` oder ähnlich).
  - Direkte Eingabe über Kommandozeile (falls REPL implementiert).
- **Ausgabe**:
  - Terminal (stdout/stderr).
  - Optional: Logging in Dateien.
- **Externe Abhängigkeiten**:
  - Rust-Standardbibliothek.
  - Ggf. externe Crates für Parsing (`nom`, `lalrpop`) oder Speicherverwaltung.

### **3.2 Abgrenzung**

- **Kein Compiler**: Der Interpreter führt Code direkt aus, keine Maschinencode-Generierung.
- **Keine IDE**: Keine integrierte Entwicklungsumgebung, aber Kompatibilität mit Texteditoren.
- **Keine Netzwerkfunktionen**: Standardbibliothek beschränkt sich auf lokale Ein-/Ausgabe.

---

## **4. Technische Anforderungen**

### **4.1 Entwicklungsumgebung**

- **Programmiersprache**: Rust (stable toolchain, mind. Version 1.70+)
- **Build-System**: Cargo
- **Externe Abhängigkeiten**:
  - Parsing: `nom` oder `lalrpop` (Parser-Kombinatoren bzw. LR-Parsing)
  - Speicherverwaltung: `Rc<RefCell<T>>` oder Arena-Allokatoren (`bumpalo`)
  - Testing: `libtest` (integrierte Testumgebung), ggf. Property-Based Testing mit `proptest`
  - Logging: `log` + `env_logger` (für Debugging)

### **4.2 Zielplattformen**

- **Betriebssysteme**:
  - Linux (x86_64, ARM)
  - Windows (x86_64, MSVC/MinGW)
  - macOS (x86_64, ARM)
- **Hardware**: Keine speziellen Anforderungen (keine GPU-Abhängigkeit)

### **4.3 Tooling & Infrastruktur**

- **Versionskontrolle**: Git (Hosting auf GitHub/GitLab)
- **CI/CD**: GitHub Actions (Automatisiertes Testen/Linting)
- **Dokumentation**:
  - `rustdoc` für API-Dokumentation
  - `mdBook` für Benutzerhandbuch

---

## **5. Zeitplan & Meilensteine**

### **5.1 Grobe Projektphasen**

1. **Vorbereitung (1 Woche)**

   - Anforderungsanalyse
   - Technologieauswahl
   - Grundgerüst des Projekts (`cargo new`)

2. **Lexer & Parser (2 Wochen)**

   - Implementierung des Tokenizers
   - Entwicklung des AST (Abstract Syntax Tree)
   - Fehlerbehandlung (Syntaxfehler)

3. **Ausführungskern (3 Wochen)**

   - Interpretation des AST
   - Variablenverwaltung & Scoping
   - Funktionsaufrufe & Closures

4. **Speicherverwaltung (2 Wochen)**

   - Garbage Collection oder Referenzzählung
   - Tests auf Memory Leaks

5. **Standardbibliothek & I/O (1 Woche)**

   - Grundlegende Funktionen (`print`, `read`, mathematische Operationen)

6. **Optimierung & Dokumentation (1 Woche)**
   - Performance-Tuning
   - Vollständige Dokumentation

### **5.2 Meilensteine**

- **M1** (Ende Woche 1): Projektsetup, Spezifikation finalisiert
- **M2** (Ende Woche 3): Funktionierender Parser (AST-Generierung)
- **M3** (Ende Woche 6): Lauffähiger Interpreter mit Variablen & Funktionen
- **M4** (Ende Woche 8): Stabiler Release mit Speicherverwaltung
- **M5** (Ende Woche 9): Finale Version mit Dokumentation

---

## **6. Risikomanagement**

### **6.1 Potenzielle Risiken & Gegenmaßnahmen**

| **Risiko**                              | **Wahrscheinlichkeit** | **Auswirkung** | **Gegenmaßnahme**                             |
| --------------------------------------- | ---------------------- | -------------- | --------------------------------------------- |
| Unklarheiten in der Sprachspezifikation | Mittel                 | Hoch           | Prototyping & frühzeitige Abnahme             |
| Performance-Probleme bei Recursion      | Hoch                   | Mittel         | Tail-Call-Optimierung oder Stack-Management   |
| Speicherleaks durch Referenzzyklen      | Mittel                 | Hoch           | Manuelle Zyklenerkennung oder Weak-References |
| Unzureichende Testabdeckung             | Hoch                   | Hoch           | Automatisierte Tests + Property-Based Testing |
| Verzögerungen durch Rust-Lernkurve      | Niedrig                | Mittel         | Pair Programming & Code-Reviews               |

---

## **7. Glossar**

- **AST (Abstract Syntax Tree)**: Baumdarstellung der Programmstruktur nach dem Parsing.
- **Closure**: Funktion mit gebundenem lexikalischem Scope.
- **Dynamic Dispatch**: Laufzeitentscheidung über Methodenaufrufe (bei dynamischer Typisierung).
- **FFI (Foreign Function Interface)**: Schnittstelle zur Einbindung von Rust-Code in andere Sprachen (oder umgekehrt).
- **GC (Garbage Collection)**: Automatische Speicherbereinigung.
- **JIT (Just-In-Time-Compilation)**: Dynamische Kompilierung zur Laufzeit.
- **REPL (Read-Eval-Print-Loop)**: Interaktive Eingabeumgebung für Code.
- **Rustc**: Der Rust-Compiler.

---
